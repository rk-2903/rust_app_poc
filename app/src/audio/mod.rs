//! Mic capture. Plain Rust — no `dioxus::prelude` — so it's usable and
//! testable independent of the UI layer.

mod ios_session;
pub mod player;
mod recorder;
mod resample;

pub use recorder::{Recorder, TARGET_SAMPLE_RATE};

// `RecorderError` isn't named outside this module yet (errors are surfaced
// via `Display`) — re-export it once something needs the type itself.
