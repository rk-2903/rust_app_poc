//! Mic capture. Plain Rust — no `dioxus::prelude` — so it's usable and
//! testable independent of the UI layer.

mod recorder;
mod resample;

pub use recorder::Recorder;

// `RecorderError` and `TARGET_SAMPLE_RATE` aren't named outside this module
// yet (errors are surfaced via `Display`; duration math stays behind
// `Recorder::duration_secs`) — re-export them once something actually needs
// the type/value, not preemptively.
