---
name: dioxus-conventions
description: Naming, folder structure, and coding conventions for this Dioxus 0.7 mobile app. Use whenever adding or restructuring Rust/Dioxus source under app/src, adding a new screen or component, or wiring platform-specific (cpal/burn-onnx/iOS/Android) code into the UI.
---

# Dioxus project conventions (this repo)

Source of truth: the official Dioxus 0.7 `Jumpstart` template
(github.com/DioxusLabs/dioxus-template, `v0.7` branch) and its bundled
`AGENTS.md`, which Dioxus itself ships for AI coding agents. This skill
distills that into rules for this specific app.

## Folder structure

```
app/
  src/
    main.rs            # asset consts, mod declarations, launch(), root App component
    components/        # reusable, presentational UI pieces
      mod.rs            # `mod x; pub use x::X;` re-exports only — no logic here
      home_screen.rs
    views/              # route-level screens (add once there is >1 screen + a Router)
      mod.rs
    audio/              # Phase 1+: cpal capture, resampling — plain Rust, no dioxus::prelude
      mod.rs
    transcription/       # Phase 2+: burn-onnx models, decode loop — plain Rust, no dioxus::prelude
      mod.rs
  assets/
    main.css             # global styles
    styling/<name>.css    # add a co-located stylesheet per component once main.css grows
```

Rules:
- One component per file. The file is `snake_case.rs`; it exports exactly one
  `PascalCase` component (or a small tightly-related family, e.g. a component
  and its private helpers).
- `components/mod.rs` and `views/mod.rs` only declare `mod` and `pub use` —
  never put component logic directly in a `mod.rs`.
- Don't create `views/` or add the `router` feature until there are genuinely
  multiple navigable screens (roadmap Phase 3 — recording view vs. transcript
  history). Until then a single root `App` rendering one component from
  `components/` is correct; adding a router earlier is premature abstraction.
- Platform/ML code that doesn't touch `dioxus::prelude` (cpal streams, the
  Burn/ONNX decode loop, WAV encoding) lives in its own module (`audio/`,
  `transcription/`), not inside a component file. Components call into it
  through plain function calls or a signal the module updates. This keeps the
  non-UI logic unit-testable with plain `cargo test`, independent of any
  renderer.

## Naming

- Files and modules: `snake_case`.
- Components and other types: `PascalCase`. A component is a function
  annotated `#[component]`; per Dioxus's own rule its name must start with a
  capital letter or contain an underscore.
- Signals/variables: `snake_case`, named for the value they hold, not the
  mechanism (`is_recording`, not `recording_signal`).

## Component & state rules (from Dioxus 0.7 / the template's own AGENTS.md)

- Props must be owned values (`String`, `Vec<T>`), never borrowed
  (`&str`, `&[T]`), and must implement `PartialEq` + `Clone`.
- Wrap a prop in `ReadOnlySignal<T>` when a child needs it to stay reactive
  and cheaply `Copy`.
- `use_signal` for local state, `use_memo` for derived/expensive values,
  `use_resource` for async work (mic permission requests, model loading),
  `use_context_provider` / `use_context` to share state (e.g. the recorder
  handle) down the tree instead of prop-drilling through every screen.
- In `rsx!`, prefer `for` loops and plain `if` over `.iter().map(...)` —
  iterators must be wrapped in `{}`, loops/conditionals don't need to be.
- Assets are declared with `asset!("/assets/...")` and linked via
  `document::Link` / `document::Stylesheet` — never a hardcoded path string.

## General engineering

- Keep components presentational: a component's body should read as
  "render this state"; anything that reaches outside the process (mic I/O,
  model inference, disk) belongs in `audio/`, `transcription/`, or a
  `storage/` module and gets called from a signal/effect, not inlined in
  `rsx!`.
- Extract a shared piece into `components/` as soon as a second screen needs
  it — not preemptively before a second use exists.
- Gate OS-specific code with `#[cfg(target_os = "ios")]` /
  `#[cfg(target_os = "android")]` inside the owning module (e.g. `audio/`),
  not scattered through UI code.
