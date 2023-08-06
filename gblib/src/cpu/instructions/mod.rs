use crate::cpu::cpu::Cpu;

pub mod instructions;
pub mod cb_instructions;

/// The two types of opcodes
#[derive(Debug, PartialEq)]
pub enum OpCode {
    Regular(u8),
    CB(u8)
}

/// Type of instruction
pub enum ExecutionType {
    ActionTaken,
    Jumped,
    JumpedActionTaken,
    None
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
    pub description: String,
    /// The handler for the instruction
    pub handler: fn(cpu: &mut Cpu, op_code: &OpCode) -> ExecutionType
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
        let instruction = get_instruction_by_opcode(&OpCode::Regular(0x00));
        assert_eq!(instruction, None);
    }

    #[test]
    fn test_get_instruction_by_opcode_cb() {
        let instruction = get_instruction_by_opcode(&OpCode::CB(0x00));
        assert_eq!(instruction, None);
    }
}