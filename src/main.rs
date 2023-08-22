mod cpu;
mod flags_register;
mod memory_bus;
mod opcodes; // TODO: Decide which to use
mod opcodes_json; // TODO: Decide which to use
mod registers;
mod rom;

use clap::Parser;
use env_logger::Builder;
use log::{debug, error, info, LevelFilter};
use rom::Rom;

const ROM_FILEPATH: &str = "roms/cpu_instrs.gb";

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    rom_path: String,
}

fn main() {
    let cli = Cli::parse();

    let mut log_builder = Builder::from_default_env();
    log_builder.filter_level(LevelFilter::Trace).init(); // TODO: provide way to set this using env var

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
