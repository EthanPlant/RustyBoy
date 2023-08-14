use crate::cartridge::Cartridge;
use crate::cpu::interrupts::{
    Interrupt, InterruptState, INTERRUPT_ENABLE_ADDR, INTERRUPT_FLAG_ADDR,
};
use crate::io::timer::{Timer, DIV_ADDR, TAC_ADDR, TIMA_ADDR, TMA_ADDR};
use crate::mbc;
use crate::mbc::rom_only::RomOnly;
use crate::mbc::Mbc;
use crate::ppu::ppu::{
    Ppu, BGP_ADDR, LCDC_ADDR, LYC_ADDR, LY_ADDR, OBP0_ADDR, OBP1_ADDR, SCX_ADDR, SCY_ADDR,
    STAT_ADDR, WX_ADDR, WY_ADDR, OAM_DMA_ADDR,
};
use crate::ppu::stat::Mode;

const ROM_START: usize = 0x0000;
const ROM_END: usize = 0x7FFF;

const VRAM_START: usize = 0x8000;
const VRAM_END: usize = 0x9FFF;

const CART_RAM_START: usize = 0xA000;
const CART_RAM_END: usize = 0xBFFF;

const WRAM_START: usize = 0xC000;
const WRAM_END: usize = 0xDFFF;
const WRAM_SIZE: usize = WRAM_END - WRAM_START + 1;

const ECHO_RAM_START: usize = 0xE000;
const ECHO_RAM_END: usize = 0xFDFF;

const OAM_START: usize = 0xFE00;
const OAM_END: usize = 0xFE9F;

const UNUSED_START: usize = 0xFEA0;
const UNUSED_END: usize = 0xFEFF;

// TODO Proper memory mapped IO
const IO_START: usize = 0xFF00;
const IO_END: usize = 0xFF7F;
const IO_SIZE: usize = IO_END - IO_START + 1;

const HRAM_START: usize = 0xFF80;
const HRAM_END: usize = 0xFFFE;
const HRAM_SIZE: usize = HRAM_END - HRAM_START + 1;

/// The MMU (Memory Management Unit) is responsible for managing the gameboy's memory
pub struct Memory {
    /// The cartridge's data
    cart: Box<dyn Mbc>,
    pub cart_title: String,
    /// Interrupt registers
    pub interrupts: InterruptState,
    /// Timer
    pub timer: Timer,
    /// The PPU
    pub ppu: Ppu,
    /// WRAM
    wram: [u8; WRAM_SIZE],
    /// IO Registers
    io: [u8; IO_SIZE],
    /// High RAM
    hram: [u8; HRAM_SIZE],
}

impl Memory {
    /// Create a new empty Memory
    pub fn new() -> Self {
        Memory {
            cart: Box::new(RomOnly::new(Cartridge::new())),
            cart_title: String::new(),
            interrupts: InterruptState::new(),
            timer: Timer::new(),
            ppu: Ppu::new(),
            wram: [0xFF; WRAM_SIZE],
            io: [0xFF; IO_SIZE],
            hram: [0xFF; HRAM_SIZE],
        }
    }

    /// Create a new Memory with a ROM file
    pub fn new_with_rom(rom_name: &str) -> Self {
        let cart = Cartridge::new_from_rom(rom_name);
        let title = cart.title.clone();
        Memory {
            cart: mbc::from_cartridge(cart),
            cart_title: title,
            interrupts: InterruptState::new(),
            timer: Timer::new(),
            ppu: Ppu::new(),
            wram: [0xFF; WRAM_SIZE],
            io: [0xFF; IO_SIZE],
            hram: [0xFF; HRAM_SIZE],
        }
    }

