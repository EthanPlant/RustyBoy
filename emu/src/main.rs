use std::path::PathBuf;

use clap::Parser;

use gblib::Gameboy;

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long)]
    rom: String,
}

fn get_cartridge(path: &PathBuf) -> Vec<u8> {
    let data = std::fs::read(path).expect("Failed to open ROM file");
    data
}

fn main() {
    let args = Args::parse();
    let _gb = Gameboy::new(get_cartridge(&PathBuf::from(args.rom)));
}
