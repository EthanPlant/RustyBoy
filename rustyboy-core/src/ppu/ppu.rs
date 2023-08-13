use crate::ppu::lcdc::Lcdc;
use crate::ppu::stat::{Mode, Stat};

pub const LCDC_ADDR: usize = 0xFF40;
pub const STAT_ADDR: usize = 0xFF41;
pub const SCY_ADDR: usize = 0xFF42;
pub const SCX_ADDR: usize = 0xFF43;
pub const LY_ADDR: usize = 0xFF44;
pub const LYC_ADDR: usize = 0xFF45;
pub const OAM_DMA_ADDR: usize = 0xFF46;
pub const BGP_ADDR: usize = 0xFF47;
pub const OBP0_ADDR: usize = 0xFF48;
pub const OBP1_ADDR: usize = 0xFF49;
pub const WY_ADDR: usize = 0xFF4A;
pub const WX_ADDR: usize = 0xFF4B;

const WIDTH: usize = 160;
const HEIGHT: usize = 144;
const MAX_SCANLINE: u8 = 154;
const HBLANK_CYCLES: u32 = 204;
const VBLANK_CYCLES: u32 = 456;
const OAM_SEARCH_CYCLES: u32 = 80;
const PIXEL_TRANSFER_CYCLES: u32 = 172;

#[derive(Copy, Clone, PartialEq)]
pub enum Color {
    White,
    LightGray,
    DarkGray,
    Black,
}

pub struct Ppu {
    /// The LCD Control register
    pub lcdc: Lcdc,
    /// The STAT register
    pub stat: Stat,
    /// Scroll Y
    pub scy: u8,
    /// Scroll X
    pub scx: u8,
    /// The current line being drawn
    pub ly: u8,
    /// LY Compare
    pub lyc: u8,
    /// Background Palette
    pub bgp: u8,
    /// Object Palette 0
    pub obp0: u8,
    /// Object Palette 1
    pub obp1: u8,
    /// Window Y Position
    pub wy: u8,
    /// Window X Position
    pub wx: u8,
    /// Frame buffer
    pub frame_buffer: [Color; WIDTH * HEIGHT],
    /// Has an LCD interrupt been fired
    pub lcd_interrupt_fired: bool,
    /// Has a VBlank interrupt been fired
    pub vblank_interrupt_fired: bool,
    /// VRAM
    pub vram: [u8; 0x2000],
    pub vram_changed: bool,
    /// OAM
    pub oam: [u8; 0xA0],
    /// The PPU's internal clock
    clock: u32,
    /// Internal window line counter
    window_line_counter: u8,
    /// Tracks the background colors for background priority checking
    bg_pixels: [u8; WIDTH * HEIGHT],
}

impl Ppu {
    /// Create a new PPU
    pub fn new() -> Self {
        Ppu {
            lcdc: Lcdc::new(),
            stat: Stat::new(),
            scy: 0,
            scx: 0,
            ly: 0x91,
            lyc: 0,
            bgp: 0xFC,
            obp0: 0,
            obp1: 0,
            wy: 0,
            wx: 0,
            frame_buffer: [Color::White; WIDTH * HEIGHT],
            lcd_interrupt_fired: false,
            vblank_interrupt_fired: false,
            vram: [0; 0x2000],
            vram_changed: false,
            oam: [0; 0xA0],
            clock: 0,
            window_line_counter: 0,
            bg_pixels: [0x00; WIDTH * HEIGHT],
        }
    }

