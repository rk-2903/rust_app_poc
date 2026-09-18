/// One saved recording: metadata + (once Phase 2 lands) its transcript.
///
/// Persisted storage (WAV file + an on-disk index) is still a pending Phase 1
/// item — for now these live only in memory, seeded with mock entries for
/// the UI, per the design in `Scribe_mobile_prototype.html`.
#[derive(Debug, Clone, PartialEq)]
pub struct RecordingEntry {
    pub id: String,
    pub title: String,
    pub date_label: String,
    pub duration_label: String,
    /// Empty means "no transcript yet" (real recordings, before Phase 2)
    /// rather than a recording that failed to transcribe.
    pub transcript: Vec<TranscriptTurn>,
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
            transcript: vec![],
        },
        RecordingEntry {
            id: "r5".to_string(),
            title: "Annual physical".to_string(),
            date_label: "Sep 3".to_string(),
            duration_label: "31 min".to_string(),
            transcript: vec![],
        },
    ]
}