    /// Step the IO devices
    pub fn step(&mut self, clock_cycles: u8) {
        self.timer.step(clock_cycles);
        self.ppu.step(clock_cycles);

        if self.timer.interrupt_fired {
            self.timer.interrupt_fired = false;
            self.interrupts.requested_interrupts |= Interrupt::Timer as u8;
        }

        if self.ppu.lcd_interrupt_fired {
            self.ppu.lcd_interrupt_fired = false;
            self.interrupts.requested_interrupts |= Interrupt::LcdStat as u8;
        }

        if self.ppu.vblank_interrupt_fired {
            self.ppu.vblank_interrupt_fired = false;
            self.interrupts.requested_interrupts |= Interrupt::VBlank as u8;
        }
    }

    /// Reads a byte from the memory address space
    pub fn get_byte<T: Into<usize>>(&self, addr: T) -> u8 {
        let addr = addr.into();
        match addr {
            ROM_START..=ROM_END => self.cart.read_byte_from_rom(addr),
            VRAM_START..=VRAM_END => self.ppu.vram[addr - VRAM_START],
            CART_RAM_START..=CART_RAM_END => self.cart.read_byte_from_ram(addr - CART_RAM_START),
            WRAM_START..=WRAM_END => self.wram[addr - WRAM_START],
            ECHO_RAM_START..=ECHO_RAM_END => {
                log::warn!("Attempted prohibited read from echo RAM {}", addr);
                self.wram[addr - ECHO_RAM_START]
            }
            OAM_START..=OAM_END => self.ppu.oam[addr - OAM_START],
            UNUSED_START..=UNUSED_END => {
                log::warn!("Attempted prohibited read from unused memory {}", addr);
                0xFF
            }
            IO_START..=IO_END => match addr {
                0xFF00 => 0xFF, // Stub this to all 1s for now to stop tetris from getting in a reboot loop
                DIV_ADDR => self.timer.divider,
                TIMA_ADDR => self.timer.counter,
                TMA_ADDR => self.timer.modulo,
                TAC_ADDR => self.timer.control,
                INTERRUPT_FLAG_ADDR => self.interrupts.requested_interrupts,
                LCDC_ADDR => self.ppu.lcdc.into(),
                STAT_ADDR => self.ppu.stat.into(),
                SCY_ADDR => self.ppu.scy,
                SCX_ADDR => self.ppu.scx,
                LY_ADDR => self.ppu.ly,
                LYC_ADDR => self.ppu.lyc,
                BGP_ADDR => self.ppu.bgp,
                OBP0_ADDR => self.ppu.obp0,
                OBP1_ADDR => self.ppu.obp1,
                WY_ADDR => self.ppu.wy,
                WX_ADDR => self.ppu.wx,
                _ => self.io[addr - IO_START],
            },
            HRAM_START..=HRAM_END => self.hram[addr - HRAM_START],
            INTERRUPT_ENABLE_ADDR => self.interrupts.enabled_interrupts,
            _ => {
                log::error!("Attempted to read from invalid memory address {}", addr);
                0xFF
            }
        }
    }

