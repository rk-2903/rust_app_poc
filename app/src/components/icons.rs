use dioxus::prelude::*;

#[component]
pub fn IconMic(color: String, size: u32) -> Element {
    rsx! {
        svg {
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "{color}",
            stroke_width: "1.6",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            rect { x: "9", y: "2", width: "6", height: "12", rx: "3" }
            path { d: "M5 11a7 7 0 0 0 14 0" }
            path { d: "M12 18v4" }
            path { d: "M8 22h8" }
        }
    }
}

#[component]
pub fn IconSettings(color: String) -> Element {
    rsx! {
        svg {
            width: "20",
            height: "20",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "{color}",
            stroke_width: "1.7",
            stroke_linecap: "round",
            line { x1: "4", y1: "6", x2: "20", y2: "6" }
            circle { cx: "14", cy: "6", r: "2.2" }
            line { x1: "4", y1: "12", x2: "20", y2: "12" }
            circle { cx: "9", cy: "12", r: "2.2" }
            line { x1: "4", y1: "18", x2: "20", y2: "18" }
            circle { cx: "16", cy: "18", r: "2.2" }
        }
    }
}

#[component]
pub fn IconChevronRight(color: String) -> Element {
    rsx! {
        svg {
            width: "16",
            height: "16",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "{color}",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M9 6l6 6-6 6" }
        }
    }
}

#[component]
pub fn IconBack(color: String) -> Element {
    rsx! {
        svg {
            width: "20",
            height: "20",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "{color}",
            stroke_width: "1.8",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M19 12H5" }
            path { d: "M11 18l-6-6 6-6" }
        }
    }
}

#[component]
pub fn IconTrash(color: String) -> Element {
    rsx! {
        svg {
            width: "18",
            height: "18",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "{color}",
            stroke_width: "1.7",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M4 7h16" }
            path { d: "M9 7V4h6v3" }
            path { d: "M6 7l1 13a2 2 0 0 0 2 2h6a2 2 0 0 0 2-2l1-13" }
            path { d: "M10 11v6" }
            path { d: "M14 11v6" }
        }
    }
}

#[component]
pub fn IconShield(color: String) -> Element {
    rsx! {
        svg {
            width: "20",
            height: "20",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "{color}",
            stroke_width: "1.7",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            rect { x: "4", y: "10", width: "16", height: "10", rx: "2" }
            path { d: "M8 10V7a4 4 0 0 1 8 0v3" }
        }
    }
}

#[component]
pub fn IconMicOff(color: String) -> Element {
    rsx! {
        svg {
            width: "38",
            height: "38",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "{color}",
            stroke_width: "1.6",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M9 9v3a3 3 0 0 0 4.6 2.55" }
            path { d: "M15 9.34V5a3 3 0 0 0-5.94-.6" }
            path { d: "M17 11a5 5 0 0 1-.11 1.06" }
            path { d: "M5 11a7 7 0 0 0 10.06 6.27" }
            path { d: "M12 18v4" }
            path { d: "M8 22h8" }
            line { x1: "2", y1: "2", x2: "22", y2: "22" }
        }
    }
}
