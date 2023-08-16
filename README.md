# RustyBoy

![Tests](https://github.com/EthanPlant/RustyBoy/actions/workflows/test.yml/badge.svg)

A Gameboy emulator written in Rust. It is currently WIP. The end goal at the moment is to get the first generation Pokemon games playable. This project was mostly written for fun and as a learning opportunity and so an emphasis is placed on clean code and usability over cycle accuracy. 

## Running RustyBoy
`./rustyboy --rom <ROM>`

Two optional flags may also be specified:

- `-o, --objects` - Show the sprite data
- `-t, --tiles` - Show the tilemap

## Project Structure
RustyBoy is split up into two seperate Rust crates in order to keep the emulator itself seperate from any frontend implementation. All code pertaining to the emulation itself is found in `rustboy-core`, while the frontend is contained within `rustyboy`.

## Tests
RustyBoy is developed with a comprehensive test suite to ensure correctness and avoid regression. These tests are ran on every commit, and can be ran automaticall with `cargo test`

## Logging
RustyBoy uses `env-logger` to output logs. By default `INFO`, `WARN`, and `ERROR` logs will be shown. The `RUST_LOG` environment variable can be set to modify the log level.

Setting `RUST_LOG` to `trace` will cause RustyBoy to output all logs (this will flood the console with a lot of text), setting `RUST_LOG` to `off` can be used to disable logging entirely if desired. For more information on possible log settings see the [env-logger](https://docs.rs/env_logger/latest/env_logger/#enabling-logging) documentation.

## Building
After cloning the repository, simply run `cargo build` in the workspace directory.

## Roadmap
- [x] CPU
    - [x] Registers and flags
    - [x] Implement instructions
    - [x] Interrupts
    - [x] Pass the boot ROM
    - [x] Pass all Blargg test ROMs
    - [ ] Implement STOP instruction
- [ ] Memory
    - [x] Proper memory map
    - [ ] Memory mapped IO
    - [ ] OAM DMA
- [x] Timer
- [ ] Cartridges
    - [x] No MBC
    - [x] MBC-1
    - [ ] MBC-5
- [x] Input
- [x] Graphics
    - [x] Draw Background
    - [x] Background scrolling
    - [x] Draw window
    - [x] Draw sprites
    - [x] Handle flipped sprites
    - [x] Handle 8x16 sprites
    - [x] Correct ordering of sprite and background tiles
    - [x] Pass DMG-acid test
- [ ] Audio
    - [ ] Square channels
    - [ ] Volume envelope
    - [ ] Frequency sweeping
    - [ ] Wave channel
    - [ ] Noise channel
- [ ] Frontends
    - [x] Desktop frontend
    - [ ] WASM frontend
- [ ] QoL Features
    - [ ] Savestates
    - [ ] Rewind
    - [ ] Speed up
    - [ ] Debugger

## Working Games
- [ ] Tetris
- [ ] Dr. Mario
- [ ] Link's Awakening
- [ ] Kirby's Dream Land
- [ ] Super Mario Land
- [ ] Pokemon Red/Blue

## Working Test ROMs
- [x] Gameboy boot ROM
- [x] Blargg cpu_instrs
- [x] Blargg instr_timing
- [x] Mooneye daa
