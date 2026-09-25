# Gravitas — Core-WASM guest modules

Two guest crates and the messages they share, compiled to `wasm32-unknown-unknown` and loaded through
`LuaRules/wasm/manifest.txt` and `LuaUI/wasm/manifest.txt`:

| crate          | environment    | ships as                          |
| -------------- | -------------- | --------------------------------- |
| `rules-synced` | `rules-synced` | `LuaRules/wasm/gravitas-rules.wasm` |
| `ui`           | `ui`           | `LuaUI/wasm/gravitas-ui.wasm`       |

## Layout

- `rules-synced`: the game rules and the unit scripts, in one module.
  - `gadgets/`: one gadget per rule (level, player, jump, fire, electric
    fields, respawn, plates and gates, ...). Each owns its state; `state.rs`
    holds the few parts several gadgets share.
  - `unit_scripts/`: the Core-CUS module with the scenery, robot and Gravit
    scripts. The gadgets attach scripts and call them directly through
    `with_cus_module`.
  - `level.rs` and `unit_scripts/gravit_animation_data.rs` are data: the
    scenario's units, areas and plate links, and the Gravit keyframes.
  - `tests/`: the in-game e2e cases, built only with the
    `headless-integration` feature (`e2e`, the visual build, adds the
    screenshots). They run in an arena that replaces the level, each case in
    its own rig, in phases (`tests/harness.rs`): the rig cases together, then
    the scene case, which checks every unit's place on the frame it asks for
    the screenshot, then a restart, then the last case restarts into the real
    level and checks it. Each case waits for what it needs rather than for a
    frame. Input, restarts and screenshots go through the ui module
    (`ui/src/shots.rs`), which the headless run loads too.
- `ui`: the widgets (HUD, camera, input, healthbars, fire particles, map
  edge, music). Each owns its state; `state.rs` holds the player unit, the
  HUD, the game clock and the fire particles.
- `shared`: the messages between them, host-tested (`just test`). The rules
  send the HUD status when it changes and the fire and electric effects; the
  ui sends the player's input when it changes. `shared/src/e2e.rs` has the
  e2e scenario's requests.

## Differences from the Lua version

- There is a restart (HUD button after the game ends): the Lua game had
  none. Units destroyed by a restart die without explosions, respawns or a
  game over.
- The fire and electric-field effects are drawn by the ui module; the Lua
  game used LUPS. The HUD is RmlUi instead of Chili.
- A robot deciding whether to flee measures its distance to the hazard in
  3D. `unit_robots.lua` has a typo (`(y-ax)*(y-ay)`) in that distance; it
  only changes how far the robot looks for other hazards.
- The robots' escape search reads an electric field's `elecSize`;
  `unit_robots.lua` drops that value and uses 100. The level's fields are
  all size 100, so this changes nothing in play.
- `game_fall_damage.lua` pairs up collisions that have no attacker by their
  damage. On this engine only feature collisions lack one, so the port
  skips the pairing and gives each the 2.5× damage Lua gives an unmatched one.
- The camera looks 9 frames of the player's velocity ahead;
  `shoot_n_run.lua` looks 30 frames of a smoothed velocity, times the game
  speed, ahead, which puts the player near the screen edge while walking.
- A jump with `jumpSpeed` 0 is refused; in Lua (5.1) its wall check loops
  forever and the game hangs.

## Commands

    just fmt      # cargo fmt --all
    just clippy   # clippy over the workspace, warnings denied
    just test     # host tests of the rules-ui messages (shared/)
    just build    # fmt + clippy, then a release build
    just copy     # build, then install both .wasm files into the game

`just copy` is the one to run after editing; the game loads the copied
artifacts, not the ones under `target/`.

## Running

    ../../../../run-wasm.sh --game gravitas
