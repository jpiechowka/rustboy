# RustBoy

RustBoy is a GameBoy emulator written in Rust.

## Features

* Implemented fully in Rust with no unsafe code blocks
* Parsing GB opcodes from JSON (https://gbdev.io/gb-opcodes/Opcodes.json) using `serde` and `serde_json`
  * file is included in the binary using `include_str` macro
  * opcodes are stored as two `HashMap<String, OpcodeDetails>` (`unprefixed` and `cbprefixed` opcodes)
  * TODO: info on fetching
* Logging using `log` and `env_logger` crates
  * Can be controlled by environment variables like `RUST_LOG`
  * For configuration details see https://docs.rs/env_logger/latest/env_logger/

## Left to do / implement

* CLI with configurable arguments using `clap`
* Mapping memory from cartridge to RAM
* Sound
* Graphics (with upscaling)
* Real time overview of executed instructions and CPU registers
* Cheats support
* Automated compiling and releasing binaries for all operating systems
* Browser support
* Do some basic performance optimizations
  * Add benchmarks
  * Use vector or some other structure for opcodes instead of HashMap (or use faster hashing algo)
  * Review and inline hot functions (https://nnethercote.github.io/perf-book/inlining.html)
  * Use String replacement that uses stack instead of allocating on heap
  * Use parallel iterators (rayon crate)
* Everything else

## Building the emulator

Install Rust (https://www.rust-lang.org/tools/install), then run the commands below:

```
git clone https://github.com/jpiechowka/rustboy.git
cd rustboy
cargo build --release
```

### Using RUSTFLAGS env variable

If you do not care that much about the compatibility of your binary on older (or other types of) processors, you can tell the compiler to generate the newest (and potentially fastest) instructions specific to a certain CPU architecture by using `RUSTFLAGS`environment variable (https://nnethercote.github.io/perf-book/build-configuration.html#cpu-specific-instructions)

```
RUSTFLAGS="-C target-cpu=native" cargo build --release
```

The full command to clone and build will be:

```
git clone https://github.com/jpiechowka/rustboy.git
cd rustboy
RUSTFLAGS="-C target-cpu=native" cargo build --release
```

On Windows you need to follow this pattern: https://superuser.com/a/1049433

## Running the emulator

TODO: Provide details, flags and arguments for the CLI

## Test ROMs

* https://github.com/c-sp/gameboy-test-roms

## Learning resources

* https://en.wikipedia.org/wiki/Game_Boy

### GameBoy emulator development

* https://youtu.be/B7seNuQncvU
* https://youtu.be/HyzD8pNlpwI
* https://rylev.github.io/DMG-01
* https://github.com/rylev/DMG-01
* https://gbdev.io/
* https://gbdev.io/resources.html
* https://gbdev.io/gb-opcodes/optables/
* https://gbdev.io/gb-opcodes/Opcodes.json
* https://imrannazar.com/GameBoy-Emulation-in-JavaScript:-The-CPU
* https://read.cv/mehdi/uNGQ7pgWb2CO1QfJkb1n
* https://yushiomote.org/posts/gameboy-emu
* https://cturt.github.io/cinoop.html
* https://www.slideshare.net/youtang5/gameboy-emulator-in-rust-and-web-assembly

### Rust resources

* https://doc.rust-lang.org/stable/book
* https://doc.rust-lang.org/rust-by-example
* https://github.com/rust-lang/rustlings/
* https://doc.rust-lang.org/std/index.html
* https://nnethercote.github.io/perf-book/
* https://www.zero2prod.com

## License

RustBoy is free, open source and permissively licensed! Except where noted (below and/or in individual files), all code in this repository is dual-licensed under either:

* MIT License (`LICENSE-MIT` file or http://opensource.org/licenses/MIT)
* Apache License, Version 2.0 (`LICENSE-APACHE` file or http://www.apache.org/licenses/LICENSE-2.0)

at your option. This means you can select the license you prefer! This dual-licensing approach is the de-facto standard in the Rust ecosystem.

## Contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
