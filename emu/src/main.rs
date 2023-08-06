use clap::Parser;

use gblib::Gameboy;

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long)]
    rom: String,
}

fn main() {
    let args = Args::parse();
    let _gb = Gameboy::new(&args.rom);
}
