//! Loads Moonshine Tiny (imported from ONNX by `burn-onnx`, see `build.rs`)
//! and runs the autoregressive encoder → decoder decode loop.
//!
//! Note: pinned to `burn`/`burn-onnx` 0.22.0-pre.3 (see Cargo.toml) — as of
//! that version the generated code (and `Tensor`/`Device` generally) is no
//! longer generic over a backend type; which backend is compiled in is
//! chosen entirely by Cargo feature (`burn`'s `"ndarray"` feature here).
//!
//! `burn-onnx`'s generated `Model::default()` hardcodes an absolute path
//! into this machine's `target/` build directory (fine for `cargo test` on
//! this Mac, useless on a phone). So instead: the compiled weights
//! (`.bpk` files) are baked into the binary with `include_bytes!`, written
//! out to a per-device cache file on first use, and loaded from there via
//! `Model::from_file` — sidesteps needing to hand-construct the crate's
//! internal `Bytes` wrapper type just to call `Model::from_bytes` directly.

use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use burn::prelude::*;
use tokenizers::Tokenizer;

use super::decode::{argmax, is_finished, BOS_TOKEN_ID};

const NUM_LAYERS: usize = 6;
const NUM_HEADS: usize = 8;
const HEAD_DIM: usize = 36;

// burn-onnx's generated code (unsimplified arithmetic like `4usize - 4usize
// + i`, redundant casts, unused from_bytes/new/forward helpers per layer)
// isn't held to our own lint bar — it's regenerated from the ONNX graph on
// every build, not something we hand-edit.
#[allow(clippy::all, dead_code)]
mod encoder {
    include!(concat!(env!("OUT_DIR"), "/moonshine/encoder_model.rs"));
}
#[allow(clippy::all, dead_code)]
mod decoder {
    include!(concat!(env!("OUT_DIR"), "/moonshine/decoder_model_merged.rs"));
}

const ENCODER_WEIGHTS: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/moonshine/encoder_model.bpk"));
const DECODER_WEIGHTS: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/moonshine/decoder_model_merged.bpk"));
const TOKENIZER_JSON: &[u8] =
    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/models/moonshine-tiny/tokenizer.json"));

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TranscriptionError {
    ModelLoadFailed,
    TokenizerLoadFailed,
}

impl std::fmt::Display for TranscriptionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::ModelLoadFailed => "failed to load the transcription model",
            Self::TokenizerLoadFailed => "failed to load the tokenizer",
        };
        f.write_str(msg)
    }
}

struct Models {
    encoder: encoder::Model,
    decoder: decoder::Model,
    tokenizer: Tokenizer,
    device: Device,
}

static MODELS: OnceLock<Result<Models, TranscriptionError>> = OnceLock::new();

fn cache_file(name: &str) -> PathBuf {
    crate::storage::wav::recordings_dir()
        .parent()
        .map(|dir| dir.join("model_cache"))
        .unwrap_or_else(std::env::temp_dir)
        .join(name)
}

fn write_once(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    let existing_len = std::fs::metadata(path).ok().map(|m| m.len() as usize);
    if existing_len == Some(bytes.len()) {
        return Ok(());
    }
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    std::fs::write(path, bytes)
}

fn load_models() -> Result<Models, TranscriptionError> {
    let device = Device::default();

    let encoder_path = cache_file("encoder_model.bpk");
    let decoder_path = cache_file("decoder_model_merged.bpk");
    write_once(&encoder_path, ENCODER_WEIGHTS).map_err(|_| TranscriptionError::ModelLoadFailed)?;
    write_once(&decoder_path, DECODER_WEIGHTS).map_err(|_| TranscriptionError::ModelLoadFailed)?;

    let encoder = encoder::Model::from_file(&encoder_path, &device);
    let decoder = decoder::Model::from_file(&decoder_path, &device);
    let tokenizer =
        Tokenizer::from_bytes(TOKENIZER_JSON).map_err(|_| TranscriptionError::TokenizerLoadFailed)?;

    Ok(Models { encoder, decoder, tokenizer, device })
}

fn models() -> Result<&'static Models, TranscriptionError> {
    MODELS.get_or_init(load_models).copied_err()
}

/// `Result<&T, E: Copy>` doesn't have a clean way to turn `&Result<T, E>`
/// into `Result<&T, E>` without cloning `T` — this is that conversion.
trait CopiedErr<T, E: Copy> {
    fn copied_err(&self) -> Result<&T, E>;
}

impl<T, E: Copy> CopiedErr<T, E> for Result<T, E> {
    fn copied_err(&self) -> Result<&T, E> {
        match self {
            Ok(value) => Ok(value),
            Err(err) => Err(*err),
        }
    }
}

/// A decoder layer's self-attention (decoder_*) and cross-attention
/// (encoder_*) key/value cache, matching the ONNX graph's per-layer naming.
#[derive(Clone)]
struct LayerCache {
    decoder_key: Tensor<4>,
    decoder_value: Tensor<4>,
    encoder_key: Tensor<4>,
    encoder_value: Tensor<4>,
}

