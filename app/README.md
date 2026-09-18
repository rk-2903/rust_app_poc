# Development

```
app/
├─ assets/            # CSS and other static assets
├─ src/
│  ├─ main.rs          # entry point: asset consts, mod declarations, launch(), root App component
│  ├─ components/      # reusable UI pieces (see ../.claude/skills/dioxus-conventions)
├─ Cargo.toml          # dependencies and feature flags
├─ Dioxus.toml         # bundling config, incl. the [permissions] block (mic access)
```

Follow [.claude/skills/dioxus-conventions](../.claude/skills/dioxus-conventions/SKILL.md) when adding
screens or restructuring `src/`.

## Running the app

Make sure `dx`/`cargo` are on your PATH first (new terminal tab, or `source ~/.zshrc`).

### iOS Simulator

```bash
cd app
dx serve --platform ios
```

This boots/uses the default simulator, builds, and installs automatically.

### Real iPhone

One-time only, then it just works:

```bash
cd app
dx serve --platform ios --device "Rahul's iPhone"
```

`dx serve` builds, installs, launches, and hot-reloads on save. For a one-shot
build without the dev server / hot reload:

```bash
cd app
dx build --platform ios --device "Rahul's iPhone"
xcrun devicectl device install app --device 64678476-AD79-533A-9460-21572F4D6A56 target/dx/app/debug/ios/App.app
xcrun devicectl device process launch --device 64678476-AD79-533A-9460-21572F4D6A56 com.rahulkumar.conversationcapture
```

(Find your device's identifier with `xcrun devicectl list devices` if it
changes, e.g. after a phone reset.)

#### First-time device signing setup (already done on this Mac)

A **free** Apple ID (no paid Apple Developer Program) can't get a
provisioning profile issued through the command line alone — Apple only
issues one when Xcode itself requests it. One-time bootstrap, already done
here:

1. Created a throwaway Xcode project (`~/Desktop/workspace/Temp`) with
   **Bundle Identifier** set to exactly `com.rahulkumar.conversationcapture`
   (must match `bundle.identifier` in `Dioxus.toml`) and Team set to the
   Personal Team for `rahulkumar6611@gmail.com`.
2. Ran it once on the real iPhone (Product → Destination → device → ⌘R),
   trusted the dev certificate on the phone under Settings → General → VPN
   & Device Management.
3. That caused Xcode to request and cache a provisioning profile for
   `com.rahulkumar.conversationcapture` under
   `~/Library/Developer/Xcode/UserData/Provisioning Profiles/`, which `dx`
   now reuses automatically for every build — no need to repeat this for
   day-to-day builds.

**Free Apple ID certs/profiles expire after 7 days.** When `dx build`
starts failing with a codesigning/provisioning error again, redo step 2
only (open `~/Desktop/workspace/Temp/Temp.xcodeproj`, run it once on the
iPhone again) — the bundle ID is already correct, so this takes under a
minute.
