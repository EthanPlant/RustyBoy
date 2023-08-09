use crate::cpu::cpu::Cpu;
use crate::cpu::instructions::{functions, Instruction, InstructionType, OpCode};
use crate::cpu::registers::Flag;
use crate::mmu::Memory;

/// 0x00 - NOP
const NOP: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "NOP",
    handler: |_: &mut Cpu, _: &mut Memory, _: &OpCode| InstructionType::None,
};

/// 0x04 - INC B
const INC_B: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "INC B",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.b = functions::inc(cpu, cpu.reg.b);
        InstructionType::ActionTaken
    },
};

/// 0x05 - DEC B
const DEC_B: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "DEC B",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.b = functions::dec(cpu, cpu.reg.b);
        InstructionType::ActionTaken
    },
};

/// 0x06 - LD B, n
const LD_B_N: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "LD B n",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        cpu.reg.b = functions::get_op8(cpu, mmu, 1);
        InstructionType::ActionTaken
    },
};

/// 0x0C - INC C
const INC_C: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "INC C",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.c = functions::inc(cpu, cpu.reg.c);
        InstructionType::ActionTaken
    },
};

/// 0x0D - DEC C
const DEC_C: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "DEC C",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.c = functions::dec(cpu, cpu.reg.c);
        InstructionType::ActionTaken
    },
};

/// 0x0E - LD C, n
const LD_C_N: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "LD C n",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        cpu.reg.c = functions::get_op8(cpu, mmu, 1);
        InstructionType::ActionTaken
    },
};

/// 0x11 - LD DE, nn
const LD_DE_NN: Instruction = Instruction {
    length: 3,
    clock_cycles: 12,
    clock_cycles_condition: None,
    description: "LD DE nn",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        cpu.reg.set_de(functions::get_op16(cpu, mmu));
        InstructionType::ActionTaken
    },
};

/// 0x13 - INC DE
const INC_DE: Instruction = Instruction {
    length: 1,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "INC DE",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.set_de(cpu.reg.de().wrapping_add(1));
        InstructionType::ActionTaken
    },
};

/// 0x15 - DEC D
const DEC_D: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "DEC D",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.d = functions::dec(cpu, cpu.reg.d);
        InstructionType::ActionTaken
    },
};

/// 0x16 - LD D, n
const LD_D_N: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "LD D n",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        cpu.reg.d = functions::get_op8(cpu, mmu, 1);
        InstructionType::ActionTaken
    },
};

/// 0x17 - RLA
const RLA: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "RLA",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.a = functions::rl(cpu, cpu.reg.a);
        // Unlike normal RL operations, RLA always clears the zero flag
        cpu.reg.clear_flag(Flag::Zero);
        InstructionType::ActionTaken
    },
};

/// 0x18 - JR n
const JR_N: Instruction = Instruction {
    length: 2,
    clock_cycles: 12,
    clock_cycles_condition: None,
    description: "JR n",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        functions::jr(cpu, mmu);
        InstructionType::Jumped
    },
};

/// 0x1A - LD A, (DE)
const LD_A_DE: Instruction = Instruction {
    length: 1,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "LD A (DE)",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        cpu.reg.a = mmu.get_byte(cpu.reg.de());
        InstructionType::ActionTaken
    },
};

/// 0x1D - DEC E
const DEC_E: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "DEC E",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.e = functions::dec(cpu, cpu.reg.e);
        InstructionType::ActionTaken
    },
};

/// 0x1E - LD E, n
const LD_E_N: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "LD E n",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        cpu.reg.e = functions::get_op8(cpu, mmu, 1);
        InstructionType::ActionTaken
    },
};

/// 0x20 - JR NZ, n
const JR_NZ_N: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: Some(12),
    description: "JR NZ n",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        if !cpu.reg.check_flag(Flag::Zero) {
            functions::jr(cpu, mmu);
            return InstructionType::Jumped;
        }
        InstructionType::None
    },
};

/// 0x21 - LD HL, nn
const LD_HL_NN: Instruction = Instruction {
    length: 3,
    clock_cycles: 12,
    clock_cycles_condition: None,
    description: "LD HL nn",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        cpu.reg.set_hl(functions::get_op16(cpu, mmu));
        InstructionType::ActionTaken
    },
};

/// 0x22 - LD (HL+), A
const LD_HL_INC_A: Instruction = Instruction {
    length: 1,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "LD (HL+) A",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        functions::set_hl(cpu, mmu, cpu.reg.a);
        cpu.reg.set_hl(cpu.reg.hl().wrapping_add(1));
        InstructionType::ActionTaken
    },
};

/// 0x23 - INC HL
const INC_HL: Instruction = Instruction {
    length: 1,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "INC HL",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.set_hl(cpu.reg.hl().wrapping_add(1));
        InstructionType::ActionTaken
    },
};

/// 0x24 - INC H
const INC_H: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "INC H",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.h = functions::inc(cpu, cpu.reg.h);
        InstructionType::ActionTaken
    },
};

/// 0x28 - JR Z, n
const JR_Z_N: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: Some(12),
    description: "JR Z n",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        if cpu.reg.check_flag(Flag::Zero) {
            functions::jr(cpu, mmu);
            return InstructionType::Jumped;
        }
        InstructionType::None
    },
};

/// 0x2E - LD L, n
const LD_L_N: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "LD L, n",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        cpu.reg.l = functions::get_op8(cpu, mmu, 1);
        InstructionType::ActionTaken
    },
};

/// 0x31 - LD SP, nn
const LD_SP_NN: Instruction = Instruction {
    length: 3,
    clock_cycles: 12,
    clock_cycles_condition: None,
    description: "LD SP nn",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        cpu.reg.sp = functions::get_op16(cpu, mmu);
        InstructionType::ActionTaken
    },
};

/// 0x32 - LD (HL-), A
const LD_HL_DEC_A: Instruction = Instruction {
    length: 1,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "LD (HL-) A",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        functions::set_hl(cpu, mmu, cpu.reg.a);
        cpu.reg.set_hl(cpu.reg.hl().wrapping_sub(1));
        InstructionType::ActionTaken
    },
};

/// 0x3D - DEC A
const DEC_A: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "DEC A",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.a = functions::dec(cpu, cpu.reg.a);
        InstructionType::ActionTaken
    },
};

/// 0x3E - LD A, n
const LD_A_N: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "LD A n",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        cpu.reg.a = functions::get_op8(cpu, mmu, 1);
        InstructionType::ActionTaken
    },
};

/// 0x4F - LD C, A
const LD_C_A: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "LD C A",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.c = cpu.reg.a;
        InstructionType::ActionTaken
    },
};

/// 0x57 - LD D, A
const LD_D_A: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "LD D A",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.d = cpu.reg.a;
        InstructionType::ActionTaken
    },
};

/// 0x67 - LD H, A
const LD_H_A: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "LD H, A",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.h = cpu.reg.a;
        InstructionType::ActionTaken
    },
};

/// 0x77 - LD (HL), A
const LD_HL_A: Instruction = Instruction {
    length: 1,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "LD (HL) A",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        functions::set_hl(cpu, mmu, cpu.reg.a);
        InstructionType::ActionTaken
    },
};

/// 0x78 - LD A, B
const LD_A_B: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "LD A, B",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.a = cpu.reg.b;
        InstructionType::ActionTaken
    },
};

/// 0x7B - LD A, E
const LD_A_E: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "LD A, E",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.a = cpu.reg.e;
        InstructionType::ActionTaken
    },
};

/// 0x7C - LD A, H
const LD_A_H: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "LD A, H",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.a = cpu.reg.h;
        InstructionType::ActionTaken
    },
};

/// 0x7D - LD A, L
const LD_A_L: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "LD A, L",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.a = cpu.reg.l;
        InstructionType::ActionTaken
    },
};

/// 0x86 - ADD (HL)
const ADD_HL: Instruction = Instruction {
    length: 1,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "ADD (HL)",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        functions::add(cpu, functions::get_hl(cpu, mmu));
        InstructionType::ActionTaken
    },
};

/// 0x90 - SUB B
const SUB_B: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "SUB B",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        functions::sub(cpu, cpu.reg.b);
        InstructionType::ActionTaken
    },
};

