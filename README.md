# screeps-arena-starter-rust

Starter Rust AI for [Screeps: Arena][screeps-arena], the JavaScript-based programming
strategy game.

This uses the [`screeps-arena-game-api`] bindings from the [rustyscreeps] organization.

It's also recommended to use [`cargo-screeps`] for building the code.

The documentation is currently a bit sparse. API docs which list functions one
can use are located at https://docs.rs/screeps-arena-game-api/.

Almost all crates on https://crates.io/ are usable (only things which interact with OS
apis are broken).

Quickstart:

```sh
# cli dependencies:
# TEMPORARY - get the arena branch of the cargo-screeps tool, which supports arena
git clone https://github.com/rustyscreeps/cargo-screeps.git
cd cargo-screeps
git checkout arena
cargo install --path .
cd ..
# TEMPORARY once arena is merged, go back to simply:
cargo install cargo-screeps

# clone:
git clone https://github.com/rustyscreeps/screeps-arena-starter-rust.git
cd screeps-arena-starter-rust

# configure with the path to your arena code directory:
cp example-screeps.toml screeps.toml
nano screeps.toml

# build tool:
cargo screeps --help
# check the selected arena
cargo check --target wasm32-unknown-unknown --no-default-features --features season4-pain_and_gain
# compile plus deploy Pain and Gain using the example's pag mode
cargo screeps deploy -m pag
# or compile plus deploy Capture the Flag with its separate strategy
cargo screeps deploy -m ctf
```

## Arena selection

Cargo enables `season4-pain_and_gain` by default, including for editor analysis.
Exactly one of `season4-pain_and_gain`, `arena-capture-the-flag`,
`arena-spawn-and-swamp`, or `arena-collect-and-control` must be enabled.
Missing or conflicting selections remain compile errors.

To select another arena, disable the default first:

```sh
cargo check --no-default-features --features arena-capture-the-flag
```

`season4-pain_and_gain` enables `screeps-arena-game-api/arena-capture-the-flag`
because those API bindings are shared. Cargo dependency features do **not**
enable the bot's `arena-capture-the-flag` feature. Keep strategy gates on this
crate's features, not on the dependency's binding names:

- `src/arenas/pain_and_gain.rs`: compiled only with `season4-pain_and_gain`.
  Contains the existing move/attack logic, now with enemy-spawn lookup; owned
  creeps act only when a spawn explicitly has `my == false`.
- `src/arenas/capture_the_flag.rs`: compiled only with
  `arena-capture-the-flag`. Currently reports creep counts; there was no CTF
  gameplay strategy to migrate.
- `src/lib.rs`: shared logging, arena information, and tick dispatch.
  Spawn and Swamp / Collect and Control retain the shared diagnostics.

To inspect the enabled bindings:

```sh
cargo tree -e features --no-default-features --features season4-pain_and_gain -i screeps-arena-game-api
```

On tick 1, the bot logs `compiled arena feature: season4-pain_and_gain` when that
strategy was compiled. This identifies the binary's selection, not the arena
the game is hosting; `arena_info` is logged separately.

### cargo-screeps configuration

`cargo screeps build` reads only the top-level `[build]` options. It does not
read `[pag.build]`, even when `default_deploy_mode = "pag"`. Deployment merges
the selected mode's build options with `[build]`.

The local `screeps.toml` selects Pain and Gain in `[build]`, so both
`cargo screeps build` and `cargo screeps deploy -m pag` use it:

```toml
[build]
build_mode = "arena"
out_name = "screeps-arena-bot"
extra_options = ["--no-default-features", "--features=season4-pain_and_gain"]
```

For multiple deployment modes, use `example-screeps.toml`: leave arena features
out of `[build]` and select one under each mode's `[MODE.build]` with
`--no-default-features`. Use `cargo screeps deploy -m pag` or `-m ctf`;
a standalone `cargo screeps build` uses the Cargo default (Pain and Gain).
Do not put one arena feature globally and a different one in a mode: extra
options merge rather than replace, and the conflicting selection will fail.

`out_name = "screeps-arena-bot"` matches the imports in `javascript/main.mjs`.

### Neovim / rust-analyzer

rust-analyzer does not read `screeps.toml`, but it reads Cargo's default features.
`Cargo.toml` enables `season4-pain_and_gain` by default, so Neovim needs no
project-specific feature override to analyze Pain and Gain.

The former repository-path-based Rustaceanvim override has been removed from
the local AstroNvim configuration. Restart Neovim to reload that configuration.
Leave `cargo.noDefaultFeatures` and `cargo.allFeatures` disabled.

When analyzing another arena, set rust-analyzer's `cargo.noDefaultFeatures`
to `true` and `cargo.features` to that single arena feature. Restore Cargo
defaults when returning to Pain and Gain. Do not enable all features: the
arena strategies are intentionally mutually exclusive.


[screeps-arena]: https://store.steampowered.com/app/1137320/Screeps_Arena/
[`cargo-screeps`]: https://github.com/rustyscreeps/cargo-screeps/
[`screeps-arena-game-api`]: https://github.com/rustyscreeps/screeps-arena-game-api/
[rustyscreeps]: https://github.com/rustyscreeps/
