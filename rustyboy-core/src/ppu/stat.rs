#[derive(Copy, Clone)]
pub enum Mode {
    HBlank = 0x00,
    VBlank = 0x01,
    OamSearch = 0x02,
    PixelTransfer = 0x03,
}

#[derive(Copy, Clone)]
pub struct Stat {
    /// The current mode of the PPU
    pub mode: Mode,
    /// LYC=LY Flag
    pub lyc_ly_flag: bool,
    /// Mode 0 H-Blank Interrupt
    pub mode_0_hblank_interrupt: bool,
    /// Mode 1 V-Blank Interrupt
    pub mode_1_vblank_interrupt: bool,
    /// Mode 2 OAM Interrupt
    pub mode_2_oam_interrupt: bool,
    /// LYC=LY Interrupt
    pub lyc_ly_interrupt: bool,
}

impl Stat {
    /// Create a new Stat
    pub fn new() -> Self {
        Stat {
            mode: Mode::HBlank,
            lyc_ly_flag: false,
            mode_0_hblank_interrupt: false,
            mode_1_vblank_interrupt: false,
            mode_2_oam_interrupt: false,
            lyc_ly_interrupt: false,
        }
    }

    /// Set the value of the STAT register
    pub fn set(&mut self, value: u8) {
        self.lyc_ly_flag = value & 0b0100_0000 != 0;
        self.mode_0_hblank_interrupt = value & 0b0010_0000 != 0;
        self.mode_1_vblank_interrupt = value & 0b0001_0000 != 0;
        self.mode_2_oam_interrupt = value & 0b0000_1000 != 0;
        self.lyc_ly_interrupt = value & 0b0000_0100 != 0;
    }
}

impl Into<u8> for Stat {
    fn into(self) -> u8 {
        let mut value = 0;
        if self.lyc_ly_flag {
            value |= 0b0100_0000;
        }
        if self.mode_0_hblank_interrupt {
            value |= 0b0010_0000;
        }
        if self.mode_1_vblank_interrupt {
            value |= 0b0001_0000;
        }
        if self.mode_2_oam_interrupt {
            value |= 0b0000_1000;
        }
        if self.lyc_ly_interrupt {
            value |= 0b0000_0100;
        }
        value |= self.mode as u8;
        value
    }
}
