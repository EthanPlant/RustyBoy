use crate::cpu::instructions::Instruction;

/// Get an instruction by opcode
pub fn get_instruction(op_code: &u8) -> Option<&Instruction> {
    match op_code {
        _ => None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_instruction() {
        let instruction = get_instruction(&0x00);
        assert_eq!(instruction, None);
    }
}