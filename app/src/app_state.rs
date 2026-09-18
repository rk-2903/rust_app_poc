use dioxus::prelude::*;

use crate::audio::Recorder;
use crate::storage::{mock_recordings, RecordingEntry};

/// Shared across every view: the recordings list (mock + real, in-memory
/// only until Phase 1's on-disk storage lands) and the mic recorder, so
/// Home can start it and the Recording screen can poll/stop the same
/// instance.
#[derive(Clone, Copy)]
pub struct AppState {
    pub recordings: Signal<Vec<RecordingEntry>>,
    pub recorder: Signal<Recorder>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            recordings: Signal::new(mock_recordings()),
            recorder: Signal::new(Recorder::new()),
        }
    }
}

pub fn use_app_state() -> AppState {
    use_context::<AppState>()
}
