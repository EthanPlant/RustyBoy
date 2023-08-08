use crate::cartridge::Cartridge;

// TODO Proper memory mapping
const ROM_START: usize = 0x0000;
const ROM_END: usize = 0x7FFF;

/// The MMU (Memory Management Unit) is responsible for managing the gameboy's memory
pub struct Memory {
    /// The gameboy's memory represented as a 64 K byte array
    mem: Vec<u8>,
    /// The cartridge's data
    cart: Cartridge,
}

impl Memory {
    pub fn new_with_rom(rom_name: &str) -> Self {
        let mem = vec![0xFF; 0x10000];
        let cart = Cartridge::new_from_rom(rom_name);
        Memory { mem: mem, cart: cart }
    }

    pub fn new() -> Self {
        Memory {
            mem: vec![0; 0x10000], cart: Cartridge::new(),
        }
    }

    /// Reads a byte from the memory address space
    pub fn get_byte<T: Into<usize>>(&self, addr: T) -> u8 {
        let addr = addr.into();
        match addr {
            ROM_START..=ROM_END => self.cart.read_byte_from_rom(addr),
            _ => self.mem[addr],
        }
    }

    /// Writes a byte to the memory address space
    pub fn set_byte<T: Into<usize>>(&mut self, addr: T, v: u8) {
        self.mem[addr.into()] = v;
    }

    /// Reads a word from the memory address space
    pub fn get_word<T: Into<usize>>(&self, addr: T) -> u16 {
        let addr = addr.into();
        match addr {
            ROM_START..=ROM_END => {
                self.cart.read_word_from_rom(addr)
            }
            _ => {
                let low = self.mem[addr];
                let high = self.mem[addr + 1];
                ((high as u16) << 8) | (low as u16)
            }
        }
    }

    /// Writes a word to the memory address space
    pub fn set_word<T: Into<usize>>(&mut self, addr: T, v: u16) {
        let addr = addr.into();
        self.mem[addr] = (v & 0xFF) as u8;
        self.mem[addr + 1] = ((v >> 8) & 0xFF) as u8;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_with_rom() {
        let mem = Memory::new_with_rom("resources/test-rom.gb");
        assert_eq!(mem.get_byte(0x0101 as usize), 0xC3);
    }

    #[test]
    fn test_new() {
        let mem = Memory::new();
        assert_eq!(mem.mem[0], 0);
    }

    #[test]
    fn test_get_byte_in_cart() {
        let mem = Memory::new();
        assert_eq!(mem.get_byte(0 as usize), 0xFF);
    }

    #[test]
    fn test_get_byte_not_in_cart() {
        let mut mem = Memory::new();
        mem.mem[0xC000] = 0x12;
        assert_eq!(mem.get_byte(0xC000 as usize), 0x12);
    }

    #[test]
    fn test_set_byte_in_cart() {
        let mut mem = Memory::new();
        mem.set_byte(0 as usize, 0x12);
        assert_eq!(mem.get_byte(0 as usize), 0xFF);
    }

    #[test]
    fn test_set_byte_not_in_cart() {
        let mut mem = Memory::new();
        mem.set_byte(0xC000 as usize, 0x12);
        assert_eq!(mem.mem[0xC000], 0x12);
    }

    #[test]
    fn test_get_word_in_cart() {
        let mem = Memory::new();
        assert_eq!(mem.get_word(0 as usize), 0xFFFF);
    }

    #[test]
    fn test_get_word_not_in_cart() {
        let mut mem = Memory::new();
        mem.mem[0xC000] = 0x34;
        mem.mem[0xC001] = 0x12;
        assert_eq!(mem.get_word(0xC000 as usize), 0x1234);
    }

    #[test]
    fn test_set_word_in_cart() {
        let mut mem = Memory::new();
        mem.set_word(0 as usize, 0x1234);
        assert_eq!(mem.get_word(0 as usize), 0xFFFF);
    }

    #[test]
    fn test_set_word_not_in_cart() {
        let mut mem = Memory::new();
        mem.set_word(0xC000 as usize, 0x1234);
        assert_eq!(mem.mem[0xC000], 0x34);
        assert_eq!(mem.mem[0xC001], 0x12);
    }
}
