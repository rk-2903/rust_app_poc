use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use hound::{SampleFormat, WavSpec, WavWriter};

/// Where recordings live on disk. iOS's app sandbox exposes its writable
/// home via `$HOME` (unlike the app bundle path `std::env::current_dir()`
/// would give you, which isn't writable) — `$HOME/Documents` is the
/// standard user-visible location for a user-facing document like a
/// recording. Desktop/simulator fall back to a temp directory since
/// there's no real device to persist across runs on anyway.
pub fn recordings_dir() -> PathBuf {
    let dir = platform_documents_dir().join("recordings");
    let _ = std::fs::create_dir_all(&dir);
    dir
}

#[cfg(target_os = "ios")]
fn platform_documents_dir() -> PathBuf {
    let home = std::env::var("HOME").expect("HOME not set on iOS");
    PathBuf::from(home).join("Documents")
}

#[cfg(not(target_os = "ios"))]
fn platform_documents_dir() -> PathBuf {
    std::env::temp_dir().join("conversation_capture")
}

/// Encodes mono `f32` samples as a 16-bit PCM WAV file and returns its path.
pub fn save_recording(samples: &[f32], sample_rate: u32) -> Result<PathBuf, String> {
    if samples.is_empty() {
        return Err("no audio captured".to_string());
    }

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let path = recordings_dir().join(format!("recording_{timestamp}.wav"));

    let spec = WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };
    let mut writer = WavWriter::create(&path, spec).map_err(|e| format!("create WAV failed: {e}"))?;
    for &sample in samples {
        let amplitude = (sample.clamp(-1.0, 1.0) * i16::MAX as f32) as i16;
        writer
            .write_sample(amplitude)
            .map_err(|e| format!("write sample failed: {e}"))?;
    }
    writer.finalize().map_err(|e| format!("finalize WAV failed: {e}"))?;

    Ok(path)
}
