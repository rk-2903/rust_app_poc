use dioxus::prelude::*;

use crate::app_state::use_app_state;
use crate::components::{ConfirmDialog, IconBack, IconTrash};
use crate::storage::Speaker;
use crate::Route;

#[component]
pub fn Transcript(id: String) -> Element {
    let mut state = use_app_state();
    let nav = use_navigator();
    let mut confirming = use_signal(|| false);

    let entry = state.recordings.read().iter().find(|r| r.id == id).cloned();

    let Some(entry) = entry else {
        return rsx! {
            div { class: "screen centered-screen",
                div { class: "centered-title serif", "Recording not found" }
                Link { to: Route::Home {}, class: "link", "Back to Home" }
            }
        };
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
                if entry.transcript.is_empty() {
                    div { class: "transcript-empty",
                        "No transcript yet — on-device transcription lands in Phase 2."
                    }
                } else {
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
                }
            }

            if confirming() {
                ConfirmDialog {
                    title: format!("Delete \"{}\"?", entry.title),
                    body: "This permanently deletes the audio and transcript from this device. This can't be undone.".to_string(),
                    confirm_label: "Delete".to_string(),
                    on_cancel: move |_| confirming.set(false),
                    on_confirm: move |_| {
                        state.recordings.write().retain(|r| r.id != id);
                        confirming.set(false);
                        nav.push(Route::Home {});
                    },
                }
            }
        }
    }
}
