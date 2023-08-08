use crate::cpu::cpu::Cpu;
use crate::cpu::instructions::{functions, Instruction, InstructionType, OpCode};
use crate::mmu::Memory;

// 0x11 - RL C
const RL_C: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RL C",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.c = functions::rl(cpu, cpu.reg.c);
        InstructionType::ActionTaken
    },
};

/// 0x7C - BIT 7 H
const BIT_7_H: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "BIT 7 H",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        functions::bit(cpu, cpu.reg.h, 7);
        InstructionType::ActionTaken
    },
};

/// Get a CB instruction by opcode
pub fn get_instruction(op_code: &u8) -> Option<&Instruction> {
    match op_code {
        0x11 => Some(&RL_C),
        0x7C => Some(&BIT_7_H),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::registers::Flag;

    #[test]
    fn test_get_cb_instruction_rl_c() {
        let instruction = get_instruction(&0x11);
        let mut cpu = Cpu::new();
        cpu.reg.c = 0x80;
        (instruction.unwrap().handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x11));
        assert_eq!(cpu.reg.c, 0x01);
        assert!(!cpu.reg.check_flag(Flag::Zero));
        assert!(!cpu.reg.check_flag(Flag::Subtract));
        assert!(!cpu.reg.check_flag(Flag::HalfCarry));
        assert!(cpu.reg.check_flag(Flag::Carry));
    }

    #[test]
    fn test_get_cb_instruction_rl_c_zero() {
        let instruction = get_instruction(&0x11);
        let mut cpu = Cpu::new();
        cpu.reg.c = 0x00;
        (instruction.unwrap().handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x11));
        assert_eq!(cpu.reg.c, 0x00);
        assert!(cpu.reg.check_flag(Flag::Zero));
        assert!(!cpu.reg.check_flag(Flag::Subtract));
        assert!(!cpu.reg.check_flag(Flag::HalfCarry));
        assert!(!cpu.reg.check_flag(Flag::Carry));
    }

    #[test]
    fn test_get_cb_instruction() {
        let instruction = get_instruction(&0x00);
        assert_eq!(instruction, None);
    }

    #[test]
    fn test_get_cb_instruction_bit_7_h() {
        let instruction = get_instruction(&0x7C);
        let mut cpu = Cpu::new();
        cpu.reg.h = 0x80;
        (instruction.unwrap().handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x7C));
        assert!(!cpu.reg.check_flag(Flag::Zero));
        assert!(!cpu.reg.check_flag(Flag::Subtract));
        assert!(cpu.reg.check_flag(Flag::HalfCarry));
        assert!(!cpu.reg.check_flag(Flag::Carry));
    }

    #[test]
    fn test_get_cb_instruction_bit_7_h_zero() {
        let instruction = get_instruction(&0x7C);
        let mut cpu = Cpu::new();
        cpu.reg.h = 0x00;
        (instruction.unwrap().handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x7C));
        assert!(cpu.reg.check_flag(Flag::Zero));
        assert!(!cpu.reg.check_flag(Flag::Subtract));
        assert!(cpu.reg.check_flag(Flag::HalfCarry));
        assert!(!cpu.reg.check_flag(Flag::Carry));
    }

    #[test]
    fn test_get_cb_instruction_unimplemented() {
        let instruction = get_instruction(&0x00);
        assert_eq!(instruction, None);
    }
}
