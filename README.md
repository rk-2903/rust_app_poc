# Conversation Capture: Mobile App Roadmap

## What this is
A mobile-first Dioxus app that records a conversation, transcribes it fully
with a small on-device speech-to-text model, and will eventually tag or
segment the transcript by speaker/topic (e.g. doctor vs. patient). Phase 1
goal: capture the whole conversation and produce a full transcript.
Semantic segmentation comes later.

## Key decisions (locked in)
- **Platforms:** Android and iOS, mobile-first. Desktop is a later,
  secondary build target using the same core crates.
- **Transcription model:** Moonshine Tiny, an ONNX speech-to-text model
  (27M parameters, ~190MB fp32, smaller quantized) purpose-built for
  resource-constrained/edge devices.
- **Runtime:** Burn (pure Rust). The Moonshine ONNX graphs are imported
  with `burn-onnx` and run through Burn's own backends, no separate C++
  runtime to bundle.
- **Dev environment:** Mac, Xcode, Android Studio, free Apple ID (no paid
  Apple Developer account). This is enough to build and run on your own
  devices; see "Local install" below for what that does and doesn't allow.

## Why mobile changes the plan
The original desktop-prototype idea could shell out to a separately-built
transcription binary and read its output file. On Android/iOS that's not
viable, app sandboxing doesn't allow spawning arbitrary external
processes, so transcription has to run **in-process**, as part of the app
itself. Audio capture also moves from "any input device on the machine"
to the platform's native mic APIs.

## Why Burn + ONNX instead of a vendored Whisper build
`burn-onnx` converts an ONNX model into native Burn Rust code that runs on
any Burn backend, and speech models are explicitly one of the categories
its maintainers validate it against. That makes it a real option, not just
a hopeful one. The one thing to plan around: Moonshine (like Whisper)
exports as **two separate ONNX graphs**, an encoder and a decoder, not one
combined graph. The token-by-token generation loop that ties them together
(feeding the decoder's output back in, tracking the KV cache, stopping at
the end token) isn't part of either graph, so it has to be hand-written in
Rust regardless of which runtime is used. This is the main engineering
lift in Phase 2 below.

## Tooling & environment requirements
- Rust + `dx` CLI (Dioxus 0.7)
- Rust targets: `aarch64-linux-android`, `armv7-linux-androideabi`
  (optional), `aarch64-apple-ios`, `aarch64-apple-ios-sim`
- Xcode (iOS toolchain, simulator, signing) + an Apple ID added as a
  Personal Team under Xcode > Settings > Accounts
- Android Studio (SDK/NDK, emulator, or USB debugging to a real device)
- `cpal` — mic capture; has native Android (AAudio) and iOS (CoreAudio)
  backends
- `hound` — WAV encoding before feeding audio to the model
- `burn-onnx` — imports the Moonshine ONNX graphs into native Burn code
- `burn` with the `ndarray` backend to start (pure Rust, no GPU driver
  assumptions); `wgpu` is a later option for speed once things work
- Moonshine Tiny ONNX weights (encoder + decoder graphs), ideally a
  quantized/mobile-oriented export to keep app size down

## Roadmap

### Phase 0 — Project setup
- [x] `dx new` a Dioxus 0.7 project with mobile targets enabled
- [x] Get an empty screen running on an iOS simulator and a real iPhone
      (see [app/README.md](app/README.md)). Android emulator not set up yet
      (Android Studio/SDK install still pending — iOS was the immediate
      target).
- [x] Add mic permission via the unified `[permissions]` block in
      `app/Dioxus.toml`, which the dx 0.7.10 CLI maps to both
      `Info.plist` (`NSMicrophoneUsageDescription`) and
      `AndroidManifest.xml` (`RECORD_AUDIO`) at build time.
- [ ] Build the runtime permission-request flow (Android runtime dialog,
      iOS first-use prompt) — deferred to Phase 1: this fires automatically
      the first time `cpal` opens an input stream, once
      `NSMicrophoneUsageDescription` is set (already done above), so
      there's nothing to build here independent of the cpal wiring below.

