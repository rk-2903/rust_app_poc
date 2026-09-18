use std::time::Duration;

use dioxus::prelude::*;

use crate::app_state::use_app_state;
use crate::storage::RecordingEntry;
use crate::Route;

fn format_timer(total_secs: f32) -> String {
    let total = total_secs.max(0.0) as u32;
    format!("{}:{:02}", total / 60, total % 60)
}

fn format_duration_label(total_secs: f32) -> String {
    let total = total_secs.max(0.0) as u32;
    if total < 60 {
        format!("{total}s")
    } else {
        format!("{}m {}s", total / 60, total % 60)
    }
}

#[component]
pub fn Recording() -> Element {
    let mut state = use_app_state();
    let nav = use_navigator();
    let mut elapsed = use_signal(|| 0.0_f32);

    {
        let recorder = state.recorder;
        use_future(move || async move {
            loop {
                elapsed.set(recorder.read().duration_secs());
                tokio::time::sleep(Duration::from_millis(200)).await;
            }
        });
    }

    let cancel_recording = move |_| {
        let mut recorder = state.recorder.write();
        recorder.stop();
        recorder.take_samples();
        drop(recorder);
        nav.push(Route::Home {});
    };

    let stop_recording = move |_| {
        let mut recorder = state.recorder.write();
        recorder.stop();
        let duration_secs = recorder.duration_secs();
        let samples = recorder.take_samples();
        drop(recorder);

        let audio_path = match crate::storage::wav::save_recording(&samples, crate::audio::TARGET_SAMPLE_RATE) {
            Ok(path) => Some(path.to_string_lossy().into_owned()),
            Err(err) => {
                eprintln!("[recording] failed to save WAV: {err}");
                None
            }
        };

        let id = format!("real-{}", instant_id());
        state.recordings.write().insert(
            0,
            RecordingEntry {
                id: id.clone(),
                title: "New recording".to_string(),
                date_label: "Just now".to_string(),
                duration_label: format_duration_label(duration_secs),
                audio_path,
                transcript: Vec::new(),
            },
        );
        nav.push(Route::Transcript { id });
    };

    rsx! {
        div { class: "screen theme-dark",
            div { class: "topbar space-between",
                a {
                    style: "font-size: 15px; font-weight: 600; color: var(--muted); padding: 10px 4px; min-height: 44px; display: flex; align-items: center; cursor: pointer;",
                    onclick: cancel_recording,
                    "Cancel"
                }
                div { class: "recording-status",
                    span { class: "recording-dot" }
                    span { class: "recording-status-label", "Recording" }
                }
            }

            div { class: "recording-center",
                div { class: "recording-timer serif", "{format_timer(elapsed())}" }
                div { class: "waveform",
                    for i in 0..10 {
                        div {
                            key: "{i}",
                            class: "wave-bar",
                            style: "height: {22 + (i * 7) % 34}px; animation-delay: {i as f32 * 0.12}s;",
                        }
                    }
                }
                div { class: "recording-caption", "Listening for both voices — nothing leaves this device." }
            }

            div { class: "recording-footer",
                button {
                    class: "stop-button",
                    "aria-label": "Stop recording",
                    onclick: stop_recording,
                    svg {
                        width: "22",
                        height: "22",
                        view_box: "0 0 24 24",
                        fill: "none",
                        rect { x: "6", y: "6", width: "12", height: "12", rx: "3", fill: "#3F7268" }
                    }
                }
                div { class: "stop-button-label", "Stop" }
            }
        }
    }
}

/// Cheap unique-enough id for a session's worth of recordings (no clock
/// dependency needed since these never persist across app restarts yet).
fn instant_id() -> u64 {
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}
