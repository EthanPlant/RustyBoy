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
    #[arg(short, long)]
    tiles: bool,
    #[arg(short, long)]
    objects: bool,
}

fn generate_pixels(frame: &mut [u8], &framebuffer: &[Color; 160 * 144]) {
    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        let color = framebuffer[i];
        let mut rgb = match color {
            Color::White => [255, 255, 255, 255],
            Color::LightGray => [192, 192, 192, 255],
            Color::DarkGray => [96, 96, 96, 255],
            Color::Black => [0, 0, 0, 255],
        };
        pixel.copy_from_slice(&mut rgb);
    }
}

fn generate_tiles(frame: &mut [u8], vram: &[u8]) {
    for i in 0..384 {
        let tile = &vram[i * 16..(i + 1) * 16];
        let tile_x = (i % 16) * 8;
        let tile_y = (i / 16) * 8;
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
                let pixel_index = (pixel_y * 16 * 8 + pixel_x) * 4;
                frame[pixel_index..pixel_index + 4].copy_from_slice(&mut rgb);
            }
        }
    }
}

fn generate_objects(frame: &mut [u8], oam: &[u8], vram: &[u8]) {
    for i in 0..40 {
        let object = &oam[i * 4..(i + 1) * 4];
        let tile_addr = object[2];
        let tile = &vram[tile_addr as usize * 16..(tile_addr as usize + 1) * 16];
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
                let pixel_x = i % 7 * 8 + k as usize;
                let pixel_y = i / 7 * 8 + j as usize;
                if pixel_x < 56 && pixel_y < 96 {
                    let pixel_index = (pixel_y as usize * 56 + pixel_x as usize) * 4;
                    frame[pixel_index..pixel_index + 4].copy_from_slice(&mut rgb);
                }
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
        .with_inner_size(winit::dpi::LogicalSize::new(128 * 4, 192 * 4))
        .build(&event_loop)
        .unwrap();

    let object_window = WindowBuilder::new()
        .with_title("Objects")
        .with_inner_size(winit::dpi::LogicalSize::new(56 * 6, 48 * 6))
        .build(&event_loop)
        .unwrap();

    if !args.tiles {
        tile_window.set_visible(false);
    }

    if !args.objects {
        object_window.set_visible(false);
    }

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(160, 144, surface_texture).unwrap()
    };

    let mut tile_pixels = {
        let window_size = tile_window.inner_size();
        let surface_texture =
            SurfaceTexture::new(window_size.width, window_size.height, &tile_window);
        Pixels::new(16 * 8, 24 * 8, surface_texture).unwrap()
    };

    let mut object_pixels = {
        let window_size = object_window.inner_size();
        let surface_texture =
            SurfaceTexture::new(window_size.width, window_size.height, &object_window);
        Pixels::new(56, 48, surface_texture).unwrap()
    };

    tile_pixels.render().expect("Failed to render tiles!");
    object_pixels.render().expect("Failed to render objects!");

    // If no log level is specified, default to info or above
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let mut gb = Gameboy::new(&args.rom);
    Window::set_title(
        &window,
        ("RustyBoy - ".to_owned() + &gb.mmu.cart_title).as_str(),
    );

    event_loop.run(move |event, _, control_flow| {
        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = ControlFlow::WaitUntil(next_frame_time);

        match event {
            Event::RedrawRequested(_) => {}
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
        generate_pixels(pixels.frame_mut(), &gb.mmu.ppu.frame_buffer);
        generate_objects(object_pixels.frame_mut(), &gb.mmu.ppu.oam, &gb.mmu.ppu.vram);
        pixels.render().expect("Failed to render!");
        object_pixels.render().expect("Failed to render objects!");
        if gb.mmu.ppu.vram_changed {
            generate_tiles(tile_pixels.frame_mut(), &gb.mmu.ppu.vram);
            tile_pixels.render().expect("Failed to render tiles!");
            gb.mmu.ppu.vram_changed = false;
        }
    });
}
