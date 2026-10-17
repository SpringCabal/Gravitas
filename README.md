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
