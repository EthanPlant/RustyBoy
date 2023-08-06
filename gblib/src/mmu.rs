/// The MMU (Memory Management Unit) is responsible for managing the gameboy's memory
pub struct Memory {
    /// The gameboy's memory represented as a 64 K byte array
    mem: Vec<u8>,
}

impl Memory {
    pub fn new_with_rom(rom_name: &str) -> Self {
        let mut mem = vec![0; 0x10000];
        let rom_data = std::fs::read(rom_name).expect("Failed to open ROM file");
        for (i, b) in rom_data.iter().enumerate() {
            mem[i] = *b;
        }
        Memory { mem: mem }
    }

    pub fn new() -> Self {
        Memory { mem: vec![0; 0x10000] }
    }

    /// Reads a byte from the memory address space
    pub fn get_byte<T: Into<usize>>(&self, addr: T) -> u8 {
        self.mem[addr.into()]
    }

    /// Writes a byte to the memory address space
    pub fn set_byte<T: Into<usize>>(&mut self, addr: T, v: u8) {
        self.mem[addr.into()] = v;
    }

    /// Reads a word from the memory address space
    pub fn get_word<T: Into<usize>>(&self, addr: T) -> u16 {
        let addr = addr.into();
        (self.mem[addr] as u16) | ((self.mem[addr + 1] as u16) << 8)
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
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_get_byte() {
        let mut mem = Memory::new();
        mem.set_byte(0x1234 as u16, 0x12);
        assert_eq!(mem.get_byte(0x1234 as u16), 0x12);
    }

    #[test]
    fn test_set_byte() {
        let mut mem = Memory::new();
        mem.set_byte(0x1234 as u16, 0x12);
        assert_eq!(mem.get_byte(0x1234 as u16), 0x12);
    }

    #[test]
    fn test_get_word() {
        let mut mem = Memory::new();
        mem.set_word(0x1234 as u16, 0x1234);
        assert_eq!(mem.get_word(0x1234 as u16), 0x1234);
    }

    #[test]
    fn test_set_word() {
        let mut mem = Memory::new();
        mem.set_word(0x1234 as u16, 0x1234);
        assert_eq!(mem.get_word(0x1234 as u16), 0x1234);
    }

    #[test]
    fn test_new_with_rom() {
        let mem = Memory::new_with_rom("resources/test-file");
        assert_eq!(mem.get_byte(0 as u16), 'T' as u8);
    }

    #[test]
    fn test_new() {
        let mem = Memory::new();
        assert_eq!(mem.get_byte(0 as u16), 0);
    }
}