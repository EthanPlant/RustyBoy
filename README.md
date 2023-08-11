# RustyBoy

[![codecov](https://codecov.io/gh/EthanPlant/RustyBoy/branch/main/graph/badge.svg?token=AXDXVSM1SA)](https://codecov.io/gh/EthanPlant/RustyBoy)
![Tests](https://github.com/EthanPlant/RustyBoy/actions/workflows/test.yml/badge.svg)

A Gameboy emulator written in Rust. It is currently WIP. The end goal at the moment is to get the first generation Pokemon games playable. This project was mostly written for fun and as a learning opportunity and so an emphasis is placed on clean code and usability over cycle accuracy. 

## Running RustyBoy
`./rustyboy --rom <ROM>`

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
- [ ] CPU
    - [x] Registers and flags
    - [ ] Implement instructions
    - [ ] Interrupts
    - [ ] Pass the boot ROM
    - [ ] Pass all Blargg test ROMs
        - [x] 01-special
        - [ ] 02-interrupts
        - [x] 03-op sp,hl
        - [x] 04-op r,imm
        - [x] 05-op rp
        - [x] 06-ld r,r
        - [x] 07-jr,jp,call,ret,rst
        - [x] 08-misc instrs
        - [x] 09-op r,r
        - [x] 10-bit ops
        - [ ] 11-op a,(hl)
- [ ] Memory
    - [x] Proper memory map
    - [ ] Memory mapped IO
    - [ ] OAM DMA
- [ ] Timer
- [ ] Cartridges
    - [x] No MBC
    - [ ] MBC-1
    - [ ] MBC-5
- [ ] Input
- [ ] Graphics
    - [ ] Draw Background
    - [ ] Background scrolling
    - [ ] Draw window
    - [ ] Draw sprites
    - [ ] Handle flipped sprites
    - [ ] Handle 8x16 sprites
    - [ ] Correct ordering of sprite and background tiles
    - [ ] Pass DMG-acid test
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

## Working ROMs
- [x] Gameboy boot ROM
- [ ] Blargg tests
- [ ] Tetris
- [ ] Dr. Mario
- [ ] Link's Awakening
- [ ] Kirby's Dream Land
- [ ] Super Mario Land
- [ ] Pokemon Red/Blue