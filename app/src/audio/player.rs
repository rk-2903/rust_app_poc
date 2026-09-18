//! Playback of a saved recording. iOS-only for now (via `AVAudioPlayer`) —
//! there's no Android playback path yet since Android isn't set up at all.

#[cfg(target_os = "ios")]
mod ios {
    use objc2::rc::Retained;
    use objc2::AnyThread;
    use objc2_avf_audio::AVAudioPlayer;
    use objc2_foundation::{NSString, NSURL};
    use std::cell::RefCell;

    thread_local! {
        static PLAYER: RefCell<Option<Retained<AVAudioPlayer>>> = const { RefCell::new(None) };
    }

    pub fn play(path: &str) {
        stop();
        unsafe {
            let ns_path = NSString::from_str(path);
            let url = NSURL::fileURLWithPath(&ns_path);
            match AVAudioPlayer::initWithContentsOfURL_error(AVAudioPlayer::alloc(), &url) {
                Ok(player) => {
                    let _: bool = player.play();
                    PLAYER.with(|p| *p.borrow_mut() = Some(player));
                }
                Err(err) => eprintln!("[player] AVAudioPlayer init failed: {err}"),
            }
        }
    }

    pub fn stop() {
        PLAYER.with(|p| {
            if let Some(player) = p.borrow().as_ref() {
                unsafe { player.stop() };
            }
            *p.borrow_mut() = None;
        });
    }

    pub fn is_playing() -> bool {
        PLAYER.with(|p| match p.borrow().as_ref() {
            Some(player) => unsafe { player.isPlaying() },
            None => false,
        })
    }
}

#[cfg(not(target_os = "ios"))]
mod fallback {
    pub fn play(_path: &str) {
        eprintln!("[player] playback isn't implemented on this platform yet");
    }

    pub fn stop() {}

    pub fn is_playing() -> bool {
        false
    }
}

#[cfg(target_os = "ios")]
pub use ios::{is_playing, play, stop};

#[cfg(not(target_os = "ios"))]
pub use fallback::{is_playing, play, stop};
