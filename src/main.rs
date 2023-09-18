use std::path::PathBuf;

use clap::Parser;
use env_logger::Env;
use log::{debug, error, info};

use rustboy::cpu::opcodes_json::Opcodes;
use rustboy::rom::Rom;

const DEFAULT_LOG_LEVEL: &str = "debug";
const ROM_FILEPATH: &str = "roms/cpu_instrs.gb";

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Sets a path to the ROM file to run
    #[arg(short, long, value_name = "FILE")]
    rom_path: Option<PathBuf>,

    /// Sets the render resolution scaling in range [1-10]
    #[arg(short, long, default_value_t = 1)]
    render_scale: u8,
}

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or(DEFAULT_LOG_LEVEL)).init();

    // let cli = Cli::parse(); // TODO: implement CLI

    // TODO: move this part of code
    let opcodes = Opcodes::new();
    match opcodes.unprefixed.get("0x00") {
        Some(opcode_details) => {
            info!("{:#?}", opcode_details);
        }
        None => panic!("Unable to get opcode details from HashMap"),
    }

    debug!("Loading ROM from file: {}", ROM_FILEPATH);
    match Rom::load_from_file(ROM_FILEPATH) {
        Ok(rom) => {
            info!("Successfully loaded ROM from file: {}", ROM_FILEPATH);
            match rom.title() {
                Ok(title) => info!("Parsed ROM title: {}", title),
                Err(err) => error!("Error when parsing ROM title: {}", err),
            }
        }
        Err(err) => {
            error!(
                "Error when loading ROM from file: {}, error: {}",
                ROM_FILEPATH, err
            );
        }
    }
}
