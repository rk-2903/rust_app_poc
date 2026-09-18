use std::time::Duration;

use dioxus::prelude::*;

use crate::audio::Recorder;

/// Start/Stop mic capture, showing elapsed recording time and the sample
/// count captured on stop. Phase 2 will feed those samples to the
/// transcription model instead of just reporting their count.
#[component]
pub fn RecorderControls() -> Element {
    let mut recorder = use_signal(Recorder::new);
    let mut is_recording = use_signal(|| false);
    let mut duration_secs = use_signal(|| 0.0_f32);
    let mut error = use_signal(|| None::<String>);
    let mut last_capture: Signal<Option<(usize, f32)>> = use_signal(|| None);

    // Polls the recorder's sample count while recording so the displayed
    // duration ticks up; the recorder itself is updated from cpal's own
    // audio-thread callback, not from this loop.
    use_future(move || async move {
        loop {
            if is_recording() {
                duration_secs.set(recorder.read().duration_secs());
            }
            tokio::time::sleep(Duration::from_millis(200)).await;
        }
    });

    let toggle_recording = move |_| {
        if is_recording() {
            let mut recorder = recorder.write();
            recorder.stop();
            let duration = recorder.duration_secs();
            let samples = recorder.take_samples();
            last_capture.set(Some((samples.len(), duration)));
            drop(recorder);
            is_recording.set(false);
        } else {
            match recorder.write().start() {
                Ok(()) => {
                    error.set(None);
                    is_recording.set(true);
                }
                Err(err) => error.set(Some(err.to_string())),
            }
        }
    };

    rsx! {
        div { id: "recorder",
            button { onclick: toggle_recording,
                if is_recording() { "Stop" } else { "Start Recording" }
            }
            if is_recording() {
                p { "Recording... {duration_secs():.1}s" }
            }
            if let Some((sample_count, duration)) = last_capture() {
                p { "Captured {sample_count} samples ({duration:.1}s)" }
            }
            if let Some(message) = error() {
                p { class: "error", "{message}" }
            }
        }
    }
}