/// 0xAF - XOR A
const XOR_A: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "XOR A",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        functions::xor_bytes(cpu, cpu.reg.a, cpu.reg.a);
        InstructionType::ActionTaken
    },
};

/// 0xBE - CP (HL)
const CP_HL: Instruction = Instruction {
    length: 1,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "CP (HL)",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        functions::cp(cpu, functions::get_hl(cpu, mmu));
        InstructionType::ActionTaken
    },
};

/// 0xC1 - POP BC
const POP_BC: Instruction = Instruction {
    length: 1,
    clock_cycles: 12,
    clock_cycles_condition: None,
    description: "POP BC",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        let val = functions::pop(cpu, mmu);
        cpu.reg.set_bc(val);
        InstructionType::ActionTaken
    },
};

/// 0xC5 - PUSH BC
const PUSH_BC: Instruction = Instruction {
    length: 1,
    clock_cycles: 16,
    clock_cycles_condition: None,
    description: "PUSH BC",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        functions::push(cpu, mmu, cpu.reg.bc());
        InstructionType::ActionTaken
    },
};

/// 0xC9 - RET
const RET: Instruction = Instruction {
    length: 1,
    clock_cycles: 16,
    clock_cycles_condition: None,
    description: "RET",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        cpu.reg.pc = functions::pop(cpu, mmu);
        InstructionType::Jumped
    },
};

/// 0xCD - CALL nn
const CALL_NN: Instruction = Instruction {
    length: 3,
    clock_cycles: 24,
    clock_cycles_condition: None,
    description: "CALL nn",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        functions::call(cpu, mmu);
        InstructionType::Jumped
    },
};

/// 0xE0 - LDH (n), A
const LDH_N_A: Instruction = Instruction {
    length: 2,
    clock_cycles: 12,
    clock_cycles_condition: None,
    description: "LDH (n) A",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        mmu.set_byte(0xFF00 + functions::get_op8(cpu, mmu, 1) as u16, cpu.reg.a);
        InstructionType::ActionTaken
    },
};

/// 0xE2 - LD (C), A
const LDH_C_A: Instruction = Instruction {
    length: 1,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "LDH (C) A",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        mmu.set_byte(0xFF00 + cpu.reg.c as u16, cpu.reg.a);
        InstructionType::ActionTaken
    },
};

/// 0xEA - LD (nn), A
const LD_NN_A: Instruction = Instruction {
    length: 3,
    clock_cycles: 16,
    clock_cycles_condition: None,
    description: "LD (nn) A",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        mmu.set_byte(functions::get_op16(cpu, mmu), cpu.reg.a);
        InstructionType::ActionTaken
    },
};

/// 0xF0 - LDH A, (n)
const LDH_A_N: Instruction = Instruction {
    length: 2,
    clock_cycles: 12,
    clock_cycles_condition: None,
    description: "LDH A (n)",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        cpu.reg.a = mmu.get_byte(0xFF00 + functions::get_op8(cpu, mmu, 1) as u16);
        InstructionType::ActionTaken
    },
};

/// 0xFE - CP A, n
const CP_A_N: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "CP A, n",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        functions::cp(cpu, functions::get_op8(cpu, mmu, 1));
        InstructionType::ActionTaken
    },
};