    /// Writes a byte to the memory address space
    pub fn set_byte<T: Into<usize>>(&mut self, addr: T, v: u8) {
        let addr = addr.into();
        log::trace!(
            "Writing {:02X} to {:04X}, replacing {:02X}",
            v,
            addr,
            self.get_byte(addr)
        );
        match addr {
            ROM_START..=ROM_END => self.cart.write_byte_to_rom(addr, v),
            VRAM_START..=VRAM_END => {
                self.ppu.vram_changed = true;
                self.ppu.vram[addr - VRAM_START] = v
            }
            CART_RAM_START..=CART_RAM_END => self.cart.write_byte_to_ram(addr - CART_RAM_START, v),
            WRAM_START..=WRAM_END => self.wram[addr - WRAM_START] = v,
            ECHO_RAM_START..=ECHO_RAM_END => {
                log::warn!("Attempted prohibited write to echo RAM {}", addr);
                self.wram[addr - ECHO_RAM_START] = v
            }
            OAM_START..=OAM_END => self.ppu.oam[addr - OAM_START] = v,
            UNUSED_START..=UNUSED_END => {
                log::warn!("Attempted prohibited write to unused memory {}", addr);
            }

            IO_START..=IO_END => {
                match addr {
                    DIV_ADDR => self.timer.divider = 0, // All writes to DIV reset it to 0
                    TIMA_ADDR => self.timer.counter = v,
                    TMA_ADDR => self.timer.modulo = v,
                    TAC_ADDR => self.timer.control = v,
                    INTERRUPT_FLAG_ADDR => self.interrupts.requested_interrupts = v,
                    LCDC_ADDR => {
                        if !self.ppu.lcdc.enabled && v & 0x80 != 0 {
                            // If the LCD is turned on, switch to mode 2
                            self.ppu.stat.mode = Mode::OamSearch;
                        }
                        if v & 0x80 == 0 && self.ppu.lcdc.enabled {
                            // If the LCD is turned off, switch to mode 0
                            self.ppu.stat.mode = Mode::HBlank;
                            self.ppu.ly = 0;
                        }
                        self.ppu.lcdc.set(v);
                    }
                    STAT_ADDR => self.ppu.stat.set(v),
                    SCY_ADDR => self.ppu.scy = v,
                    SCX_ADDR => self.ppu.scx = v,
                    LY_ADDR => self.ppu.ly = v,
                    LYC_ADDR => {
                        self.ppu.lyc = v;
                        self.ppu.check_lyc(); // Check if LYC=LY
                    }
                    OAM_DMA_ADDR => {
                        let start = (v as u16) << 8;
                        for i in 0..0xA0 {
                            self.ppu.oam[i] = self.get_byte(start as usize + i);
                        }
                    }
                    BGP_ADDR => self.ppu.bgp = v,
                    OBP0_ADDR => self.ppu.obp0 = v,
                    OBP1_ADDR => self.ppu.obp1 = v,
                    WY_ADDR => self.ppu.wy = v,
                    WX_ADDR => self.ppu.wx = v,
                    _ => self.io[addr - IO_START] = v,
                }
            }
            HRAM_START..=HRAM_END => self.hram[addr - HRAM_START] = v,
            INTERRUPT_ENABLE_ADDR => self.interrupts.enabled_interrupts = v,
            _ => {
                log::error!("Attempted to write to invalid memory address {}", addr);
            }
        }
    }

    /// Reads a word from the memory address space
    pub fn get_word<T: Into<usize>>(&self, addr: T) -> u16 {
        let addr = addr.into();
        let l = self.get_byte(addr);
        let h = self.get_byte(addr + 1);
        ((h as u16) << 8) | (l as u16)
    }

    /// Writes a word to the memory address space
    pub fn set_word<T: Into<usize>>(&mut self, addr: T, v: u16) {
        let addr = addr.into();
        let l = (v & 0x00FF) as u8;
        let h = ((v & 0xFF00) >> 8) as u8;
        self.set_byte(addr, l);
        self.set_byte(addr + 1, h);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let mem = Memory::new();
        assert_eq!(mem.get_byte(0x0101 as usize), 0xFF);
    }

    #[test]
    fn test_new_with_rom() {
        let mem = Memory::new_with_rom("resources/test-rom.gb");
        assert_eq!(mem.get_byte(0x0101 as usize), 0xC3);
    }

    #[test]
    fn test_get_byte_rom() {
        let mem = Memory::new();
        assert_eq!(mem.get_byte(ROM_START), 0xFF);
    }

    #[test]
    fn test_get_byte_vram() {
        let mut mem = Memory::new();
        mem.ppu.vram[0x00] = 0x01;
        assert_eq!(mem.get_byte(VRAM_START), 0x01);
    }

    #[test]
    fn test_get_byte_cart_ram() {
        let mem = Memory::new();
        assert_eq!(mem.get_byte(CART_RAM_START), 0xFF);
    }

