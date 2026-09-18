use dioxus::prelude::*;

use super::icons::{IconChevronRight, IconTrash};
use crate::Route;

/// One row in a recordings list (Home's "Recent conversations" and the
/// "All Recordings" screen), linking to that recording's transcript, with
/// an optional delete button beside it.
#[component]
pub fn RecordingRow(
    to: Route,
    title: String,
    meta: String,
    show_delete: bool,
    on_delete: EventHandler<()>,
) -> Element {
    rsx! {
        div { class: "row-with-action",
            Link { to, class: "row",
                div { style: "display: flex; flex-direction: column; gap: 2px; min-width: 0;",
                    div { class: "row-title", "{title}" }
                    div { class: "row-meta", "{meta}" }
                }
                IconChevronRight { color: "var(--muted)".to_string() }
            }
            if show_delete {
                button {
                    class: "icon-button",
                    "aria-label": "Delete recording",
                    onclick: move |_| on_delete.call(()),
                    IconTrash { color: "var(--muted)".to_string() }
                }
            }
        }
    }
}
