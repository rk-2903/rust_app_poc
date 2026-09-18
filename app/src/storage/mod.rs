//! Recording metadata + audio + (future) transcript storage. Plain Rust —
//! no `dioxus::prelude` — so it stays testable independent of the UI layer.
//!
//! The recordings *list* (the in-memory index of `RecordingEntry`) doesn't
//! survive an app restart yet — that's the one remaining Phase 1 item. The
//! audio *file* behind a real recording is genuinely written to disk via
//! `wav::save_recording`.

mod recording_entry;
pub mod wav;

pub use recording_entry::{mock_recordings, RecordingEntry, Speaker};

// `TranscriptTurn` isn't named outside this module yet (views iterate
// `RecordingEntry::transcript` without spelling out the element type) —
// re-export it once something needs to construct one directly (Phase 2).
