use clap::Parser;
use env_logger::Env;
use pixels::{Pixels, SurfaceTexture};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use rustyboy_core::{gameboy::Gameboy, ppu::ppu::Color};

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long)]
    rom: String,
}

fn generate_pixels(frame: &mut [u8], &framebuffer: &[Color; 160 * 144]) {
    for (i, pixel) in frame.chunks_exact_mut(3).enumerate() {
        let color = framebuffer[i / 3];
        let mut rgb = match color {
            Color::White => [255, 255, 255],
            Color::LightGray => [192, 192, 192],
            Color::DarkGray => [96, 96, 96],
            Color::Black => [0, 0, 0],
        };
        pixel.copy_from_slice(&mut rgb);
    }
}

fn main() {
    let args = Args::parse();

    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("RustyBoy")
        .with_inner_size(winit::dpi::LogicalSize::new(160 * 4, 144 * 4))
        .build(&event_loop)
        .unwrap();

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(160, 144, surface_texture).unwrap()
    };

    // If no log level is specified, default to info or above
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let mut gb = Gameboy::new(&args.rom);
    Window::set_title(
        &window,
        ("RustyBoy - ".to_owned() + &gb.mmu.cart_title).as_str(),
    );

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window.id() == window_id => *control_flow = ControlFlow::Exit,
            _ => (),
        }

        gb.step();
        generate_pixels(pixels.frame_mut(), &gb.mmu.ppu.frame_buffer);
        pixels.render().expect("Failed to render!");
    });
}
