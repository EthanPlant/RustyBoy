use crate::cpu::instructions;
use crate::cpu::instructions::{Instruction, OpCode};
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
    pub fn new() -> Self {
        let cpu_mmu = Memory::new();
        let registers = Registers::new();
        Cpu { reg: registers, mmu: cpu_mmu }
    }

    pub fn new_with_rom(rom_name: &str) ->  Cpu {
        let cpu_mmu = Memory::new_with_rom(rom_name);
        let registers = Registers::new();
        Cpu { reg: registers, mmu: cpu_mmu }
    }

    /// Step through the emulator
    pub fn step(&mut self) {
        let op_code = self.read_opcode();

        let instruction = match instructions::get_instruction_by_opcode(&op_code) {
            Some(instruction) => instruction,
            None => {
                match op_code {
                    OpCode::CB(value) => panic!(
                        "Unimplemented CB instruction! 0x:{:X} PC: 0x:{:X}",
                        value, self.reg.pc,
                    ),
                    OpCode::Regular(value) => panic!(
                        "Unimplemented instruction! 0x:{:X} PC: 0x:{:X}",
                        value, self.reg.pc,
                    ),
                };

            }
        };

        self.execute_instruction(instruction, &op_code)
    }

    /// Execute an instruction
    fn execute_instruction(&mut self, instruction: &Instruction, op_code: &OpCode) {
        let result = (instruction.handler)(self, op_code);

        // Update the program counter
        match result {
            _ => {
                self.reg.pc += instruction.length;
            }
        }
    }

    /// Read an opcode from memory
    fn read_opcode(&mut self) -> OpCode {
        let opcode = self.mmu.get_byte(self.reg.pc);
        match opcode {
            0xCB => OpCode::CB(self.mmu.get_byte(self.reg.pc + 1)),
            _ => OpCode::Regular(opcode),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let cpu = Cpu::new();
        assert_eq!(cpu.reg.pc, 0);
    }

    #[test]
    fn test_new_with_rom() {
        let cpu = Cpu::new_with_rom("resources/test-file");
        assert_eq!(cpu.mmu.get_byte(0 as u16), 'T' as u8);
    }

    #[test]
    fn test_read_opcode() {
        let mut cpu = Cpu::new();
        cpu.reg.pc = 0;
        assert_eq!(cpu.read_opcode(), OpCode::Regular(0));
    }

    #[test]
    fn test_read_opcode_cb() {
        let mut cpu = Cpu::new();
        cpu.reg.pc = 0;
        cpu.mmu.set_byte(0 as u8, 0xCB);
        cpu.mmu.set_byte(1 as u8, 0x00);
        assert_eq!(cpu.read_opcode(), OpCode::CB(0));
    }

    #[test]
    fn test_execute_instruction() {
        let mut cpu = Cpu::new();
        cpu.reg.pc = 0;
        let instruction = Instruction {
            length: 1,
            clock_cycles: 1,
            clock_cycles_condition: None,
            description: String::from("Test instruction"),
            handler: |cpu: &mut Cpu, op_code: &OpCode| {
                assert_eq!(cpu.reg.pc, 0);
                assert_eq!(op_code, &OpCode::Regular(0));
            },
        };
        cpu.execute_instruction(&instruction, &OpCode::Regular(0));
        assert_eq!(cpu.reg.pc, 1);
    }

    #[test]
    #[should_panic(expected = "Unimplemented instruction! 0x:0 PC: 0x:0")]
    fn test_step_unimplemented_instruction() {
        let mut cpu = Cpu::new();
        cpu.step();
    }

    #[test]
    #[should_panic(expected = "Unimplemented CB instruction! 0x:0 PC: 0x:0")]
    fn test_step_unimplemented_cb_instruction() {
        let mut cpu = Cpu::new();
        cpu.mmu.set_byte(0 as u8, 0xCB);
        cpu.mmu.set_byte(1 as u8, 0x00);
        cpu.step();
    }
}