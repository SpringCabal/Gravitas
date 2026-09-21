# Gravitas — Core-WASM guest modules

Two guest crates, both compiled to `wasm32-unknown-unknown` and loaded through
`LuaRules/wasm/manifest.txt` and `LuaUI/wasm/manifest.txt`:

| crate          | environment    | ships as                          |
| -------------- | -------------- | --------------------------------- |
| `rules-synced` | `rules-synced` | `LuaRules/wasm/gravitas-rules.wasm` |
| `ui`           | `ui`           | `LuaUI/wasm/gravitas-ui.wasm`       |

## Commands

    just fmt      # cargo fmt --all
    just clippy   # clippy over the workspace, warnings denied
    just build    # fmt + clippy, then a release build
    just copy     # build, then install both .wasm files into the game

`just copy` is the one to run after editing; the game loads the copied
artifacts, not the ones under `target/`.

## Running

    ../../../../run-wasm.sh --game gravitas
