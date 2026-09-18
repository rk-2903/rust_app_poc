use dioxus::prelude::*;

/// Bottom-sheet confirmation used before any destructive action (currently:
/// deleting a recording's audio + transcript).
#[component]
pub fn ConfirmDialog(
    title: String,
    body: String,
    confirm_label: String,
    on_cancel: EventHandler<()>,
    on_confirm: EventHandler<()>,
) -> Element {
    rsx! {
        div { class: "modal-overlay", onclick: move |_| on_cancel.call(()),
            div {
                class: "modal-sheet",
                onclick: move |evt| evt.stop_propagation(),
                div { class: "modal-title", "{title}" }
                div { class: "modal-body", "{body}" }
                div { class: "modal-actions",
                    button {
                        class: "modal-button",
                        onclick: move |_| on_cancel.call(()),
                        "Cancel"
                    }
                    button {
                        class: "modal-button destructive",
                        onclick: move |_| on_confirm.call(()),
                        "{confirm_label}"
                    }
                }
            }
        }
    }
}
