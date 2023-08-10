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

/// 0x01 - LD BC, nn
const LD_BC_NN: Instruction = Instruction {
    length: 3,
    clock_cycles: 12,
    clock_cycles_condition: None,
    description: "LD BC nn",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        cpu.reg.set_bc(functions::get_op16(cpu, mmu));
        InstructionType::ActionTaken
    },
};

/// 0x03 - INC BC
const INC_BC: Instruction = Instruction {
    length: 1,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "INC BC",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.set_bc(cpu.reg.bc().wrapping_add(1));
        InstructionType::ActionTaken
    },
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

/// 0x12 - LD (DE), A
const LD_DE_A: Instruction = Instruction {
    length: 1,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "LD (DE) A",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        mmu.set_byte(cpu.reg.de(), cpu.reg.a);
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

/// 0x14 - INC D
const INC_D: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "INC D",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.d = functions::inc(cpu, cpu.reg.d);
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

/// 0x1C - INC E
const INC_E: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "INC E",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.e = functions::inc(cpu, cpu.reg.e);
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

/// 0x1F - RRA
const RRA: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "RRA",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.a = functions::rr(cpu, cpu.reg.a);
        // Unlike normal RR operations, RRA always clears the zero flag
        cpu.reg.clear_flag(Flag::Zero);
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

/// 0x25 - DEC H
const DEC_H: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "DEC H",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.h = functions::dec(cpu, cpu.reg.h);
        InstructionType::ActionTaken
    },
};

/// 0x26 - LD H, n
const LD_H_N: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "LD H n",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        cpu.reg.h = functions::get_op8(cpu, mmu, 1);
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

/// 0x2A - LD A, (HL+)
const LD_A_HL_INC: Instruction = Instruction {
    length: 1,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "LD A (HL+)",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        cpu.reg.a = mmu.get_byte(cpu.reg.hl());
        cpu.reg.set_hl(cpu.reg.hl().wrapping_add(1));
        InstructionType::ActionTaken
    },
};

/// 0x2C - INC L
const INC_L: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "INC L",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.l = functions::inc(cpu, cpu.reg.l);
        InstructionType::ActionTaken
    },
};

/// 0x2D - DEC L
const DEC_L: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "DEC L",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.l = functions::dec(cpu, cpu.reg.l);
        InstructionType::ActionTaken
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

/// 0x30 - JR NC, n
const JR_NC_N: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: Some(12),
    description: "JR NC n",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        if !cpu.reg.check_flag(Flag::Carry) {
            functions::jr(cpu, mmu);
            return InstructionType::Jumped;
        }
        InstructionType::None
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

/// 0x35 - DEC (HL)
const DEC_HL: Instruction = Instruction {
    length: 1,
    clock_cycles: 12,
    clock_cycles_condition: None,
    description: "DEC (HL)",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        let value = functions::dec(cpu, mmu.get_byte(cpu.reg.hl()));
        mmu.set_byte(cpu.reg.hl(), value);
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

/// 0x46 - LD B, (HL)
const LD_B_HL: Instruction = Instruction {
    length: 1,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "LD B (HL)",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        cpu.reg.b = functions::get_hl(cpu, mmu);
        InstructionType::ActionTaken
    },
};

/// 0x47 - LD B, A
const LD_B_A: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "LD B A",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.b = cpu.reg.a;
        InstructionType::ActionTaken
    },
};

/// 0x4E - LD C, (HL)
const LD_C_HL: Instruction = Instruction {
    length: 1,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "LD C (HL)",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        cpu.reg.c = functions::get_hl(cpu, mmu);
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

/// 0x56 - LD D, (HL)
const LD_D_HL: Instruction = Instruction {
    length: 1,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "LD D (HL)",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        cpu.reg.d = functions::get_hl(cpu, mmu);
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

/// 0x5F - LD E, A
const LD_E_A: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "LD E A",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.e = cpu.reg.a;
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

/// 0x6E - LD L, (HL)
const LD_L_HL: Instruction = Instruction {
    length: 1,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "LD L (HL)",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        cpu.reg.l = functions::get_hl(cpu, mmu);
        InstructionType::ActionTaken
    },
};

/// 0x70 - LD (HL), B
const LD_HL_B: Instruction = Instruction {
    length: 1,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "LD (HL) B",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        functions::set_hl(cpu, mmu, cpu.reg.b);
        InstructionType::ActionTaken
    },
};

/// 0x71 - LD (HL), C
const LD_HL_C: Instruction = Instruction {
    length: 1,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "LD (HL) C",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        functions::set_hl(cpu, mmu, cpu.reg.c);
        InstructionType::ActionTaken
    },
};

/// 0x72 - LD (HL), D
const LD_HL_D: Instruction = Instruction {
    length: 1,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "LD (HL) D",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        functions::set_hl(cpu, mmu, cpu.reg.d);
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

/// 0x79 - LD A, C
const LD_A_C: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "LD A, C",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.a = cpu.reg.c;
        InstructionType::ActionTaken
    },
};

/// 0x7A - LD A, D
const LD_A_D: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "LD A, D",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.a = cpu.reg.d;
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

/// 0xA9 - XOR C
const XOR_C: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "XOR C",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        functions::xor_bytes(cpu, cpu.reg.a, cpu.reg.c);
        InstructionType::ActionTaken
    },
};

/// 0xAE - XOR (HL)
const XOR_HL: Instruction = Instruction {
    length: 1,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "XOR (HL)",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        functions::xor_bytes(cpu, cpu.reg.a, functions::get_hl(cpu, mmu));
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

/// 0xB1 - OR C
const OR_C: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "OR B",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        functions::or(cpu, cpu.reg.b);
        InstructionType::ActionTaken
    },
};

/// 0xB6 - OR (HL)
const OR_HL: Instruction = Instruction {
    length: 1,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "OR (HL)",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        functions::or(cpu, functions::get_hl(cpu, mmu));
        InstructionType::ActionTaken
    },
};

/// 0xB7 - OR A
const OR_A: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "OR A",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        functions::or(cpu, cpu.reg.a);
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

/// 0xC3 - JP nn
const JP_NN: Instruction = Instruction {
    length: 3,
    clock_cycles: 16,
    clock_cycles_condition: None,
    description: "JP nn",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        cpu.reg.pc = functions::get_op16(cpu, mmu);
        InstructionType::Jumped
    },
};

/// 0xC4 - CALL NZ, nn
const CALL_NZ_NN: Instruction = Instruction {
    length: 3,
    clock_cycles: 12,
    clock_cycles_condition: Some(24),
    description: "CALL NZ, nn",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        if !cpu.reg.check_flag(Flag::Zero) {
            functions::call(cpu, mmu);
            return InstructionType::Jumped;
        }
        InstructionType::None
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

/// 0xC6 => ADD n
const ADD_N: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "ADD A n",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        functions::add(cpu, functions::get_op8(cpu, mmu, 1));
        InstructionType::ActionTaken
    },
};

/// 0xC8 - RET Z
const RET_Z: Instruction = Instruction {
    length: 1,
    clock_cycles: 20,
    clock_cycles_condition: Some(8),
    description: "RET Z",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        if cpu.reg.check_flag(Flag::Zero) {
            cpu.reg.pc = functions::pop(cpu, mmu);
            return InstructionType::Jumped;
        }
        InstructionType::None
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

/// 0xCE - ADC n
const ADC_N: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "ADC n",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        functions::adc(cpu, functions::get_op8(cpu, mmu, 1));
        InstructionType::ActionTaken
    },
};

