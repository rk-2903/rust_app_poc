use dioxus::prelude::*;

use crate::app_state::use_app_state;
use crate::components::{IconMic, IconSettings, RecordingRow};
use crate::Route;

#[component]
pub fn Home() -> Element {
    let state = use_app_state();
    let nav = use_navigator();
    let mut recorder = state.recorder;

    let start_recording = move |_| {
        let result = recorder.write().start();
        match result {
            Ok(()) => {
                nav.push(Route::Recording {});
            }
            Err(_) => {
                nav.push(Route::NoMicAccess {});
            }
        }
    };

    let recent = state.recordings.read().iter().take(3).cloned().collect::<Vec<_>>();

    rsx! {
        div { class: "screen",
            div { class: "topbar space-between",
                div { class: "topbar-title serif", "Scribe" }
                Link { to: Route::Settings {}, class: "icon-button", "aria-label": "Settings",
                    IconSettings { color: "#3F7268".to_string() }
                }
            }

            div { class: "home-hero",
                button {
                    class: "record-button",
                    "aria-label": "Start recording",
                    onclick: start_recording,
                    IconMic { color: "#FFFFFF".to_string(), size: 42 }
                }
                div { style: "display: flex; flex-direction: column; align-items: center; gap: 3px;",
                    div { class: "record-button-label", "Start recording" }
                    div { class: "record-button-caption", "Processed entirely on this device" }
                }
            }

            div { class: "section",
                div { class: "section-header",
                    div { class: "section-label", "Recent conversations" }
                    Link { to: Route::AllRecordings {}, class: "link", "See all" }
                }
                div { class: "list",
                    for entry in recent {
                        RecordingRow {
                            key: "{entry.id}",
                            to: Route::Transcript { id: entry.id.clone() },
                            title: entry.title.clone(),
                            meta: "{entry.date_label} · {entry.duration_label}",
                            show_delete: false,
                            on_delete: move |_| {},
                        }
                    }
                }
            }
        }
    }
}
