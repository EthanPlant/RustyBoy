use crate::cpu::instructions::{get_instruction_by_opcode, Instruction, InstructionType, OpCode};
use crate::cpu::interrupts::{handle_interrupts, pending_interrupt};
use crate::cpu::registers::Registers;
use crate::mmu::Memory;

/// Emulation of the Gameboy CPU
#[derive(Clone, Debug, PartialEq)]
pub struct Cpu {
    /// The CPU registers
    pub reg: Registers,
    /// Interrupt Master Enable
    pub ime: bool,
    /// Boolean to track if EI was called to emulate EI delay
    pub ei: bool,
    /// Boolean to track if the CPU is halted
    pub halted: bool,
}

impl Cpu {
    /// Create a new CPU
    pub fn new() -> Self {
        let mut registers = Registers::new();
        registers.a = 0x01;
        registers.f = 0xB0;
        registers.b = 0x00;
        registers.c = 0x13;
        registers.d = 0x00;
        registers.e = 0xD8;
        registers.h = 0x01;
        registers.l = 0x4D;
        registers.sp = 0xFFFE;
        registers.pc = 0x0100;
        Cpu {
            reg: registers,
            ime: false,
            ei: false,
            halted: false,
        }
    }

    /// Step through the emulator
    /// Returns the number of cycles used
    pub fn step(&mut self, mmu: &mut Memory) -> u8 {
        log::trace!("A:{:02X} F:{:02X} B:{:02X} C:{:02X} D:{:02X} E:{:02X} H:{:02X} L:{:02X} SP:{:02X} PC: {:04X} ({:02X} {:02X} {:02X} {:02X})",
        self.reg.a, self.reg.f, self.reg.b, self.reg.c, self.reg.d, self.reg.e, self.reg.h, self.reg.l, self.reg.sp, self.reg.pc, mmu.get_byte(self.reg.pc), mmu.get_byte(self.reg.pc + 1), mmu.get_byte(self.reg.pc + 2), mmu.get_byte(self.reg.pc + 3));

        let op_code = self.read_opcode(mmu);

        let instruction = match get_instruction_by_opcode(&op_code) {
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

        if self.halted && pending_interrupt(mmu) {
            self.halted = false;
            log::trace!("Exiting HALT");
            if !self.ime {
                // HALT bug
                // If interrupts are disabled but there's a pending interrupt, HALT ends
                // but the PC isn't incremented
                self.reg.pc -= 1;
            }
        }

        // If we're halted and there are no pending interrupts, exit early
        if self.halted {
            return 4;
        }

        // Process any interrupts before executing the next instruction
        let cycles_used = self.check_interrupts(mmu);
        if cycles_used != 0 {
                return cycles_used;
        }

        log::trace!("Executing instruction: {}", instruction.description);

        self.execute_instruction(mmu, instruction, &op_code)
    }

    /// Handle interrupts
    /// Returns the number of cycles used to handle an interrupt if one occured
    fn check_interrupts(&mut self, mmu: &mut Memory) -> u8 {
        if self.ime {
            match handle_interrupts(self, mmu) {
                Some(cycles) => {
                    self.ime = false;
                    return cycles;
                }
                _ => {
                    return 0;
                }
            }
        } else {
            if self.ei {
                log::trace!("Enabling interrupts!");
                self.ime = true;
                self.ei = false;
            }
        }

        0
    }

    /// Execute an instruction
    fn execute_instruction(
        &mut self,
        mmu: &mut Memory,
        instruction: &Instruction,
        op_code: &OpCode,
    ) -> u8 {
        let result = (instruction.handler)(self, mmu, op_code);
        // Update the program counter based on the instruction length and type
        match result {
            InstructionType::Jumped => match instruction.clock_cycles_condition {
                Some(cycles) => {
                    return cycles;
                }
                _ => {
                    return instruction.clock_cycles;
                }
            },
            _ => {
                self.reg.pc += instruction.length;
                return instruction.clock_cycles;
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
        assert_eq!(cpu.reg.a, 0x01);
        assert_eq!(cpu.reg.f, 0xB0);
        assert_eq!(cpu.reg.b, 0x00);
        assert_eq!(cpu.reg.c, 0x13);
        assert_eq!(cpu.reg.d, 0x00);
        assert_eq!(cpu.reg.e, 0xD8);
        assert_eq!(cpu.reg.h, 0x01);
        assert_eq!(cpu.reg.l, 0x4D);
        assert_eq!(cpu.reg.sp, 0xFFFE);
        assert_eq!(cpu.reg.pc, 0x100);
        assert_eq!(cpu.ime, false)
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
}
