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

    use crate::cpu::registers::{Flag, Registers};

    #[test]
    fn test_get_instruction_rl_c() {
        let instruction = get_instruction(&0x11);
        assert_eq!(instruction.unwrap(), &RL_C);
        assert_eq!(instruction.unwrap().length, 2);
        assert_eq!(instruction.unwrap().clock_cycles, 8);
        assert_eq!(instruction.unwrap().clock_cycles_condition, None);
    }

    #[test]
    fn test_rl_c_bit_seven() {
        let mut cpu = Cpu::new();
        cpu.reg.c = 0x80;
        cpu.reg.clear_all_flags();

        let expected_cpu = Cpu {
            reg: Registers {
                c: 0x00,
                f: 0x90, // Z__C
                ..cpu.reg
            },
        };

        (&RL_C.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x11));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    fn test_rl_c_bit_seven_clear() {
        let mut cpu = Cpu::new();
        cpu.reg.c = 0x00;
        cpu.reg.clear_all_flags();

        let expected_cpu = Cpu {
            reg: Registers {
                c: 0x00,
                f: 0x80, // ___C
                ..cpu.reg
            },
        };

        (&RL_C.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x11));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    fn test_rl_c_bit_seven_carry() {
        let mut cpu = Cpu::new();
        cpu.reg.c = 0x80;
        cpu.reg.set_flag(Flag::Carry);

        let expected_cpu = Cpu {
            reg: Registers {
                c: 0x01,
                f: 0x10, // ___C
                ..cpu.reg
            },
        };

        (&RL_C.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x11));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    fn test_rl_c_bit_seven_clear_carry() {
        let mut cpu = Cpu::new();
        cpu.reg.c = 0x00;
        cpu.reg.set_flag(Flag::Carry);

        let expected_cpu = Cpu {
            reg: Registers {
                c: 0x01,
                f: 0x00, // ____
                ..cpu.reg
            },
        };

        (&RL_C.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x11));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    fn test_get_instruction_bit_7_h() {
        let instruction = get_instruction(&0x7C);
        assert_eq!(instruction.unwrap(), &BIT_7_H);
        assert_eq!(instruction.unwrap().length, 2);
        assert_eq!(instruction.unwrap().clock_cycles, 8);
        assert_eq!(instruction.unwrap().clock_cycles_condition, None);
    }

    #[test]
    fn test_bit_7_h_one() {
        let mut cpu = Cpu::new();
        cpu.reg.h = 0x80;

        let expected_cpu = Cpu {
            reg: Registers {
                f: 0x20, // __H_
                ..cpu.reg
            },
        };

        (&BIT_7_H.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x7C));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    fn test_bit_7_h_zero() {
        let mut cpu = Cpu::new();
        cpu.reg.h = 0x00;

        let expected_cpu = Cpu {
            reg: Registers {
                f: 0xA0, // Z_H_
                ..cpu.reg
            },
        };

        (&BIT_7_H.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x7C));
        assert_eq!(cpu, expected_cpu);
    }
}
