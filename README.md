# Gravitas WASM

Rust/WASM port of SpringCabal’s Ludum Dare game Gravitas.

Use the [SpringCabal development repository](https://github.com/gajop/springcabal-dev) for engine setup, launching, builds and verification.

## Differences from Lua

- The HUD uses RmlUi instead of Chili.
- Restart resets the current level inside the rules module instead of restarting the engine.

## Scoped SDK port

Engine calls use `spring-sdk` and explicit Spring references. The game and its
unit scripts share the addon module. Queued commands receive a fresh reference,
and unit-script tasks acquire one only for their synchronous engine calls.
Synced random draw order is preserved.

Verification in the dedicated port worktree:

- All 19 in-game cases pass on Linux and Wine, including the real level,
  collisions, shooting, jump, death, respawn and restart.
- Nine shared Rust tests and strict default/all-feature Clippy pass. The release
  build contains no test cases and completes the normal level headlessly.
- The real-input suite passes movement, four jump controls, push, pull, pushing
  just beyond weapon range, and Ctrl+Q back to the menu on Linux and Wine.
- That last aiming case also failed before the SDK port. The fix restores the
  Lua gun's three-dimensional direction and weapon range test instead of
  flattening a distant endpoint to terrain height. The test's assertions and
  thresholds are unchanged.
- The visual arena and fire checks complete. Manually measured arena/HUD pixels
  meet their existing limits, but the suite rejects the installed Mesa package
  revision `.4` versus recorded `.2`. No goldens or limits were changed.

The Wine control run passes with software OpenGL. The Zink run crashed before
gameplay; that driver path is still unresolved. Native Windows remains untested.

## Fire rendering

Flame and smoke quads use a reusable packed vertex stream and a compatibility
vertex shader. Texture order, premultiplied blending, depth testing, fog and
the separate heat-distortion pass are retained. Native buffers are defined
once and replaced when capacity grows; the widget releases them on shutdown.
The immediate path remains available if stream creation or drawing fails,
with the API error logged. The development repository's existing five-frame
fire parity probe checks the full fire/heat footprint against Lua.

Release builds use whole-program LTO and one code generation unit. The
workspace profile applies to both installed modules and verification builds.

## Optional Rust jobs

The default `rust_jobs=off` keeps the serial path. Set the game modoption
`rust_jobs=on` to calculate particle sampling and billboard axes in same-tick jobs.
`rust_jobs=check` also compares worker output with inline computation; it is
for verification, not timing. RNG, engine calls and result application keep
their original order. A dedicated worker module has an 8 MiB fixed memory.

For measurements, use the springcabal-dev performance runner with
`--rust-jobs off|on|check --worker-threads 1|4`; compare jobs at the same
engine pool width.