    #[test]
    fn test_get_byte_wram() {
        let mut mem = Memory::new();
        mem.wram[0x00] = 0x01;
        assert_eq!(mem.get_byte(WRAM_START), 0x01);
    }

    #[test]
    fn test_get_byte_echo_ram() {
        let mut mem = Memory::new();
        mem.wram[0x00] = 0x01;
        assert_eq!(mem.get_byte(ECHO_RAM_START), 0x01);
    }

    #[test]
    fn test_get_byte_oam() {
        let mut mem = Memory::new();
        mem.ppu.oam[0x00] = 0x01;
        assert_eq!(mem.get_byte(OAM_START), 0x01);
    }

    #[test]
    fn test_get_byte_unused() {
        let mem = Memory::new();
        assert_eq!(mem.get_byte(UNUSED_START), 0xFF);
    }

    #[test]
    fn test_get_byte_io() {
        let mut mem = Memory::new();
        mem.io[0x01] = 0x01;
        assert_eq!(mem.get_byte(0xFF01 as usize), 0x01);
    }

    #[test]
    fn test_get_byte_hram() {
        let mut mem = Memory::new();
        mem.hram[0x00] = 0x01;
        assert_eq!(mem.get_byte(HRAM_START), 0x01);
    }

    #[test]
    fn test_get_word_rom() {
        let mem = Memory::new();
        assert_eq!(mem.get_word(ROM_START), 0xFFFF);
    }

    #[test]
    fn test_get_word_vram() {
        let mut mem = Memory::new();
        mem.ppu.vram[0x00] = 0x01;
        mem.ppu.vram[0x01] = 0x02;
        assert_eq!(mem.get_word(VRAM_START), 0x0201);
    }

    #[test]
    fn test_get_word_cart_ram() {
        let mem = Memory::new();
        assert_eq!(mem.get_word(CART_RAM_START), 0xFFFF);
    }

    #[test]
    fn test_get_word_wram() {
        let mut mem = Memory::new();
        mem.wram[0x00] = 0x01;
        mem.wram[0x01] = 0x02;
        assert_eq!(mem.get_word(WRAM_START), 0x0201);
    }

    #[test]
    fn test_get_word_echo_ram() {
        let mut mem = Memory::new();
        mem.wram[0x00] = 0x01;
        mem.wram[0x01] = 0x02;
        assert_eq!(mem.get_word(ECHO_RAM_START), 0x0201);
    }

    #[test]
    fn test_get_word_oam() {
        let mut mem = Memory::new();
        mem.ppu.oam[0x00] = 0x01;
        mem.ppu.oam[0x01] = 0x02;
        assert_eq!(mem.get_word(OAM_START), 0x0201);
    }

    #[test]
    fn test_get_word_unused() {
        let mem = Memory::new();
        assert_eq!(mem.get_word(UNUSED_START), 0xFFFF);
    }

    #[test]
    fn test_get_word_io() {
        let mut mem = Memory::new();
        mem.io[0x01] = 0x01;
        mem.io[0x02] = 0x02;
        assert_eq!(mem.get_word(0xFF01 as usize), 0x0201);
    }

    #[test]
    fn test_get_word_hram() {
        let mut mem = Memory::new();
        mem.hram[0x00] = 0x01;
        mem.hram[0x01] = 0x02;
        assert_eq!(mem.get_word(HRAM_START), 0x0201);
    }

    #[test]
    fn test_set_byte_rom() {
        let mut mem = Memory::new();
        mem.set_byte(ROM_START, 0x01);
        assert_eq!(mem.cart.read_byte_from_ram(0x0000), 0xFF);
    }

    #[test]
    fn test_set_byte_vram() {
        let mut mem = Memory::new();
        mem.set_byte(VRAM_START, 0x01);
        assert_eq!(mem.ppu.vram[0x0000], 0x01);
    }

    #[test]
    fn test_set_byte_cart_ram() {
        let mut mem = Memory::new();
        mem.set_byte(CART_RAM_START, 0x01);
        assert_eq!(mem.cart.read_byte_from_ram(0x0000), 0x01);
    }

