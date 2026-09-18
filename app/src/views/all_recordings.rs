use dioxus::prelude::*;

use crate::app_state::use_app_state;
use crate::components::{ConfirmDialog, IconBack, RecordingRow};
use crate::Route;

#[component]
pub fn AllRecordings() -> Element {
    let mut state = use_app_state();
    let mut confirming_id: Signal<Option<String>> = use_signal(|| None);

    let recordings = state.recordings.read().clone();
    let confirming_title = confirming_id
        .read()
        .as_ref()
        .and_then(|id| recordings.iter().find(|r| &r.id == id))
        .map(|r| r.title.clone());

    rsx! {
        div { class: "screen",
            div { class: "topbar",
                Link { to: Route::Home {}, class: "icon-button plain", "aria-label": "Back",
                    IconBack { color: "var(--ink)".to_string() }
                }
                div { class: "topbar-title serif", "All Recordings" }
            }
            div { class: "subtitle-text", "Audio and transcript are stored together on this device." }

            div { class: "recordings-list",
                for entry in recordings.iter().cloned() {
                    RecordingRow {
                        key: "{entry.id}",
                        to: Route::Transcript { id: entry.id.clone() },
                        title: entry.title.clone(),
                        meta: "{entry.date_label} · {entry.duration_label}",
                        show_delete: true,
                        on_delete: {
                            let id = entry.id.clone();
                            move |_| confirming_id.set(Some(id.clone()))
                        },
                    }
                }
            }

            if let Some(title) = confirming_title {
                ConfirmDialog {
                    title: format!("Delete \"{title}\"?"),
                    body: "This permanently deletes the audio and transcript from this device. This can't be undone.".to_string(),
                    confirm_label: "Delete".to_string(),
                    on_cancel: move |_| confirming_id.set(None),
                    on_confirm: move |_| {
                        if let Some(id) = confirming_id.read().clone() {
                            let mut recordings = state.recordings.write();
                            if let Some(entry) = recordings.iter().find(|r| r.id == id) {
                                if let Some(path) = &entry.audio_path {
                                    let _ = std::fs::remove_file(path);
                                }
                            }
                            recordings.retain(|r| r.id != id);
                        }
                        confirming_id.set(None);
                    },
                }
            }
        }
    }
}
