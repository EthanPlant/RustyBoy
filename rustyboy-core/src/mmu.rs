use crate::cartridge::Cartridge;

const ROM_START: usize = 0x0000;
const ROM_END: usize = 0x7FFF;
const VRAM_START: usize = 0x8000;
const VRAM_END: usize = 0x9FFF;
const CART_RAM_START: usize = 0xA000;
const CART_RAM_END: usize = 0xBFFF;
const WRAM_START: usize = 0xC000;
const WRAM_END: usize = 0xDFFF;
const ECHO_RAM_START: usize = 0xE000;
const ECHO_RAM_END: usize = 0xFDFF;
const OAM_START: usize = 0xFE00;
const OAM_END: usize = 0xFE9F;
const UNUSED_START: usize = 0xFEA0;
const UNUSED_END: usize = 0xFEFF;
// TODO Proper memory mapped IO
const IO_START: usize = 0xFF00;
const IO_END: usize = 0xFF7F;
const HRAM_START: usize = 0xFF80;
const HRAM_END: usize = 0xFFFE;
const IE: usize = 0xFFFF;

/// The MMU (Memory Management Unit) is responsible for managing the gameboy's memory
pub struct Memory {
    /// The cartridge's data
    cart: Cartridge,
    vram: [u8; VRAM_END - VRAM_START],
    wram: [u8; WRAM_END - WRAM_START],
    oam: [u8; OAM_END - OAM_START],
    io: [u8; IO_END - IO_START],
    hram: [u8; HRAM_END - HRAM_START],
}

impl Memory {
    /// Create a new empty Memory
    pub fn new() -> Self {
        Memory {
            cart: Cartridge::new(),
            vram: [0xFF; VRAM_END - VRAM_START],
            wram: [0xFF; WRAM_END - WRAM_START],
            oam: [0xFF; OAM_END - OAM_START],
            io: [0xFF; IO_END - IO_START],
            hram: [0xFF; HRAM_END - HRAM_START],
        }
    }

    /// Create a new Memory with a ROM file
    pub fn new_with_rom(rom_name: &str) -> Self {
        let cart = Cartridge::new_from_rom(rom_name);
        let mut io: [u8; IO_END - IO_START] = [0xFF; IO_END - IO_START];
        io[0x44] = 0x90; // Stub LY to 0x90 (144) to simulate VBlank
        Memory {
            cart: cart,
            vram: [0xFF; VRAM_END - VRAM_START],
            wram: [0xFF; WRAM_END - WRAM_START],
            oam: [0xFF; OAM_END - OAM_START],
            io: [0xFF; IO_END - IO_START],
            hram: [0xFF; HRAM_END - HRAM_START],
        }
    }

    /// Reads a byte from the memory address space
    pub fn get_byte<T: Into<usize>>(&self, addr: T) -> u8 {
        let addr = addr.into();
        match addr {
            ROM_START..=ROM_END => self.cart.read_byte_from_rom(addr),
            VRAM_START..=VRAM_END => self.vram[addr - VRAM_START],
            CART_RAM_START..=CART_RAM_END => self.cart.read_byte_from_ram(addr - CART_RAM_START),
            WRAM_START..=WRAM_END => self.wram[addr - WRAM_START],
            ECHO_RAM_START..=ECHO_RAM_END => {
                log::warn!("Attempted prohibited read from echo RAM {}", addr);
                self.wram[addr - ECHO_RAM_START]
            }
            OAM_START..=OAM_END => self.oam[addr - OAM_START],
            UNUSED_START..=UNUSED_END => {
                log::warn!("Attempted prohibited read from unused memory {}", addr);
                0xFF
            }
            IO_START..=IO_END => self.io[addr - IO_START],
            HRAM_START..=HRAM_END => self.hram[addr - HRAM_START],
            // TODO handle interrupt enable register
            IE => 0xFF,
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
            VRAM_START..=VRAM_END => self.vram[addr - VRAM_START] = v,
            CART_RAM_START..=CART_RAM_END => self.cart.write_byte_to_ram(addr - CART_RAM_START, v),
            WRAM_START..=WRAM_END => self.wram[addr - WRAM_START] = v,
            ECHO_RAM_START..=ECHO_RAM_END => {
                log::warn!("Attempted prohibited write to echo RAM {}", addr);
                self.wram[addr - ECHO_RAM_START] = v
            }
            OAM_START..=OAM_END => self.oam[addr - OAM_START] = v,
            UNUSED_START..=UNUSED_END => {
                log::warn!("Attempted prohibited write to unused memory {}", addr);
            }
            IO_START..=IO_END => {
                // Logging for Blargg tests
                if addr == 0xFF02 && v == 0x81 {
                    print!("{}", self.io[0xFF01 - IO_START] as char);
                } else {
                    self.io[addr - IO_START] = v;
                }
            }
            HRAM_START..=HRAM_END => self.hram[addr - HRAM_START] = v,
            // TODO handle interrupt enable register
            IE => (),
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
        mem.vram[0x00] = 0x01;
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
        mem.oam[0x00] = 0x01;
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
        mem.io[0x00] = 0x01;
        assert_eq!(mem.get_byte(IO_START), 0x01);
    }

    #[test]
    fn test_get_byte_hram() {
        let mut mem = Memory::new();
        mem.hram[0x00] = 0x01;
        assert_eq!(mem.get_byte(HRAM_START), 0x01);
    }

    #[test]
    fn test_get_byte_ie() {
        let mem = Memory::new();
        assert_eq!(mem.get_byte(IE), 0xFF);
    }

    #[test]
    fn test_get_word_rom() {
        let mem = Memory::new();
        assert_eq!(mem.get_word(ROM_START), 0xFFFF);
    }

    #[test]
    fn test_get_word_vram() {
        let mut mem = Memory::new();
        mem.vram[0x00] = 0x01;
        mem.vram[0x01] = 0x02;
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
        mem.oam[0x00] = 0x01;
        mem.oam[0x01] = 0x02;
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
        mem.io[0x00] = 0x01;
        mem.io[0x01] = 0x02;
        assert_eq!(mem.get_word(IO_START), 0x0201);
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
        assert_eq!(mem.vram[0x0000], 0x01);
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
        assert_eq!(mem.oam[0x0000], 0x01);
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
    fn test_set_byte_ie() {
        let mut mem = Memory::new();
        mem.set_byte(IE, 0x01);
        assert_eq!(mem.get_byte(IE), 0xFF);
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
        assert_eq!(mem.vram[0x0000], 0x02);
        assert_eq!(mem.vram[0x0001], 0x01);
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
        assert_eq!(mem.oam[0x0000], 0x02);
        assert_eq!(mem.oam[0x0001], 0x01);
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
