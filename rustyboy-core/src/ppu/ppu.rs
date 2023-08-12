use crate::ppu::lcdc::Lcdc;
use crate::ppu::stat::Stat;

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
}

impl Ppu {
    /// Create a new PPU
    pub fn new() -> Self {
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
        }
    }
}
