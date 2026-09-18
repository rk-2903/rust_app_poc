use std::time::Duration;

use dioxus::prelude::*;

use crate::app_state::use_app_state;
use crate::components::{ConfirmDialog, IconBack, IconPlay, IconStop, IconTrash};
use crate::storage::Speaker;
use crate::Route;

#[component]
pub fn Transcript(id: String) -> Element {
    let mut state = use_app_state();
    let nav = use_navigator();
    let mut confirming = use_signal(|| false);
    let mut is_playing = use_signal(|| false);

    // `AVAudioPlayer` finishes on its own when the clip ends; poll so the
    // button falls back out of "Playing" state without needing a callback
    // wired all the way from the platform layer.
    use_future(move || async move {
        loop {
            if is_playing() && !crate::audio::player::is_playing() {
                is_playing.set(false);
            }
            tokio::time::sleep(Duration::from_millis(300)).await;
        }
    });

    let entry = state.recordings.read().iter().find(|r| r.id == id).cloned();

    let Some(entry) = entry else {
        return rsx! {
            div { class: "screen centered-screen",
                div { class: "centered-title serif", "Recording not found" }
                Link { to: Route::Home {}, class: "link", "Back to Home" }
            }
        };
    };

    let toggle_playback = {
        let audio_path = entry.audio_path.clone();
        move |_| {
            let Some(path) = audio_path.clone() else {
                return;
            };
            if is_playing() {
                crate::audio::player::stop();
                is_playing.set(false);
            } else {
                crate::audio::player::play(&path);
                is_playing.set(true);
            }
        }
    };

    rsx! {
        div { class: "screen",
            div { class: "topbar space-between",
                Link { to: Route::Home {}, class: "icon-button plain", "aria-label": "Back",
                    IconBack { color: "var(--ink)".to_string() }
                }
                div { class: "topbar-title-group",
                    div { class: "topbar-title serif", style: "font-size: 17px;", "{entry.title}" }
                    div { class: "topbar-subtitle", "{entry.date_label} · {entry.duration_label}" }
                }
                button {
                    class: "icon-button plain",
                    "aria-label": "Delete recording",
                    onclick: move |_| confirming.set(true),
                    IconTrash { color: "var(--ink)".to_string() }
                }
            }

            if entry.audio_path.is_some() {
                div { class: "playback-row",
                    button { class: "play-button", onclick: toggle_playback,
                        if is_playing() {
                            IconStop { color: "var(--ink)".to_string() }
                            "Stop"
                        } else {
                            IconPlay { color: "var(--ink)".to_string() }
                            "Play recording"
                        }
                    }
                }
            }

            if !entry.transcript.is_empty() {
                div { class: "legend",
                    div { class: "legend-item",
                        span { class: "legend-dot doctor" }
                        "Doctor"
                    }
                    div { class: "legend-item",
                        span { class: "legend-dot patient" }
                        "Patient"
                    }
                }
            }

            div { class: "transcript-body",
                if entry.transcribing {
                    div { class: "transcript-empty", "Transcribing on-device… this can take a minute." }
                } else if let Some(text) = &entry.transcript_text {
                    if text.is_empty() {
                        div { class: "transcript-empty", "No speech detected." }
                    } else {
                        div { class: "turn-text", "{text}" }
                        div { class: "transcript-footer", "Transcribed on-device using Moonshine Tiny" }
                    }
                } else if !entry.transcript.is_empty() {
                    for turn in entry.transcript.iter() {
                        div { class: "turn",
                            div { class: "turn-header",
                                span {
                                    class: if turn.speaker == Speaker::Doctor { "speaker-label doctor" } else { "speaker-label patient" },
                                    "{turn.speaker.label()}"
                                }
                                span { class: "turn-time", "{turn.time_label}" }
                            }
                            div { class: "turn-text", "{turn.text}" }
                        }
                    }
                    div { class: "transcript-footer", "Transcribed on-device using Moonshine Tiny" }
                } else {
                    div { class: "transcript-empty", "No transcript yet." }
                }
            }

            if confirming() {
                ConfirmDialog {
                    title: format!("Delete \"{}\"?", entry.title),
                    body: "This permanently deletes the audio and transcript from this device. This can't be undone.".to_string(),
                    confirm_label: "Delete".to_string(),
                    on_cancel: move |_| confirming.set(false),
                    on_confirm: move |_| {
                        crate::audio::player::stop();
                        if let Some(path) = &entry.audio_path {
                            let _ = std::fs::remove_file(path);
                        }
                        state.recordings.write().retain(|r| r.id != id);
                        confirming.set(false);
                        nav.push(Route::Home {});
                    },
                }
            }
        }
    }
}
