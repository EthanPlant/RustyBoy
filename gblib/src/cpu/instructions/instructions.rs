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
    handler: |_: &mut Cpu, _: &mut Memory, _: &OpCode| {
        InstructionType::None
    }
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
    }
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
    }
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
    }
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
    }
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
    }
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
    }
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
    }
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
    }
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
    }
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
    }
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
    }
};

/// 0x18 - JR n
const JR_N: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "JR n",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        functions::jr(cpu, mmu);
        InstructionType::Jumped
    }
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
    }
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
    }
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
    }
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
            return InstructionType::Jumped
        }
        InstructionType::None
    }
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
    }
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
    }
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
    }
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
    }
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
            return InstructionType::Jumped
        }
        InstructionType::None
    }
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
    }
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
    }
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
    }
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
    }
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
    }
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
    }
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
    }
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
    }
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
    }
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
    }
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
    }
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
    }
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
    }
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
    }
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
    }
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
    }
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
    }
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
    }
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
    }
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
    }
};

/// 0xCD - CALL nn
const CALL_NN: Instruction = Instruction {
    length: 3,
    clock_cycles: 12,
    clock_cycles_condition: None,
    description: "CALL nn",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        functions::call(cpu, mmu);
        InstructionType::Jumped
    }
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
    }
};

