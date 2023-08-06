use crate::cpu::registers::Registers;
use crate::mmu::Memory;

/// Emulation of the Gameboy CPU
pub struct Cpu {
    /// The CPU registers
    reg: Registers,
    /// The MMU (Memory Management Unit)
    pub mmu: Memory,
}

impl Cpu {
    pub fn new(rom_name: &str) ->  Cpu {
        let cpu_mmu = Memory::new_with_rom(rom_name);
        let registers = Registers::new();
        Cpu { reg: registers, mmu: cpu_mmu }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let cpu = Cpu::new("resources/test-file");
        assert_eq!(cpu.mmu.get_byte(0 as u16), 'T' as u8);
    }
}