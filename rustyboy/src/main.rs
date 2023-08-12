
use clap::Parser;
use env_logger::Env;
use winit::{window::WindowBuilder, event_loop::{ControlFlow, EventLoop}, event::{Event, WindowEvent}};

use rustyboy_core::gameboy::Gameboy;

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long)]
    rom: String,
}

fn main() {
    let args = Args::parse();

    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("RustyBoy")
        .with_inner_size(winit::dpi::LogicalSize::new(160, 144))
        .build(&event_loop)
        .unwrap();

    // If no log level is specified, default to info or above
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let mut gb = Gameboy::new(&args.rom);
    
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        gb.step();

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window.id() == window_id => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}