/// 0xE2 - LD (C), A
const LD_PTR_C_A: Instruction = Instruction {
    length: 1,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "LD (C) A",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        mmu.set_byte(0xFF00 + cpu.reg.c as u16, cpu.reg.a);
        InstructionType::ActionTaken
    }
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
    }
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
    }
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
    }
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
        0xE2 => Some(&LD_PTR_C_A),
        0xEA => Some(&LD_NN_A),

        0xF0 => Some(&LDH_A_N),
        0xFE => Some(&CP_A_N),

        _ => None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_instruction_nop() {
        let instruction = get_instruction(&0x00);
        assert!(instruction == Some(&NOP));
        let mut cpu = Cpu::new();
        (instruction.unwrap().handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x00));
        assert_eq!(cpu.reg.pc, 0x0000);
    }

    #[test]
    fn test_get_instruction_inc_b() {
        let instruction = get_instruction(&0x04);
        assert!(instruction == Some(&INC_B));
        let mut cpu = Cpu::new();
        cpu.reg.b = 0x00;
        (instruction.unwrap().handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x04));
        assert_eq!(cpu.reg.b, 0x01);
    }

    #[test]
    fn test_get_instruction_inc_b_zero() {
        let instruction = get_instruction(&0x04);
        assert!(instruction == Some(&INC_B));
        let mut cpu = Cpu::new();
        cpu.reg.b = 0xFF;
        (instruction.unwrap().handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x04));
        assert_eq!(cpu.reg.b, 0x00);
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    fn test_get_instruction_inc_b_half_carry() {
        let instruction = get_instruction(&0x04);
        assert!(instruction == Some(&INC_B));
        let mut cpu = Cpu::new();
        cpu.reg.b = 0x0F;
        (instruction.unwrap().handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x04));
        assert_eq!(cpu.reg.b, 0x10);
        assert!(cpu.reg.check_flag(Flag::HalfCarry));
    }

    #[test]
    fn test_get_instruction_dec_b() {
        let instruction = get_instruction(&0x05);
        assert!(instruction == Some(&DEC_B));
        let mut cpu = Cpu::new();
        cpu.reg.b = 0x02;
        (instruction.unwrap().handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x05));
        assert_eq!(cpu.reg.b, 0x01);
    }

    #[test]
    fn test_get_instruction_dec_b_zero() {
        let instruction = get_instruction(&0x05);
        assert!(instruction == Some(&DEC_B));
        let mut cpu = Cpu::new();
        cpu.reg.b = 0x01;
        (instruction.unwrap().handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x05));
        assert_eq!(cpu.reg.b, 0x00);
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    fn test_get_instruction_dec_b_half_carry() {
        let instruction = get_instruction(&0x05);
        assert!(instruction == Some(&DEC_B));
        let mut cpu = Cpu::new();
        cpu.reg.b = 0x10;
        (instruction.unwrap().handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x05));
        assert_eq!(cpu.reg.b, 0x0F);
        assert!(cpu.reg.check_flag(Flag::HalfCarry));
    }

    #[test]
    fn test_get_instruction_dec_b_overflow() {
        let instruction = get_instruction(&0x05);
        assert!(instruction == Some(&DEC_B));
        let mut cpu = Cpu::new();
        cpu.reg.b = 0x00;
        (instruction.unwrap().handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x05));
        assert_eq!(cpu.reg.b, 0xFF);
    }

    #[test]
    fn test_get_instruction_ld_b_n() {
        let instruction = get_instruction(&0x06);
        assert!(instruction == Some(&LD_B_N));
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        mmu.set_byte(0x01 as u16, 0x12);
        (instruction.unwrap().handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x06));
        assert_eq!(cpu.reg.b, 0x12);
    }

    #[test]
    fn test_get_instruction_inc_c() {
        let instruction = get_instruction(&0x0C);
        assert!(instruction == Some(&INC_C));
        let mut cpu = Cpu::new();
        cpu.reg.c = 0x00;
        (instruction.unwrap().handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x0C));
        assert_eq!(cpu.reg.c, 0x01);
    }

    #[test]
    fn test_get_instruction_inc_c_zero() {
        let instruction = get_instruction(&0x0C);
        assert!(instruction == Some(&INC_C));
        let mut cpu = Cpu::new();
        cpu.reg.c = 0xFF;
        (instruction.unwrap().handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x0C));
        assert_eq!(cpu.reg.c, 0x00);
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    fn test_get_instruction_inc_c_half_carry() {
        let instruction = get_instruction(&0x0C);
        assert!(instruction == Some(&INC_C));
        let mut cpu = Cpu::new();
        cpu.reg.c = 0x0F;
        (instruction.unwrap().handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x0C));
        assert_eq!(cpu.reg.c, 0x10);
        assert!(cpu.reg.check_flag(Flag::HalfCarry));
    }

    #[test]
    fn test_get_instruction_dec_c() {
        let instruction = get_instruction(&0x0D);
        assert!(instruction == Some(&DEC_C));
        let mut cpu = Cpu::new();
        cpu.reg.c = 0x02;
        (instruction.unwrap().handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x0D));
        assert_eq!(cpu.reg.c, 0x01);
    }

    #[test]
    fn test_get_instruction_dec_c_zero() {
        let instruction = get_instruction(&0x0D);
        assert!(instruction == Some(&DEC_C));
        let mut cpu = Cpu::new();
        cpu.reg.c = 0x01;
        (instruction.unwrap().handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x0D));
        assert_eq!(cpu.reg.c, 0x00);
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    fn test_get_instruction_dec_c_half_carry() {
        let instruction = get_instruction(&0x0D);
        assert!(instruction == Some(&DEC_C));
        let mut cpu = Cpu::new();
        cpu.reg.c = 0x10;
        (instruction.unwrap().handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x0D));
        assert_eq!(cpu.reg.c, 0x0F);
        assert!(cpu.reg.check_flag(Flag::HalfCarry));
    }

    #[test]
    fn test_get_instruction_dec_c_overflow() {
        let instruction = get_instruction(&0x0D);
        assert!(instruction == Some(&DEC_C));
        let mut cpu = Cpu::new();
        cpu.reg.c = 0x00;
        (instruction.unwrap().handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x0D));
        assert_eq!(cpu.reg.c, 0xFF);
    }

    #[test]
    fn test_get_instruction_ld_c_n() {
        let instruction = get_instruction(&0x0E);
        assert!(instruction == Some(&LD_C_N));
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        mmu.set_byte(0x01 as u16, 0x12);
        (instruction.unwrap().handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x0E));
        assert_eq!(cpu.reg.c, 0x12);
    }

    #[test]
    fn test_get_instruction_ld_de_nn() {
        let instruction = get_instruction(&0x11);
        assert!(instruction == Some(&LD_DE_NN));
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        mmu.set_word(0x01 as u8, 0x1234);
        (instruction.unwrap().handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x11));
        assert_eq!(cpu.reg.d, 0x12);
        assert_eq!(cpu.reg.e, 0x34);
    }

    #[test]
    fn test_get_instruction_dec_d() {
        let instruction = get_instruction(&0x15);
        assert!(instruction == Some(&DEC_D));
        let mut cpu = Cpu::new();
        cpu.reg.d = 0x02;
        (instruction.unwrap().handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x15));
        assert_eq!(cpu.reg.d, 0x01);
    }

    #[test]
    fn test_get_instruction_dec_d_zero() {
        let instruction = get_instruction(&0x15);
        assert!(instruction == Some(&DEC_D));
        let mut cpu = Cpu::new();
        cpu.reg.d = 0x01;
        (instruction.unwrap().handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x15));
        assert_eq!(cpu.reg.d, 0x00);
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    fn test_get_instruction_dec_d_half_carry() {
        let instruction = get_instruction(&0x15);
        assert!(instruction == Some(&DEC_D));
        let mut cpu = Cpu::new();
        cpu.reg.d = 0x10;
        (instruction.unwrap().handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x15));
        assert_eq!(cpu.reg.d, 0x0F);
        assert!(cpu.reg.check_flag(Flag::HalfCarry));
    }

    #[test]
    fn test_get_instruction_dec_d_overflow() {
        let instruction = get_instruction(&0x15);
        assert!(instruction == Some(&DEC_D));
        let mut cpu = Cpu::new();
        cpu.reg.d = 0x00;
        (instruction.unwrap().handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x15));
        assert_eq!(cpu.reg.d, 0xFF);
    }

    #[test]
    fn test_get_instruction_inc_de() {
        let instruction = get_instruction(&0x13);
        assert!(instruction == Some(&INC_DE));
        let mut cpu = Cpu::new();
        cpu.reg.d = 0x12;
        cpu.reg.e = 0x34;
        (instruction.unwrap().handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x13));
        assert_eq!(cpu.reg.d, 0x12);
        assert_eq!(cpu.reg.e, 0x35);
    }

    #[test]
    fn test_get_instruction_inc_de_overflow() {
        let instruction = get_instruction(&0x13);
        assert!(instruction == Some(&INC_DE));
        let mut cpu = Cpu::new();
        cpu.reg.d = 0xFF;
        cpu.reg.e = 0xFF;
        (instruction.unwrap().handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x13));
        assert_eq!(cpu.reg.d, 0x00);
        assert_eq!(cpu.reg.e, 0x00);
    }

    #[test]
    fn test_get_instruction_ld_d_n() {
        let instruction = get_instruction(&0x16);
        assert!(instruction == Some(&LD_D_N));
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        mmu.set_byte(0x01 as u16, 0x12);
        (instruction.unwrap().handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x16));
        assert_eq!(cpu.reg.d, 0x12);
    }

    #[test]
    fn test_get_instruction_rla() {
        let instruction = get_instruction(&0x17);
        assert!(instruction == Some(&RLA));
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x80;
        (instruction.unwrap().handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x17));
        assert_eq!(cpu.reg.a, 0x01);
        assert!(cpu.reg.check_flag(Flag::Carry));
    }

    #[test]
    fn test_get_instruction_rla_zero() {
        let instruction = get_instruction(&0x17);
        assert!(instruction == Some(&RLA));
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x00;
        (instruction.unwrap().handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x17));
        assert_eq!(cpu.reg.a, 0x00);
        assert!(!cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    fn test_get_instruction_jr_n() {
        let instruction = get_instruction(&0x18);
        assert!(instruction == Some(&JR_N));
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.pc = 0x000A;
        mmu.set_byte(0x000B as u16, 0xFB);
        (instruction.unwrap().handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x18));
        assert_eq!(cpu.reg.pc, 0x0007);
    }

    #[test]
    fn test_get_instruction_ld_a_de() {
        let instruction = get_instruction(&0x1A);
        assert!(instruction == Some(&LD_A_DE));
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.d = 0x12;
        cpu.reg.e = 0x34;
        mmu.set_byte(0x1234 as u16, 0x12);
        (instruction.unwrap().handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x1A));
        assert_eq!(cpu.reg.a, 0x12);
    }

    #[test]
    fn test_get_instruction_dec_e() {
        let instruction = get_instruction(&0x1D);
        assert!(instruction == Some(&DEC_E));
        let mut cpu = Cpu::new();
        cpu.reg.e = 0x02;
        (instruction.unwrap().handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x1D));
        assert_eq!(cpu.reg.e, 0x01);
    }

    #[test]
    fn test_get_instruction_dec_e_zero() {
        let instruction = get_instruction(&0x1D);
        assert!(instruction == Some(&DEC_E));
        let mut cpu = Cpu::new();
        cpu.reg.e = 0x01;
        (instruction.unwrap().handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x1D));
        assert_eq!(cpu.reg.e, 0x00);
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    fn test_get_instruction_dec_e_half_carry() {
        let instruction = get_instruction(&0x1D);
        assert!(instruction == Some(&DEC_E));
        let mut cpu = Cpu::new();
        cpu.reg.e = 0x10;
        (instruction.unwrap().handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x1D));
        assert_eq!(cpu.reg.e, 0x0F);
        assert!(cpu.reg.check_flag(Flag::HalfCarry));
    }

    #[test]
    fn test_get_instruction_dec_e_overflow() {
        let instruction = get_instruction(&0x1D);
        assert!(instruction == Some(&DEC_E));
        let mut cpu = Cpu::new();
        cpu.reg.e = 0x00;
        (instruction.unwrap().handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x1D));
        assert_eq!(cpu.reg.e, 0xFF);
    }

    #[test]
    fn test_get_instruction_ld_e_n() {
        let instruction = get_instruction(&0x1E);
        assert!(instruction == Some(&LD_E_N));
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        mmu.set_byte(0x01 as u16, 0x12);
        (instruction.unwrap().handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x1E));
        assert_eq!(cpu.reg.e, 0x12);
    }

    #[test]
    fn test_get_instruction_jr_nz_n_zero_flag_set() {
        let instruction = get_instruction(&0x20);
        assert!(instruction == Some(&JR_NZ_N));
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.pc = 0x000A;
        mmu.set_byte(0x000B as u16, 0xFB);
        cpu.reg.set_flag(Flag::Zero);
        let t = (instruction.unwrap().handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x20));
        assert_eq!(cpu.reg.pc, 0x000A);
        assert!(t == InstructionType::None)
    }

    #[test]
    fn test_get_instruction_jr_nz_n_zero_flag_not_set() {
        let instruction = get_instruction(&0x20);
        assert!(instruction == Some(&JR_NZ_N));
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.pc = 0x000A;
        mmu.set_byte(0x000B as u16, 0xFB);
        let t= (instruction.unwrap().handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x20));
        assert_eq!(cpu.reg.pc, 0x0007);
        assert!(t == InstructionType::Jumped)
    }

    #[test]
    fn test_get_instruction_ld_hl_nn() {
        let instruction = get_instruction(&0x21);
        assert!(instruction == Some(&LD_HL_NN));
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        mmu.set_word(0x01 as u8, 0x1234);
        (instruction.unwrap().handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x21));
        assert_eq!(cpu.reg.h, 0x12);
        assert_eq!(cpu.reg.l, 0x34);
    }

    #[test]
    fn test_get_instruction_ld_hl_inc_a() {
        let instruction = get_instruction(&0x22);
        assert!(instruction == Some(&LD_HL_INC_A));
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.a = 0x12;
        cpu.reg.h = 0x12;
        cpu.reg.l = 0x34;
        (instruction.unwrap().handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x22));
        assert_eq!(mmu.get_byte(0x1234 as u16), 0x12);
        assert_eq!(cpu.reg.h, 0x12);
        assert_eq!(cpu.reg.l, 0x35);
    }

    #[test]
    fn test_get_instruction_ld_hl_inc_a_overflow() {
        let instruction = get_instruction(&0x22);
        assert!(instruction == Some(&LD_HL_INC_A));
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.a = 0x12;
        cpu.reg.h = 0xFF;
        cpu.reg.l = 0xFF;
        (instruction.unwrap().handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x22));
        assert_eq!(mmu.get_byte(0xFFFF as u16), 0x12);
        assert_eq!(cpu.reg.h, 0x00);
        assert_eq!(cpu.reg.l, 0x00);
    }

    #[test]
    fn test_get_instruction_inc_hl() {
        let instruction = get_instruction(&0x23);
        assert!(instruction == Some(&INC_HL));
        let mut cpu = Cpu::new();
        cpu.reg.h = 0x12;
        cpu.reg.l = 0x34;
        (instruction.unwrap().handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x23));
        assert_eq!(cpu.reg.h, 0x12);
        assert_eq!(cpu.reg.l, 0x35);
    }

    #[test]
    fn test_get_instruction_inc_hl_overflow() {
        let instruction = get_instruction(&0x23);
        assert!(instruction == Some(&INC_HL));
        let mut cpu = Cpu::new();
        cpu.reg.h = 0xFF;
        cpu.reg.l = 0xFF;
        (instruction.unwrap().handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x23));
        assert_eq!(cpu.reg.h, 0x00);
        assert_eq!(cpu.reg.l, 0x00);
    }
    
    #[test]
    fn test_get_instruction_inc_h() {
        let instruction = get_instruction(&0x24);
        assert!(instruction == Some(&INC_H));
        let mut cpu = Cpu::new();
        cpu.reg.h = 0x00;
        (instruction.unwrap().handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x24));
        assert_eq!(cpu.reg.h, 0x01);
    }

    #[test]
    fn test_get_instruction_inc_h_zero() {
        let instruction = get_instruction(&0x24);
        assert!(instruction == Some(&INC_H));
        let mut cpu = Cpu::new();
        cpu.reg.h = 0xFF;
        (instruction.unwrap().handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x24));
        assert_eq!(cpu.reg.h, 0x00);
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    fn test_get_instruction_inc_h_half_carry() {
        let instruction = get_instruction(&0x24);
        assert!(instruction == Some(&INC_H));
        let mut cpu = Cpu::new();
        cpu.reg.h = 0x0F;
        (instruction.unwrap().handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x24));
        assert_eq!(cpu.reg.h, 0x10);
        assert!(cpu.reg.check_flag(Flag::HalfCarry));
    }

    #[test]
    fn test_get_instruction_jr_z_n_zero_flag_set() {
        let instruction = get_instruction(&0x28);
        assert!(instruction == Some(&JR_Z_N));
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.pc = 0x000A;
        mmu.set_byte(0x000B as u16, 0xFB);
        cpu.reg.set_flag(Flag::Zero);
        let t = (instruction.unwrap().handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x28));
        assert_eq!(cpu.reg.pc, 0x0007);
        assert!(t == InstructionType::Jumped)
    }

    #[test]
    fn test_get_instruction_jr_z_n_zero_flag_not_set() {
        let instruction = get_instruction(&0x28);
        assert!(instruction == Some(&JR_Z_N));
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.pc = 0x000A;
        mmu.set_byte(0x000B as u16, 0xFB);
        let t = (instruction.unwrap().handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x28));
        assert_eq!(cpu.reg.pc, 0x000A);
        assert!(t == InstructionType::None)
    }

    #[test]
    fn test_get_instruction_ld_l_n() {
        let instruction = get_instruction(&0x2E);
        assert!(instruction == Some(&LD_L_N));
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        mmu.set_byte(0x01 as u16, 0x12);
        (instruction.unwrap().handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x2E));
        assert_eq!(cpu.reg.l, 0x12);
    }

    #[test]
    fn test_get_instruction_ld_sp_nn() {
        let instruction = get_instruction(&0x31);
        assert!(instruction == Some(&LD_SP_NN));
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        mmu.set_word(0x01 as u8, 0x1234);
        (instruction.unwrap().handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x31));
        assert_eq!(cpu.reg.sp, 0x1234);
    }

    #[test]
    fn test_get_instruction_ld_hl_dec_a() {
        let instruction = get_instruction(&0x32);
        assert!(instruction == Some(&LD_HL_DEC_A));
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.a = 0x12;
        cpu.reg.h = 0x12;
        cpu.reg.l = 0x34;
        (instruction.unwrap().handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x32));
        assert_eq!(mmu.get_byte(0x1234 as u16), 0x12);
        assert_eq!(cpu.reg.h, 0x12);
        assert_eq!(cpu.reg.l, 0x33);
    }

    #[test]
    fn test_get_instruction_ld_hl_dec_a_overflow() {
        let instruction = get_instruction(&0x32);
        assert!(instruction == Some(&LD_HL_DEC_A));
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.a = 0x12;
        cpu.reg.h = 0x00;
        cpu.reg.l = 0x00;
        (instruction.unwrap().handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x32));
        assert_eq!(mmu.get_byte(0x0000 as u16), 0x12);
        assert_eq!(cpu.reg.h, 0xFF);
        assert_eq!(cpu.reg.l, 0xFF);
    }

    #[test]
    fn test_get_instruction_dec_a() {
        let instruction = get_instruction(&0x3D);
        assert!(instruction == Some(&DEC_A));
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x02;
        (instruction.unwrap().handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x3D));
        assert_eq!(cpu.reg.a, 0x01);
    }

    #[test]
    fn test_get_instruction_dec_a_zero() {
        let instruction = get_instruction(&0x3D);
        assert!(instruction == Some(&DEC_A));
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x01;
        (instruction.unwrap().handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x3D));
        assert_eq!(cpu.reg.a, 0x00);
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    fn test_get_instruction_dec_a_half_carry() {
        let instruction = get_instruction(&0x3D);
        assert!(instruction == Some(&DEC_A));
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x10;
        (instruction.unwrap().handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x3D));
        assert_eq!(cpu.reg.a, 0x0F);
        assert!(cpu.reg.check_flag(Flag::HalfCarry));
    }

    #[test]
    fn test_get_instruction_dec_a_overflow() {
        let instruction = get_instruction(&0x3D);
        assert!(instruction == Some(&DEC_A));
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x00;
        (instruction.unwrap().handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x3D));
        assert_eq!(cpu.reg.a, 0xFF);
    }

    #[test]
    fn test_get_instruction_ld_a_n() {
        let instruction = get_instruction(&0x3E);
        assert!(instruction == Some(&LD_A_N));
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        mmu.set_byte(0x01 as u16, 0x12);
        (instruction.unwrap().handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x3E));
        assert_eq!(cpu.reg.a, 0x12);
    }

    #[test]
    fn test_get_instruction_ld_c_a() {
        let instruction = get_instruction(&0x4F);
        assert!(instruction == Some(&LD_C_A));
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x12;
        (instruction.unwrap().handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x4F));
        assert_eq!(cpu.reg.c, 0x12);
    }

    #[test]
    fn test_get_instruction_ld_d_a() {
        let instruction = get_instruction(&0x57);
        assert!(instruction == Some(&LD_D_A));
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x12;
        (instruction.unwrap().handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x57));
        assert_eq!(cpu.reg.d, 0x12);
    }

    #[test]
    fn test_get_instruction_ld_h_a() {
        let instruction = get_instruction(&0x67);
        assert!(instruction == Some(&LD_H_A));
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x12;
        (instruction.unwrap().handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x67));
        assert_eq!(cpu.reg.h, 0x12);
    }

    #[test]
    fn test_get_instruction_ld_hl_a() {
        let instruction = get_instruction(&0x77);
        assert!(instruction == Some(&LD_HL_A));
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.a = 0x12;
        cpu.reg.h = 0x12;
        cpu.reg.l = 0x34;
        (instruction.unwrap().handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x77));
        assert_eq!(mmu.get_byte(0x1234 as u16), 0x12);
    }

    #[test]
    fn test_get_instruction_ld_a_b() {
        let instruction = get_instruction(&0x78);
        assert!(instruction == Some(&LD_A_B));
        let mut cpu = Cpu::new();
        cpu.reg.b = 0x12;
        (instruction.unwrap().handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x78));
        assert_eq!(cpu.reg.a, 0x12);
    }

    #[test]
    fn test_get_instruction_ld_a_e() {
        let instruction = get_instruction(&0x7B);
        assert!(instruction == Some(&LD_A_E));
        let mut cpu = Cpu::new();
        cpu.reg.e = 0x12;
        (instruction.unwrap().handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x7B));
        assert_eq!(cpu.reg.a, 0x12);
    }

    #[test]
    fn test_get_instruction_ld_a_h() {
        let instruction = get_instruction(&0x7C);
        assert!(instruction == Some(&LD_A_H));
        let mut cpu = Cpu::new();
        cpu.reg.h = 0x12;
        (instruction.unwrap().handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x7C));
        assert_eq!(cpu.reg.a, 0x12);
    }

    #[test]
    fn test_get_instruction_ld_a_l() {
        let instruction = get_instruction(&0x7D);
        assert!(instruction == Some(&LD_A_L));
        let mut cpu = Cpu::new();
        cpu.reg.l = 0x12;
        (instruction.unwrap().handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x7D));
        assert_eq!(cpu.reg.a, 0x12);
    }

    #[test]
    fn test_get_instruction_add_hl() {
        let instruction = get_instruction(&0x86);
        assert!(instruction == Some(&ADD_HL));
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.h = 0x12;
        cpu.reg.l = 0x34;
        cpu.reg.a = 0x12;
        mmu.set_byte(0x1234 as u16, 0x34);
        (instruction.unwrap().handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x86));
        assert_eq!(cpu.reg.a, 0x46);
    }

    #[test]
    fn test_get_instruction_add_hl_zero() {
        let instruction = get_instruction(&0x86);
        assert!(instruction == Some(&ADD_HL));
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.h = 0x12;
        cpu.reg.l = 0x34;
        cpu.reg.a = 0x00;
        mmu.set_byte(0x1234 as u16, 0x00);
        (instruction.unwrap().handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x86));
        assert_eq!(cpu.reg.a, 0x00);
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    fn test_get_instruction_add_hl_half_carry() {
        let instruction = get_instruction(&0x86);
        assert!(instruction == Some(&ADD_HL));
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.h = 0x12;
        cpu.reg.l = 0x34;
        cpu.reg.a = 0x0F;
        mmu.set_byte(0x1234 as u16, 0x01);
        (instruction.unwrap().handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x86));
        assert_eq!(cpu.reg.a, 0x10);
        assert!(cpu.reg.check_flag(Flag::HalfCarry));
    }

    #[test]
    fn test_get_instruction_add_hl_carry() {
        let instruction = get_instruction(&0x86);
        assert!(instruction == Some(&ADD_HL));
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.h = 0x12;
        cpu.reg.l = 0x34;
        cpu.reg.a = 0xFF;
        mmu.set_byte(0x1234 as u16, 0x01);
        (instruction.unwrap().handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x86));
        assert_eq!(cpu.reg.a, 0x00);
        assert!(cpu.reg.check_flag(Flag::Carry));
    }

    #[test]
    fn test_get_instruction_sub_b() {
        let instruction = get_instruction(&0x90);
        assert!(instruction == Some(&SUB_B));
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x12;
        cpu.reg.b = 0x34;
        (instruction.unwrap().handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x90));
        assert_eq!(cpu.reg.a, 0xDE);
        assert!(cpu.reg.check_flag(Flag::Subtract));
        assert!(cpu.reg.check_flag(Flag::HalfCarry));
        assert!(cpu.reg.check_flag(Flag::Carry));
    }

    #[test]
    fn test_get_instruction_sub_b_zero() {
        let instruction = get_instruction(&0x90);
        assert!(instruction == Some(&SUB_B));
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x12;
        cpu.reg.b = 0x12;
        (instruction.unwrap().handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x90));
        assert_eq!(cpu.reg.a, 0x00);
        assert!(cpu.reg.check_flag(Flag::Zero));
        assert!(cpu.reg.check_flag(Flag::Subtract));
        assert!(!cpu.reg.check_flag(Flag::HalfCarry));
        assert!(!cpu.reg.check_flag(Flag::Carry));
    }

    #[test]
    fn test_get_instruction_sub_b_half_carry() {
        let instruction = get_instruction(&0x90);
        assert!(instruction == Some(&SUB_B));
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x10;
        cpu.reg.b = 0x01;
        (instruction.unwrap().handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x90));
        assert_eq!(cpu.reg.a, 0x0F);
        assert!(cpu.reg.check_flag(Flag::Subtract));
        assert!(cpu.reg.check_flag(Flag::HalfCarry));
        assert!(!cpu.reg.check_flag(Flag::Carry));
    }

    #[test]
    fn test_get_instruction_sub_b_carry() {
        let instruction = get_instruction(&0x90);
        assert!(instruction == Some(&SUB_B));
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x00;
        cpu.reg.b = 0x01;
        (instruction.unwrap().handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x90));
        assert_eq!(cpu.reg.a, 0xFF);
        assert!(cpu.reg.check_flag(Flag::Subtract));
        assert!(cpu.reg.check_flag(Flag::Carry));
    }

    #[test]
    fn test_get_instruction_xor_a() {
        let instruction = get_instruction(&0xAF);
        assert!(instruction == Some(&XOR_A));
        let mut cpu = Cpu::new();
        (instruction.unwrap().handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0xAF));
        assert_eq!(cpu.reg.a, 0);
        assert!(cpu.reg.check_flag(Flag::Zero));
        assert!(!cpu.reg.check_flag(Flag::HalfCarry));
        assert!(!cpu.reg.check_flag(Flag::Subtract));
        assert!(!cpu.reg.check_flag(Flag::Carry));
    }

    #[test]
    fn test_get_instruction_cp_hl() {
        let instruction = get_instruction(&0xBE);
        assert!(instruction == Some(&CP_HL));
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.a = 0xFF;
        cpu.reg.h = 0x12;
        cpu.reg.l = 0x34;
        mmu.set_byte(0x1234 as u16, 0x34);
        (instruction.unwrap().handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xBE));
        assert_eq!(cpu.reg.a, 0xFF);
        assert!(!cpu.reg.check_flag(Flag::Zero));
        assert!(cpu.reg.check_flag(Flag::Subtract));
        assert!(!cpu.reg.check_flag(Flag::HalfCarry));
        assert!(!cpu.reg.check_flag(Flag::Carry));
    }

    #[test]
    fn test_get_instruction_cp_hl_zero() {
        let instruction = get_instruction(&0xBE);
        assert!(instruction == Some(&CP_HL));
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.a = 0x34;
        cpu.reg.h = 0x12;
        cpu.reg.l = 0x34;
        mmu.set_byte(0x1234 as u16, 0x34);
        (instruction.unwrap().handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xBE));
        assert_eq!(cpu.reg.a, 0x34);
        assert!(cpu.reg.check_flag(Flag::Zero));
        assert!(cpu.reg.check_flag(Flag::Subtract));
        assert!(!cpu.reg.check_flag(Flag::HalfCarry));
        assert!(!cpu.reg.check_flag(Flag::Carry));
    }

    #[test]
    fn test_get_instruction_cp_hl_half_carry() {
        let instruction = get_instruction(&0xBE);
        assert!(instruction == Some(&CP_HL));
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.a = 0x10;
        cpu.reg.h = 0x12;
        cpu.reg.l = 0x34;
        mmu.set_byte(0x1234 as u16, 0x01);
        (instruction.unwrap().handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xBE));
        assert_eq!(cpu.reg.a, 0x10);
        assert!(cpu.reg.check_flag(Flag::Subtract));
        assert!(cpu.reg.check_flag(Flag::HalfCarry));
        assert!(!cpu.reg.check_flag(Flag::Carry));
    }

    #[test]
    fn test_get_instruction_cp_hl_carry() {
        let instruction = get_instruction(&0xBE);
        assert!(instruction == Some(&CP_HL));
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.a = 0x01;
        cpu.reg.h = 0x12;
        cpu.reg.l = 0x34;
        mmu.set_byte(0x1234 as u16, 0x10);
        (instruction.unwrap().handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xBE));
        assert_eq!(cpu.reg.a, 0x01);
        assert!(cpu.reg.check_flag(Flag::Subtract));
        assert!(cpu.reg.check_flag(Flag::Carry));
    }

    #[test]
    fn test_get_instruction_pop_bc() {
        let instruction = get_instruction(&0xC1);
        assert!(instruction == Some(&POP_BC));
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.sp = 0xFFFC;
        mmu.set_word(0xFFFC as u16, 0x1234);
        (instruction.unwrap().handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xC1));
        assert_eq!(cpu.reg.sp, 0xFFFE);
        assert_eq!(cpu.reg.b, 0x12);
        assert_eq!(cpu.reg.c, 0x34);
    }

    #[test]
    fn test_get_instruction_push_bc() {
        let instruction = get_instruction(&0xC5);
        assert!(instruction == Some(&PUSH_BC));
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.sp = 0xFFFE;
        cpu.reg.b = 0x12;
        cpu.reg.c = 0x34;
        (instruction.unwrap().handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xC5));
        assert_eq!(cpu.reg.sp, 0xFFFC);
        assert_eq!(mmu.get_word(cpu.reg.sp), 0x1234);
    }

    #[test]
    fn test_get_instruction_ret() {
        let instruction = get_instruction(&0xC9);
        assert!(instruction == Some(&RET));
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.sp = 0xFFFC;
        mmu.set_word(0xFFFC as u16, 0x1234);
        (instruction.unwrap().handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xC9));
        assert_eq!(cpu.reg.sp, 0xFFFE);
        assert_eq!(cpu.reg.pc, 0x1234);
    }

    #[test]
    fn test_get_instruction_call_nn() {
        let instruction = get_instruction(&0xCD);
        assert!(instruction == Some(&CALL_NN));
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.sp = 0xFFFE;
        mmu.set_word(0x01 as u8, 0x1234);
        (instruction.unwrap().handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xCD));
        assert_eq!(cpu.reg.sp, 0xFFFC);
        assert_eq!(mmu.get_word(cpu.reg.sp), 0x0003);
        assert_eq!(cpu.reg.pc, 0x1234);
    }

    #[test]
    fn test_get_instruction_ldh_n_a() {
        let instruction = get_instruction(&0xE0);
        assert!(instruction == Some(&LDH_N_A));
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.a = 0x12;
        mmu.set_byte(0x01 as u16, 0x34);
        (instruction.unwrap().handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xE0));
        assert_eq!(mmu.get_byte(0xFF34 as u16), 0x12);
    }

    #[test]
    fn test_get_instruction_ld_ptr_c_a() {
        let instruction = get_instruction(&0xE2);
        assert!(instruction == Some(&LD_PTR_C_A));
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.c = 0x12;
        cpu.reg.a = 0x34;
        (instruction.unwrap().handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xE2));
        assert_eq!(mmu.get_byte(0xFF12 as u16), 0x34);
    }

    #[test]
    fn test_get_instruction_ld_nn_a() {
        let instruction = get_instruction(&0xEA);
        assert!(instruction == Some(&LD_NN_A));
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.a = 0x12;
        mmu.set_word(0x01 as u8, 0x1234);
        (instruction.unwrap().handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xEA));
        assert_eq!(mmu.get_byte(0x1234 as u16), 0x12);
    }

    #[test]
    fn test_get_instruction_ldh_a_n() {
        let instruction = get_instruction(&0xF0);
        assert!(instruction == Some(&LDH_A_N));
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        mmu.set_byte(0x01 as u16, 0x34);
        mmu.set_byte(0xFF34 as u16, 0x12);
        (instruction.unwrap().handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xF0));
        assert_eq!(cpu.reg.a, 0x12);
    }

    #[test]
    fn test_get_instruction_cp_a_n() {
        let instruction = get_instruction(&0xFE);
        assert!(instruction == Some(&CP_A_N));
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.a = 0xFF;
        mmu.set_byte(0x01 as u16, 0x00);
        (instruction.unwrap().handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xFE));
        assert_eq!(cpu.reg.a, 0xFF);
        assert_eq!(mmu.get_byte(0x01 as u16), 0x00);
        assert!(cpu.reg.check_flag(Flag::Subtract));
        assert!(!cpu.reg.check_flag(Flag::HalfCarry));
        assert!(!cpu.reg.check_flag(Flag::Carry));
    }

    #[test]
    fn test_get_instruction_cp_a_n_equal() {
        let instruction = get_instruction(&0xFE);
        assert!(instruction == Some(&CP_A_N));
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.a = 0x00;
        mmu.set_byte(0x01 as u16, 0x00);
        (instruction.unwrap().handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xFE));
        assert_eq!(cpu.reg.a, 0x00);
        assert_eq!(mmu.get_byte(0x01 as u16), 0x00);
        assert!(cpu.reg.check_flag(Flag::Zero));
        assert!(cpu.reg.check_flag(Flag::Subtract));
        assert!(!cpu.reg.check_flag(Flag::HalfCarry));
        assert!(!cpu.reg.check_flag(Flag::Carry));
    }

    #[test]
    fn test_get_instruction_cp_a_n_half_carry() {
        let instruction = get_instruction(&0xFE);
        assert!(instruction == Some(&CP_A_N));
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.a = 0x01;
        mmu.set_byte(0x01 as u16, 0x1F);
        (instruction.unwrap().handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xFE));
        assert_eq!(cpu.reg.a, 0x01);
        assert_eq!(mmu.get_byte(0x01 as u16), 0x1F);
        assert!(cpu.reg.check_flag(Flag::Subtract));
        assert!(cpu.reg.check_flag(Flag::HalfCarry));
    }

    #[test]
    fn test_get_instruction_cp_a_n_carry() {
        let instruction = get_instruction(&0xFE);
        assert!(instruction == Some(&CP_A_N));
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.a = 0x00;
        mmu.set_byte(0x01 as u16, 0x01);
        (instruction.unwrap().handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xFE));
        assert_eq!(cpu.reg.a, 0x00);
        assert_eq!(mmu.get_byte(0x01 as u16), 0x01);
        assert!(cpu.reg.check_flag(Flag::Subtract));
        assert!(cpu.reg.check_flag(Flag::Carry));
    }

    #[test]
    fn test_get_instruction_unimplemnted_instruction() {
        let instruction = get_instruction(&0xD3);
        assert_eq!(instruction, None);
    }
}