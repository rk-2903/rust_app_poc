# Vendored patch: burn-std 0.22.0-pre.3

`burn-src/crates/burn-std` is `tracel-ai/burn`'s `burn-std` crate at tag
`v0.22.0-pre.3`, with one function patched:
`TensorData::into_vec_unchecked`'s misaligned-bytes fallback
(`src/data/tensor/conversion.rs`) retried the exact same zero-copy cast
that had just failed, which can't succeed since the misalignment is a
property of the source buffer, not of the cast call. This surfaced as a
hard panic (`PodCastError(TargetAlignmentGreaterAndInputNotAligned)`) when
loading Moonshine's burnpack (`.bpk`) weight files on `burn-ndarray`.

The fix replaces that fallback with an actual memcopy into a freshly
(correctly) aligned buffer. `Cargo.toml`'s workspace-inherited dependency
versions were also resolved to explicit ones (pulled from the same tag's
workspace root `Cargo.toml`) since this is no longer built inside the
`tracel-ai/burn` workspace.

Wired in via `[patch.crates-io]` in `app/Cargo.toml`. **Remove this vendor
directory and that patch entry once a burn release ships with the real
fix** (this function, on `main`, as of this writing) — check
`crates/burn-std/src/data/tensor/conversion.rs` upstream before deleting.