### Phase 1 — Mic capture, local storage & recordings list
- [ ] Add cpal; confirm it opens the default input device on a **real**
      iPhone and a real Android device (simulators/emulators often don't
      expose a working mic — verified so far only that it fails gracefully
      on the iOS simulator's fake device; real-iPhone confirmation pending)
- [x] Reuse the dedicated-thread recorder pattern (cpal streams must stay
      on the thread that creates them)
- [x] Downmix to mono + resample to 16kHz
- [x] Wire Start/Stop UI to the recorder
- [ ] On stop, WAV-encode the captured samples (`hound`) and save to the
      app's local storage directory, keyed by a timestamp-based ID —
      `RecordingEntry` (the schema below) currently holds a real recording's
      metadata in memory only, not yet its audio on disk
- [x] Design the storage schema — `RecordingEntry { id, title, date_label,
      duration_label, transcript: Vec<TranscriptTurn> }` in
      `app/src/storage/`, so Phase 2 attaches a transcript to an existing
      entry instead of redesigning storage. Still in-memory only (seeded
      with mock entries matching the design); moving it to on-disk
      persistence is the remaining item above.
- [x] Add an "All Recordings" list view (past recordings by date/duration,
      tap-through to a Transcript detail screen) — the app's first
      multi-screen navigation, so this is also where `views/` + the
      `router` feature got introduced per
      [dioxus-conventions](.claude/skills/dioxus-conventions/SKILL.md)
- [x] Add delete for a recording (from the list and its detail screen),
      gated behind a confirmation dialog; on confirm, removes the entry
      (and once Phase 2 adds transcripts + Phase 1's disk storage lands,
      will remove the WAV file too) from the in-memory store
- [x] UI redesigned end-to-end to match `Scribe_mobile_prototype.html`
      (Home, All Recordings, Recording, Transcript, Settings, No Mic
      Access) — verified on the iOS simulator, the real iPhone, and a web
      build (used for precise DOM-level interaction testing)

### Phase 2 — On-device transcription
- [ ] Run `burn-onnx` against Moonshine Tiny's encoder graph and its
      decoder graph separately, generating two native Burn models
- [ ] Write the decode loop by hand: feed the encoder's output into the
      decoder, track the KV cache across steps, stop at the end-of-sequence
      token
- [ ] Start on the `ndarray` backend; benchmark before considering `wgpu`
- [ ] Decide model distribution: bundle the ~190MB (or smaller quantized)
      weights in the app package vs. download on first launch
- [ ] Run inference in-process on a background thread/task so the UI stays
      responsive
- [ ] Benchmark transcription time for a ~1 minute clip on a real
      mid-range Android phone and a real iPhone; fall back to a smaller
      quantized export if it's too slow

### Phase 3 — Full conversation capture (this project's phase-1 goal)
- [ ] Wire Stop → transcribe → attach the transcript to the recording's
      entry in the Phase 1 storage schema, and show it in the recording's
      detail screen
- [ ] Handle app backgrounding/interruptions during recording gracefully
      (phone calls, notifications)

### Phase 4 — Semantic capture (doctor/patient style segmentation)
- [ ] Add turn/speaker segmentation on top of the transcript (start with
      simple silence-gap heuristics; later, proper speaker diarization or
      a classifier)
- [ ] Tag segments by role or topic (doctor vs. patient), decide whether
      this is rule-based, a small classifier, or an LLM call on the
      transcript
- [ ] **Future scope: multi-speaker diarization** ("who spoke when" across
      3+ people, not just doctor/patient). This is a distinct technique
      from transcription, not a byproduct of it:
      1. A small speaker-embedding model (ECAPA-TDNN/x-vector/d-vector
         style) converts short rolling audio windows into a fixed-size
         "voiceprint" vector, independent of what's being said.
      2. Cluster embeddings — segments with similar voiceprints get grouped
         under an anonymous label (Speaker 1/2/3); the speaker count can be
         given or estimated from the clustering itself.
      3. Optional named identification is a separate step on top: an
         enrollment flow (record a short reference sample per known
         person) to match new voiceprints against.
      Needs a second on-device model imported the same way as Moonshine
      (via `burn-onnx`) plus a clustering step — real added scope, not
      free from Phase 2's transcription work.
- [ ] Surface tagged segments in the UI (collapsible sections,
      highlighting)

### Phase 5 — Polish & distribution
- [ ] App icon, splash screen, basic settings screen (model choice, mic
      device if applicable)
- [ ] Error states: no mic permission, no input device, model failed to
      load/download
- [ ] Build release artifacts (`dx bundle --platform android` /
      `--platform ios`) and check store-review requirements (privacy
      disclosures for microphone + on-device audio processing, especially
      relevant given the medical-conversation use case)
- [ ] Revisit the desktop build using the same core crates once mobile is
      stable

## Local install (Mac + free Apple ID)
1. `rustup target add aarch64-apple-ios aarch64-apple-ios-sim aarch64-linux-android`,
   then `cargo install dioxus-cli`
2. Install Xcode and Android Studio
3. In Xcode > Settings > Accounts, add your Apple ID; it shows up as a
   free "Personal Team" signing identity
4. Android: enable Developer Mode + USB debugging on the phone, plug it
   in, run `dx serve --platform android`. No signing account needed for a
   local debug install
5. iOS: `dx` (0.7.10) can't request a provisioning profile from Apple by
   itself — only Xcode can. One-time bootstrap: create a throwaway Xcode
   project (File > New > Project > iOS > App) with **Bundle Identifier**
   set to exactly `bundle.identifier` from `app/Dioxus.toml`
   (`com.rahulkumar.conversationcapture`) and Team set to your Personal
   Team, run it once on the real iPhone (Product > Destination > pick the
   phone > ⌘R), then trust the dev cert on the phone under Settings >
   General > VPN & Device Management. That caches a provisioning profile
   `dx` reuses from then on. See [app/README.md](app/README.md) for the
   exact commands to build/install/launch after that.
6. A free Apple ID signs apps for **7 days**. When `dx build`/`dx serve`
   starts failing with a codesigning error again, redo step 5's Xcode run
   once (bundle ID is already set, so it's quick) to refresh it. Android
   installs have no such expiry.

## Open decisions to revisit
- Whether Moonshine Tiny's ONNX ops all import cleanly through
  `burn-onnx` as-is, or whether some need custom operator hooks
- `ndarray` vs `wgpu` backend for mobile, needs real-device benchmarking
- Model size vs. accuracy vs. app size trade-off (Tiny vs. Base, quantized
  vs. full precision)
- Bundled vs. downloaded model weights
- Whether semantic segmentation runs on-device or calls out to a server

## Note on data sensitivity
Since the target use case is doctor/patient conversations, treat
recordings and transcripts as sensitive health data from the start:
minimize what's stored, encrypt anything persisted locally, and confirm
what regulatory requirements (e.g. HIPAA if used in the US) apply before
this goes beyond a prototype.