/// Get an instruction by opcode
pub fn get_instruction(op_code: &u8) -> Option<&Instruction> {
    match op_code {
        0x00 => Some(&NOP),
        0x04 => Some(&INC_B),
        0x05 => Some(&DEC_B),
        0x06 => Some(&LD_B_N),
        0x0C => Some(&INC_C),
        0x0D => Some(&DEC_C),
        0x0E => Some(&LD_C_N),

        0x11 => Some(&LD_DE_NN),
        0x15 => Some(&DEC_D),
        0x13 => Some(&INC_DE),
        0x16 => Some(&LD_D_N),
        0x17 => Some(&RLA),
        0x18 => Some(&JR_N),
        0x1A => Some(&LD_A_DE),
        0x1D => Some(&DEC_E),
        0x1E => Some(&LD_E_N),

        0x20 => Some(&JR_NZ_N),
        0x21 => Some(&LD_HL_NN),
        0x22 => Some(&LD_HL_INC_A),
        0x23 => Some(&INC_HL),
        0x24 => Some(&INC_H),
        0x28 => Some(&JR_Z_N),
        0x2E => Some(&LD_L_N),

        0x31 => Some(&LD_SP_NN),
        0x32 => Some(&LD_HL_DEC_A),
        0x3D => Some(&DEC_A),
        0x3E => Some(&LD_A_N),

        0x4F => Some(&LD_C_A),

        0x57 => Some(&LD_D_A),

        0x67 => Some(&LD_H_A),

        0x77 => Some(&LD_HL_A),
        0x78 => Some(&LD_A_B),
        0x7B => Some(&LD_A_E),
        0x7C => Some(&LD_A_H),
        0x7D => Some(&LD_A_L),

        0x86 => Some(&ADD_HL),

        0x90 => Some(&SUB_B),

        0xAF => Some(&XOR_A),

        0xBE => Some(&CP_HL),

        0xC1 => Some(&POP_BC),
        0xC5 => Some(&PUSH_BC),
        0xC9 => Some(&RET),
        0xCD => Some(&CALL_NN),

        0xE0 => Some(&LDH_N_A),
        0xE2 => Some(&LDH_C_A),
        0xEA => Some(&LD_NN_A),

        0xF0 => Some(&LDH_A_N),
        0xFE => Some(&CP_A_N),

        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::cpu::registers::Registers;

    #[test]
    pub fn test_get_instruction_nop() {
        let instruction = get_instruction(&0x00);
        assert_eq!(instruction.unwrap(), &NOP);
        assert_eq!(instruction.unwrap().length, 1);
        assert_eq!(instruction.unwrap().clock_cycles, 4);
    }

    #[test]
    pub fn test_nop() {
        let mut cpu = Cpu::new();
        let expected_cpu = Cpu::new();
        (&NOP.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x00));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_get_instruction_inc_b() {
        let instruction = get_instruction(&0x04);
        assert_eq!(instruction.unwrap(), &INC_B);
        assert_eq!(instruction.unwrap().length, 1);
        assert_eq!(instruction.unwrap().clock_cycles, 4);
    }

    #[test]
    pub fn test_inc_b() {
        let mut cpu = Cpu::new();
        cpu.reg.b = 0x01;

        let expected_cpu = Cpu {
            reg: Registers { b: 0x02, ..cpu.reg },
            ..cpu
        };

        (&INC_B.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x04));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_inc_b_zero() {
        let mut cpu = Cpu::new();
        cpu.reg.b = 0xFF;

        let expected_cpu = Cpu {
            reg: Registers {
                f: 0xA0, // Z_H_
                b: 0x00,
                ..cpu.reg
            },
            ..cpu
        };

        (&INC_B.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x04));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_inc_b_half_carry() {
        let mut cpu = Cpu::new();
        cpu.reg.b = 0x0F;

        let expected_cpu = Cpu {
            reg: Registers {
                f: 0x20, // __H_
                b: 0x10,
                ..cpu.reg
            },
            ..cpu
        };

        (&INC_B.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x04));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_get_instruction_dec_b() {
        let instruction = get_instruction(&0x05);
        assert_eq!(instruction.unwrap(), &DEC_B);
        assert_eq!(instruction.unwrap().length, 1);
        assert_eq!(instruction.unwrap().clock_cycles, 4);
    }

    #[test]
    pub fn test_dec_b() {
        let mut cpu = Cpu::new();
        cpu.reg.b = 0x02;

        let expected_cpu = Cpu {
            reg: Registers {
                b: 0x01,
                f: 0x40, // _N__
                ..cpu.reg
            },
            ..cpu
        };

        (&DEC_B.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x05));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_dec_b_zero() {
        let mut cpu = Cpu::new();
        cpu.reg.b = 0x01;

        let expected_cpu = Cpu {
            reg: Registers {
                b: 0x00,
                f: 0xC0, // ZN__
                ..cpu.reg
            },
            ..cpu
        };

        (&DEC_B.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x05));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_dec_b_half_carry() {
        let mut cpu = Cpu::new();
        cpu.reg.b = 0x10;

        let expected_cpu = Cpu {
            reg: Registers {
                b: 0x0F,
                f: 0x60, // _NH_
                ..cpu.reg
            },
            ..cpu
        };

        (&DEC_B.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x05));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_dec_b_underflow() {
        let mut cpu = Cpu::new();
        cpu.reg.b = 0x00;

        let expected_cpu = Cpu {
            reg: Registers {
                b: 0xFF,
                f: 0x60, // _NH_
                ..cpu.reg
            },
            ..cpu
        };

        (&DEC_B.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x05));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_get_instruction_ld_b_n() {
        let instruction = get_instruction(&0x06);
        assert_eq!(instruction.unwrap(), &LD_B_N);
        assert_eq!(instruction.unwrap().length, 2);
        assert_eq!(instruction.unwrap().clock_cycles, 8);
    }

    #[test]
    pub fn test_ld_b_n() {
        let mut cpu = Cpu::new();
        cpu.reg.pc = 0xC000;
        let mut mmu = Memory::new();
        mmu.set_byte(0xC001 as usize, 0x02);

        let expected_cpu = Cpu {
            reg: Registers { b: 0x02, ..cpu.reg },
            ..cpu
        };

        (&LD_B_N.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x06));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_get_instruction_inc_c() {
        let instruction = get_instruction(&0x0C);
        assert_eq!(instruction.unwrap(), &INC_C);
        assert_eq!(instruction.unwrap().length, 1);
        assert_eq!(instruction.unwrap().clock_cycles, 4);
    }

    #[test]
    pub fn test_inc_c() {
        let mut cpu = Cpu::new();
        cpu.reg.c = 0x01;

        let expected_cpu = Cpu {
            reg: Registers { c: 0x02, ..cpu.reg },
            ..cpu
        };

        (&INC_C.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x0C));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_inc_c_zero() {
        let mut cpu = Cpu::new();
        cpu.reg.c = 0xFF;

        let expected_cpu = Cpu {
            reg: Registers {
                f: 0xA0, // Z_H_
                c: 0x00,
                ..cpu.reg
            },
            ..cpu
        };

        (&INC_C.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x0C));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_inc_c_half_carry() {
        let mut cpu = Cpu::new();
        cpu.reg.c = 0x0F;

        let expected_cpu = Cpu {
            reg: Registers {
                f: 0x20, // __H_
                c: 0x10,
                ..cpu.reg
            },
            ..cpu
        };

        (&INC_C.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x0C));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_get_instruction_dec_c() {
        let instruction = get_instruction(&0x0D);
        assert_eq!(instruction.unwrap(), &DEC_C);
        assert_eq!(instruction.unwrap().length, 1);
        assert_eq!(instruction.unwrap().clock_cycles, 4);
    }

    #[test]
    pub fn test_dec_c() {
        let mut cpu = Cpu::new();
        cpu.reg.c = 0x02;

        let expected_cpu = Cpu {
            reg: Registers {
                c: 0x01,
                f: 0x40, // _N__
                ..cpu.reg
            },
            ..cpu
        };

        (&DEC_C.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x0D));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_dec_c_zero() {
        let mut cpu = Cpu::new();
        cpu.reg.c = 0x01;

        let expected_cpu = Cpu {
            reg: Registers {
                c: 0x00,
                f: 0xC0, // ZN__
                ..cpu.reg
            },
            ..cpu
        };

        (&DEC_C.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x0D));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_dec_c_half_carry() {
        let mut cpu = Cpu::new();
        cpu.reg.c = 0x10;

        let expected_cpu = Cpu {
            reg: Registers {
                c: 0x0F,
                f: 0x60, // _NH_
                ..cpu.reg
            },
            ..cpu
        };

        (&DEC_C.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x0D));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_dec_c_underflow() {
        let mut cpu = Cpu::new();
        cpu.reg.c = 0x00;

        let expected_cpu = Cpu {
            reg: Registers {
                c: 0xFF,
                f: 0x60, // _NH_
                ..cpu.reg
            },
            ..cpu
        };

        (&DEC_C.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x0D));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_get_instruction_ld_c_n() {
        let instruction = get_instruction(&0x0E);
        assert_eq!(instruction.unwrap(), &LD_C_N);
        assert_eq!(instruction.unwrap().length, 2);
        assert_eq!(instruction.unwrap().clock_cycles, 8);
    }

    #[test]
    pub fn test_ld_c_n() {
        let mut cpu = Cpu::new();
        cpu.reg.pc = 0xC000;
        let mut mmu = Memory::new();
        mmu.set_byte(0xC001 as usize, 0x02);

        let expected_cpu = Cpu {
            reg: Registers { c: 0x02, ..cpu.reg },
            ..cpu
        };

        (&LD_C_N.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x0E));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_get_instruction_ld_de_nn() {
        let instruction = get_instruction(&0x11);
        assert_eq!(instruction.unwrap(), &LD_DE_NN);
        assert_eq!(instruction.unwrap().length, 3);
        assert_eq!(instruction.unwrap().clock_cycles, 12);
    }

    #[test]
    pub fn test_ld_de_nn() {
        let mut cpu = Cpu::new();
        cpu.reg.pc = 0xC000;
        let mut mmu = Memory::new();
        mmu.set_word(0xC001 as usize, 0x0302);

        let expected_cpu = Cpu {
            reg: Registers {
                d: 0x03,
                e: 0x02,
                ..cpu.reg
            },
            ..cpu
        };

        (&LD_DE_NN.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x11));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_get_instruction_dec_d() {
        let instruction = get_instruction(&0x15);
        assert_eq!(instruction.unwrap(), &DEC_D);
        assert_eq!(instruction.unwrap().length, 1);
        assert_eq!(instruction.unwrap().clock_cycles, 4);
    }

    #[test]
    pub fn test_dec_d() {
        let mut cpu = Cpu::new();
        cpu.reg.d = 0x02;

        let expected_cpu = Cpu {
            reg: Registers {
                d: 0x01,
                f: 0x40, // _N__
                ..cpu.reg
            },
            ..cpu
        };

        (&DEC_D.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x15));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_dec_d_zero() {
        let mut cpu = Cpu::new();
        cpu.reg.d = 0x01;

        let expected_cpu = Cpu {
            reg: Registers {
                d: 0x00,
                f: 0xC0, // ZN__
                ..cpu.reg
            },
            ..cpu
        };

        (&DEC_D.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x15));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_dec_d_half_carry() {
        let mut cpu = Cpu::new();
        cpu.reg.d = 0x10;

        let expected_cpu = Cpu {
            reg: Registers {
                d: 0x0F,
                f: 0x60, // _NH_
                ..cpu.reg
            },
            ..cpu
        };

        (&DEC_D.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x15));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_dec_d_underflow() {
        let mut cpu = Cpu::new();
        cpu.reg.d = 0x00;

        let expected_cpu = Cpu {
            reg: Registers {
                d: 0xFF,
                f: 0x60, // _NH_
                ..cpu.reg
            },
            ..cpu
        };

        (&DEC_D.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x15));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_get_instruction_inc_de() {
        let instruction = get_instruction(&0x13);
        assert_eq!(instruction.unwrap(), &INC_DE);
        assert_eq!(instruction.unwrap().length, 1);
        assert_eq!(instruction.unwrap().clock_cycles, 8);
    }

    #[test]
    pub fn test_inc_de() {
        let mut cpu = Cpu::new();
        cpu.reg.d = 0x00;
        cpu.reg.e = 0x00;

        let expected_cpu = Cpu {
            reg: Registers {
                d: 0x00,
                e: 0x01,
                ..cpu.reg
            },
            ..cpu
        };

        (&INC_DE.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x13));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_inc_de_overflow() {
        let mut cpu = Cpu::new();
        cpu.reg.d = 0xFF;
        cpu.reg.e = 0xFF;

        let expected_cpu = Cpu {
            reg: Registers {
                d: 0x00,
                e: 0x00,
                ..cpu.reg
            },
            ..cpu
        };

        (&INC_DE.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x13));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_inc_de_e_overflow() {
        let mut cpu = Cpu::new();
        cpu.reg.d = 0x00;
        cpu.reg.e = 0xFF;

        let expected_cpu = Cpu {
            reg: Registers {
                d: 0x01,
                e: 0x00,
                ..cpu.reg
            },
            ..cpu
        };

        (&INC_DE.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x13));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_get_instruction_ld_d_n() {
        let instruction = get_instruction(&0x16);
        assert_eq!(instruction.unwrap(), &LD_D_N);
        assert_eq!(instruction.unwrap().length, 2);
        assert_eq!(instruction.unwrap().clock_cycles, 8);
    }

    #[test]
    pub fn test_ld_d_n() {
        let mut cpu = Cpu::new();
        cpu.reg.pc = 0xC000;
        let mut mmu = Memory::new();
        mmu.set_byte(0xC001 as usize, 0x02);

        let expected_cpu = Cpu {
            reg: Registers { d: 0x02, ..cpu.reg },
            ..cpu
        };

        (&LD_D_N.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x16));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_get_instruction_rla() {
        let instruction = get_instruction(&0x17);
        assert_eq!(instruction.unwrap(), &RLA);
        assert_eq!(instruction.unwrap().length, 1);
        assert_eq!(instruction.unwrap().clock_cycles, 4);
    }

    #[test]
    pub fn test_rla_bit_seven_set() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x80;

        let expected_cpu = Cpu {
            reg: Registers {
                a: 0x00,
                f: 0x10, // ___C
                ..cpu.reg
            },
            ..cpu
        };

        (&RLA.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x17));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_rla_bit_seven_set_carry() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x80;
        cpu.reg.f = 0x10;

        let expected_cpu = Cpu {
            reg: Registers {
                a: 0x01,
                f: 0x10, // ___C
                ..cpu.reg
            },
            ..cpu
        };

        (&RLA.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x17));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_rla_bit_seven_clear() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x00;

        let expected_cpu = Cpu {
            reg: Registers { a: 0x00, ..cpu.reg },
            ..cpu
        };

        (&RLA.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x17));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_rla_bit_seven_clear_carry() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x00;
        cpu.reg.f = 0x10;

        let expected_cpu = Cpu {
            reg: Registers {
                a: 0x01,
                f: 0x00, // ____
                ..cpu.reg
            },
            ..cpu
        };

        (&RLA.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x17));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_get_instruction_jr_n() {
        let instruction = get_instruction(&0x18);
        assert_eq!(instruction.unwrap(), &JR_N);
        assert_eq!(instruction.unwrap().length, 2);
        assert_eq!(instruction.unwrap().clock_cycles, 12);
    }

    #[test]
    pub fn test_jr_n() {
        let mut cpu = Cpu::new();
        cpu.reg.pc = 0xC000;
        let mut mmu = Memory::new();
        mmu.set_byte(0xC001 as usize, 0x02);

        let expected_cpu = Cpu {
            reg: Registers {
                pc: 0xC004,
                ..cpu.reg
            },
            ..cpu
        };

        (&JR_N.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x18));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_jr_n_negative() {
        let mut cpu = Cpu::new();
        cpu.reg.pc = 0xC000;
        let mut mmu = Memory::new();
        mmu.set_byte(0xC001 as usize, 0xFE);

        let expected_cpu = Cpu {
            reg: Registers {
                pc: 0xC000,
                ..cpu.reg
            },
            ..cpu
        };

        (&JR_N.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x18));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_get_instruction_ld_a_de() {
        let instruction = get_instruction(&0x1A);
        assert_eq!(instruction.unwrap(), &LD_A_DE);
        assert_eq!(instruction.unwrap().length, 1);
        assert_eq!(instruction.unwrap().clock_cycles, 8);
    }

    #[test]
    pub fn test_ld_a_de() {
        let mut cpu = Cpu::new();
        cpu.reg.d = 0xC0;
        cpu.reg.e = 0x00;
        let mut mmu = Memory::new();
        mmu.set_byte(0xC000 as usize, 0x03);

        let expected_cpu = Cpu {
            reg: Registers { a: 0x03, ..cpu.reg },
            ..cpu
        };

        (&LD_A_DE.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x1A));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_get_instruction_dec_e() {
        let instruction = get_instruction(&0x1D);
        assert_eq!(instruction.unwrap(), &DEC_E);
        assert_eq!(instruction.unwrap().length, 1);
        assert_eq!(instruction.unwrap().clock_cycles, 4);
    }

    #[test]
    pub fn test_dec_e() {
        let mut cpu = Cpu::new();
        cpu.reg.e = 0x02;

        let expected_cpu = Cpu {
            reg: Registers {
                e: 0x01,
                f: 0x40, // _N__
                ..cpu.reg
            },
            ..cpu
        };

        (&DEC_E.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x1D));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_dec_e_zero() {
        let mut cpu = Cpu::new();
        cpu.reg.e = 0x01;

        let expected_cpu = Cpu {
            reg: Registers {
                e: 0x00,
                f: 0xC0, // ZN__
                ..cpu.reg
            },
            ..cpu
        };

        (&DEC_E.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x1D));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_dec_e_half_carry() {
        let mut cpu = Cpu::new();
        cpu.reg.e = 0x10;

        let expected_cpu = Cpu {
            reg: Registers {
                e: 0x0F,
                f: 0x60, // _NH_
                ..cpu.reg
            },
            ..cpu
        };

        (&DEC_E.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x1D));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_dec_e_underflow() {
        let mut cpu = Cpu::new();
        cpu.reg.e = 0x00;

        let expected_cpu = Cpu {
            reg: Registers {
                e: 0xFF,
                f: 0x60, // _NH_
                ..cpu.reg
            },
            ..cpu
        };

        (&DEC_E.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x1D));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_get_instruction_ld_e_n() {
        let instruction = get_instruction(&0x1E);
        assert_eq!(instruction.unwrap(), &LD_E_N);
        assert_eq!(instruction.unwrap().length, 2);
        assert_eq!(instruction.unwrap().clock_cycles, 8);
    }

    #[test]
    pub fn test_ld_e_n() {
        let mut cpu = Cpu::new();
        cpu.reg.pc = 0xC000;
        let mut mmu = Memory::new();
        mmu.set_byte(0xC001 as usize, 0x02);

        let expected_cpu = Cpu {
            reg: Registers { e: 0x02, ..cpu.reg },
            ..cpu
        };

        (&LD_E_N.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x1E));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_get_instruction_jr_nz_n() {
        let instruction = get_instruction(&0x20);
        assert_eq!(instruction.unwrap(), &JR_NZ_N);
        assert_eq!(instruction.unwrap().length, 2);
        assert_eq!(instruction.unwrap().clock_cycles, 8);
        assert_eq!(instruction.unwrap().clock_cycles_condition.unwrap(), 12);
    }

    #[test]
    pub fn test_jr_nz_n_zero() {
        let mut cpu = Cpu::new();
        cpu.reg.f = 0x80;
        cpu.reg.pc = 0xC000;
        let mut mmu = Memory::new();
        mmu.set_byte(0xC001 as usize, 0x02);

        let expected_cpu = Cpu {
            reg: Registers {
                pc: 0xC000,
                ..cpu.reg
            },
            ..cpu
        };

        (&JR_NZ_N.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x20));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_jr_nz_zero_negative() {
        let mut cpu = Cpu::new();
        cpu.reg.f = 0x80;
        cpu.reg.pc = 0xC000;
        let mut mmu = Memory::new();
        mmu.set_byte(0xC001 as usize, 0xFE);

        let expected_cpu = Cpu {
            reg: Registers {
                pc: 0xC000,
                ..cpu.reg
            },
            ..cpu
        };

        (&JR_NZ_N.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x20));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_jr_nz_n_not_zero() {
        let mut cpu = Cpu::new();
        cpu.reg.pc = 0xC000;
        let mut mmu = Memory::new();
        mmu.set_byte(0xC001 as usize, 0x02);

        let expected_cpu = Cpu {
            reg: Registers {
                pc: 0xC004,
                ..cpu.reg
            },
            ..cpu
        };

        (&JR_NZ_N.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x20));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_jr_nz_n_not_zero_negative() {
        let mut cpu = Cpu::new();
        cpu.reg.pc = 0xC000;
        let mut mmu = Memory::new();
        mmu.set_byte(0xC001 as usize, 0xFE);

        let expected_cpu = Cpu {
            reg: Registers {
                pc: 0xC000,
                ..cpu.reg
            },
            ..cpu
        };

        (&JR_NZ_N.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x20));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_get_instruction_ld_hl_nn() {
        let instruction = get_instruction(&0x21);
        assert_eq!(instruction.unwrap(), &LD_HL_NN);
        assert_eq!(instruction.unwrap().length, 3);
        assert_eq!(instruction.unwrap().clock_cycles, 12);
    }

    #[test]
    pub fn test_ld_hl_nn() {
        let mut cpu = Cpu::new();
        cpu.reg.pc = 0xC000;
        let mut mmu = Memory::new();
        mmu.set_word(0xC001 as usize, 0x0302);

        let expected_cpu = Cpu {
            reg: Registers {
                h: 0x03,
                l: 0x02,
                ..cpu.reg
            },
            ..cpu
        };

        (&LD_HL_NN.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x21));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_get_instruction_ld_hl_inc_a() {
        let instruction = get_instruction(&0x22);
        assert_eq!(instruction.unwrap(), &LD_HL_INC_A);
        assert_eq!(instruction.unwrap().length, 1);
        assert_eq!(instruction.unwrap().clock_cycles, 8);
    }

    #[test]
    pub fn test_ld_hl_inc_a() {
        let mut cpu = Cpu::new();
        cpu.reg.h = 0xC0;
        cpu.reg.l = 0x00;
        cpu.reg.a = 0x03;
        let mut mmu = Memory::new();

        let expected_cpu = Cpu {
            reg: Registers {
                h: 0xC0,
                l: 0x01,
                ..cpu.reg
            },
            ..cpu
        };

        (&LD_HL_INC_A.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x22));
        assert_eq!(cpu, expected_cpu);
        assert_eq!(mmu.get_byte(0xC000 as usize), 0x03);
    }

    #[test]
    pub fn test_ld_hl_inc_a_overflow() {
        let mut cpu = Cpu::new();
        cpu.reg.h = 0xFF;
        cpu.reg.l = 0xFF;
        cpu.reg.a = 0x03;
        let mut mmu = Memory::new();

        let expected_cpu = Cpu {
            reg: Registers {
                h: 0x00,
                l: 0x00,
                ..cpu.reg
            },
            ..cpu
        };

        (&LD_HL_INC_A.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x22));
        assert_eq!(cpu, expected_cpu);
        assert_eq!(mmu.get_byte(0xFFFF as usize), 0x03);
    }

    #[test]
    pub fn test_ld_hl_inc_a_overflow_l() {
        let mut cpu = Cpu::new();
        cpu.reg.h = 0xC0;
        cpu.reg.l = 0xFF;
        cpu.reg.a = 0x03;
        let mut mmu = Memory::new();

        let expected_cpu = Cpu {
            reg: Registers {
                h: 0xC1,
                l: 0x00,
                ..cpu.reg
            },
            ..cpu
        };

        (&LD_HL_INC_A.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x22));
        assert_eq!(cpu, expected_cpu);
        assert_eq!(mmu.get_byte(0xC0FF as usize), 0x03);
    }

    #[test]
    pub fn test_get_instruction_inc_hl() {
        let instruction = get_instruction(&0x23);
        assert_eq!(instruction.unwrap(), &INC_HL);
        assert_eq!(instruction.unwrap().length, 1);
        assert_eq!(instruction.unwrap().clock_cycles, 8);
    }

    #[test]
    pub fn test_inc_hl() {
        let mut cpu = Cpu::new();
        cpu.reg.h = 0x00;
        cpu.reg.l = 0x00;

        let expected_cpu = Cpu {
            reg: Registers {
                h: 0x00,
                l: 0x01,
                ..cpu.reg
            },
            ..cpu
        };

        (&INC_HL.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x23));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_inc_hl_overflow() {
        let mut cpu = Cpu::new();
        cpu.reg.h = 0xFF;
        cpu.reg.l = 0xFF;

        let expected_cpu = Cpu {
            reg: Registers {
                h: 0x00,
                l: 0x00,
                ..cpu.reg
            },
            ..cpu
        };

        (&INC_HL.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x23));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_inc_hl_overflow_l() {
        let mut cpu = Cpu::new();
        cpu.reg.h = 0xC0;
        cpu.reg.l = 0xFF;

        let expected_cpu = Cpu {
            reg: Registers {
                h: 0xC1,
                l: 0x00,
                ..cpu.reg
            },
            ..cpu
        };

        (&INC_HL.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x23));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_get_instruction_inc_h() {
        let instruction = get_instruction(&0x24);
        assert_eq!(instruction.unwrap(), &INC_H);
        assert_eq!(instruction.unwrap().length, 1);
        assert_eq!(instruction.unwrap().clock_cycles, 4);
    }

    #[test]
    pub fn test_inc_h() {
        let mut cpu = Cpu::new();
        cpu.reg.h = 0x02;

        let expected_cpu = Cpu {
            reg: Registers { h: 0x03, ..cpu.reg },
            ..cpu
        };

        (&INC_H.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x24));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_inc_h_zero() {
        let mut cpu = Cpu::new();
        cpu.reg.h = 0xFF;

        let expected_cpu = Cpu {
            reg: Registers {
                h: 0x00,
                f: 0xA0, // Z_H_
                ..cpu.reg
            },
            ..cpu
        };

        (&INC_H.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x24));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_inc_h_half_carry() {
        let mut cpu = Cpu::new();
        cpu.reg.h = 0x0F;

        let expected_cpu = Cpu {
            reg: Registers {
                h: 0x10,
                f: 0x20, // __H_
                ..cpu.reg
            },
            ..cpu
        };

        (&INC_H.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x24));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_get_instruction_jr_z_n() {
        let instruction = get_instruction(&0x28);
        assert_eq!(instruction.unwrap(), &JR_Z_N);
        assert_eq!(instruction.unwrap().length, 2);
        assert_eq!(instruction.unwrap().clock_cycles, 8);
        assert_eq!(instruction.unwrap().clock_cycles_condition.unwrap(), 12);
    }

    #[test]
    pub fn test_jr_z_n_zero() {
        let mut cpu = Cpu::new();
        cpu.reg.f = 0x80;
        cpu.reg.pc = 0xC000;
        let mut mmu = Memory::new();
        mmu.set_byte(0xC001 as usize, 0x02);

        let expected_cpu = Cpu {
            reg: Registers {
                pc: 0xC004,
                ..cpu.reg
            },
            ..cpu
        };

        (&JR_Z_N.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x28));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_jr_z_n_zero_negative() {
        let mut cpu = Cpu::new();
        cpu.reg.f = 0x80;
        cpu.reg.pc = 0xC000;
        let mut mmu = Memory::new();
        mmu.set_byte(0xC001 as usize, 0xFE);

        let expected_cpu = Cpu {
            reg: Registers {
                pc: 0xC000,
                ..cpu.reg
            },
            ..cpu
        };

        (&JR_Z_N.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x28));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_jr_z_n_not_zero() {
        let mut cpu = Cpu::new();
        cpu.reg.pc = 0xC000;
        let mut mmu = Memory::new();
        mmu.set_byte(0xC001 as usize, 0x02);

        let expected_cpu = Cpu {
            reg: Registers {
                pc: 0xC000,
                ..cpu.reg
            },
            ..cpu
        };

        (&JR_Z_N.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x28));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_jr_z_n_not_zero_negative() {
        let mut cpu = Cpu::new();
        cpu.reg.pc = 0xC000;
        let mut mmu = Memory::new();
        mmu.set_byte(0xC001 as usize, 0xFE);

        let expected_cpu = Cpu {
            reg: Registers {
                pc: 0xC000,
                ..cpu.reg
            },
            ..cpu
        };

        (&JR_Z_N.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x28));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_get_instruction_ld_l_n() {
        let instruction = get_instruction(&0x2E);
        assert_eq!(instruction.unwrap(), &LD_L_N);
        assert_eq!(instruction.unwrap().length, 2);
        assert_eq!(instruction.unwrap().clock_cycles, 8);
    }

    #[test]
    pub fn test_ld_l_n() {
        let mut cpu = Cpu::new();
        cpu.reg.pc = 0xC000;
        let mut mmu = Memory::new();
        mmu.set_byte(0xC001 as usize, 0x02);

        let expected_cpu = Cpu {
            reg: Registers { l: 0x02, ..cpu.reg },
            ..cpu
        };

        (&LD_L_N.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x2E));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_get_instruction_ld_sp_nn() {
        let instruction = get_instruction(&0x31);
        assert_eq!(instruction.unwrap(), &LD_SP_NN);
        assert_eq!(instruction.unwrap().length, 3);
        assert_eq!(instruction.unwrap().clock_cycles, 12);
    }

    #[test]
    pub fn test_ld_sp_nn() {
        let mut cpu = Cpu::new();
        cpu.reg.pc = 0xC000;
        let mut mmu = Memory::new();
        mmu.set_word(0xC001 as usize, 0x0302);

        let expected_cpu = Cpu {
            reg: Registers {
                sp: 0x0302,
                ..cpu.reg
            },
            ..cpu
        };

        (&LD_SP_NN.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x31));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_get_instruction_ld_hl_dec_a() {
        let instruction = get_instruction(&0x32);
        assert_eq!(instruction.unwrap(), &LD_HL_DEC_A);
        assert_eq!(instruction.unwrap().length, 1);
        assert_eq!(instruction.unwrap().clock_cycles, 8);
    }

    #[test]
    pub fn test_ld_hl_dec_a() {
        let mut cpu = Cpu::new();
        cpu.reg.h = 0xC0;
        cpu.reg.l = 0x01;
        cpu.reg.a = 0x03;
        let mut mmu = Memory::new();

        let expected_cpu = Cpu {
            reg: Registers {
                h: 0xC0,
                l: 0x00,
                ..cpu.reg
            },
            ..cpu
        };

        (&LD_HL_DEC_A.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x32));
        assert_eq!(cpu, expected_cpu);
        assert_eq!(mmu.get_byte(0xC001 as usize), 0x03);
    }

    #[test]
    pub fn test_ld_hl_dec_a_underflow() {
        let mut cpu = Cpu::new();
        cpu.reg.h = 0x00;
        cpu.reg.l = 0x00;
        cpu.reg.a = 0x03;
        let mut mmu = Memory::new();

        let expected_cpu = Cpu {
            reg: Registers {
                h: 0xFF,
                l: 0xFF,
                ..cpu.reg
            },
            ..cpu
        };

        (&LD_HL_DEC_A.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x32));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_ld_hl_dec_a_underflow_l() {
        let mut cpu = Cpu::new();
        cpu.reg.h = 0xC0;
        cpu.reg.l = 0x00;
        cpu.reg.a = 0x03;
        let mut mmu = Memory::new();

        let expected_cpu = Cpu {
            reg: Registers {
                h: 0xBF,
                l: 0xFF,
                ..cpu.reg
            },
            ..cpu
        };

        (&LD_HL_DEC_A.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x32));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_get_instruction_dec_a() {
        let instruction = get_instruction(&0x3D);
        assert_eq!(instruction.unwrap(), &DEC_A);
        assert_eq!(instruction.unwrap().length, 1);
        assert_eq!(instruction.unwrap().clock_cycles, 4);
    }

    #[test]
    pub fn test_dec_a() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x02;

        let expected_cpu = Cpu {
            reg: Registers {
                a: 0x01,
                f: 0x40, // _N__
                ..cpu.reg
            },
            ..cpu
        };

        (&DEC_A.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x3D));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_dec_a_zero() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x01;

        let expected_cpu = Cpu {
            reg: Registers {
                a: 0x00,
                f: 0xC0, // ZN__
                ..cpu.reg
            },
            ..cpu
        };

        (&DEC_A.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x3D));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_dec_a_half_carry() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x10;

        let expected_cpu = Cpu {
            reg: Registers {
                a: 0x0F,
                f: 0x60, // _NH_
                ..cpu.reg
            },
            ..cpu
        };

        (&DEC_A.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x3D));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_dec_a_underflow() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x00;

        let expected_cpu = Cpu {
            reg: Registers {
                a: 0xFF,
                f: 0x60, // _NH_
                ..cpu.reg
            },
            ..cpu
        };

        (&DEC_A.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x3D));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_get_instruction_ld_a_n() {
        let instruction = get_instruction(&0x3E);
        assert_eq!(instruction.unwrap(), &LD_A_N);
        assert_eq!(instruction.unwrap().length, 2);
        assert_eq!(instruction.unwrap().clock_cycles, 8);
    }

    #[test]
    pub fn test_ld_a_n() {
        let mut cpu = Cpu::new();
        cpu.reg.pc = 0xC000;
        let mut mmu = Memory::new();
        mmu.set_byte(0xC001 as usize, 0x02);

        let expected_cpu = Cpu {
            reg: Registers { a: 0x02, ..cpu.reg },
            ..cpu
        };

        (&LD_A_N.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x3E));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_get_instruction_ld_c_a() {
        let instruction = get_instruction(&0x4F);
        assert_eq!(instruction.unwrap(), &LD_C_A);
        assert_eq!(instruction.unwrap().length, 1);
        assert_eq!(instruction.unwrap().clock_cycles, 4);
    }

    #[test]
    pub fn test_ld_c_a() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x02;

        let expected_cpu = Cpu {
            reg: Registers { c: 0x02, ..cpu.reg },
            ..cpu
        };

        (&LD_C_A.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x4F));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_get_instruction_ld_d_a() {
        let instruction = get_instruction(&0x57);
        assert_eq!(instruction.unwrap(), &LD_D_A);
        assert_eq!(instruction.unwrap().length, 1);
        assert_eq!(instruction.unwrap().clock_cycles, 4);
    }

    #[test]
    pub fn test_ld_d_a() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x02;

        let expected_cpu = Cpu {
            reg: Registers { d: 0x02, ..cpu.reg },
            ..cpu
        };

        (&LD_D_A.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x57));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_get_instruction_ld_h_a() {
        let instruction = get_instruction(&0x67);
        assert_eq!(instruction.unwrap(), &LD_H_A);
        assert_eq!(instruction.unwrap().length, 1);
        assert_eq!(instruction.unwrap().clock_cycles, 4);
    }

    #[test]
    pub fn test_ld_h_a() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x02;

        let expected_cpu = Cpu {
            reg: Registers { h: 0x02, ..cpu.reg },
            ..cpu
        };

        (&LD_H_A.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x67));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_get_instruction_ld_hl_a() {
        let instruction = get_instruction(&0x77);
        assert_eq!(instruction.unwrap(), &LD_HL_A);
        assert_eq!(instruction.unwrap().length, 1);
        assert_eq!(instruction.unwrap().clock_cycles, 8);
    }

    #[test]
    pub fn test_ld_hl_a() {
        let mut cpu = Cpu::new();
        cpu.reg.h = 0xC0;
        cpu.reg.l = 0x00;
        cpu.reg.a = 0x03;
        let mut mmu = Memory::new();

        let expected_cpu = Cpu {
            reg: Registers {
                h: 0xC0,
                l: 0x00,
                ..cpu.reg
            },
            ..cpu
        };

        (&LD_HL_A.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x77));
        assert_eq!(cpu, expected_cpu);
        assert_eq!(mmu.get_byte(0xC000 as usize), 0x03);
    }

    #[test]
    pub fn test_get_instruction_ld_a_b() {
        let instruction = get_instruction(&0x78);
        assert_eq!(instruction.unwrap(), &LD_A_B);
        assert_eq!(instruction.unwrap().length, 1);
        assert_eq!(instruction.unwrap().clock_cycles, 4);
    }

    #[test]
    pub fn test_ld_a_b() {
        let mut cpu = Cpu::new();
        cpu.reg.b = 0x02;

        let expected_cpu = Cpu {
            reg: Registers { a: 0x02, ..cpu.reg },
            ..cpu
        };

        (&LD_A_B.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x78));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_get_instruction_ld_a_e() {
        let instruction = get_instruction(&0x7B);
        assert_eq!(instruction.unwrap(), &LD_A_E);
        assert_eq!(instruction.unwrap().length, 1);
        assert_eq!(instruction.unwrap().clock_cycles, 4);
    }

    #[test]
    pub fn test_ld_a_e() {
        let mut cpu = Cpu::new();
        cpu.reg.e = 0x02;

        let expected_cpu = Cpu {
            reg: Registers { a: 0x02, ..cpu.reg },
            ..cpu
        };

        (&LD_A_E.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x7B));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_get_instruction_ld_a_h() {
        let instruction = get_instruction(&0x7C);
        assert_eq!(instruction.unwrap(), &LD_A_H);
        assert_eq!(instruction.unwrap().length, 1);
        assert_eq!(instruction.unwrap().clock_cycles, 4);
    }

    #[test]
    pub fn test_ld_a_h() {
        let mut cpu = Cpu::new();
        cpu.reg.h = 0x02;

        let expected_cpu = Cpu {
            reg: Registers { a: 0x02, ..cpu.reg },
            ..cpu
        };

        (&LD_A_H.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x7C));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_get_instruction_ld_a_l() {
        let instruction = get_instruction(&0x7D);
        assert_eq!(instruction.unwrap(), &LD_A_L);
        assert_eq!(instruction.unwrap().length, 1);
        assert_eq!(instruction.unwrap().clock_cycles, 4);
    }

    #[test]
    pub fn test_get_instruction_add_hl() {
        let instruction = get_instruction(&0x86);
        assert_eq!(instruction.unwrap(), &ADD_HL);
        assert_eq!(instruction.unwrap().length, 1);
        assert_eq!(instruction.unwrap().clock_cycles, 8);
    }

    #[test]
    pub fn test_add_hl() {
        let mut cpu = Cpu::new();
        let mut mmu: Memory = Memory::new();
        cpu.reg.a = 0x01;
        cpu.reg.h = 0xC0;
        cpu.reg.l = 0x00;
        mmu.set_byte(0xC000 as usize, 0x02);

        let expected_cpu = Cpu {
            reg: Registers { a: 0x03, ..cpu.reg },
            ..cpu
        };

        (&ADD_HL.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x86));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_add_hl_overflow() {
        let mut cpu = Cpu::new();
        let mut mmu: Memory = Memory::new();
        cpu.reg.a = 0xFF;
        cpu.reg.h = 0xC0;
        cpu.reg.l = 0x00;
        mmu.set_byte(0xC000 as usize, 0x02);

        let expected_cpu = Cpu {
            reg: Registers {
                a: 0x01,
                f: 0x30, // __HC
                ..cpu.reg
            },
            ..cpu
        };

        (&ADD_HL.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x86));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_add_hl_half_carry() {
        let mut cpu = Cpu::new();
        let mut mmu: Memory = Memory::new();
        cpu.reg.a = 0x0F;
        cpu.reg.h = 0xC0;
        cpu.reg.l = 0x00;
        mmu.set_byte(0xC000 as usize, 0x02);

        let expected_cpu = Cpu {
            reg: Registers {
                a: 0x11,
                f: 0x20, // __H_
                ..cpu.reg
            },
            ..cpu
        };

        (&ADD_HL.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x86));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_add_hl_zero() {
        let mut cpu = Cpu::new();
        let mut mmu: Memory = Memory::new();
        cpu.reg.a = 0x00;
        cpu.reg.h = 0xC0;
        cpu.reg.l = 0x00;
        mmu.set_byte(0xC000 as usize, 0x00);

        let expected_cpu = Cpu {
            reg: Registers {
                a: 0x00,
                f: 0x80, // Z___
                ..cpu.reg
            },
            ..cpu
        };

        (&ADD_HL.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x86));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_get_instruction_sub_b() {
        let instruction = get_instruction(&0x90);
        assert_eq!(instruction.unwrap(), &SUB_B);
        assert_eq!(instruction.unwrap().length, 1);
        assert_eq!(instruction.unwrap().clock_cycles, 4);
    }

    #[test]
    pub fn test_sub_b() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x02;
        cpu.reg.b = 0x01;

        let expected_cpu = Cpu {
            reg: Registers {
                a: 0x01,
                f: 0x40, // _N__
                ..cpu.reg
            },
            ..cpu
        };

        (&SUB_B.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x90));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_sub_b_zero() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x01;
        cpu.reg.b = 0x01;

        let expected_cpu = Cpu {
            reg: Registers {
                a: 0x00,
                f: 0xC0, // ZN__
                ..cpu.reg
            },
            ..cpu
        };

        (&SUB_B.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x90));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_sub_b_half_carry() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x10;
        cpu.reg.b = 0x01;

        let expected_cpu = Cpu {
            reg: Registers {
                a: 0x0F,
                f: 0x60, // _NH_
                ..cpu.reg
            },
            ..cpu
        };

        (&SUB_B.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x90));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_sub_b_underflow() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x00;
        cpu.reg.b = 0x01;

        let expected_cpu = Cpu {
            reg: Registers {
                a: 0xFF,
                f: 0x70, // ZNH_
                ..cpu.reg
            },
            ..cpu
        };

        (&SUB_B.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x90));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_get_instruction_xor_a() {
        let instruction = get_instruction(&0xAF);
        assert_eq!(instruction.unwrap(), &XOR_A);
        assert_eq!(instruction.unwrap().length, 1);
        assert_eq!(instruction.unwrap().clock_cycles, 4);
    }

    #[test]
    pub fn xor_a() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x01;

        let expected_cpu = Cpu {
            reg: Registers {
                a: 0x00,
                f: 0x80, // Z___
                ..cpu.reg
            },
            ..cpu
        };

        (&XOR_A.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0xAF));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_get_instruction_cp_hl() {
        let instruction = get_instruction(&0xBE);
        assert_eq!(instruction.unwrap(), &CP_HL);
        assert_eq!(instruction.unwrap().length, 1);
        assert_eq!(instruction.unwrap().clock_cycles, 8);
    }

    #[test]
    pub fn test_cp_hl_equal() {
        let mut cpu = Cpu::new();
        let mut mmu: Memory = Memory::new();
        cpu.reg.a = 0x01;
        cpu.reg.h = 0xC0;
        cpu.reg.l = 0x00;
        mmu.set_byte(0xC000 as usize, 0x01);

        let expected_cpu = Cpu {
            reg: Registers {
                a: 0x01,
                f: 0xC0, // ZN__
                ..cpu.reg
            },
            ..cpu
        };

        (&CP_HL.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xBE));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_cp_hl_greater_than() {
        let mut cpu = Cpu::new();
        let mut mmu: Memory = Memory::new();
        cpu.reg.a = 0x0F;
        cpu.reg.h = 0xC0;
        cpu.reg.l = 0x00;
        mmu.set_byte(0xC000 as usize, 0x10);

        let expected_cpu = Cpu {
            reg: Registers {
                a: 0x0F,
                f: 0x50, // _N_C
                ..cpu.reg
            },
            ..cpu
        };

        (&CP_HL.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xBE));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_cp_hl_less_than() {
        let mut cpu = Cpu::new();
        let mut mmu: Memory = Memory::new();
        cpu.reg.a = 0x01;
        cpu.reg.h = 0xC0;
        cpu.reg.l = 0x00;
        mmu.set_byte(0xC000 as usize, 0x00);

        let expected_cpu = Cpu {
            reg: Registers {
                a: 0x01,
                f: 0x40, // _N__
                ..cpu.reg
            },
            ..cpu
        };

        (&CP_HL.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xBE));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_cp_hl_half_carry() {
        let mut cpu = Cpu::new();
        let mut mmu: Memory = Memory::new();
        cpu.reg.a = 0x20;
        cpu.reg.h = 0xC0;
        cpu.reg.l = 0x00;
        mmu.set_byte(0xC000 as usize, 0x1F);

        let expected_cpu = Cpu {
            reg: Registers {
                a: 0x20,
                f: 0x60, // _NH
                ..cpu.reg
            },
            ..cpu
        };

        (&CP_HL.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xBE));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_get_instruction_pop_bc() {
        let instruction = get_instruction(&0xC1);
        assert_eq!(instruction.unwrap(), &POP_BC);
        assert_eq!(instruction.unwrap().length, 1);
        assert_eq!(instruction.unwrap().clock_cycles, 12);
    }

    #[test]
    pub fn test_pop_bc() {
        let mut cpu = Cpu::new();
        let mut mmu: Memory = Memory::new();
        cpu.reg.sp = 0xC000;
        mmu.set_word(0xC000 as usize, 0x0302);

        let expected_cpu = Cpu {
            reg: Registers {
                b: 0x03,
                c: 0x02,
                sp: 0xC002,
                ..cpu.reg
            },
            ..cpu
        };

        (&POP_BC.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xC1));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_get_instruction_push_bc() {
        let instruction = get_instruction(&0xC5);
        assert_eq!(instruction.unwrap(), &PUSH_BC);
        assert_eq!(instruction.unwrap().length, 1);
        assert_eq!(instruction.unwrap().clock_cycles, 16);
    }

    #[test]
    pub fn test_push_bc() {
        let mut cpu = Cpu::new();
        let mut mmu: Memory = Memory::new();
        cpu.reg.b = 0x03;
        cpu.reg.c = 0x02;
        cpu.reg.sp = 0xFFFE;

        let expected_cpu = Cpu {
            reg: Registers {
                sp: 0xFFFC,
                ..cpu.reg
            },
            ..cpu
        };

        (&PUSH_BC.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xC5));
        assert_eq!(cpu, expected_cpu);
        assert_eq!(mmu.get_word(0xFFFC as usize), 0x0302);
    }

    #[test]
    pub fn test_get_instruction_ret() {
        let instruction = get_instruction(&0xC9);
        assert_eq!(instruction.unwrap(), &RET);
        assert_eq!(instruction.unwrap().length, 1);
        assert_eq!(instruction.unwrap().clock_cycles, 16);
    }

    #[test]
    pub fn test_ret() {
        let mut cpu = Cpu::new();
        let mut mmu: Memory = Memory::new();
        cpu.reg.sp = 0xFFFC;
        mmu.set_word(0xFFFC as usize, 0x0302);

        let expected_cpu = Cpu {
            reg: Registers {
                pc: 0x0302,
                sp: 0xFFFE,
                ..cpu.reg
            },
            ..cpu
        };

        (&RET.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xC9));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_get_instruction_call_nn() {
        let instruction = get_instruction(&0xCD);
        assert_eq!(instruction.unwrap(), &CALL_NN);
        assert_eq!(instruction.unwrap().length, 3);
        assert_eq!(instruction.unwrap().clock_cycles, 24);
    }

    #[test]
    pub fn test_call_nn() {
        let mut cpu = Cpu::new();
        let mut mmu: Memory = Memory::new();
        cpu.reg.pc = 0xC000;
        cpu.reg.sp = 0xFFFE;
        mmu.set_word(0xC001 as usize, 0x0302);

        let expected_cpu = Cpu {
            reg: Registers {
                pc: 0x0302,
                sp: 0xFFFC,
                ..cpu.reg
            },
            ..cpu
        };

        (&CALL_NN.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xCD));
        assert_eq!(cpu, expected_cpu);
        assert_eq!(mmu.get_word(0xFFFC as usize), 0xC003);
    }

    #[test]
    pub fn test_get_instruction_ldh_n_a() {
        let instruction = get_instruction(&0xE0);
        assert_eq!(instruction.unwrap(), &LDH_N_A);
        assert_eq!(instruction.unwrap().length, 2);
        assert_eq!(instruction.unwrap().clock_cycles, 12);
    }

    #[test]
    pub fn test_ldh_n_a() {
        let mut cpu = Cpu::new();
        let mut mmu: Memory = Memory::new();
        cpu.reg.a = 0x01;
        cpu.reg.pc = 0xC000;
        mmu.set_byte(0xC001 as usize, 0x01);

        let expected_cpu = Cpu {
            reg: Registers { ..cpu.reg },
            ..cpu
        };

        (&LDH_N_A.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xE0));
        assert_eq!(cpu, expected_cpu);
        assert_eq!(mmu.get_byte(0xFF01 as usize), 0x01);
    }

    #[test]
    pub fn test_get_instruction_ldh_c_a() {
        let instruction = get_instruction(&0xE2);
        assert_eq!(instruction.unwrap(), &LDH_C_A);
        assert_eq!(instruction.unwrap().length, 1);
        assert_eq!(instruction.unwrap().clock_cycles, 8);
    }

    #[test]
    pub fn test_ldh_c_a() {
        let mut cpu = Cpu::new();
        let mut mmu: Memory = Memory::new();
        cpu.reg.a = 0x01;
        cpu.reg.c = 0x01;

        let expected_cpu = Cpu {
            reg: Registers { ..cpu.reg },
            ..cpu
        };

        (&LDH_C_A.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xE2));
        assert_eq!(cpu, expected_cpu);
        assert_eq!(mmu.get_byte(0xFF01 as usize), 0x01);
    }

    #[test]
    pub fn test_get_instruction_ld_nn_a() {
        let instruction = get_instruction(&0xEA);
        assert_eq!(instruction.unwrap(), &LD_NN_A);
        assert_eq!(instruction.unwrap().length, 3);
        assert_eq!(instruction.unwrap().clock_cycles, 16);
    }

    #[test]
    pub fn test_ld_nn_a() {
        let mut cpu = Cpu::new();
        let mut mmu: Memory = Memory::new();
        cpu.reg.a = 0x01;
        cpu.reg.pc = 0xC000;
        mmu.set_word(0xC001 as usize, 0xC003);

        let expected_cpu = Cpu {
            reg: Registers { ..cpu.reg },
            ..cpu
        };

        (&LD_NN_A.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xEA));
        assert_eq!(cpu, expected_cpu);
        assert_eq!(mmu.get_byte(0xC003 as usize), 0x01);
    }

    #[test]
    pub fn test_get_instruction_ldh_a_n() {
        let instruction = get_instruction(&0xF0);
        assert_eq!(instruction.unwrap(), &LDH_A_N);
        assert_eq!(instruction.unwrap().length, 2);
        assert_eq!(instruction.unwrap().clock_cycles, 12);
    }

    #[test]
    pub fn test_ldh_a_n() {
        let mut cpu = Cpu::new();
        let mut mmu: Memory = Memory::new();
        cpu.reg.pc = 0xC000;
        mmu.set_byte(0xC001 as usize, 0x01);
        mmu.set_byte(0xFF01 as usize, 0x01);

        let expected_cpu = Cpu {
            reg: Registers { a: 0x01, ..cpu.reg },
            ..cpu
        };

        (&LDH_A_N.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xF0));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_get_instruction_cp_a_n() {
        let instruction = get_instruction(&0xFE);
        assert_eq!(instruction.unwrap(), &CP_A_N);
        assert_eq!(instruction.unwrap().length, 2);
        assert_eq!(instruction.unwrap().clock_cycles, 8);
    }

    #[test]
    pub fn test_cp_a_n_equal() {
        let mut cpu = Cpu::new();
        let mut mmu: Memory = Memory::new();
        cpu.reg.a = 0x01;
        cpu.reg.pc = 0xC000;
        mmu.set_byte(0xC001 as usize, 0x01);

        let expected_cpu = Cpu {
            reg: Registers {
                a: 0x01,
                f: 0xC0, // ZN__
                ..cpu.reg
            },
            ..cpu
        };

        (&CP_A_N.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xFE));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_cp_a_n_greater_than() {
        let mut cpu = Cpu::new();
        let mut mmu: Memory = Memory::new();
        cpu.reg.a = 0x0F;
        cpu.reg.pc = 0xC000;
        mmu.set_byte(0xC001 as usize, 0x10);

        let expected_cpu = Cpu {
            reg: Registers {
                a: 0x0F,
                f: 0x50, // _N_C
                ..cpu.reg
            },
            ..cpu
        };

        (&CP_A_N.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xFE));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_cp_a_n_less_than() {
        let mut cpu = Cpu::new();
        let mut mmu: Memory = Memory::new();
        cpu.reg.a = 0x01;
        cpu.reg.pc = 0xC000;
        mmu.set_byte(0xC001 as usize, 0x00);

        let expected_cpu = Cpu {
            reg: Registers {
                a: 0x01,
                f: 0x40, // _N__
                ..cpu.reg
            },
            ..cpu
        };

        (&CP_A_N.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xFE));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_cp_a_n_half_carry() {
        let mut cpu = Cpu::new();
        let mut mmu: Memory = Memory::new();
        cpu.reg.a = 0x20;
        cpu.reg.pc = 0xC000;
        mmu.set_byte(0xC001 as usize, 0x1F);

        let expected_cpu = Cpu {
            reg: Registers {
                a: 0x20,
                f: 0x60, // _NH_
                ..cpu.reg
            },
            ..cpu
        };

        (&CP_A_N.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xFE));
        assert_eq!(cpu, expected_cpu);
    }
}
