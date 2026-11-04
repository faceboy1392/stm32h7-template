hi its me liam

## Installation

First, install `rustup` (follow https://rustup.rs/), Rust's toolchain manager. This also installs Rust itself and Cargo (Rust's official package manager and build system).

On windows, it may prompt you to install C++ built tools. This might take some time.

Then run:
```sh
rustup target add thumbv7em-none-eabihf # compilation target for our CPU
cargo install probe-rs-tools --locked # probe-rs helps flash and debug code to the board
```
These are global on your computer. On Linux, you need to configure udev rules so that probe-rs has the right permissions. Follow https://probe.rs/docs/getting-started/probe-setup/

If you are using vscode (recommended for rust), you are also going to want to get the [rust-analyzer extension](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

Now clone this repo:
```bash
git clone https://github.com/faceboy1392/stm32h7-template
cd stm32h7-template
```
To flash the code to the board, you run:
```bash
cargo run --release
```

## Project structure

In case you are unfamiliar with embedded rust or cargo project structures:

- `src/main.rs` our actual code
- `target/` build output
- `Cargo.toml` defines our project and its dependencies
  - `cortex-m` and `cortex-m-rt` provide necessary utilities for our processor architecture
  - `defmt` and `defmt-rtt` help send debug logs
  - `embassy-stm32` is our hardware abstraction layer and gives us access to things like peripherals, all strongly typed
  - `embassy-executor` helps set up our asynchronous runtime. tl;dr: whenever you `.await` something, the executor has a chance to pass control over to another task while that first thing is awaited, giving you single threaded concurrency.
  - `embassy-time` has utilities for timing, such as delays
  - `panic-probe` is a panic handler. Panics in rust are generally for unrecoverable errors, and panic-probe helps send info about that panic over the debug connection
- `Cargo.lock` is auto generated to keep track of dependency chains
- `.cargo/config.toml` tells cargo what to do when you run `cargo run`, in this case flashing the code to the chip using `probe-rs` with the appropriate flags.
- `memory.x` defines our memory layout. Not instantiating our variables or anything, just defining the overall regions of memory our processor has available.
- `build.rs` makes sure the linker finds memory.x and passes it the linker scripts it needs. read the comments in it for more details
- `.vscode/settings.json` clarifies some embedded environment details so rust-analyzer in vscode doesn't get confused
