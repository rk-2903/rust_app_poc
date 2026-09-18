//! Mic capture on a dedicated OS thread. The thread owns the `cpal::Stream`
//! for its whole lifetime (building it, playing it, and finally dropping it
//! to stop capture) so audio I/O never touches the UI thread.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{self, Sender};
use std::sync::Arc;
use std::thread::{self, JoinHandle};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{SampleFormat, StreamConfig};

use super::resample::{downmix_to_mono, resample_linear};

/// Sample rate the on-device transcription model expects (Phase 2).
pub const TARGET_SAMPLE_RATE: u32 = 16_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecorderError {
    NoInputDevice,
    NoSupportedConfig,
    BuildStreamFailed,
    PlayStreamFailed,
}

impl std::fmt::Display for RecorderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::NoInputDevice => "no microphone input device available",
            Self::NoSupportedConfig => "input device has no supported stream config",
            Self::BuildStreamFailed => "failed to build the input stream",
            Self::PlayStreamFailed => "failed to start the input stream",
        };
        f.write_str(msg)
    }
}

/// Records mono 16kHz audio from the platform's default input device.
///
/// `sample_count` is exposed via an `AtomicUsize` (rather than a
/// `Mutex<Vec<f32>>` counter) so the UI can poll recording progress without
/// contending with the audio callback for a lock; the accumulated samples
/// themselves live behind a `Mutex` since they're only read once, at `stop`.
pub struct Recorder {
    samples: Arc<std::sync::Mutex<Vec<f32>>>,
    sample_count: Arc<AtomicUsize>,
    stop_tx: Option<Sender<()>>,
    thread: Option<JoinHandle<()>>,
}

impl Default for Recorder {
    fn default() -> Self {
        Self::new()
    }
}

impl Recorder {
    pub fn new() -> Self {
        Self {
            samples: Arc::new(std::sync::Mutex::new(Vec::new())),
            sample_count: Arc::new(AtomicUsize::new(0)),
            stop_tx: None,
            thread: None,
        }
    }

    pub fn is_recording(&self) -> bool {
        self.thread.is_some()
    }

    /// Mono 16kHz samples captured so far. Cheap to call while recording.
    pub fn sample_count(&self) -> usize {
        self.sample_count.load(Ordering::Relaxed)
    }

    pub fn duration_secs(&self) -> f32 {
        self.sample_count() as f32 / TARGET_SAMPLE_RATE as f32
    }

    /// Starts capture on a dedicated thread. A no-op if already recording.
    pub fn start(&mut self) -> Result<(), RecorderError> {
        if self.is_recording() {
            return Ok(());
        }

        self.samples.lock().unwrap().clear();
        self.sample_count.store(0, Ordering::Relaxed);

        let samples = Arc::clone(&self.samples);
        let sample_count = Arc::clone(&self.sample_count);
        let (stop_tx, stop_rx) = mpsc::channel::<()>();
        let (ready_tx, ready_rx) = mpsc::channel::<Result<(), RecorderError>>();

        let thread = thread::spawn(move || {
            let stream = match build_input_stream(samples, sample_count) {
                Ok(stream) => stream,
                Err(err) => {
                    let _ = ready_tx.send(Err(err));
                    return;
                }
            };
            if stream.play().is_err() {
                let _ = ready_tx.send(Err(RecorderError::PlayStreamFailed));
                return;
            }
            let _ = ready_tx.send(Ok(()));
            // Block here for the stream's whole lifetime; dropping `stream`
            // on this same thread (when the recv unblocks) stops capture.
            let _ = stop_rx.recv();
        });

        self.thread = Some(thread);
        self.stop_tx = Some(stop_tx);

        ready_rx.recv().unwrap_or(Err(RecorderError::BuildStreamFailed))
    }

    /// Stops capture and joins the recorder thread. A no-op if not recording.
    pub fn stop(&mut self) {
        if let Some(stop_tx) = self.stop_tx.take() {
            let _ = stop_tx.send(());
        }
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }

    /// Takes the accumulated mono 16kHz samples, leaving the buffer empty.
    pub fn take_samples(&mut self) -> Vec<f32> {
        let mut samples = self.samples.lock().unwrap();
        std::mem::take(&mut *samples)
    }
}

fn build_input_stream(
    samples: Arc<std::sync::Mutex<Vec<f32>>>,
    sample_count: Arc<AtomicUsize>,
) -> Result<cpal::Stream, RecorderError> {
    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .ok_or(RecorderError::NoInputDevice)?;
    let supported_config = device
        .default_input_config()
        .map_err(|_| RecorderError::NoSupportedConfig)?;

    let sample_format = supported_config.sample_format();
    let channels = supported_config.channels();
    let source_rate = supported_config.sample_rate();
    let stream_config: StreamConfig = supported_config.into();

    let err_fn = |err| eprintln!("audio input stream error: {err}");

    // Each cpal callback buffer is resampled independently, so there's a tiny
    // discontinuity at every chunk boundary (sub-millisecond, inaudible for
    // speech but not phase-accurate). Fine for Phase 1/2 transcription; a
    // streaming resampler would be needed if this audio were ever played back.
    macro_rules! push_samples {
        ($data:expr) => {{
            let mono = downmix_to_mono($data, channels);
            let resampled = resample_linear(&mono, source_rate, TARGET_SAMPLE_RATE);
            sample_count.fetch_add(resampled.len(), Ordering::Relaxed);
            samples.lock().unwrap().extend_from_slice(&resampled);
        }};
    }

    let stream = match sample_format {
        SampleFormat::F32 => device.build_input_stream(
            stream_config,
            move |data: &[f32], _: &_| push_samples!(data),
            err_fn,
            None,
        ),
        SampleFormat::I16 => device.build_input_stream(
            stream_config,
            move |data: &[i16], _: &_| {
                let floats: Vec<f32> = data.iter().map(|s| *s as f32 / i16::MAX as f32).collect();
                push_samples!(&floats)
            },
            err_fn,
            None,
        ),
        SampleFormat::U16 => device.build_input_stream(
            stream_config,
            move |data: &[u16], _: &_| {
                let floats: Vec<f32> = data
                    .iter()
                    .map(|s| (*s as f32 / u16::MAX as f32) * 2.0 - 1.0)
                    .collect();
                push_samples!(&floats)
            },
            err_fn,
            None,
        ),
        _ => return Err(RecorderError::NoSupportedConfig),
    };

    stream.map_err(|_| RecorderError::BuildStreamFailed)
}
