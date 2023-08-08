use crate::cpu::cpu::Cpu;
use crate::mmu::Memory;

pub mod instructions;
pub mod cb_instructions;
mod functions;

#[derive(Debug, PartialEq)]
pub enum InstructionType {
    None,
    ActionTaken,
    Jumped,
}

/// The two types of opcodes
#[derive(Debug, PartialEq)]
pub enum OpCode {
    Regular(u8),
    CB(u8)
}

/// Information about an instructions
#[derive(Debug, PartialEq)]
pub struct Instruction {
    /// Length of the instruction and arguments
    pub length: u16,
    /// Number of clock cycles the instruction takes
    pub clock_cycles: u8,
    /// Number of clock cycles the instruction takes if a condition is met
    pub clock_cycles_condition: Option<u8>,
    /// Description of the instruction
    pub description: &'static str,
    /// The handler for the instruction
    pub handler: fn(cpu: &mut Cpu, mmu: &mut Memory, op_code: &OpCode) -> InstructionType,
}

/// Get an instruction by opcode
pub fn get_instruction_by_opcode(op_code: &OpCode) -> Option<&Instruction> {
    match op_code {
        OpCode::Regular(value) => instructions::get_instruction(&value),
        OpCode::CB(value) => cb_instructions::get_instruction(&value),
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_get_instruction_by_opcode() {
        let nop = Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "NOP",
            handler: |_: &mut Cpu, _: &mut Memory, _: &OpCode| {
                InstructionType::None
            }
        };
        let instruction = get_instruction_by_opcode(&OpCode::Regular(0x00));
        assert_eq!(instruction.unwrap().clock_cycles, nop.clock_cycles);
        assert_eq!(instruction.unwrap().length, nop.length);
        assert_eq!(instruction.unwrap().clock_cycles_condition, nop.clock_cycles_condition);
        assert_eq!(instruction.unwrap().description, nop.description);
    }

    #[test]
    fn test_get_instruction_by_opcode_unimplemented() {
        let instruction = get_instruction_by_opcode(&OpCode::Regular(0xD3));
        assert_eq!(instruction, None);
    }

    #[test]
    fn test_get_instruction_by_opcode_cb() {
        let rl_c = Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "RL C",
            handler: |_: &mut Cpu, _: &mut Memory, _: &OpCode| {
                InstructionType::None
            }
        };
        let instruction = get_instruction_by_opcode(&OpCode::CB(0x11));
        assert_eq!(instruction.unwrap().clock_cycles, rl_c.clock_cycles);
        assert_eq!(instruction.unwrap().length, rl_c.length);
        assert_eq!(instruction.unwrap().clock_cycles_condition, rl_c.clock_cycles_condition);
        assert_eq!(instruction.unwrap().description, rl_c.description);
    }

    #[test]
    fn test_get_instruction_by_opcode_cb_unimplemented() {
        let instruction = get_instruction_by_opcode(&OpCode::CB(0x00));
        assert_eq!(instruction, None);
    }
}