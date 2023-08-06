use crate::cpu::instructions::Instruction;

/// Get an instruction by opcode
pub fn get_instruction(op_code: &u8) -> Option<&Instruction> {
    match op_code {
        _ => None
    }
}