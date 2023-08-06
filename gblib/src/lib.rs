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