//! On-device transcription via Moonshine Tiny (imported from ONNX by
//! `burn-onnx`, see `build.rs` and `scripts/download_models.sh`).

mod decode;
mod model;

pub use model::transcribe;

// `TranscriptionError` isn't named outside this module yet (the view just
// discards the error and shows "no transcript") — re-export it once
// something needs to distinguish failure reasons in the UI.
