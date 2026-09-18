//! Shared, reusable UI components for the app.

mod confirm_dialog;
pub use confirm_dialog::ConfirmDialog;

mod icons;
pub use icons::{
    IconBack, IconChevronRight, IconMic, IconMicOff, IconPlay, IconSettings, IconShield, IconStop,
    IconTrash,
};

mod recording_row;
pub use recording_row::RecordingRow;
