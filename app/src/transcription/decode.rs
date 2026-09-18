//! Pure token-selection logic for the autoregressive decode loop — no Burn,
//! no I/O — so it's testable with plain `cargo test` independent of the
//! actual model. Values from Moonshine Tiny's `generation_config.json`
//! (`UsefulSensors/moonshine-tiny`): bos/decoder_start = 1, eos = pad = 2,
//! max_length = 194.

pub const BOS_TOKEN_ID: i64 = 1;
pub const EOS_TOKEN_ID: i64 = 2;
pub const MAX_NEW_TOKENS: usize = 194;

/// Greedy decoding: picks the highest-scoring token. Moonshine Tiny is a
/// small enough model that beam search/sampling isn't worth the added
/// complexity for a first pass — revisit if greedy output turns out noisy.
pub fn argmax(logits: &[f32]) -> i64 {
    logits
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.total_cmp(b.1))
        .map(|(i, _)| i as i64)
        .unwrap_or(EOS_TOKEN_ID)
}

/// Stop once the model emits end-of-sequence, or we hit the model's own
/// trained generation cap (longer than that and quality degrades — it's
/// not just a safety valve).
pub fn is_finished(token_id: i64, generated_so_far: usize) -> bool {
    token_id == EOS_TOKEN_ID || generated_so_far >= MAX_NEW_TOKENS
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn argmax_picks_highest_score() {
        assert_eq!(argmax(&[0.1, 0.9, 0.3]), 1);
        assert_eq!(argmax(&[5.0, 1.0]), 0);
    }

    #[test]
    fn argmax_on_empty_falls_back_to_eos() {
        assert_eq!(argmax(&[]), EOS_TOKEN_ID);
    }

    #[test]
    fn finished_on_eos_token() {
        assert!(is_finished(EOS_TOKEN_ID, 3));
        assert!(!is_finished(42, 3));
    }

    #[test]
    fn finished_at_max_new_tokens() {
        assert!(is_finished(42, MAX_NEW_TOKENS));
        assert!(!is_finished(42, MAX_NEW_TOKENS - 1));
    }
}
