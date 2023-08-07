use crate::cpu::cpu::Cpu;
use crate::mmu::Memory;

pub struct Gameboy {
    mmu: Memory,
    cpu: Cpu,
}

impl Gameboy {
    /// Create a new Gameboy
    pub fn new(rom_name: &str) -> Self {
        let mmu = Memory::new_with_rom(rom_name);
        let cpu: Cpu = Cpu::new();
        Self { cpu, mmu }
    }

    /// Step through the emulation
    pub fn step(&mut self) {
        self.cpu.step(&self.mmu);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let gameboy = Gameboy::new("resources/test-file");
        assert_eq!(gameboy.mmu.get_byte(0 as u8), 'T' as u8)
    }
}