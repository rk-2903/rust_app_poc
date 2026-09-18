//! Pure sample-rate conversion helpers. No I/O, no cpal — kept separate from
//! `recorder.rs` so it's testable with plain `cargo test`.

/// Averages interleaved multi-channel samples down to a single mono channel.
/// A no-op if `channels <= 1`.
pub fn downmix_to_mono(samples: &[f32], channels: u16) -> Vec<f32> {
    if channels <= 1 {
        return samples.to_vec();
    }
    let channels = channels as usize;
    samples
        .chunks(channels)
        .map(|frame| frame.iter().sum::<f32>() / frame.len() as f32)
        .collect()
}

/// Linear-interpolation resample. Good enough for speech (Moonshine expects
/// 16kHz mono); not a substitute for a proper sinc resampler if audio
/// quality ever matters beyond transcription.
pub fn resample_linear(input: &[f32], from_rate: u32, to_rate: u32) -> Vec<f32> {
    if from_rate == to_rate || input.is_empty() {
        return input.to_vec();
    }

    let ratio = from_rate as f64 / to_rate as f64;
    let out_len = ((input.len() as f64) / ratio).floor() as usize;

    (0..out_len)
        .map(|i| {
            let src_pos = i as f64 * ratio;
            let idx = src_pos.floor() as usize;
            let frac = src_pos - idx as f64;
            let a = input[idx];
            let b = input.get(idx + 1).copied().unwrap_or(a);
            (a as f64 + (b as f64 - a as f64) * frac) as f32
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn downmix_mono_is_noop() {
        let samples = vec![0.1, 0.2, 0.3];
        assert_eq!(downmix_to_mono(&samples, 1), samples);
    }

    #[test]
    fn downmix_stereo_averages_channels() {
        let samples = vec![1.0, -1.0, 0.5, 0.5];
        assert_eq!(downmix_to_mono(&samples, 2), vec![0.0, 0.5]);
    }

    #[test]
    fn resample_same_rate_is_noop() {
        let samples = vec![0.1, 0.2, 0.3];
        assert_eq!(resample_linear(&samples, 16_000, 16_000), samples);
    }

    #[test]
    fn resample_halves_length_when_downsampling_by_half() {
        let samples: Vec<f32> = (0..100).map(|i| i as f32).collect();
        let out = resample_linear(&samples, 32_000, 16_000);
        assert_eq!(out.len(), 50);
        // Downsampling by exactly 2x should land on every other input sample.
        assert_eq!(out[0], 0.0);
        assert_eq!(out[1], 2.0);
        assert_eq!(out[49], 98.0);
    }

    #[test]
    fn resample_interpolates_between_samples() {
        let samples = vec![0.0, 10.0, 20.0, 30.0];
        // 3 in -> 4 out is upsampling; ratio = 3/4 = 0.75 per output step.
        let out = resample_linear(&samples, 3, 4);
        assert_eq!(out.len(), 5);
        assert_eq!(out[0], 0.0);
        assert_eq!(out[1], 7.5);
    }
}
