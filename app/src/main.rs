use dioxus::prelude::*;

use components::HomeScreen;

/// Shared UI components for the app.
mod components;

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Meta { name: "viewport", content: "width=device-width, initial-scale=1" }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        HomeScreen {}
    }
}
