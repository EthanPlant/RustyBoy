use clap::Parser;
use env_logger::Env;

use rustyboy_core::gameboy::Gameboy;

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long)]
    rom: String,
}

fn main() {
    let args = Args::parse();

    // If no log level is specified, default to info or above
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let mut gb = Gameboy::new(&args.rom);
    loop {
        gb.step();
    }
}
