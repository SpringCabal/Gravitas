# Gravitas — Core-WASM guest modules

Two guest crates, both compiled to `wasm32-unknown-unknown` and loaded through
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
  - `tests/`: the in-game e2e cases, built only with the `e2e` feature.
- `ui`: the widgets (HUD, camera, input, healthbars, fire particles, map
  edge, music). Each owns its state; `state.rs` holds the player unit and
  the HUD, and the fire particle state.

The rules send the HUD status (`GRAVITAS_STATUS|...`) when it changes and
effect messages (`GRAVITAS_FX|...`); the ui sends the player's input when it
changes.

## Commands

    just fmt      # cargo fmt --all
    just clippy   # clippy over the workspace, warnings denied
    just build    # fmt + clippy, then a release build
    just copy     # build, then install both .wasm files into the game

`just copy` is the one to run after editing; the game loads the copied
artifacts, not the ones under `target/`.

## Running

    ../../../../run-wasm.sh --game gravitas
