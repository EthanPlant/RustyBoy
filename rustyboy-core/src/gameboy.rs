use crate::cpu::cpu::Cpu;
use crate::mmu::Memory;
use crate::sysclock::{Clock, CYCLES_PER_FRAME};

pub struct Gameboy {
    pub mmu: Memory,
    cpu: Cpu,
    clock: Clock,
}

impl Gameboy {
    /// Create a new Gameboy
    pub fn new(rom_name: &str) -> Self {
        let mmu = Memory::new_with_rom(rom_name);
        let cpu: Cpu = Cpu::new();
        let clock = Clock::new();
        Self { cpu, mmu, clock }
    }

    /// Step through the emulation
    pub fn step(&mut self) {
        while self.clock.clock_cycles_passed < CYCLES_PER_FRAME {
            let cycles = self.cpu.step(&mut self.mmu);
            self.mmu.step(cycles);
            self.clock.cycle(cycles);
        }

        self.clock.reset();
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
}
