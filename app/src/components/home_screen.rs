use dioxus::prelude::*;

/// Placeholder landing screen for Phase 0. Later phases replace this with
/// recording controls and the live transcript view.
#[component]
pub fn HomeScreen() -> Element {
    rsx! {
        div { id: "app",
            h1 { "Conversation Capture" }
            p { "Phase 0: project scaffold running." }
        }
    }
}
