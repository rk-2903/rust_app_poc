//! Recording metadata + (future) transcript storage. Plain Rust — no
//! `dioxus::prelude` — so it stays testable independent of the UI layer.
//!
//! Persistence to disk (WAV files + an on-disk index) is still a pending
//! Phase 1 item; today this only holds in-memory mock data plus whatever
//! real recordings are made this session.

mod recording_entry;

pub use recording_entry::{mock_recordings, RecordingEntry, Speaker};

// `TranscriptTurn` isn't named outside this module yet (views iterate
// `RecordingEntry::transcript` without spelling out the element type) —
// re-export it once something needs to construct one directly (Phase 2).