    pub fn step(&mut self, clock_cycles: u8) {
        if self.lcdc.enabled {
            self.clock += clock_cycles as u32;
            match self.stat.mode {
                Mode::HBlank => {
                    if self.clock >= HBLANK_CYCLES {
                        self.clock = 0;
                        self.draw();
                        self.ly += 1;
                        self.check_lyc();
                        if self.ly >= HEIGHT as u8 {
                            self.stat.mode = Mode::VBlank;
                            self.vblank_interrupt_fired = true;
                            if self.stat.mode_1_vblank_interrupt {
                                self.lcd_interrupt_fired = true;
                            }
                        } else {
                            self.stat.mode = Mode::OamSearch;
                            if self.stat.mode_2_oam_interrupt {
                                self.lcd_interrupt_fired = true;
                            }
                        }
                    }
                }
                Mode::VBlank => {
                    if self.clock >= VBLANK_CYCLES {
                        self.clock = 0;
                        self.ly += 1;
                        self.check_lyc();
                        if self.ly == MAX_SCANLINE as u8 {
                            self.stat.mode = Mode::OamSearch;
                            self.ly = 0;
                            self.window_line_counter = 0;
                            if self.stat.mode_2_oam_interrupt {
                                self.lcd_interrupt_fired = true;
                            }
                        }
                    }
                }
                Mode::OamSearch => {
                    if self.clock >= OAM_SEARCH_CYCLES {
                        self.clock = 0;
                        self.stat.mode = Mode::PixelTransfer;
                    }
                }
                Mode::PixelTransfer => {
                    if self.clock >= PIXEL_TRANSFER_CYCLES {
                        self.clock = 0;
                        self.stat.mode = Mode::HBlank;
                        if self.stat.mode_0_hblank_interrupt {
                            self.lcd_interrupt_fired = true;
                        }
                    }
                }
            }
        } else {
            self.frame_buffer = [Color::White; WIDTH * HEIGHT];
            self.clock = 0;
        }
    }

    pub fn check_lyc(&mut self) {
        if self.ly == self.lyc {
            self.stat.lyc_ly_flag = true;
            if self.stat.lyc_ly_interrupt {
                self.lcd_interrupt_fired = true;
            }
        } else {
            self.stat.lyc_ly_flag = false;
        }
    }

    fn draw(&mut self) {
        if self.lcdc.enabled {
            if self.lcdc.background_enabled {
                self.draw_background();

                if self.lcdc.window_enabled {
                   self.draw_window();
                }
            }

            if self.lcdc.objects_enabled {
                self.draw_objects();
            }
        }
    }

    fn draw_background(&mut self) {
        let tilemap: u16 = if self.lcdc.background_tile_map {
            0x9C00
        } else {
            0x9800
        };

        let tiledata: u16 = if self.lcdc.background_tile_data {
            0x8000
        } else {
            0x8800
        };

        let y_pos = self.scy.wrapping_add(self.ly);
        let tile_row = ((y_pos / 8) as u16) * 32;
        for pixel in 0..WIDTH {
            let x_pos = self.scx.wrapping_add(pixel as u8);
            let tile_col = x_pos / 8;
            let tile_no =
                self.vram[(tilemap + tile_row as u16 + tile_col as u16) as usize - 0x8000];
            let tile_addr = if self.lcdc.background_tile_data {
                tiledata + (tile_no as u16 * 16)
            } else {
                tiledata + ((tile_no as i8 as i16 + 128) as u16 * 16)
            };
            let tile_line = (y_pos % 8) * 2;
            let lo = self.vram[(tile_addr + tile_line as u16) as usize - 0x8000];
            let hi = self.vram[(tile_addr + tile_line as u16 + 1) as usize - 0x8000];
            let color_val = (lo >> (7 - (x_pos % 8)) & 0x1) | (hi >> (7 - (x_pos % 8)) & 0x1) << 1;
            let color = (self.bgp >> (color_val * 2)) & 0x3;
            self.frame_buffer[self.ly as usize * WIDTH + pixel as usize] = match color {
                0 => Color::White,
                1 => Color::LightGray,
                2 => Color::DarkGray,
                3 => Color::Black,
                _ => Color::Black,
            };
            self.bg_pixels[self.ly as usize * WIDTH + pixel as usize] = color_val;
        }
    }

