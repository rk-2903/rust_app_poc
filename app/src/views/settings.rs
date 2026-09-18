use dioxus::prelude::*;

use crate::components::{IconBack, IconChevronRight, IconShield};
use crate::Route;

#[component]
pub fn Settings() -> Element {
    rsx! {
        div { class: "screen",
            div { class: "topbar",
                Link { to: Route::Home {}, class: "icon-button plain", "aria-label": "Back",
                    IconBack { color: "var(--ink)".to_string() }
                }
                div { class: "topbar-title serif", "Settings" }
            }

            div { class: "settings-body",
                div { class: "settings-group",
                    div { class: "section-label", style: "padding-left: 4px;", "Transcription" }
                    div { class: "settings-card",
                        div { class: "settings-row",
                            span { class: "settings-row-label", "Model" }
                            span { class: "settings-row-value",
                                "Moonshine Tiny"
                                IconChevronRight { color: "var(--muted)".to_string() }
                            }
                        }
                        div { class: "settings-row",
                            span { class: "settings-row-label", "Microphone" }
                            span { class: "settings-row-value",
                                "Built-in Microphone"
                                IconChevronRight { color: "var(--muted)".to_string() }
                            }
                        }
                    }
                }

                div { class: "settings-group",
                    div { class: "section-label", style: "padding-left: 4px;", "Privacy" }
                    div { class: "privacy-card",
                        IconShield { color: "#3F7268".to_string() }
                        div { class: "privacy-text",
                            "All recording and transcription happen on this device. Audio and transcripts are never uploaded."
                        }
                    }
                }

                div { class: "settings-group",
                    div { class: "section-label", style: "padding-left: 4px;", "About" }
                    div { class: "settings-card",
                        div { class: "settings-row",
                            span { class: "settings-row-label", "Version" }
                            span { class: "settings-row-value", "0.1.0" }
                        }
                        div { class: "settings-row",
                            span { class: "settings-row-label", "Send feedback" }
                            IconChevronRight { color: "var(--muted)".to_string() }
                        }
                    }
                }
            }
        }
    }
}
