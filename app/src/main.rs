use dioxus::prelude::*;

use views::{AllRecordings, Home, NoMicAccess, Recording, Settings, Transcript};

mod app_state;
/// Mic capture (cpal), independent of the UI layer.
mod audio;
/// Shared, reusable UI components.
mod components;
/// Recording metadata + (future) transcript storage.
mod storage;
/// On-device transcription (Moonshine Tiny via burn-onnx).
mod transcription;
/// Route-level screens.
mod views;

const MAIN_CSS: Asset = asset!("/assets/main.css");

#[derive(Debug, Clone, PartialEq, Routable)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/recordings")]
    AllRecordings {},
    #[route("/recording")]
    Recording {},
    #[route("/transcript/:id")]
    Transcript { id: String },
    #[route("/settings")]
    Settings {},
    #[route("/no-mic-access")]
    NoMicAccess {},
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    use_context_provider(app_state::AppState::new);

    rsx! {
        document::Meta { name: "viewport", content: "width=device-width, initial-scale=1" }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    }
}
