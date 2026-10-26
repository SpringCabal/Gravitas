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

The default renderer evaluates curves and billboards on the GPU, matching
Lua's particle equations. Persistent instance buffers store particle birth
times; only newly emitted records are uploaded. Four texture groups retain
live particles in one pass. Buffer compaction reuses capacity and preserves
render order. Flame, smoke and heat distortion use the same uncapped budget
as the Lua verification fixture. The five-frame image probe compares the
complete fire and heat footprint against Lua.

`fire_renderer=cpu` keeps the CPU reference renderer and its immediate fallback.
Both paths release buffers and shaders at shutdown. Errors are logged.

## Optional Rust jobs

`rust_jobs=off` is the default. With `fire_renderer=cpu`, `rust_jobs=on`
calculates particle curves and billboard axes in same-tick jobs;
`rust_jobs=check` also compares worker output with inline computation.
The default GPU renderer has no CPU curve work to dispatch, so enabling jobs
there reports no active game jobs. Compare CPU jobs off/on at identical
`--worker-threads 1|4`; compare Lua and Rust using their GPU defaults.

Release builds use whole-program LTO and one code generation unit.
