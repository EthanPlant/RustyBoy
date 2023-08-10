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
        self.cpu.step(&mut self.mmu);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let gameboy = Gameboy::new("resources/test-rom.gb");
        assert_eq!(gameboy.mmu.get_word(0x0100 as usize), 0xC300);
    }

    #[test]
    fn test_step() {
        let mut gameboy = Gameboy::new("resources/test-rom.gb");
        gameboy.step();
        assert_eq!(gameboy.cpu.reg.pc, 0x0101);
    }
}