    #[test]
    fn test_set_byte_wram() {
        let mut mem = Memory::new();
        mem.set_byte(WRAM_START, 0x01);
        assert_eq!(mem.wram[0x0000], 0x01);
    }

    #[test]
    fn test_set_byte_echo_ram() {
        let mut mem = Memory::new();
        mem.set_byte(ECHO_RAM_START, 0x01);
        assert_eq!(mem.wram[0x0000], 0x01);
    }

    #[test]
    fn test_set_byte_oam() {
        let mut mem = Memory::new();
        mem.set_byte(OAM_START, 0x01);
        assert_eq!(mem.ppu.oam[0x0000], 0x01);
    }

    #[test]
    fn test_set_byte_unused() {
        let mut mem = Memory::new();
        mem.set_byte(UNUSED_START, 0x01);
        assert_eq!(mem.get_byte(UNUSED_START), 0xFF);
    }

    #[test]
    fn test_set_byte_io() {
        let mut mem = Memory::new();
        mem.set_byte(IO_START, 0x01);
        assert_eq!(mem.io[0x0000], 0x01);
    }

    #[test]
    fn test_set_byte_hram() {
        let mut mem = Memory::new();
        mem.set_byte(HRAM_START, 0x01);
        assert_eq!(mem.hram[0x0000], 0x01);
    }

    #[test]
    fn test_set_word_rom() {
        let mut mem = Memory::new();
        mem.set_word(ROM_START, 0x0102);
        assert_eq!(mem.cart.read_byte_from_ram(0x0000), 0xFF);
    }

    #[test]
    fn test_set_word_vram() {
        let mut mem = Memory::new();
        mem.set_word(VRAM_START, 0x0102);
        assert_eq!(mem.ppu.vram[0x0000], 0x02);
        assert_eq!(mem.ppu.vram[0x0001], 0x01);
    }

    #[test]
    fn test_set_word_cart_ram() {
        let mut mem = Memory::new();
        mem.set_word(CART_RAM_START, 0x0102);
        assert_eq!(mem.cart.read_byte_from_ram(0x0000), 0x02);
        assert_eq!(mem.cart.read_byte_from_ram(0x0001), 0x01);
    }

    #[test]
    fn test_set_word_wram() {
        let mut mem = Memory::new();
        mem.set_word(WRAM_START, 0x0102);
        assert_eq!(mem.wram[0x0000], 0x02);
        assert_eq!(mem.wram[0x0001], 0x01);
    }

    #[test]
    fn test_set_word_echo_ram() {
        let mut mem = Memory::new();
        mem.set_word(ECHO_RAM_START, 0x0102);
        assert_eq!(mem.wram[0x0000], 0x02);
        assert_eq!(mem.wram[0x0001], 0x01);
    }

    #[test]
    fn test_set_word_oam() {
        let mut mem = Memory::new();
        mem.set_word(OAM_START, 0x0102);
        assert_eq!(mem.ppu.oam[0x0000], 0x02);
        assert_eq!(mem.ppu.oam[0x0001], 0x01);
    }

    #[test]
    fn test_set_word_unused() {
        let mut mem = Memory::new();
        mem.set_word(UNUSED_START, 0x0102);
        assert_eq!(mem.get_byte(UNUSED_START), 0xFF);
    }

    #[test]
    fn test_set_word_io() {
        let mut mem = Memory::new();
        mem.set_word(IO_START, 0x0102);
        assert_eq!(mem.io[0x0000], 0x02);
        assert_eq!(mem.io[0x0001], 0x01);
    }

    #[test]
    fn test_set_word_hram() {
        let mut mem = Memory::new();
        mem.set_word(HRAM_START, 0x0102);
        assert_eq!(mem.hram[0x0000], 0x02);
        assert_eq!(mem.hram[0x0001], 0x01);
    }
}
