use dioxus::prelude::*;

use crate::app_state::use_app_state;
use crate::components::IconMicOff;
use crate::Route;

#[component]
pub fn NoMicAccess() -> Element {
    let state = use_app_state();
    let mut recorder = state.recorder;
    let nav = use_navigator();

    // Placeholder: a real "Open Settings" would deep-link into the OS
    // privacy settings (platform-specific, not built yet). For now this
    // just retries starting the recorder, which is what the user would
    // effectively do after actually granting the permission themselves.
    let retry = move |_| {
        if recorder.write().start().is_ok() {
            nav.push(Route::Recording {});
        }
    };

    rsx! {
        div { class: "screen centered-screen",
            div { class: "centered-icon",
                IconMicOff { color: "var(--muted)".to_string() }
            }
            div { class: "centered-title serif", "Microphone access needed" }
            div { class: "centered-body",
                "Scribe needs microphone access to capture and transcribe conversations. Nothing is recorded until you allow it."
            }
            button { class: "primary-button", onclick: retry, "Open Settings" }
            Link { to: Route::Home {}, class: "text-link-button", "Not now" }
        }
    }
}
