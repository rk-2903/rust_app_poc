//! Route-level screens. Each variant of `Route` in `main.rs` renders one of
//! these.

mod all_recordings;
pub use all_recordings::AllRecordings;

mod home;
pub use home::Home;

mod no_mic_access;
pub use no_mic_access::NoMicAccess;

mod recording;
pub use recording::Recording;

mod settings;
pub use settings::Settings;

mod transcript;
pub use transcript::Transcript;
