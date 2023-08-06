pub mod cpu;
mod mmu;

use cpu::cpu::Cpu;

/// The Gameboy emulator
pub struct Gameboy {
    cpu: Cpu,
}

impl Gameboy {
    /// Initialize the emulator
    pub fn new(rom_name: &str) -> Self {
        let cpu = Cpu::new(rom_name);
        Self { cpu: cpu }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let gb = Gameboy::new("resources/test-file");
        assert_eq!(gb.cpu.mmu.get_byte(0 as u16), 'T' as u8);
    }
}