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

fn generate_tiles(frame: &mut [u8], vram: &[u8]) {
    for i in 0..384 {
        let tile = &vram[i * 16..(i + 1) * 16];
        let tile_x = (i % 12) * 8;
        let tile_y = (i / 12) * 8;
        for j in 0..8 {
            let lo = tile[j * 2];
            let hi = tile[j * 2 + 1];
            for k in 0..8 {
                let color = match ((hi >> (7 - k)) & 1, (lo >> (7 - k)) & 1) {
                    (0, 0) => Color::White,
                    (0, 1) => Color::LightGray,
                    (1, 0) => Color::DarkGray,
                    (1, 1) => Color::Black,
                    _ => unreachable!(),
                };
                let mut rgb = match color {
                    Color::White => [255, 255, 255, 255],
                    Color::LightGray => [192, 192, 192, 255],
                    Color::DarkGray => [96, 96, 96, 255],
                    Color::Black => [0, 0, 0, 255],
                };
                let pixel_x = tile_x + k;
                let pixel_y = tile_y + j;
                let pixel_index = (pixel_y * 12 * 8 + pixel_x) * 4;
                frame[pixel_index..pixel_index + 4].copy_from_slice(&mut rgb);
            }
        }
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

    let tile_window = WindowBuilder::new()
        .with_title("Tiles")
        .with_inner_size(winit::dpi::LogicalSize::new(96 * 3, 256 * 3))
        .build(&event_loop)
        .unwrap();

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(160, 144, surface_texture).unwrap()
    };

    let mut tile_pixels = {
        let window_size = tile_window.inner_size();
        let surface_texture =
            SurfaceTexture::new(window_size.width, window_size.height, &tile_window);
        Pixels::new(12 * 8, 32 * 8, surface_texture).unwrap()
    };

    tile_pixels.render().expect("Failed to render tiles!");

    // If no log level is specified, default to info or above
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let mut gb = Gameboy::new(&args.rom);
    Window::set_title(
        &window,
        ("RustyBoy - ".to_owned() + &gb.mmu.cart_title).as_str(),
    );

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::RedrawRequested(_) => {
                generate_pixels(pixels.frame_mut(), &gb.mmu.ppu.frame_buffer);
                pixels.render().expect("Failed to render!");
                if gb.mmu.ppu.vram_changed {
                    generate_tiles(tile_pixels.frame_mut(), &gb.mmu.ppu.vram);
                    tile_pixels.render().expect("Failed to render tiles!");
                    gb.mmu.ppu.vram_changed = false;
                }
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window.id() == window_id => *control_flow = ControlFlow::Exit,

            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                window_id,
            } if window.id() == window_id => pixels
                .resize_surface(size.width, size.height)
                .expect("Failed to resize surface!"),

            _ => (),
        }

        gb.step();
        window.request_redraw();
    });
}
