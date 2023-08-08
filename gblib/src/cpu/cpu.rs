use crate::cpu::instructions;
use crate::cpu::instructions::{Instruction, InstructionType, OpCode};
use crate::cpu::registers::Registers;
use crate::mmu::Memory;

/// Emulation of the Gameboy CPU
pub struct Cpu {
    /// The CPU registers
    pub reg: Registers,
}

impl Cpu {
    pub fn new() -> Self {
        let mut registers = Registers::new();
        registers.pc = 0x0100;
        Cpu { reg: registers }
    }

    /// Step through the emulator
    pub fn step(&mut self, mmu: &mut Memory) {
        let op_code = self.read_opcode(mmu);

        let instruction = match instructions::get_instruction_by_opcode(&op_code) {
            Some(instruction) => instruction,
            None => {
                match op_code {
                    OpCode::CB(value) => panic!(
                        "Unimplemented CB instruction! {:#04X} PC: {:#06X}",
                        value, self.reg.pc,
                    ),
                    OpCode::Regular(value) => panic!(
                        "Unimplemented instruction! {:#04X} PC: {:#06X}",
                        value, self.reg.pc,
                    ),
                };
            }
        };

        println!("A: {:02X} F: {:02X} B: {:02X} C: {:02X} D: {:02X} E: {:02X} H: {:02X} L: {:02X} SP: {:02X} PC: 00:{:04X} ({:02X} {:02X} {:02X} {:02X})",
        self.reg.a, self.reg.f, self.reg.b, self.reg.c, self.reg.d, self.reg.e, self.reg.h, self.reg.l, self.reg.sp, self.reg.pc, mmu.get_byte(self.reg.pc), mmu.get_byte(self.reg.pc + 1), mmu.get_byte(self.reg.pc + 2), mmu.get_byte(self.reg.pc + 3));

        self.execute_instruction(mmu, instruction, &op_code)
    }

    /// Execute an instruction
    fn execute_instruction(
        &mut self,
        mmu: &mut Memory,
        instruction: &Instruction,
        op_code: &OpCode,
    ) {
        let result = (instruction.handler)(self, mmu, op_code);
        // Update the program counter
        match result {
            InstructionType::Jumped => {}
            _ => {
                self.reg.pc += instruction.length;
            }
        }
    }

    /// Read an opcode from memory
    fn read_opcode(&mut self, mmu: &Memory) -> OpCode {
        let opcode = mmu.get_byte(self.reg.pc);
        match opcode {
            0xCB => OpCode::CB(mmu.get_byte(self.reg.pc + 1)),
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
        assert_eq!(cpu.reg.pc, 0x100);
    }

    #[test]
    fn test_read_opcode() {
        let mut mmu = Memory::new();
        let mut cpu = Cpu::new();
        cpu.reg.pc = 0xC000;
        mmu.set_byte(0xC000 as usize, 0x00 as u8);
        assert_eq!(cpu.read_opcode(&mmu), OpCode::Regular(0));
    }

    #[test]
    fn test_read_opcode_cb() {
        let mut mmu = Memory::new();
        let mut cpu = Cpu::new();
        cpu.reg.pc = 0xC000;
        mmu.set_byte(0xC000 as usize, 0xCB);
        mmu.set_byte(0xC001 as usize, 0x00);
        assert_eq!(cpu.read_opcode(&mmu), OpCode::CB(0));
    }

    #[test]
    fn test_execute_instruction() {
        let mut mmu = Memory::new();
        let mut cpu = Cpu::new();
        cpu.reg.pc = 0;
        let instruction = Instruction {
            length: 1,
            clock_cycles: 1,
            clock_cycles_condition: None,
            description: "Test instruction",
            handler: |cpu: &mut Cpu, _: &mut Memory, op_code: &OpCode| {
                assert_eq!(cpu.reg.pc, 0);
                assert_eq!(op_code, &OpCode::Regular(0));
                InstructionType::ActionTaken
            },
        };
        cpu.execute_instruction(&mut mmu, &instruction, &OpCode::Regular(0));
        assert_eq!(cpu.reg.pc, 1);
    }

    #[test]
    fn test_execute_instruction_jumped() {
        let mut mmu = Memory::new();
        let mut cpu = Cpu::new();
        cpu.reg.pc = 0;
        let instruction = Instruction {
            length: 1,
            clock_cycles: 1,
            clock_cycles_condition: None,
            description: "Test instruction",
            handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
                cpu.reg.pc = 10;
                InstructionType::Jumped
            },
        };
        cpu.execute_instruction(&mut mmu, &instruction, &OpCode::Regular(0));
        assert_eq!(cpu.reg.pc, 10);
    }

    #[test]
    fn test_step() {
        let mut mmu = Memory::new();
        let mut cpu = Cpu::new();
        cpu.reg.pc = 0xC000;
        mmu.set_byte(0xC000 as usize, 0x00 as u8);
        cpu.step(&mut mmu);
        assert_eq!(cpu.reg.pc, 0xC001);
    }

    #[test]
    fn test_step_cb() {
        let mut mmu = Memory::new();
        let mut cpu = Cpu::new();
        cpu.reg.pc = 0xC000;
        mmu.set_byte(0xC000 as usize, 0xCB as u8);
        mmu.set_byte(0xC001 as usize, 0x7C as u8);
        cpu.step(&mut mmu);
        assert_eq!(cpu.reg.pc, 0xC002);
    }

    #[test]
    #[should_panic(expected = "Unimplemented instruction! 0xD3 PC: 0xC000")]
    fn test_step_unimplemented_instruction() {
        let mut mmu = Memory::new();
        let mut cpu = Cpu::new();
        cpu.reg.pc = 0xC000;
        mmu.set_byte(0xC000 as usize, 0xD3 as u8);
        cpu.step(&mut mmu);
    }

    #[test]
    #[should_panic(expected = "Unimplemented CB instruction! 0x00 PC: 0xC000")]
    fn test_step_unimplemented_cb_instruction() {
        let mut mmu = Memory::new();
        let mut cpu = Cpu::new();
        cpu.reg.pc = 0xC000;
        mmu.set_byte(0xC000 as usize, 0xCB);
        mmu.set_byte(0xC001 as usize, 0x00);
        cpu.step(&mut mmu);
    }
}