/// 0xD0 - RET NC
const RET_NC: Instruction = Instruction {
    length: 1,
    clock_cycles: 20,
    clock_cycles_condition: Some(8),
    description: "RET NC",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        if !cpu.reg.check_flag(Flag::Carry) {
            cpu.reg.pc = functions::pop(cpu, mmu);
            return InstructionType::Jumped;
        }
        InstructionType::None
    },
};

/// 0xD1 - POP DE
const POP_DE: Instruction = Instruction {
    length: 1,
    clock_cycles: 12,
    clock_cycles_condition: None,
    description: "POP DE",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        let val = functions::pop(cpu, mmu);
        cpu.reg.set_de(val);
        InstructionType::ActionTaken
    },
};

/// 0xD5 - PUSH DE
const PUSH_DE: Instruction = Instruction {
    length: 1,
    clock_cycles: 16,
    clock_cycles_condition: None,
    description: "PUSH DE",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        functions::push(cpu, mmu, cpu.reg.de());
        InstructionType::ActionTaken
    },
};

/// 0xD6 - SUB n
const SUB_N: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SUB n",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        functions::sub(cpu, functions::get_op8(cpu, mmu, 1));
        InstructionType::ActionTaken
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

/// 0xE1 - POP HL
const POP_HL: Instruction = Instruction {
    length: 1,
    clock_cycles: 12,
    clock_cycles_condition: None,
    description: "POP HL",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        let val = functions::pop(cpu, mmu);
        cpu.reg.set_hl(val);
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

/// 0xE5 - PUSH HL
const PUSH_HL: Instruction = Instruction {
    length: 1,
    clock_cycles: 16,
    clock_cycles_condition: None,
    description: "PUSH HL",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        functions::push(cpu, mmu, cpu.reg.hl());
        InstructionType::ActionTaken
    },
};

/// 0xE6 - AND n
const AND_N: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "AND n",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        functions::and(cpu, functions::get_op8(cpu, mmu, 1));
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

/// 0xEE - XOR n
const XOR_N: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "XOR n",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        functions::xor_bytes(cpu, cpu.reg.a, functions::get_op8(cpu, mmu, 1));
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

/// 0xF1 - POP AF
const POP_AF: Instruction = Instruction {
    length: 1,
    clock_cycles: 12,
    clock_cycles_condition: None,
    description: "POP AF",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        let val = functions::pop(cpu, mmu);
        cpu.reg.set_af(val);
        InstructionType::ActionTaken
    },
};

/// 0xF3 - DI
const DI: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "DI",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        log::trace!("Disabling IME");
        cpu.interrupts_enabled = false;
        InstructionType::ActionTaken
    },
};

/// 0xF5 - PUSH AF
const PUSH_AF: Instruction = Instruction {
    length: 1,
    clock_cycles: 16,
    clock_cycles_condition: None,
    description: "PUSH AF",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        functions::push(cpu, mmu, cpu.reg.af());
        InstructionType::ActionTaken
    },
};

