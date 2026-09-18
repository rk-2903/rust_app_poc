use dioxus::prelude::*;

use super::RecorderControls;

#[component]
pub fn HomeScreen() -> Element {
    rsx! {
        div { id: "app",
            h1 { "Conversation Capture" }
            RecorderControls {}
        }
    }
}