    fn draw_window(&mut self) {
        let window_x = self.wx.wrapping_sub(7);
        let window_y = self.wy;
        if window_x > 159 || window_y > 143 || self.ly < window_y {
            return;
        }
        let tilemap: u16 = if self.lcdc.window_tile_map {
            0x9C00
        } else {
            0x9800
        };
        let tiledata: u16 = if self.lcdc.background_tile_data {
            0x8000
        } else {
            0x8800
        };
        let y_pos = self.window_line_counter;
        let tile_row = ((y_pos / 8) as u16) * 32;
        for pixel in window_x..WIDTH as u8 {
            let x_pos = pixel as u8 - window_x;
            let tile_col = x_pos / 8;
            let tile_no =
                self.vram[(tilemap + tile_row as u16 + tile_col as u16) as usize - 0x8000];
            
            let tile_addr = if self.lcdc.background_tile_data {
                tiledata + (tile_no as u16 * 16)
            } else {
                tiledata + ((tile_no as i8 as i16 + 128) as u16 * 16)
            };
            let tile_line = (y_pos % 8) * 2;
            let lo = self.vram[(tile_addr + tile_line as u16) as usize - 0x8000];
            let hi = self.vram[(tile_addr + tile_line as u16 + 1) as usize - 0x8000];
            let color_val = (lo >> (7 - (x_pos % 8)) & 0x1) | (hi >> (7 - (x_pos % 8)) & 0x1) << 1;
            let color = (self.bgp >> (color_val * 2)) & 0x3;
            self.frame_buffer[self.ly as usize * WIDTH + pixel as usize] = match color {
                0 => Color::White,
                1 => Color::LightGray,
                2 => Color::DarkGray,
                3 => Color::Black,
                _ => Color::Black,
            };
            self.bg_pixels[self.ly as usize * WIDTH + pixel as usize] = color_val;
        }
        self.window_line_counter += 1;
    }

    fn draw_objects(&mut self) {
        let mut object_count = 0;
        let mut object_x_coords: [u8; 10] = [0; 10];

        for i in 0..40 {
            let mut should_skip = false;
            if object_count >= 10 {
                break;
            }
            
            let object_y = self.oam[i * 4].wrapping_sub(16);
            let object_x = self.oam[i * 4 + 1].wrapping_sub(8);
            for j in 0..object_count {
                if object_x_coords[j] == object_x {
                    should_skip = true;
                }
            }
            if should_skip {
                continue;
            }

            let object_tile = if self.lcdc.objects_size {
                self.oam[i * 4 + 2] & 0xFE
            } else {
                self.oam[i * 4 + 2]
            };
            let object_flags = self.oam[i * 4 + 3];
            let object_height = if self.lcdc.objects_size { 16 } else { 8 };
            if object_y <= self.ly && self.ly < object_y + object_height {
                object_count += 1;
                object_x_coords[object_count - 1] = object_x;
                let bg_priority = object_flags & 0x80 != 0;
                let y_flip = object_flags & 0x40 != 0;
                let x_flip = object_flags & 0x20 != 0;
                let palette = if object_flags & 0x10 != 0 { self.obp1 } else { self.obp0 };
                let tile_line = if y_flip {
                    (object_height - 1 - (self.ly - object_y)) * 2
                } else {
                    (self.ly - object_y) * 2
                };
                let tile_addr = 0x8000 + (object_tile as u16 * 16) + tile_line as u16;
                let lo = self.vram[tile_addr as usize - 0x8000];
                let hi = self.vram[tile_addr as usize + 1 - 0x8000];
                for pixel in 0..8 {
                    let color_val = if x_flip {
                        (lo >> pixel & 0x1) | (hi >> pixel & 0x1) << 1
                    } else {
                        (lo >> (7 - pixel) & 0x1) | (hi >> (7 - pixel) & 0x1) << 1
                    };
                    if color_val == 0 {
                        continue;
                    }
                    let color = (palette >> (color_val * 2)) & 0x3;
                    let x_pos = object_x.wrapping_add(pixel);
                    if x_pos < 8 || x_pos >= 168 {
                        continue;
                    }
                    if bg_priority && self.bg_pixels[self.ly as usize * WIDTH + x_pos as usize] != 0 {
                        continue;
                    }
                    self.frame_buffer[self.ly as usize * WIDTH + x_pos as usize] = match color {
                        0 => Color::White,
                        1 => Color::LightGray,
                        2 => Color::DarkGray,
                        3 => Color::Black,
                        _ => Color::Black,
                    };
                }
            }
        }
    }
}