/// 0xFA - LD A, [nn]
const LD_A_NN: Instruction = Instruction {
    length: 3,
    clock_cycles: 16,
    clock_cycles_condition: None,
    description: "LD A [nn]",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        cpu.reg.a = mmu.get_byte(functions::get_op16(cpu, mmu));
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
        0x01 => Some(&LD_BC_NN),
        0x03 => Some(&INC_BC),
        0x04 => Some(&INC_B),
        0x05 => Some(&DEC_B),
        0x06 => Some(&LD_B_N),
        0x0C => Some(&INC_C),
        0x0D => Some(&DEC_C),
        0x0E => Some(&LD_C_N),

        0x11 => Some(&LD_DE_NN),
        0x12 => Some(&LD_DE_A),
        0x13 => Some(&INC_DE),
        0x14 => Some(&INC_D),
        0x15 => Some(&DEC_D),
        0x16 => Some(&LD_D_N),
        0x17 => Some(&RLA),
        0x18 => Some(&JR_N),
        0x1A => Some(&LD_A_DE),
        0x1C => Some(&INC_E),
        0x1D => Some(&DEC_E),
        0x1E => Some(&LD_E_N),
        0x1F => Some(&RRA),

        0x20 => Some(&JR_NZ_N),
        0x21 => Some(&LD_HL_NN),
        0x22 => Some(&LD_HL_INC_A),
        0x23 => Some(&INC_HL),
        0x24 => Some(&INC_H),
        0x25 => Some(&DEC_H),
        0x26 => Some(&LD_H_N),
        0x28 => Some(&JR_Z_N),
        0x2A => Some(&LD_A_HL_INC),
        0x2C => Some(&INC_L),
        0x2D => Some(&DEC_L),
        0x2E => Some(&LD_L_N),

        0x30 => Some(&JR_NC_N),
        0x31 => Some(&LD_SP_NN),
        0x32 => Some(&LD_HL_DEC_A),
        0x35 => Some(&DEC_HL),
        0x3D => Some(&DEC_A),
        0x3E => Some(&LD_A_N),

        0x46 => Some(&LD_B_HL),
        0x47 => Some(&LD_B_A),
        0x4E => Some(&LD_C_HL),
        0x4F => Some(&LD_C_A),

        0x56 => Some(&LD_D_HL),
        0x57 => Some(&LD_D_A),
        0x5F => Some(&LD_E_A),

        0x67 => Some(&LD_H_A),
        0x6E => Some(&LD_L_HL),

        0x70 => Some(&LD_HL_B),
        0x71 => Some(&LD_HL_C),
        0x72 => Some(&LD_HL_D),
        0x77 => Some(&LD_HL_A),
        0x78 => Some(&LD_A_B),
        0x79 => Some(&LD_A_C),
        0x7A => Some(&LD_A_D),
        0x7B => Some(&LD_A_E),
        0x7C => Some(&LD_A_H),
        0x7D => Some(&LD_A_L),

        0x86 => Some(&ADD_HL),

        0x90 => Some(&SUB_B),

        0xA9 => Some(&XOR_C),
        0xAE => Some(&XOR_HL),
        0xAF => Some(&XOR_A),

        0xB1 => Some(&OR_C),
        0xB6 => Some(&OR_HL),
        0xB7 => Some(&OR_A),
        0xBE => Some(&CP_HL),

        0xC1 => Some(&POP_BC),
        0xC3 => Some(&JP_NN),
        0xC4 => Some(&CALL_NZ_NN),
        0xC5 => Some(&PUSH_BC),
        0xC6 => Some(&ADD_N),
        0xC8 => Some(&RET_Z),
        0xC9 => Some(&RET),
        0xCD => Some(&CALL_NN),
        0xCE => Some(&ADC_N),

        0xD0 => Some(&RET_NC),
        0xD1 => Some(&POP_DE),
        0xD5 => Some(&PUSH_DE),
        0xD6 => Some(&SUB_N),

        0xE0 => Some(&LDH_N_A),
        0xE1 => Some(&POP_HL),
        0xE2 => Some(&LDH_C_A),
        0xE5 => Some(&PUSH_HL),
        0xE6 => Some(&AND_N),
        0xEA => Some(&LD_NN_A),
        0xEE => Some(&XOR_N),

        0xF0 => Some(&LDH_A_N),
        0xF1 => Some(&POP_AF),
        0xF3 => Some(&DI),
        0xF5 => Some(&PUSH_AF),
        0xFA => Some(&LD_A_NN),
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
    pub fn test_get_instruction_ld_bc_nn() {
        let instruction = get_instruction(&0x01);
        assert_eq!(instruction.unwrap(), &LD_BC_NN);
        assert_eq!(instruction.unwrap().length, 3);
        assert_eq!(instruction.unwrap().clock_cycles, 12);
    }

    #[test]
    pub fn test_ld_bc_nn() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.pc = 0xC000;
        mmu.set_word(0xC001 as usize, 0x0102);

        let expected_cpu = Cpu {
            reg: Registers {
                b: 0x01,
                c: 0x02,
                ..cpu.reg
            },
            ..cpu
        };

        (&LD_BC_NN.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x01));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_get_instruction_inc_bc() {
        let instruction = get_instruction(&0x03);
        assert_eq!(instruction.unwrap(), &INC_BC);
        assert_eq!(instruction.unwrap().length, 1);
        assert_eq!(instruction.unwrap().clock_cycles, 8);
    }

    #[test]
    pub fn test_inc_bc() {
        let mut cpu = Cpu::new();
        cpu.reg.set_bc(0x0101);
        cpu.reg.clear_all_flags();

        let expected_cpu = Cpu {
            reg: Registers {
                b: 0x01,
                c: 0x02,
                ..cpu.reg
            },
            ..cpu
        };

        (&INC_BC.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x03));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_inc_bc_overflow() {
        let mut cpu = Cpu::new();
        cpu.reg.set_bc(0xFFFF);

        let expected_cpu = Cpu {
            reg: Registers {
                b: 0x00,
                c: 0x00,
                ..cpu.reg
            },
            ..cpu
        };

        (&INC_BC.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x03));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_inc_bc_overflow_c() {
        let mut cpu = Cpu::new();
        cpu.reg.set_bc(0x00FF);

        let expected_cpu = Cpu {
            reg: Registers {
                b: 0x01,
                c: 0x00,
                ..cpu.reg
            },
            ..cpu
        };

        (&INC_BC.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x03));
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
        cpu.reg.clear_all_flags();

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
        cpu.reg.clear_all_flags();

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
    pub fn test_get_instruction_ld_de_a() {
        let instruction = get_instruction(&0x12);
        assert_eq!(instruction.unwrap(), &LD_DE_A);
        assert_eq!(instruction.unwrap().length, 1);
        assert_eq!(instruction.unwrap().clock_cycles, 8);
    }

    #[test]
    pub fn test_ld_de_a() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.a = 0x02;
        cpu.reg.d = 0xC0;
        cpu.reg.e = 0x00;

        (&LD_DE_A.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x12));
        assert_eq!(mmu.get_byte(0xC000 as usize), 0x02);
    }

    #[test]
    pub fn test_get_instruction_inc_d() {
        let instruction = get_instruction(&0x14);
        assert_eq!(instruction.unwrap(), &INC_D);
        assert_eq!(instruction.unwrap().length, 1);
        assert_eq!(instruction.unwrap().clock_cycles, 4);
    }

    #[test]
    pub fn test_inc_d() {
        let mut cpu = Cpu::new();
        cpu.reg.d = 0x01;
        cpu.reg.clear_all_flags();

        let expected_cpu = Cpu {
            reg: Registers { d: 0x02, ..cpu.reg },
            ..cpu
        };

        (&INC_D.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x14));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_inc_d_zero() {
        let mut cpu = Cpu::new();
        cpu.reg.d = 0xFF;

        let expected_cpu = Cpu {
            reg: Registers {
                f: 0xA0, // Z_H_
                d: 0x00,
                ..cpu.reg
            },
            ..cpu
        };

        (&INC_D.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x14));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_inc_d_half_carry() {
        let mut cpu = Cpu::new();
        cpu.reg.d = 0x0F;

        let expected_cpu = Cpu {
            reg: Registers {
                f: 0x20, // __H_
                d: 0x10,
                ..cpu.reg
            },
            ..cpu
        };

        (&INC_D.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x14));
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
        cpu.reg.clear_flag(Flag::Carry);

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
        cpu.reg.clear_flag(Flag::Carry);

        let expected_cpu = Cpu {
            reg: Registers {
                a: 0x00,
                f: 0x00, // ____
                ..cpu.reg
            },
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
    pub fn test_get_instruction_inc_e() {
        let instruction = get_instruction(&0x1C);
        assert_eq!(instruction.unwrap(), &INC_E);
        assert_eq!(instruction.unwrap().length, 1);
        assert_eq!(instruction.unwrap().clock_cycles, 4);
    }

    #[test]
    pub fn test_inc_e() {
        let mut cpu = Cpu::new();
        cpu.reg.e = 0x01;
        cpu.reg.clear_all_flags();

        let expected_cpu = Cpu {
            reg: Registers { e: 0x02, ..cpu.reg },
            ..cpu
        };

        (&INC_E.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x1C));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_inc_e_zero() {
        let mut cpu = Cpu::new();
        cpu.reg.e = 0xFF;

        let expected_cpu = Cpu {
            reg: Registers {
                f: 0xA0, // Z_H_
                e: 0x00,
                ..cpu.reg
            },
            ..cpu
        };

        (&INC_E.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x1C));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_inc_e_half_carry() {
        let mut cpu = Cpu::new();
        cpu.reg.e = 0x0F;

        let expected_cpu = Cpu {
            reg: Registers {
                f: 0x20, // __H_
                e: 0x10,
                ..cpu.reg
            },
            ..cpu
        };

        (&INC_E.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x1C));
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
    pub fn test_get_instruction_rra() {
        let instruction = get_instruction(&0x1F);
        assert_eq!(instruction.unwrap(), &RRA);
        assert_eq!(instruction.unwrap().length, 1);
        assert_eq!(instruction.unwrap().clock_cycles, 4);
    }

    #[test]
    pub fn test_rra_bit_zero_set() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x01;
        cpu.reg.clear_flag(Flag::Carry);

        let expected_cpu = Cpu {
            reg: Registers {
                a: 0x00,
                f: 0x10, // ___C
                ..cpu.reg
            },
            ..cpu
        };

        (&RRA.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x1F));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_rra_bit_zero_set_carry() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x01;
        cpu.reg.f = 0x10;

        let expected_cpu = Cpu {
            reg: Registers {
                a: 0x80,
                f: 0x10, // ___C
                ..cpu.reg
            },
            ..cpu
        };

        (&RRA.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x1F));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_rra_bit_zero_clear() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x00;
        cpu.reg.clear_flag(Flag::Carry);

        let expected_cpu = Cpu {
            reg: Registers {
                a: 0x00,
                f: 0x00, // ____
                ..cpu.reg
            },
            ..cpu
        };

        (&RRA.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x1F));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_rra_bit_zero_clear_carry() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x00;
        cpu.reg.f = 0x10;

        let expected_cpu = Cpu {
            reg: Registers {
                a: 0x80,
                f: 0x00, // ____
                ..cpu.reg
            },
            ..cpu
        };

        (&RRA.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x1F));
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
        cpu.reg.clear_flag(Flag::Zero);
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
        cpu.reg.clear_flag(Flag::Zero);
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
    #[ignore = "Ignore for now as reading and writing to 0xFFFF isn't properly implemented"]
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
        cpu.reg.clear_all_flags();

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
    pub fn test_get_instruction_dec_h() {
        let instruction = get_instruction(&0x25);
        assert_eq!(instruction.unwrap(), &DEC_H);
        assert_eq!(instruction.unwrap().length, 1);
        assert_eq!(instruction.unwrap().clock_cycles, 4);
    }

    #[test]
    pub fn test_dec_h() {
        let mut cpu = Cpu::new();
        cpu.reg.h = 0x02;

        let expected_cpu = Cpu {
            reg: Registers {
                h: 0x01,
                f: 0x40, // _N__
                ..cpu.reg
            },
            ..cpu
        };

        (&DEC_H.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x25));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_get_instruction_ld_h_n() {
        let instruction = get_instruction(&0x26);
        assert_eq!(instruction.unwrap(), &LD_H_N);
        assert_eq!(instruction.unwrap().length, 2);
        assert_eq!(instruction.unwrap().clock_cycles, 8);
    }

    #[test]
    pub fn test_ld_h_n() {
        let mut cpu = Cpu::new();
        cpu.reg.pc = 0xC000;
        let mut mmu = Memory::new();
        mmu.set_byte(0xC001 as usize, 0x02);

        let expected_cpu = Cpu {
            reg: Registers { h: 0x02, ..cpu.reg },
            ..cpu
        };

        (&LD_H_N.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x26));
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
        cpu.reg.clear_flag(Flag::Zero);
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
        cpu.reg.clear_flag(Flag::Zero);
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
    pub fn test_get_instruction_ld_a_hl_inc() {
        let instruction = get_instruction(&0x2A);
        assert_eq!(instruction.unwrap(), &LD_A_HL_INC);
        assert_eq!(instruction.unwrap().length, 1);
        assert_eq!(instruction.unwrap().clock_cycles, 8);
    }

    #[test]
    pub fn test_ld_a_hl_inc() {
        let mut cpu = Cpu::new();
        cpu.reg.h = 0xC0;
        cpu.reg.l = 0x00;
        let mut mmu = Memory::new();
        mmu.set_byte(0xC000 as usize, 0x03);

        let expected_cpu = Cpu {
            reg: Registers {
                a: 0x03,
                h: 0xC0,
                l: 0x01,
                ..cpu.reg
            },
            ..cpu
        };

        (&LD_A_HL_INC.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x2A));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    #[ignore = "Ignore for now as reading and writing to 0xFFFF isn't properly implemented"]
    pub fn test_ld_a_hl_inc_overflow() {
        let mut cpu = Cpu::new();
        cpu.reg.h = 0xFF;
        cpu.reg.l = 0xFF;
        let mut mmu = Memory::new();
        mmu.set_byte(0xFFFF as usize, 0x03);

        let expected_cpu = Cpu {
            reg: Registers {
                a: 0x03,
                h: 0x00,
                l: 0x00,
                ..cpu.reg
            },
            ..cpu
        };

        (&LD_A_HL_INC.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x2A));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_ld_a_hl_inc_overflow_l() {
        let mut cpu = Cpu::new();
        cpu.reg.h = 0xC0;
        cpu.reg.l = 0xFF;
        let mut mmu = Memory::new();
        mmu.set_byte(0xC0FF as usize, 0x03);

        let expected_cpu = Cpu {
            reg: Registers {
                a: 0x03,
                h: 0xC1,
                l: 0x00,
                ..cpu.reg
            },
            ..cpu
        };

        (&LD_A_HL_INC.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x2A));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_get_instruction_inc_l() {
        let instruction = get_instruction(&0x2C);
        assert_eq!(instruction.unwrap(), &INC_L);
        assert_eq!(instruction.unwrap().length, 1);
        assert_eq!(instruction.unwrap().clock_cycles, 4);
    }

    #[test]
    pub fn test_inc_l() {
        let mut cpu = Cpu::new();
        cpu.reg.l = 0x02;
        cpu.reg.clear_all_flags();

        let expected_cpu = Cpu {
            reg: Registers { l: 0x03, ..cpu.reg },
            ..cpu
        };

        (&INC_L.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x2C));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_inc_l_zero() {
        let mut cpu = Cpu::new();
        cpu.reg.l = 0xFF;

        let expected_cpu = Cpu {
            reg: Registers {
                l: 0x00,
                f: 0xA0, // Z_H_
                ..cpu.reg
            },
            ..cpu
        };

        (&INC_L.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x2C));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_inc_l_half_carry() {
        let mut cpu = Cpu::new();
        cpu.reg.l = 0x0F;

        let expected_cpu = Cpu {
            reg: Registers {
                l: 0x10,
                f: 0x20, // __H_
                ..cpu.reg
            },
            ..cpu
        };

        (&INC_L.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x2C));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_get_instruction_dec_l() {
        let instruction = get_instruction(&0x2D);
        assert_eq!(instruction.unwrap(), &DEC_L);
        assert_eq!(instruction.unwrap().length, 1);
        assert_eq!(instruction.unwrap().clock_cycles, 4);
    }

    #[test]
    pub fn test_dec_l() {
        let mut cpu = Cpu::new();
        cpu.reg.l = 0x02;

        let expected_cpu = Cpu {
            reg: Registers {
                l: 0x01,
                f: 0x40, // _N__
                ..cpu.reg
            },
            ..cpu
        };

        (&DEC_L.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x2D));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_dec_l_zero() {
        let mut cpu = Cpu::new();
        cpu.reg.l = 0x01;

        let expected_cpu = Cpu {
            reg: Registers {
                l: 0x00,
                f: 0xC0, // ZN__
                ..cpu.reg
            },
            ..cpu
        };

        (&DEC_L.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x2D));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_dec_l_half_carry() {
        let mut cpu = Cpu::new();
        cpu.reg.l = 0x10;

        let expected_cpu = Cpu {
            reg: Registers {
                l: 0x0F,
                f: 0x60, // _NH_
                ..cpu.reg
            },
            ..cpu
        };

        (&DEC_L.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x2D));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_dec_l_underflow() {
        let mut cpu = Cpu::new();
        cpu.reg.l = 0x00;

        let expected_cpu = Cpu {
            reg: Registers {
                l: 0xFF,
                f: 0x60, // _NH_
                ..cpu.reg
            },
            ..cpu
        };

        (&DEC_L.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x2D));
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
    pub fn test_get_instruction_jr_nc_n() {
        let instruction = get_instruction(&0x30);
        assert_eq!(instruction.unwrap(), &JR_NC_N);
        assert_eq!(instruction.unwrap().length, 2);
        assert_eq!(instruction.unwrap().clock_cycles, 8);
        assert_eq!(instruction.unwrap().clock_cycles_condition.unwrap(), 12);
    }

    #[test]
    pub fn test_jr_nc_n_carry() {
        let mut cpu = Cpu::new();
        cpu.reg.f = 0x10;
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

        (&JR_NC_N.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x30));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_jr_nc_n_carry_negative() {
        let mut cpu = Cpu::new();
        cpu.reg.f = 0x10;
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

        (&JR_NC_N.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x30));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_jr_nc_n_not_carry() {
        let mut cpu = Cpu::new();
        cpu.reg.pc = 0xC000;
        cpu.reg.clear_flag(Flag::Carry);
        let mut mmu = Memory::new();
        mmu.set_byte(0xC001 as usize, 0x02);

        let expected_cpu = Cpu {
            reg: Registers {
                pc: 0xC004,
                ..cpu.reg
            },
            ..cpu
        };

        (&JR_NC_N.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x30));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_jr_nc_n_not_carry_negative() {
        let mut cpu = Cpu::new();
        cpu.reg.pc = 0xC000;
        cpu.reg.clear_flag(Flag::Carry);
        let mut mmu = Memory::new();
        mmu.set_byte(0xC001 as usize, 0xFE);

        let expected_cpu = Cpu {
            reg: Registers {
                pc: 0xC000,
                ..cpu.reg
            },
            ..cpu
        };

        (&JR_NC_N.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x30));
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
    pub fn test_get_instruction_ld_b_hl() {
        let instruction = get_instruction(&0x46);
        assert_eq!(instruction.unwrap(), &LD_B_HL);
        assert_eq!(instruction.unwrap().length, 1);
        assert_eq!(instruction.unwrap().clock_cycles, 8);
    }

    #[test]
    pub fn test_ld_b_hl() {
        let mut cpu = Cpu::new();
        cpu.reg.h = 0xC0;
        cpu.reg.l = 0x00;
        let mut mmu = Memory::new();
        mmu.set_byte(0xC000 as usize, 0x03);

        let expected_cpu = Cpu {
            reg: Registers { b: 0x03, ..cpu.reg },
            ..cpu
        };

        (&LD_B_HL.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x46));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_get_instruction_ld_b_a() {
        let instruction = get_instruction(&0x47);
        assert_eq!(instruction.unwrap(), &LD_B_A);
        assert_eq!(instruction.unwrap().length, 1);
        assert_eq!(instruction.unwrap().clock_cycles, 4);
    }

    #[test]
    pub fn test_ld_b_a() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x02;

        let expected_cpu = Cpu {
            reg: Registers { b: 0x02, ..cpu.reg },
            ..cpu
        };

        (&LD_B_A.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x47));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_get_instruction_ld_c_hl() {
        let instruction = get_instruction(&0x4E);
        assert_eq!(instruction.unwrap(), &LD_C_HL);
        assert_eq!(instruction.unwrap().length, 1);
        assert_eq!(instruction.unwrap().clock_cycles, 8);
    }

    #[test]
    pub fn test_ld_c_hl() {
        let mut cpu = Cpu::new();
        cpu.reg.h = 0xC0;
        cpu.reg.l = 0x00;
        let mut mmu = Memory::new();
        mmu.set_byte(0xC000 as usize, 0x03);

        let expected_cpu = Cpu {
            reg: Registers { c: 0x03, ..cpu.reg },
            ..cpu
        };

        (&LD_C_HL.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x4E));
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
    pub fn test_get_instruction_ld_d_hl() {
        let instruction = get_instruction(&0x56);
        assert_eq!(instruction.unwrap(), &LD_D_HL);
        assert_eq!(instruction.unwrap().length, 1);
        assert_eq!(instruction.unwrap().clock_cycles, 8);
    }

    #[test]
    pub fn test_ld_d_hl() {
        let mut cpu = Cpu::new();
        cpu.reg.h = 0xC0;
        cpu.reg.l = 0x00;
        let mut mmu = Memory::new();
        mmu.set_byte(0xC000 as usize, 0x03);

        let expected_cpu = Cpu {
            reg: Registers { d: 0x03, ..cpu.reg },
            ..cpu
        };

        (&LD_D_HL.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x56));
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
    pub fn test_get_instruction_ld_e_a() {
        let instruction = get_instruction(&0x5F);
        assert_eq!(instruction.unwrap(), &LD_E_A);
        assert_eq!(instruction.unwrap().length, 1);
        assert_eq!(instruction.unwrap().clock_cycles, 4);
    }

    #[test]
    pub fn test_ld_e_a() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x02;

        let expected_cpu = Cpu {
            reg: Registers { e: 0x02, ..cpu.reg },
            ..cpu
        };

        (&LD_E_A.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x5F));
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
    pub fn test_get_instruction_ld_hl_b() {
        let instruction = get_instruction(&0x70);
        assert_eq!(instruction.unwrap(), &LD_HL_B);
        assert_eq!(instruction.unwrap().length, 1);
        assert_eq!(instruction.unwrap().clock_cycles, 8);
    }

    #[test]
    pub fn test_ld_hl_b() {
        let mut cpu = Cpu::new();
        cpu.reg.b = 0x02;
        cpu.reg.h = 0xC0;
        cpu.reg.l = 0x00;
        let mut mmu = Memory::new();

        let expected_cpu = Cpu {
            reg: Registers { ..cpu.reg },
            ..cpu
        };

        (&LD_HL_B.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x70));
        assert_eq!(cpu, expected_cpu);
        assert_eq!(mmu.get_byte(0xC000 as usize), 0x02);
    }

    #[test]
    pub fn test_get_instruction_ld_hl_c() {
        let instruction = get_instruction(&0x71);
        assert_eq!(instruction.unwrap(), &LD_HL_C);
        assert_eq!(instruction.unwrap().length, 1);
        assert_eq!(instruction.unwrap().clock_cycles, 8);
    }

    #[test]
    pub fn test_ld_hl_c() {
        let mut cpu = Cpu::new();
        cpu.reg.c = 0x02;
        cpu.reg.h = 0xC0;
        cpu.reg.l = 0x00;
        let mut mmu = Memory::new();

        let expected_cpu = Cpu {
            reg: Registers { ..cpu.reg },
            ..cpu
        };

        (&LD_HL_C.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x71));
        assert_eq!(cpu, expected_cpu);
        assert_eq!(mmu.get_byte(0xC000 as usize), 0x02);
    }

    #[test]
    pub fn test_get_instruction_ld_hl_d() {
        let instruction = get_instruction(&0x72);
        assert_eq!(instruction.unwrap(), &LD_HL_D);
        assert_eq!(instruction.unwrap().length, 1);
        assert_eq!(instruction.unwrap().clock_cycles, 8);
    }

    #[test]
    pub fn test_ld_hl_d() {
        let mut cpu = Cpu::new();
        cpu.reg.d = 0x02;
        cpu.reg.h = 0xC0;
        cpu.reg.l = 0x00;
        let mut mmu = Memory::new();

        let expected_cpu = Cpu {
            reg: Registers { ..cpu.reg },
            ..cpu
        };

        (&LD_HL_D.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x72));
        assert_eq!(cpu, expected_cpu);
        assert_eq!(mmu.get_byte(0xC000 as usize), 0x02);
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
    pub fn test_get_instruction_ld_a_c() {
        let instruction = get_instruction(&0x79);
        assert_eq!(instruction.unwrap(), &LD_A_C);
        assert_eq!(instruction.unwrap().length, 1);
        assert_eq!(instruction.unwrap().clock_cycles, 4);
    }

    #[test]
    pub fn test_ld_a_c() {
        let mut cpu = Cpu::new();
        cpu.reg.c = 0x02;

        let expected_cpu = Cpu {
            reg: Registers { a: 0x02, ..cpu.reg },
            ..cpu
        };

        (&LD_A_C.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x79));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_get_instruction_ld_a_d() {
        let instruction = get_instruction(&0x7A);
        assert_eq!(instruction.unwrap(), &LD_A_D);
        assert_eq!(instruction.unwrap().length, 1);
        assert_eq!(instruction.unwrap().clock_cycles, 4);
    }

    #[test]
    pub fn test_ld_a_d() {
        let mut cpu = Cpu::new();
        cpu.reg.d = 0x02;

        let expected_cpu = Cpu {
            reg: Registers { a: 0x02, ..cpu.reg },
            ..cpu
        };

        (&LD_A_D.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x7A));
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
    pub fn test_ld_a_l() {
        let mut cpu = Cpu::new();
        cpu.reg.l = 0x02;

        let expected_cpu = Cpu {
            reg: Registers { a: 0x02, ..cpu.reg },
            ..cpu
        };

        (&LD_A_L.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x7D));
        assert_eq!(cpu, expected_cpu);
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
        cpu.reg.clear_all_flags();
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
    pub fn test_get_instruction_xor_c() {
        let instruction = get_instruction(&0xA9);
        assert_eq!(instruction.unwrap(), &XOR_C);
        assert_eq!(instruction.unwrap().length, 1);
        assert_eq!(instruction.unwrap().clock_cycles, 4);
    }

    #[test]
    pub fn test_xor_c() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x01;
        cpu.reg.c = 0x02;

        let expected_cpu = Cpu {
            reg: Registers {
                a: 0x03,
                f: 0x00, // ____
                ..cpu.reg
            },
            ..cpu
        };

        (&XOR_C.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0xA9));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_xor_c_zero() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x01;
        cpu.reg.c = 0x01;

        let expected_cpu = Cpu {
            reg: Registers {
                a: 0x00,
                f: 0x80, // Z___
                ..cpu.reg
            },
            ..cpu
        };

        (&XOR_C.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0xA9));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_get_instruction_xor_hl() {
        let instruction = get_instruction(&0xAE);
        assert_eq!(instruction.unwrap(), &XOR_HL);
        assert_eq!(instruction.unwrap().length, 1);
        assert_eq!(instruction.unwrap().clock_cycles, 8);
    }

    #[test]
    pub fn test_xor_hl() {
        let mut cpu = Cpu::new();
        let mut mmu: Memory = Memory::new();
        cpu.reg.a = 0x01;
        cpu.reg.h = 0xC0;
        cpu.reg.l = 0x00;
        mmu.set_byte(0xC000 as usize, 0x02);

        let expected_cpu = Cpu {
            reg: Registers {
                a: 0x03,
                f: 0x00, // ____
                ..cpu.reg
            },
            ..cpu
        };

        (&XOR_HL.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xAE));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_xor_hl_zero() {
        let mut cpu = Cpu::new();
        let mut mmu: Memory = Memory::new();
        cpu.reg.a = 0x01;
        cpu.reg.h = 0xC0;
        cpu.reg.l = 0x00;
        mmu.set_byte(0xC000 as usize, 0x01);

        let expected_cpu = Cpu {
            reg: Registers {
                a: 0x00,
                f: 0x80, // Z___
                ..cpu.reg
            },
            ..cpu
        };

        (&XOR_HL.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xAE));
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
    pub fn test_xor_a() {
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
    pub fn test_get_instruction_or_c() {
        let instruction = get_instruction(&0xB1);
        assert_eq!(instruction.unwrap(), &OR_C);
        assert_eq!(instruction.unwrap().length, 1);
        assert_eq!(instruction.unwrap().clock_cycles, 4);
    }

    #[test]
    pub fn test_or_c() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x01;
        cpu.reg.c = 0x01;

        let expected_cpu = Cpu {
            reg: Registers {
                a: 0x01,
                f: 0x00, // ____
                ..cpu.reg
            },
            ..cpu
        };

        (&OR_C.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0xB0));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_or_c_zero() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x00;
        cpu.reg.c = 0x00;

        let expected_cpu = Cpu {
            reg: Registers {
                a: 0x00,
                f: 0x80, // Z___
                ..cpu.reg
            },
            ..cpu
        };

        (&OR_C.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0xB0));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_get_instruction_or_a() {
        let instruction = get_instruction(&0xB7);
        assert_eq!(instruction.unwrap(), &OR_A);
        assert_eq!(instruction.unwrap().length, 1);
        assert_eq!(instruction.unwrap().clock_cycles, 4);
    }

    #[test]
    pub fn test_or_a() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x01;

        let expected_cpu = Cpu {
            reg: Registers {
                a: 0x01,
                f: 0x00, // ____
                ..cpu.reg
            },
            ..cpu
        };

        (&OR_A.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0xB7));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_or_a_zero() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x00;

        let expected_cpu = Cpu {
            reg: Registers {
                a: 0x00,
                f: 0x80, // Z___
                ..cpu.reg
            },
            ..cpu
        };

        (&OR_A.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0xB7));
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
    pub fn test_get_instruction_jp_nn() {
        let instruction = get_instruction(&0xC3);
        assert_eq!(instruction.unwrap(), &JP_NN);
        assert_eq!(instruction.unwrap().length, 3);
        assert_eq!(instruction.unwrap().clock_cycles, 16);
    }

    #[test]
    pub fn test_jp_nn() {
        let mut cpu = Cpu::new();
        let mut mmu: Memory = Memory::new();
        cpu.reg.pc = 0xC000;
        mmu.set_word(0xC001 as usize, 0x0302);

        let expected_cpu = Cpu {
            reg: Registers {
                pc: 0x0302,
                ..cpu.reg
            },
            ..cpu
        };

        (&JP_NN.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xC3));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_get_instruction_call_nz_nn() {
        let instruction = get_instruction(&0xC4);
        assert_eq!(instruction.unwrap(), &CALL_NZ_NN);
        assert_eq!(instruction.unwrap().length, 3);
        assert_eq!(instruction.unwrap().clock_cycles, 12);
        assert_eq!(instruction.unwrap().clock_cycles_condition.unwrap(), 24);
    }

    #[test]
    pub fn test_call_nz_nn() {
        let mut cpu = Cpu::new();
        let mut mmu: Memory = Memory::new();
        cpu.reg.pc = 0xC000;
        cpu.reg.sp = 0xFFFE;
        cpu.reg.clear_flag(Flag::Zero);
        mmu.set_word(0xC001 as usize, 0x0302);

        let expected_cpu = Cpu {
            reg: Registers {
                pc: 0x0302,
                sp: 0xFFFC,
                ..cpu.reg
            },
            ..cpu
        };

        (&CALL_NZ_NN.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xC4));
        assert_eq!(cpu, expected_cpu);
        assert_eq!(mmu.get_word(0xFFFC as usize), 0xC003);
    }

    #[test]
    pub fn test_call_nz_nn_zero() {
        let mut cpu = Cpu::new();
        let mut mmu: Memory = Memory::new();
        cpu.reg.pc = 0xC000;
        cpu.reg.sp = 0xFFFE;
        cpu.reg.set_flag(Flag::Zero);
        mmu.set_word(0xC001 as usize, 0x0302);

        let expected_cpu = Cpu {
            reg: Registers {
                pc: 0xC000,
                sp: 0xFFFE,
                ..cpu.reg
            },
            ..cpu
        };

        (&CALL_NZ_NN.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xC4));
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
    pub fn test_get_instruction_add_n() {
        let instruction = get_instruction(&0xC6);
        assert_eq!(instruction.unwrap(), &ADD_N);
        assert_eq!(instruction.unwrap().length, 2);
        assert_eq!(instruction.unwrap().clock_cycles, 8);
    }

    #[test]
    pub fn test_add_n() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.a = 0x01;
        cpu.reg.pc = 0xC000;
        cpu.reg.clear_all_flags();
        mmu.set_byte(0xC001 as usize, 0x01);

        let expected_cpu = Cpu {
            reg: Registers {
                a: 0x02,
                f: 0x00, // ____
                ..cpu.reg
            },
            ..cpu
        };

        (&ADD_N.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xC6));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_add_n_overflow() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.a = 0xFF;
        cpu.reg.pc = 0xC000;
        mmu.set_byte(0xC001 as usize, 0x01);

        let expected_cpu = Cpu {
            reg: Registers {
                a: 0x00,
                f: 0xB0, // Z_HC
                ..cpu.reg
            },
            ..cpu
        };

        (&ADD_N.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xC6));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_add_n_half_carry() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.a = 0x0F;
        cpu.reg.pc = 0xC000;
        mmu.set_byte(0xC001 as usize, 0x01);

        let expected_cpu = Cpu {
            reg: Registers {
                a: 0x10,
                f: 0x20, // _H__
                ..cpu.reg
            },
            ..cpu
        };

        (&ADD_N.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xC6));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_add_a_n_zero() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.a = 0x00;
        cpu.reg.pc = 0xC000;
        mmu.set_byte(0xC001 as usize, 0x00);

        let expected_cpu = Cpu {
            reg: Registers {
                a: 0x00,
                f: 0x80, // Z___
                ..cpu.reg
            },
            ..cpu
        };

        (&ADD_N.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xC6));
        assert_eq!(cpu, expected_cpu);
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
    pub fn test_get_instruction_adc_n() {
        let instruction = get_instruction(&0xCE);
        assert_eq!(instruction.unwrap(), &ADC_N);
        assert_eq!(instruction.unwrap().length, 2);
        assert_eq!(instruction.unwrap().clock_cycles, 8);
    }

    #[test]
    pub fn test_adc_n() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.a = 0x01;
        cpu.reg.pc = 0xC000;
        cpu.reg.set_flag(Flag::Carry);
        mmu.set_byte(0xC001 as usize, 0x01);

        let expected_cpu = Cpu {
            reg: Registers {
                a: 0x03,
                f: 0x00, // ____
                ..cpu.reg
            },
            ..cpu
        };

        (&ADC_N.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xCE));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_get_instruction_ret_nc() {
        let instruction = get_instruction(&0xD0);
        assert_eq!(instruction.unwrap(), &RET_NC);
        assert_eq!(instruction.unwrap().length, 1);
        assert_eq!(instruction.unwrap().clock_cycles, 20);
        assert_eq!(instruction.unwrap().clock_cycles_condition.unwrap(), 8);
    }

    #[test]
    pub fn test_ret_nc() {
        let mut cpu = Cpu::new();
        let mut mmu: Memory = Memory::new();
        cpu.reg.sp = 0xFFFC;
        cpu.reg.clear_flag(Flag::Carry);
        mmu.set_word(0xFFFC as usize, 0x0302);

        let expected_cpu = Cpu {
            reg: Registers {
                pc: 0x0302,
                sp: 0xFFFE,
                ..cpu.reg
            },
            ..cpu
        };

        (&RET_NC.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xD0));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_ret_nc_carry() {
        let mut cpu = Cpu::new();
        let mut mmu: Memory = Memory::new();
        cpu.reg.sp = 0xFFFC;
        cpu.reg.set_flag(Flag::Carry);
        mmu.set_word(0xFFFC as usize, 0x0302);

        let expected_cpu = Cpu {
            reg: Registers {
                pc: 0x0100,
                sp: 0xFFFC,
                ..cpu.reg
            },
            ..cpu
        };

        (&RET_NC.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xD0));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_get_instruction_pop_de() {
        let instruction = get_instruction(&0xD1);
        assert_eq!(instruction.unwrap(), &POP_DE);
        assert_eq!(instruction.unwrap().length, 1);
        assert_eq!(instruction.unwrap().clock_cycles, 12);
    }

    #[test]
    pub fn test_pop_de() {
        let mut cpu = Cpu::new();
        let mut mmu: Memory = Memory::new();
        cpu.reg.sp = 0xC000;
        mmu.set_word(0xC000 as usize, 0x0302);

        let expected_cpu = Cpu {
            reg: Registers {
                d: 0x03,
                e: 0x02,
                sp: 0xC002,
                ..cpu.reg
            },
            ..cpu
        };

        (&POP_DE.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xD1));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_get_instruction_push_de() {
        let instruction = get_instruction(&0xD5);
        assert_eq!(instruction.unwrap(), &PUSH_DE);
        assert_eq!(instruction.unwrap().length, 1);
        assert_eq!(instruction.unwrap().clock_cycles, 16);
    }

    #[test]
    pub fn test_push_de() {
        let mut cpu = Cpu::new();
        let mut mmu: Memory = Memory::new();
        cpu.reg.d = 0x03;
        cpu.reg.e = 0x02;
        cpu.reg.sp = 0xFFFE;

        let expected_cpu = Cpu {
            reg: Registers {
                sp: 0xFFFC,
                ..cpu.reg
            },
            ..cpu
        };

        (&PUSH_DE.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xD5));
        assert_eq!(cpu, expected_cpu);
        assert_eq!(mmu.get_word(0xFFFC as usize), 0x0302);
    }

    #[test]
    pub fn test_get_instruction_sub_n() {
        let instruction = get_instruction(&0xD6);
        assert_eq!(instruction.unwrap(), &SUB_N);
        assert_eq!(instruction.unwrap().length, 2);
        assert_eq!(instruction.unwrap().clock_cycles, 8);
    }

    #[test]
    pub fn test_sub_n() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.a = 0x02;
        cpu.reg.pc = 0xC000;
        mmu.set_byte(0xC001 as usize, 0x01);

        let expected_cpu = Cpu {
            reg: Registers {
                a: 0x01,
                f: 0x40, // _N__
                ..cpu.reg
            },
            ..cpu
        };

        (&SUB_N.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xD6));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_sub_n_zero() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.a = 0x01;
        cpu.reg.pc = 0xC000;
        mmu.set_byte(0xC001 as usize, 0x01);

        let expected_cpu = Cpu {
            reg: Registers {
                a: 0x00,
                f: 0xC0, // ZN__
                ..cpu.reg
            },
            ..cpu
        };

        (&SUB_N.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xD6));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_sub_n_half_carry() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.a = 0x10;
        cpu.reg.pc = 0xC000;
        mmu.set_byte(0xC001 as usize, 0x01);

        let expected_cpu = Cpu {
            reg: Registers {
                a: 0x0F,
                f: 0x60, // _NH_
                ..cpu.reg
            },
            ..cpu
        };

        (&SUB_N.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xD6));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_sub_n_underflow() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.a = 0x00;
        cpu.reg.pc = 0xC000;
        mmu.set_byte(0xC001 as usize, 0x01);

        let expected_cpu = Cpu {
            reg: Registers {
                a: 0xFF,
                f: 0x70, // _NHC
                ..cpu.reg
            },
            ..cpu
        };

        (&SUB_N.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xD6));
        assert_eq!(cpu, expected_cpu);
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
    pub fn test_get_instruction_pop_hl() {
        let instruction = get_instruction(&0xE1);
        assert_eq!(instruction.unwrap(), &POP_HL);
        assert_eq!(instruction.unwrap().length, 1);
        assert_eq!(instruction.unwrap().clock_cycles, 12);
    }

    #[test]
    pub fn test_pop_hl() {
        let mut cpu = Cpu::new();
        let mut mmu: Memory = Memory::new();
        cpu.reg.sp = 0xC000;
        mmu.set_word(0xC000 as usize, 0x0302);

        let expected_cpu = Cpu {
            reg: Registers {
                h: 0x03,
                l: 0x02,
                sp: 0xC002,
                ..cpu.reg
            },
            ..cpu
        };

        (&POP_HL.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xE1));
        assert_eq!(cpu, expected_cpu);
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
    pub fn test_get_instruction_push_hl() {
        let instruction = get_instruction(&0xE5);
        assert_eq!(instruction.unwrap(), &PUSH_HL);
        assert_eq!(instruction.unwrap().length, 1);
        assert_eq!(instruction.unwrap().clock_cycles, 16);
    }

    #[test]
    pub fn test_push_hl() {
        let mut cpu = Cpu::new();
        let mut mmu: Memory = Memory::new();
        cpu.reg.h = 0x03;
        cpu.reg.l = 0x02;
        cpu.reg.sp = 0xFFFE;

        let expected_cpu = Cpu {
            reg: Registers {
                sp: 0xFFFC,
                ..cpu.reg
            },
            ..cpu
        };

        (&PUSH_HL.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xE5));
        assert_eq!(cpu, expected_cpu);
        assert_eq!(mmu.get_word(0xFFFC as usize), 0x0302);
    }

    #[test]
    pub fn test_get_instruction_and_n() {
        let instruction = get_instruction(&0xE6);
        assert_eq!(instruction.unwrap(), &AND_N);
        assert_eq!(instruction.unwrap().length, 2);
        assert_eq!(instruction.unwrap().clock_cycles, 8);
    }

    #[test]
    pub fn test_and_n() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.a = 0x01;
        cpu.reg.pc = 0xC000;
        mmu.set_byte(0xC001 as usize, 0x01);

        let expected_cpu = Cpu {
            reg: Registers {
                a: 0x01,
                f: 0x20, // __H_
                ..cpu.reg
            },
            ..cpu
        };

        (&AND_N.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0xE6));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_and_n_zero() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.a = 0x00;
        cpu.reg.pc = 0xC000;
        mmu.set_byte(0xC001 as usize, 0x01);

        let expected_cpu = Cpu {
            reg: Registers {
                a: 0x00,
                f: 0xA0, // Z_H_
                ..cpu.reg
            },
            ..cpu
        };

        (&AND_N.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0xE6));
        assert_eq!(cpu, expected_cpu);
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
    pub fn test_get_instruction_xor_n() {
        let instruction = get_instruction(&0xEE);
        assert_eq!(instruction.unwrap(), &XOR_N);
        assert_eq!(instruction.unwrap().length, 2);
        assert_eq!(instruction.unwrap().clock_cycles, 8);
    }

    #[test]
    pub fn test_xor_n() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.a = 0x01;
        cpu.reg.pc = 0xC000;
        mmu.set_byte(0xC001 as usize, 0x02);

        let expected_cpu = Cpu {
            reg: Registers {
                a: 0x03,
                f: 0x00, // ____
                ..cpu.reg
            },
            ..cpu
        };

        (&XOR_N.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xEE));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_xor_n_zero() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.a = 0x01;
        cpu.reg.pc = 0xC000;
        mmu.set_byte(0xC001 as usize, 0x01);

        let expected_cpu = Cpu {
            reg: Registers {
                a: 0x00,
                f: 0x80, // Z___
                ..cpu.reg
            },
            ..cpu
        };

        (&XOR_N.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xEE));
        assert_eq!(cpu, expected_cpu);
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
    pub fn test_get_instruction_pop_af() {
        let instruction = get_instruction(&0xF1);
        assert_eq!(instruction.unwrap(), &POP_AF);
        assert_eq!(instruction.unwrap().length, 1);
        assert_eq!(instruction.unwrap().clock_cycles, 12);
    }

    #[test]
    pub fn test_pop_af() {
        let mut cpu = Cpu::new();
        let mut mmu: Memory = Memory::new();
        cpu.reg.sp = 0xC000;
        mmu.set_word(0xC000 as usize, 0x0302);

        let expected_cpu = Cpu {
            reg: Registers {
                a: 0x03,
                f: 0x02,
                sp: 0xC002,
                ..cpu.reg
            },
            ..cpu
        };

        (&POP_AF.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xF1));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_get_instruction_di() {
        let instruction = get_instruction(&0xF3);
        assert_eq!(instruction.unwrap(), &DI);
        assert_eq!(instruction.unwrap().length, 1);
        assert_eq!(instruction.unwrap().clock_cycles, 4);
    }

    #[test]
    pub fn test_di() {
        let mut cpu = Cpu::new();
        cpu.interrupts_enabled = true;

        let expected_cpu = Cpu {
            reg: Registers { ..cpu.reg },
            interrupts_enabled: false,
            ..cpu
        };

        (&DI.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0xF3));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_get_instruction_push_af() {
        let instruction = get_instruction(&0xF5);
        assert_eq!(instruction.unwrap(), &PUSH_AF);
        assert_eq!(instruction.unwrap().length, 1);
        assert_eq!(instruction.unwrap().clock_cycles, 16);
    }

    #[test]
    pub fn test_push_af() {
        let mut cpu = Cpu::new();
        let mut mmu: Memory = Memory::new();
        cpu.reg.a = 0x03;
        cpu.reg.f = 0x02;
        cpu.reg.sp = 0xFFFE;

        let expected_cpu = Cpu {
            reg: Registers {
                sp: 0xFFFC,
                ..cpu.reg
            },
            ..cpu
        };

        (&PUSH_AF.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xF5));
        assert_eq!(cpu, expected_cpu);
        assert_eq!(mmu.get_word(0xFFFC as usize), 0x0302);
    }

    #[test]
    pub fn test_get_instruction_ld_a_nn() {
        let instruction = get_instruction(&0xFA);
        assert_eq!(instruction.unwrap(), &LD_A_NN);
        assert_eq!(instruction.unwrap().length, 3);
        assert_eq!(instruction.unwrap().clock_cycles, 16);
    }

    #[test]
    pub fn test_ld_a_nn() {
        let mut cpu = Cpu::new();
        let mut mmu: Memory = Memory::new();
        cpu.reg.pc = 0xC000;
        mmu.set_word(0xC001 as usize, 0xC003);
        mmu.set_byte(0xC003 as usize, 0x01);

        let expected_cpu = Cpu {
            reg: Registers { a: 0x01, ..cpu.reg },
            ..cpu
        };

        (&LD_A_NN.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xFA));
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
