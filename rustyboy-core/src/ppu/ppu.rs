use crate::ppu::lcdc::Lcdc;
use crate::ppu::stat::{Mode, Stat};

pub const LCDC_ADDR: usize = 0xFF40;
pub const STAT_ADDR: usize = 0xFF41;
pub const SCY_ADDR: usize = 0xFF42;
pub const SCX_ADDR: usize = 0xFF43;
pub const LY_ADDR: usize = 0xFF44;
pub const LYC_ADDR: usize = 0xFF45;
pub const BGP_ADDR: usize = 0xFF47;
pub const OBP0_ADDR: usize = 0xFF48;
pub const OBP1_ADDR: usize = 0xFF49;
pub const WY_ADDR: usize = 0xFF4A;
pub const WX_ADDR: usize = 0xFF4B;

const WIDTH: usize = 160;
const HEIGHT: usize = 144;
const HBLANK_CYCLES: u32 = 204;
const VBLANK_CYCLES: u32 = 4560;
const OAM_SEARCH_CYCLES: u32 = 80;
const PIXEL_TRANSFER_CYCLES: u32 = 172;

#[derive(Copy, Clone)]
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
    /// OAM
    pub oam: [u8; 0xA0],
    /// The PPU's internal clock
    clock: u32,
}

impl Ppu {
    /// Create a new PPU
    pub fn new() -> Self {
        let mut frame_buffer = [Color::Black; WIDTH * HEIGHT];
        for i in 0..WIDTH * HEIGHT {
            match i % 4 {
                0 => frame_buffer[i] = Color::White,
                1 => frame_buffer[i] = Color::LightGray,
                2 => frame_buffer[i] = Color::DarkGray,
                3 => frame_buffer[i] = Color::Black,
                _ => unreachable!(),
            }
        }
        Ppu {
            lcdc: Lcdc::new(),
            stat: Stat::new(),
            scy: 0,
            scx: 0,
            ly: 0x90,
            lyc: 0,
            bgp: 0,
            obp0: 0,
            obp1: 0,
            wy: 0,
            wx: 0,
            frame_buffer: frame_buffer,
            lcd_interrupt_fired: false,
            vblank_interrupt_fired: false,
            vram: [0; 0x2000],
            oam: [0; 0xA0],
            clock: 0,
        }
    }

    pub fn step(&mut self, clock_cycles: u8) {
        // Get current mode
        let mode = self.stat.mode;
        if self.lcdc.enabled {
            self.clock += clock_cycles as u32;
            match mode {
                Mode::HBlank => {
                    if self.clock >= HBLANK_CYCLES {
                        self.clock = 0;
                        // TODO drawing
                        self.frame_buffer = [Color::Black; WIDTH * HEIGHT];
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
                        self.frame_buffer = [Color::DarkGray; WIDTH * HEIGHT];
                        self.clock = 0;
                        self.ly += 1;
                        self.check_lyc();
                        if (self.ly as u32) >= (HEIGHT as u32 + 10) {
                            self.stat.mode = Mode::OamSearch;
                            self.ly = 0;
                            if self.stat.mode_2_oam_interrupt {
                                self.lcd_interrupt_fired = true;
                            }
                        }
                    }
                }
                Mode::OamSearch => {
                    if self.clock >= OAM_SEARCH_CYCLES {
                        self.frame_buffer = [Color::LightGray; WIDTH * HEIGHT];
                        self.clock = 0;
                        self.stat.mode = Mode::PixelTransfer;
                    }
                }
                Mode::PixelTransfer => {
                    if self.clock >= PIXEL_TRANSFER_CYCLES {
                        self.clock = 0;
                        self.stat.mode = Mode::HBlank;
                        self.frame_buffer = [Color::White; WIDTH * HEIGHT];
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
}