impl LayerCache {
    fn empty(device: &Device) -> Self {
        let empty = || Tensor::<4>::zeros([1, NUM_HEADS, 0, HEAD_DIM], device);
        Self {
            decoder_key: empty(),
            decoder_value: empty(),
            encoder_key: empty(),
            encoder_value: empty(),
        }
    }
}

/// Runs Moonshine Tiny end to end on mono 16kHz samples (exactly what
/// `Recorder::take_samples` produces) and returns the transcribed text.
pub fn transcribe(samples: &[f32]) -> Result<String, TranscriptionError> {
    let models = models()?;
    let device = &models.device;

    let input_values =
        Tensor::<2>::from_data(TensorData::new(samples.to_vec(), [1, samples.len()]), device);
    let encoder_hidden_states = models.encoder.forward(input_values);

    let mut caches: [LayerCache; NUM_LAYERS] = std::array::from_fn(|_| LayerCache::empty(device));
    let mut generated_ids: Vec<i64> = Vec::new();
    let mut next_input_id = BOS_TOKEN_ID;
    let mut use_cache_branch = false;

    loop {
        let input_ids =
            Tensor::<2, Int>::from_data(TensorData::new(vec![next_input_id], [1, 1]), device);
        let use_cache_tensor = Tensor::<1, Bool>::from_bool([use_cache_branch], device);

        let [c0, c1, c2, c3, c4, c5] = &caches;
        let (
            logits,
            k0d, v0d, k0e, v0e,
            k1d, v1d, k1e, v1e,
            k2d, v2d, k2e, v2e,
            k3d, v3d, k3e, v3e,
            k4d, v4d, k4e, v4e,
            k5d, v5d, k5e, v5e,
        ) = models.decoder.forward(
            input_ids,
            encoder_hidden_states.clone(),
            c0.decoder_key.clone(), c0.decoder_value.clone(), c0.encoder_key.clone(), c0.encoder_value.clone(),
            c1.decoder_key.clone(), c1.decoder_value.clone(), c1.encoder_key.clone(), c1.encoder_value.clone(),
            c2.decoder_key.clone(), c2.decoder_value.clone(), c2.encoder_key.clone(), c2.encoder_value.clone(),
            c3.decoder_key.clone(), c3.decoder_value.clone(), c3.encoder_key.clone(), c3.encoder_value.clone(),
            c4.decoder_key.clone(), c4.decoder_value.clone(), c4.encoder_key.clone(), c4.encoder_value.clone(),
            c5.decoder_key.clone(), c5.decoder_value.clone(), c5.encoder_key.clone(), c5.encoder_value.clone(),
            use_cache_tensor,
        );

        caches = [
            LayerCache { decoder_key: k0d, decoder_value: v0d, encoder_key: k0e, encoder_value: v0e },
            LayerCache { decoder_key: k1d, decoder_value: v1d, encoder_key: k1e, encoder_value: v1e },
            LayerCache { decoder_key: k2d, decoder_value: v2d, encoder_key: k2e, encoder_value: v2e },
            LayerCache { decoder_key: k3d, decoder_value: v3d, encoder_key: k3e, encoder_value: v3e },
            LayerCache { decoder_key: k4d, decoder_value: v4d, encoder_key: k4e, encoder_value: v4e },
            LayerCache { decoder_key: k5d, decoder_value: v5d, encoder_key: k5e, encoder_value: v5e },
        ];

        let vocab_size = logits.dims()[2];
        let logits_vec: Vec<f32> = logits
            .reshape([vocab_size])
            .into_data()
            .try_to_vec::<f32>()
            .unwrap_or_default();
        let next_id = argmax(&logits_vec);

        if is_finished(next_id, generated_ids.len()) {
            break;
        }
        generated_ids.push(next_id);
        next_input_id = next_id;
        use_cache_branch = true;
    }

    let ids: Vec<u32> = generated_ids.iter().map(|&id| id as u32).collect();
    models
        .tokenizer
        .decode(&ids, true)
        .map_err(|_| TranscriptionError::TokenizerLoadFailed)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Loads the real ~110MB of weights and runs a full decode loop on
    /// silence. Not run by default (`cargo test` stays fast) — this is
    /// purely a smoke test that the encoder/decoder wiring and tensor
    /// shapes are correct, since `argmax` on a silent clip won't produce
    /// meaningful *text*. Run explicitly: `cargo test -- --ignored`.
    #[test]
    #[ignore]
    fn transcribe_silence_does_not_panic() {
        let one_second_of_silence = vec![0.0_f32; 16_000];
        let result = transcribe(&one_second_of_silence);
        println!("transcribe(silence) = {result:?}");
        assert!(result.is_ok(), "transcribe should not error on valid input: {result:?}");
    }
}
