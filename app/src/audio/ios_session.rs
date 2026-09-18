//! iOS requires an active `AVAudioSession` in a recording-capable category
//! before CoreAudio will open an input stream. Without this, `cpal`'s
//! `build_input_stream`/`play` calls on a real iPhone fail (or silently
//! capture nothing) regardless of mic permission. This also happens to be
//! what triggers the OS's one-time mic permission prompt: iOS asks the
//! first time a session is activated in a record-capable category, then
//! remembers the answer permanently (no re-prompting) until the user
//! changes it in Settings.
//!
//! Called from the dedicated recorder thread (`recorder.rs`), not the UI
//! thread — `AVAudioSession` category/activation calls are documented as
//! thread-safe, unlike UIKit.

#[cfg(target_os = "ios")]
pub fn configure_audio_session() {
    use objc2_avf_audio::{AVAudioSession, AVAudioSessionCategoryOptions, AVAudioSessionCategoryPlayAndRecord};

    unsafe {
        let session = AVAudioSession::sharedInstance();
        let Some(category) = AVAudioSessionCategoryPlayAndRecord else {
            eprintln!("[audio] AVAudioSessionCategoryPlayAndRecord unavailable");
            return;
        };
        let options = AVAudioSessionCategoryOptions::DefaultToSpeaker;
        if let Err(err) = session.setCategory_withOptions_error(category, options) {
            eprintln!("[audio] AVAudioSession setCategory failed: {err}");
            return;
        }
        if let Err(err) = session.setActive_error(true) {
            eprintln!("[audio] AVAudioSession setActive failed: {err}");
        }
    }
}

#[cfg(not(target_os = "ios"))]
pub fn configure_audio_session() {
    // Desktop/simulator: cpal talks to CoreAudio/other backends directly,
    // no session activation step needed.
}
