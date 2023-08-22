# RustBoy

RustBoy is a GameBoy emulator written in Rust.

## Features

* Logging using `log` and `env_logger` crates
* Parsing GB opcodes from JSON (https://gbdev.io/gb-opcodes/Opcodes.json) using `serde`
  * file is included in the binary using `include_str` macro
  * opcodes are stored as two `HashMap<String, OpcodeDetails>` (`unprefixed` and `cbprefixed` opcodes)
  * lookup can be done using `.get()` (https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.get)

## Left to do / implement

* CLI with configurable arguments using `clap`
* Sound
* Graphics (with upscaling)
* Automated compiling and releasing binaries for all operating systems
* Everything else

## Building the emulator

Install Rust (https://www.rust-lang.org/tools/install), then run the commands below:

```
git clone https://github.com/jpiechowka/rustboy.git
cd rustboy
cargo build --release
```

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

### Rust resources

* https://doc.rust-lang.org/stable/book
* https://doc.rust-lang.org/rust-by-example
* https://github.com/rust-lang/rustlings/
* https://doc.rust-lang.org/std/index.html
* https://www.zero2prod.com

## License

RustBoy is free, open source and permissively licensed! Except where noted (below and/or in individual files), all code in this repository is dual-licensed under either:

* MIT License (`LICENSE-MIT` file or http://opensource.org/licenses/MIT)
* Apache License, Version 2.0 (`LICENSE-APACHE` file or http://www.apache.org/licenses/LICENSE-2.0)

at your option. This means you can select the license you prefer! This dual-licensing approach is the de-facto standard in the Rust ecosystem.

## Contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
