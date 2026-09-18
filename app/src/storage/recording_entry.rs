/// One saved recording: metadata, its audio file (if any), and (once
/// Phase 2 lands) its transcript.
///
/// The on-disk index (so this list survives an app restart) is still a
/// pending Phase 1 item — for now the list itself lives only in memory,
/// seeded with mock entries for the UI, per the design in
/// `Scribe_mobile_prototype.html`. The audio *file* a real recording
/// produces is genuinely written to disk (`storage::wav::save_recording`);
/// only the metadata pointing to it doesn't survive a restart yet.
#[derive(Debug, Clone, PartialEq)]
pub struct RecordingEntry {
    pub id: String,
    pub title: String,
    pub date_label: String,
    pub duration_label: String,
    /// `None` for the mock entries (there's no real audio behind them).
    pub audio_path: Option<String>,
    /// Speaker-attributed turns — only ever populated for the mock entries
    /// today. Real transcription (Phase 2) doesn't diarize by speaker yet
    /// (that's Phase 4), so real recordings' text goes in `transcript_text`
    /// instead.
    pub transcript: Vec<TranscriptTurn>,
    /// The flat transcript text for a real (non-mock) recording, once
    /// Phase 2 transcription finishes. `None` while `transcribing` or if
    /// transcription hasn't run / failed.
    pub transcript_text: Option<String>,
    /// Set right after Stop, cleared when background transcription (which
    /// takes real time — tens of seconds on-device) finishes.
    pub transcribing: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Speaker {
    Doctor,
    Patient,
}

impl Speaker {
    pub fn label(self) -> &'static str {
        match self {
            Speaker::Doctor => "Doctor",
            Speaker::Patient => "Patient",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TranscriptTurn {
    pub speaker: Speaker,
    pub time_label: String,
    pub text: String,
}

fn turn(speaker: Speaker, time_label: &str, text: &str) -> TranscriptTurn {
    TranscriptTurn {
        speaker,
        time_label: time_label.to_string(),
        text: text.to_string(),
    }
}

/// The design's mock recordings, so the list/transcript screens have
/// something to show before real storage (this Phase's remaining work) and
/// transcription (Phase 2) exist.
pub fn mock_recordings() -> Vec<RecordingEntry> {
    use Speaker::{Doctor, Patient};

    vec![
        RecordingEntry {
            id: "r1".to_string(),
            title: "Follow-up visit".to_string(),
            date_label: "Today".to_string(),
            duration_label: "12 min".to_string(),
            audio_path: None,
            transcript_text: None,
            transcribing: false,
            transcript: vec![
                turn(Doctor, "10:02 AM", "How have you been feeling since we adjusted the dosage last month?"),
                turn(Patient, "10:03 AM", "Better, actually. The headaches are almost gone, but I've been a little more tired in the afternoons."),
                turn(Doctor, "10:04 AM", "That's a known side effect and it usually settles within a few weeks. Let's keep the dose steady and check your bloodwork at the next visit."),
                turn(Patient, "10:05 AM", "Okay, that sounds good. Should I still avoid the grapefruit juice?"),
                turn(Doctor, "10:05 AM", "Yes, please keep avoiding it — it can interact with this medication."),
            ],
        },
        RecordingEntry {
            id: "r2".to_string(),
            title: "New patient intake".to_string(),
            date_label: "Tuesday".to_string(),
            duration_label: "24 min".to_string(),
            audio_path: None,
            transcript_text: None,
            transcribing: false,
            transcript: vec![
                turn(Doctor, "9:00 AM", "Let's start with your medical history — any conditions I should know about?"),
                turn(Patient, "9:01 AM", "Just seasonal allergies, and my mother has type 2 diabetes."),
            ],
        },
        RecordingEntry {
            id: "r3".to_string(),
            title: "Post-op check-in".to_string(),
            date_label: "Sep 12".to_string(),
            duration_label: "9 min".to_string(),
            audio_path: None,
            transcript_text: None,
            transcribing: false,
            transcript: vec![
                turn(Doctor, "2:15 PM", "How's the incision site looking today?"),
                turn(Patient, "2:15 PM", "Much better, barely any swelling now."),
            ],
        },
        RecordingEntry {
            id: "r4".to_string(),
            title: "Medication review".to_string(),
            date_label: "Sep 9".to_string(),
            duration_label: "15 min".to_string(),
            audio_path: None,
            transcript_text: None,
            transcribing: false,
            transcript: vec![],
        },
        RecordingEntry {
            id: "r5".to_string(),
            title: "Annual physical".to_string(),
            date_label: "Sep 3".to_string(),
            duration_label: "31 min".to_string(),
            audio_path: None,
            transcript_text: None,
            transcribing: false,
            transcript: vec![],
        },
    ]
}
