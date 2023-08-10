use crate::cpu::cpu::Cpu;
use crate::cpu::instructions::{functions, Instruction, InstructionType, OpCode};
use crate::mmu::Memory;

/// 0x11 - RL C
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

/// 0x19 - RR C
const RR_C: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RR C",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.c = functions::rr(cpu, cpu.reg.c);
        InstructionType::ActionTaken
    },
};

/// 0x1A -- RR D
const RR_D: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RR D",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.d = functions::rr(cpu, cpu.reg.d);
        InstructionType::ActionTaken
    },
};

/// 0x38 - SRL B
const SRL_B: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SRL B",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.b = functions::srl(cpu, cpu.reg.b);
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
        0x19 => Some(&RR_C),
        0x1A => Some(&RR_D),

        0x38 => Some(&SRL_B),

        0x7C => Some(&BIT_7_H),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::cpu::registers::Flag;

    #[test]
    pub fn test_get_instruction_rl_c() {
        let instruction = get_instruction(&0x11).unwrap();
        assert_eq!(instruction, &RL_C);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_rl_c() {
        let mut cpu = Cpu::new();
        cpu.reg.c = 0x80;
        cpu.reg.clear_flag(Flag::Carry);
        (&RL_C.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x11));
        assert_eq!(cpu.reg.c, 0x00);
    }

    #[test]
    pub fn test_get_instruction_rr_c() {
        let instruction = get_instruction(&0x19).unwrap();
        assert_eq!(instruction, &RR_C);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_rr_c() {
        let mut cpu = Cpu::new();
        cpu.reg.c = 0x80;
        cpu.reg.clear_flag(Flag::Carry);
        (&RR_C.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x19));
        assert_eq!(cpu.reg.c, 0x40);
    }

    #[test]
    pub fn test_get_instruction_rr_d() {
        let instruction = get_instruction(&0x1A).unwrap();
        assert_eq!(instruction, &RR_D);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_rr_d() {
        let mut cpu = Cpu::new();
        cpu.reg.d = 0x80;
        cpu.reg.clear_flag(Flag::Carry);
        (&RR_D.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x1A));
        assert_eq!(cpu.reg.d, 0x40);
    }

    #[test]
    pub fn test_get_instruction_srl_b() {
        let instruction = get_instruction(&0x38).unwrap();
        assert_eq!(instruction, &SRL_B);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_srl_b() {
        let mut cpu = Cpu::new();
        cpu.reg.b = 0x80;
        (&SRL_B.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x38));
        assert_eq!(cpu.reg.b, 0x40);
    }

    #[test]
    pub fn test_get_instruction_bit_7_h() {
        let instruction = get_instruction(&0x7C).unwrap();
        assert_eq!(instruction, &BIT_7_H);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_bit_7_h() {
        let mut cpu = Cpu::new();
        cpu.reg.h = 0x7F;
        (&BIT_7_H.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x7C));
        assert!(cpu.reg.check_flag(Flag::Zero));
    }
}
