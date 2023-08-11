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

/// 0x08 - LD (nn), SP
const LD_NN_SP: Instruction = Instruction {
    length: 3,
    clock_cycles: 20,
    clock_cycles_condition: None,
    description: "LD (nn) SP",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        let address = functions::get_op16(cpu, mmu);
        mmu.set_word(address, cpu.reg.sp);
        InstructionType::ActionTaken
    },
};

/// 0x09 - ADD HL, BC
const ADD_HL_BC: Instruction = Instruction {
    length: 1,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "ADD HL BC",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        functions::add16(cpu, cpu.reg.bc());
        InstructionType::ActionTaken
    },
};

/// 0x0B - DEC BC
const DEC_BC: Instruction = Instruction {
    length: 1,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "DEC BC",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.set_bc(cpu.reg.bc().wrapping_sub(1));
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

/// 0x19 - ADD HL, DE
const ADD_HL_DE: Instruction = Instruction {
    length: 1,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "ADD HL DE",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        functions::add16(cpu, cpu.reg.de());
        InstructionType::ActionTaken
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

/// 0x1B - DEC DE
const DEC_DE: Instruction = Instruction {
    length: 1,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "DEC DE",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.set_de(cpu.reg.de().wrapping_sub(1));
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

/// 0x27 - DAA
const DAA: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "DAA",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        functions::daa(cpu);
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

/// 0x29 - ADD HL, HL
const ADD_HL_HL: Instruction = Instruction {
    length: 1,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "ADD HL HL",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        (functions::add16(cpu, cpu.reg.hl()));
        InstructionType::ActionTaken
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

/// 0x2B - DEC HL
const DEC_HL: Instruction = Instruction {
    length: 1,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "DEC HL",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.set_hl(cpu.reg.hl().wrapping_sub(1));
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

/// 0x33 - INC SP
const INC_SP: Instruction = Instruction {
    length: 1,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "INC SP",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.sp = cpu.reg.sp.wrapping_add(1);
        InstructionType::ActionTaken
    },
};

/// 0x35 - DEC (HL)
const DEC_HL_PTR: Instruction = Instruction {
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

/// 0x36 - LD (HL), n
const LD_HL_N: Instruction = Instruction {
    length: 2,
    clock_cycles: 12,
    clock_cycles_condition: None,
    description: "LD (HL) n",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        mmu.set_byte(cpu.reg.hl(), functions::get_op8(cpu, mmu, 1));
        InstructionType::ActionTaken
    },
};

/// 0x38 - JR C, n
const JR_C_N: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: Some(12),
    description: "JR C n",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        if cpu.reg.check_flag(Flag::Carry) {
            functions::jr(cpu, mmu);
            return InstructionType::Jumped;
        }
        InstructionType::None
    },
};

/// 0x39 - ADD HL, SP
const ADD_HL_SP: Instruction = Instruction {
    length: 1,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "ADD HL SP",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        functions::add16(cpu, cpu.reg.sp);
        InstructionType::ActionTaken
    },
};

/// 0x3B - DEC SP
const DEC_SP: Instruction = Instruction {
    length: 1,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "DEC SP",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.sp = cpu.reg.sp.wrapping_sub(1);
        InstructionType::ActionTaken
    },
};

/// 0x3C - INC A
const INC_A: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "INC A",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.a = functions::inc(cpu, cpu.reg.a);
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

/// 0x40 - LD B, B
const LD_B_B: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "LD B B",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.b = cpu.reg.b;
        InstructionType::ActionTaken
    },
};

/// 0x41 - LD B, C
const LD_B_C: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "LD B C",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.b = cpu.reg.c;
        InstructionType::ActionTaken
    },
};

/// 0x42 - LD B, D
const LD_B_D: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "LD B D",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.b = cpu.reg.d;
        InstructionType::ActionTaken
    },
};

/// 0x43 - LD B, E
const LD_B_E: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "LD B E",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.b = cpu.reg.e;
        InstructionType::ActionTaken
    },
};

/// 0x44 - LD B, H
const LD_B_H: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "LD B H",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.b = cpu.reg.h;
        InstructionType::ActionTaken
    },
};

/// 0x45 - LD B, L
const LD_B_L: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "LD B L",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.b = cpu.reg.l;
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

/// 0x48 - LD C, B
const LD_C_B: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "LD C B",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.c = cpu.reg.b;
        InstructionType::ActionTaken
    },
};

/// 0x49 - LD C, C
const LD_C_C: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "LD C C",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.c = cpu.reg.c;
        InstructionType::ActionTaken
    },
};

/// 0x4A - LD C, D
const LD_C_D: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "LD C D",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.c = cpu.reg.d;
        InstructionType::ActionTaken
    },
};

/// 0x 4B - LD C, E
const LD_C_E: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "LD C E",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.c = cpu.reg.e;
        InstructionType::ActionTaken
    },
};

/// 0x4C - LD C, H
const LD_C_H: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "LD C H",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.c = cpu.reg.h;
        InstructionType::ActionTaken
    },
};

/// 0x4D - LD C, L
const LD_C_L: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "LD C L",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.c = cpu.reg.l;
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

/// 0x50 - LD D, B
const LD_D_B: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "LD D B",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.d = cpu.reg.b;
        InstructionType::ActionTaken
    },
};

/// 0x51 - LD D, C
const LD_D_C: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "LD D C",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.d = cpu.reg.c;
        InstructionType::ActionTaken
    },
};

/// 0x52 - LD D, D
const LD_D_D: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "LD D D",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.d = cpu.reg.d;
        InstructionType::ActionTaken
    },
};

/// 0x53 - LD D, E
const LD_D_E: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "LD D E",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.d = cpu.reg.e;
        InstructionType::ActionTaken
    },
};

/// 0x54 - LD D, H
const LD_D_H: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "LD D H",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.d = cpu.reg.h;
        InstructionType::ActionTaken
    },
};

/// 0x55 - LD D, L
const LD_D_L: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "LD D L",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.d = cpu.reg.l;
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

/// 0x58 - LD E, B
const LD_E_B: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "LD E B",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.e = cpu.reg.b;
        InstructionType::ActionTaken
    },
};

/// 0x59 - LD E, C
const LD_E_C: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "LD E C",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.e = cpu.reg.c;
        InstructionType::ActionTaken
    },
};

/// 0x5A - LD E, D
const LD_E_D: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "LD E D",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.e = cpu.reg.d;
        InstructionType::ActionTaken
    },
};

/// 0x5B - LD E, E
const LD_E_E: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "LD E E",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.e = cpu.reg.e;
        InstructionType::ActionTaken
    },
};

/// 0x5C - LD E, H
const LD_E_H: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "LD E H",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.e = cpu.reg.h;
        InstructionType::ActionTaken
    },
};

/// 0x5D - LD E, L
const LD_E_L: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "LD E L",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.e = cpu.reg.l;
        InstructionType::ActionTaken
    },
};

/// 0x5E - LD E, (HL)
const LD_E_HL: Instruction = Instruction {
    length: 1,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "LD E (HL)",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        cpu.reg.e = functions::get_hl(cpu, mmu);
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

/// 0x60 - LD H, B
const LD_H_B: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "LD H B",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.h = cpu.reg.b;
        InstructionType::ActionTaken
    },
};

/// 0x61 - LD H, C
const LD_H_C: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "LD H C",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.h = cpu.reg.c;
        InstructionType::ActionTaken
    },
};

/// 0x62 - LD H, D
const LD_H_D: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "LD H D",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.h = cpu.reg.d;
        InstructionType::ActionTaken
    },
};

/// 0x63 - LD H, E
const LD_H_E: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "LD H E",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.h = cpu.reg.e;
        InstructionType::ActionTaken
    },
};

/// 0x64 - LD H, H
const LD_H_H: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "LD H H",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.h = cpu.reg.h;
        InstructionType::ActionTaken
    },
};

/// 0x65 - LD H, L
const LD_H_L: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "LD H L",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.h = cpu.reg.l;
        InstructionType::ActionTaken
    },
};

/// 0x66 - LD H, (HL)
const LD_H_HL: Instruction = Instruction {
    length: 1,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "LD H (HL)",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        cpu.reg.h = functions::get_hl(cpu, mmu);
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

/// 0x68 - LD L, B
const LD_L_B: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "LD L B",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.l = cpu.reg.b;
        InstructionType::ActionTaken
    },
};

/// 0x69 - LD L, C
const LD_L_C: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "LD L C",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.l = cpu.reg.c;
        InstructionType::ActionTaken
    },
};

/// 0x6A - LD L, D
const LD_L_D: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "LD L D",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.l = cpu.reg.d;
        InstructionType::ActionTaken
    },
};

/// 0x6B - LD L, E
const LD_L_E: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "LD L E",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.l = cpu.reg.e;
        InstructionType::ActionTaken
    },
};

/// 0x6C - LD L, H
const LD_L_H: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "LD L H",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.l = cpu.reg.h;
        InstructionType::ActionTaken
    },
};

/// 0x6D - LD L, L
const LD_L_L: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "LD L L",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.l = cpu.reg.l;
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

/// 0x6F - LD L, A
const LD_L_A: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "LD L, A",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.l = cpu.reg.a;
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

/// 0x73 - LD (HL), E
const LD_HL_E: Instruction = Instruction {
    length: 1,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "LD (HL) E",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        functions::set_hl(cpu, mmu, cpu.reg.e);
        InstructionType::ActionTaken
    },
};

/// 0x74 - LD (HL), H
const LD_HL_H: Instruction = Instruction {
    length: 1,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "LD (HL) H",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        functions::set_hl(cpu, mmu, cpu.reg.h);
        InstructionType::ActionTaken
    },
};

/// 0x75 - LD (HL), L
const LD_HL_L: Instruction = Instruction {
    length: 1,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "LD (HL) L",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        functions::set_hl(cpu, mmu, cpu.reg.l);
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

/// 0x7E - LD A, (HL)
const LD_A_HL: Instruction = Instruction {
    length: 1,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "LD A, (HL)",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        cpu.reg.a = functions::get_hl(cpu, mmu);
        InstructionType::ActionTaken
    },
};

/// 0x7F - LD A, A
const LD_A_A: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "LD A, A",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.a = cpu.reg.a;
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

/// 0xAD - XOR L
const XOR_L: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "XOR L",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        functions::xor_bytes(cpu, cpu.reg.a, cpu.reg.l);
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

/// 0xB0 - OR B
const OR_B: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "OR B",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        functions::or(cpu, cpu.reg.b);
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
        functions::or(cpu, cpu.reg.c);
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

/// 0xB8 - CP B
const CP_B: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "CP B",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        functions::cp(cpu, cpu.reg.b);
        InstructionType::ActionTaken
    },
};

/// 0xB9 - CP C
const CP_C: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "CP C",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        functions::cp(cpu, cpu.reg.c);
        InstructionType::ActionTaken
    },
};

/// 0xBA - CP D
const CP_D: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "CP D",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        functions::cp(cpu, cpu.reg.d);
        InstructionType::ActionTaken
    },
};

/// 0xBB - CP E
const CP_E: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "CP E",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        functions::cp(cpu, cpu.reg.e);
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

/// 0xC0 - RET NZ
const RET_NZ: Instruction = Instruction {
    length: 1,
    clock_cycles: 8,
    clock_cycles_condition: Some(20),
    description: "RET NZ",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        if !cpu.reg.check_flag(Flag::Zero) {
            cpu.reg.pc = functions::pop(cpu, mmu);
            return InstructionType::Jumped;
        }
        InstructionType::None
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

/// 0xC2 - JP NZ, nn
const JP_NZ_NN: Instruction = Instruction {
    length: 3,
    clock_cycles: 12,
    clock_cycles_condition: Some(16),
    description: "JP NZ, nn",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        if !cpu.reg.check_flag(Flag::Zero) {
            cpu.reg.pc = functions::get_op16(cpu, mmu);
            return InstructionType::Jumped;
        }
        InstructionType::None
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

/// 0xC7 - RST 00
const RST_00: Instruction = Instruction {
    length: 1,
    clock_cycles: 16,
    clock_cycles_condition: None,
    description: "RST 00",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        functions::rst(cpu, mmu, 0x00);
        InstructionType::Jumped
    },
};

/// 0xC8 - RET Z
const RET_Z: Instruction = Instruction {
    length: 1,
    clock_cycles: 8,
    clock_cycles_condition: Some(20),
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

/// 0xCA - JP Z, nn
const JP_Z_NN: Instruction = Instruction {
    length: 3,
    clock_cycles: 12,
    clock_cycles_condition: Some(16),
    description: "JP Z, nn",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        if cpu.reg.check_flag(Flag::Zero) {
            cpu.reg.pc = functions::get_op16(cpu, mmu);
            return InstructionType::Jumped;
        }
        InstructionType::None
    },
};

/// 0xCC CALL Z, nn
const CALL_Z_NN: Instruction = Instruction {
    length: 3,
    clock_cycles: 12,
    clock_cycles_condition: Some(24),
    description: "CALL Z, nn",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        if cpu.reg.check_flag(Flag::Zero) {
            functions::call(cpu, mmu);
            return InstructionType::Jumped;
        }
        InstructionType::None
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

/// 0xCF - RST 08
const RST_08: Instruction = Instruction {
    length: 1,
    clock_cycles: 16,
    clock_cycles_condition: None,
    description: "RST 08",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        functions::rst(cpu, mmu, 0x08);
        InstructionType::Jumped
    },
};

/// 0xD0 - RET NC
const RET_NC: Instruction = Instruction {
    length: 1,
    clock_cycles: 8,
    clock_cycles_condition: Some(20),
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

/// 0xD2 - JP NC, nn
const JP_NC_NN: Instruction = Instruction {
    length: 3,
    clock_cycles: 12,
    clock_cycles_condition: Some(16),
    description: "JP NC, nn",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        if !cpu.reg.check_flag(Flag::Carry) {
            cpu.reg.pc = functions::get_op16(cpu, mmu);
            return InstructionType::Jumped;
        }
        InstructionType::None
    },
};

/// 0xD4 - CALL NC, nn
const CALL_NC_NN: Instruction = Instruction {
    length: 3,
    clock_cycles: 12,
    clock_cycles_condition: Some(24),
    description: "CALL NC, nn",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        if !cpu.reg.check_flag(Flag::Carry) {
            functions::call(cpu, mmu);
            return InstructionType::Jumped;
        }
        InstructionType::None
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

/// 0xD7 - RST 10
const RST_10: Instruction = Instruction {
    length: 1,
    clock_cycles: 16,
    clock_cycles_condition: None,
    description: "RST 10",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        functions::rst(cpu, mmu, 0x10);
        InstructionType::Jumped
    },
};

/// 0xD8 - RET C
const RET_C: Instruction = Instruction {
    length: 1,
    clock_cycles: 8,
    clock_cycles_condition: Some(20),
    description: "RET C",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        if cpu.reg.check_flag(Flag::Carry) {
            cpu.reg.pc = functions::pop(cpu, mmu);
            return InstructionType::Jumped;
        }
        InstructionType::None
    },
};

/// 0xD9 - RETI
const RETI: Instruction = Instruction {
    length: 1,
    clock_cycles: 16,
    clock_cycles_condition: None,
    description: "RETI",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        cpu.reg.pc = functions::pop(cpu, mmu);
        cpu.ime = true;
        InstructionType::Jumped
    },
};

/// 0xDA - JP C, nn
const JP_C_NN: Instruction = Instruction {
    length: 3,
    clock_cycles: 12,
    clock_cycles_condition: Some(16),
    description: "JP C, nn",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        if cpu.reg.check_flag(Flag::Carry) {
            cpu.reg.pc = functions::get_op16(cpu, mmu);
            return InstructionType::Jumped;
        }
        InstructionType::None
    },
};

/// 0xDC - CALL C, nn
const CALL_C_NN: Instruction = Instruction {
    length: 3,
    clock_cycles: 12,
    clock_cycles_condition: Some(24),
    description: "CALL C, nn",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        if cpu.reg.check_flag(Flag::Carry) {
            functions::call(cpu, mmu);
            return InstructionType::Jumped;
        }
        InstructionType::None
    },
};

/// 0xDE - SBC n
const SBC_N: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SBC n",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        functions::sbc(cpu, functions::get_op8(cpu, mmu, 1));
        InstructionType::ActionTaken
    },
};

/// 0xDF - RST 18
const RST_18: Instruction = Instruction {
    length: 1,
    clock_cycles: 16,
    clock_cycles_condition: None,
    description: "RST 18",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        functions::rst(cpu, mmu, 0x18);
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

/// 0xE7 - RST 20
const RST_20: Instruction = Instruction {
    length: 1,
    clock_cycles: 16,
    clock_cycles_condition: None,
    description: "RST 20",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        functions::rst(cpu, mmu, 0x20);
        InstructionType::Jumped
    },
};

/// 0xE8 - ADD SP, n
const ADD_SP_N: Instruction = Instruction {
    length: 2,
    clock_cycles: 16,
    clock_cycles_condition: None,
    description: "ADD SP, n",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        cpu.reg.sp = functions::add_sp(cpu, mmu);
        InstructionType::ActionTaken
    },
};

/// 0xE9 - JP HL
const JP_HL: Instruction = Instruction {
    length: 1,
    clock_cycles: 4,
    clock_cycles_condition: None,
    description: "JP HL",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.pc = cpu.reg.hl();
        InstructionType::Jumped
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

/// 0xEF - RST 28
const RST_28: Instruction = Instruction {
    length: 1,
    clock_cycles: 16,
    clock_cycles_condition: None,
    description: "RST 28",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        functions::rst(cpu, mmu, 0x28);
        InstructionType::Jumped
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

/// 0xF2 - LDH A, (C)
const LDH_A_C: Instruction = Instruction {
    length: 1,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "LDH A (C)",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        cpu.reg.a = mmu.get_byte(0xFF00 + cpu.reg.c as u16);
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
        cpu.ime = false;
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

/// 0xF6 - OR n
const OR_N: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "OR n",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        functions::or(cpu, functions::get_op8(cpu, mmu, 1));
        InstructionType::ActionTaken
    },
};

/// 0xF7 - RST 30
const RST_30: Instruction = Instruction {
    length: 1,
    clock_cycles: 16,
    clock_cycles_condition: None,
    description: "RST 30",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        functions::rst(cpu, mmu, 0x30);
        InstructionType::Jumped
    },
};

/// 0xF8 - LD HL, SP + n
const LD_HL_SP_N: Instruction = Instruction {
    length: 2,
    clock_cycles: 12,
    clock_cycles_condition: None,
    description: "LD HL, SP + n",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        let res = functions::add_sp(cpu, mmu);
        cpu.reg.set_hl(res);
        InstructionType::ActionTaken
    },
};

/// 0xF9 - LD SP, HL
const LD_SP_HL: Instruction = Instruction {
    length: 1,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "LD SP, HL",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.sp = cpu.reg.hl();
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

/// 0xFE - CP n
const CP_N: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "CP A, n",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        functions::cp(cpu, functions::get_op8(cpu, mmu, 1));
        InstructionType::ActionTaken
    },
};

/// 0xFF - RST 38
const RST_38: Instruction = Instruction {
    length: 1,
    clock_cycles: 16,
    clock_cycles_condition: None,
    description: "RST 38",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        functions::rst(cpu, mmu, 0x38);
        InstructionType::Jumped
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
        0x08 => Some(&LD_NN_SP),
        0x09 => Some(&ADD_HL_BC),
        0x0B => Some(&DEC_BC),
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
        0x19 => Some(&ADD_HL_DE),
        0x1A => Some(&LD_A_DE),
        0x1B => Some(&DEC_DE),
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
        0x27 => Some(&DAA),
        0x28 => Some(&JR_Z_N),
        0x29 => Some(&ADD_HL_HL),
        0x2A => Some(&LD_A_HL_INC),
        0x2B => Some(&DEC_HL),
        0x2C => Some(&INC_L),
        0x2D => Some(&DEC_L),
        0x2E => Some(&LD_L_N),

        0x30 => Some(&JR_NC_N),
        0x31 => Some(&LD_SP_NN),
        0x32 => Some(&LD_HL_DEC_A),
        0x33 => Some(&INC_SP),
        0x35 => Some(&DEC_HL_PTR),
        0x36 => Some(&LD_HL_N),
        0x38 => Some(&JR_C_N),
        0x39 => Some(&ADD_HL_SP),
        0x3B => Some(&DEC_SP),
        0x3C => Some(&INC_A),
        0x3D => Some(&DEC_A),
        0x3E => Some(&LD_A_N),

        0x40 => Some(&LD_B_B),
        0x41 => Some(&LD_B_C),
        0x42 => Some(&LD_B_D),
        0x43 => Some(&LD_B_E),
        0x44 => Some(&LD_B_H),
        0x45 => Some(&LD_B_L),
        0x46 => Some(&LD_B_HL),
        0x47 => Some(&LD_B_A),
        0x48 => Some(&LD_C_B),
        0x49 => Some(&LD_C_C),
        0x4A => Some(&LD_C_D),
        0x4B => Some(&LD_C_E),
        0x4C => Some(&LD_C_H),
        0x4D => Some(&LD_C_L),
        0x4E => Some(&LD_C_HL),
        0x4F => Some(&LD_C_A),

        0x50 => Some(&LD_D_B),
        0x51 => Some(&LD_D_C),
        0x52 => Some(&LD_D_D),
        0x53 => Some(&LD_D_E),
        0x54 => Some(&LD_D_H),
        0x55 => Some(&LD_D_L),
        0x56 => Some(&LD_D_HL),
        0x57 => Some(&LD_D_A),
        0x58 => Some(&LD_E_B),
        0x59 => Some(&LD_E_C),
        0x5A => Some(&LD_E_D),
        0x5B => Some(&LD_E_E),
        0x5C => Some(&LD_E_H),
        0x5D => Some(&LD_E_L),
        0x5E => Some(&LD_E_HL),
        0x5F => Some(&LD_E_A),

        0x60 => Some(&LD_H_B),
        0x61 => Some(&LD_H_C),
        0x62 => Some(&LD_H_D),
        0x63 => Some(&LD_H_E),
        0x64 => Some(&LD_H_H),
        0x65 => Some(&LD_H_L),
        0x66 => Some(&LD_H_HL),
        0x67 => Some(&LD_H_A),
        0x68 => Some(&LD_L_B),
        0x69 => Some(&LD_L_C),
        0x6A => Some(&LD_L_D),
        0x6B => Some(&LD_L_E),
        0x6C => Some(&LD_L_H),
        0x6D => Some(&LD_L_L),
        0x6E => Some(&LD_L_HL),
        0x6F => Some(&LD_L_A),

        0x70 => Some(&LD_HL_B),
        0x71 => Some(&LD_HL_C),
        0x72 => Some(&LD_HL_D),
        0x73 => Some(&LD_HL_E),
        0x74 => Some(&LD_HL_H),
        0x75 => Some(&LD_HL_L),
        0x77 => Some(&LD_HL_A),
        0x78 => Some(&LD_A_B),
        0x79 => Some(&LD_A_C),
        0x7A => Some(&LD_A_D),
        0x7B => Some(&LD_A_E),
        0x7C => Some(&LD_A_H),
        0x7D => Some(&LD_A_L),
        0x7E => Some(&LD_A_HL),
        0x7F => Some(&LD_A_A),

        0x86 => Some(&ADD_HL),

        0x90 => Some(&SUB_B),

        0xA9 => Some(&XOR_C),
        0xAD => Some(&XOR_L),
        0xAE => Some(&XOR_HL),
        0xAF => Some(&XOR_A),

        0xB0 => Some(&OR_B),
        0xB1 => Some(&OR_C),
        0xB6 => Some(&OR_HL),
        0xB7 => Some(&OR_A),
        0xB8 => Some(&CP_B),
        0xB9 => Some(&CP_C),
        0xBA => Some(&CP_D),
        0xBB => Some(&CP_E),
        0xBE => Some(&CP_HL),

        0xC0 => Some(&RET_NZ),
        0xC1 => Some(&POP_BC),
        0xC2 => Some(&JP_NZ_NN),
        0xC3 => Some(&JP_NN),
        0xC4 => Some(&CALL_NZ_NN),
        0xC5 => Some(&PUSH_BC),
        0xC6 => Some(&ADD_N),
        0xC7 => Some(&RST_00),
        0xC8 => Some(&RET_Z),
        0xC9 => Some(&RET),
        0xCA => Some(&JP_Z_NN),
        0xCC => Some(&CALL_Z_NN),
        0xCD => Some(&CALL_NN),
        0xCE => Some(&ADC_N),
        0xCF => Some(&RST_08),

        0xD0 => Some(&RET_NC),
        0xD1 => Some(&POP_DE),
        0xD2 => Some(&JP_NC_NN),
        0xD4 => Some(&CALL_NC_NN),
        0xD5 => Some(&PUSH_DE),
        0xD6 => Some(&SUB_N),
        0xD7 => Some(&RST_10),
        0xD8 => Some(&RET_C),
        0xD9 => Some(&RETI),
        0xDA => Some(&JP_C_NN),
        0xDC => Some(&CALL_C_NN),
        0xDE => Some(&SBC_N),
        0xDF => Some(&RST_18),

        0xE0 => Some(&LDH_N_A),
        0xE1 => Some(&POP_HL),
        0xE2 => Some(&LDH_C_A),
        0xE5 => Some(&PUSH_HL),
        0xE6 => Some(&AND_N),
        0xE7 => Some(&RST_20),
        0xE8 => Some(&ADD_SP_N),
        0xE9 => Some(&JP_HL),
        0xEA => Some(&LD_NN_A),
        0xEE => Some(&XOR_N),
        0xEF => Some(&RST_28),

        0xF0 => Some(&LDH_A_N),
        0xF1 => Some(&POP_AF),
        0xF2 => Some(&LDH_A_C),
        0xF3 => Some(&DI),
        0xF5 => Some(&PUSH_AF),
        0xF6 => Some(&OR_N),
        0xF7 => Some(&RST_30),
        0xF8 => Some(&LD_HL_SP_N),
        0xF9 => Some(&LD_SP_HL),
        0xFA => Some(&LD_A_NN),
        0xFE => Some(&CP_N),
        0xFF => Some(&RST_38),

        _ => None,
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn test_get_instruction_nop() {
        let instruction = get_instruction(&0x00).unwrap();
        assert_eq!(instruction, &NOP);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_nop() {
        let mut cpu = Cpu::new();
        let expected_cpu = cpu.clone();
        (&NOP.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x00));
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    pub fn test_get_instruction_ld_bc_nn() {
        let instruction = get_instruction(&0x01).unwrap();
        assert_eq!(instruction, &LD_BC_NN);
        assert_eq!(instruction.length, 3);
        assert_eq!(instruction.clock_cycles, 12);
    }

    #[test]
    pub fn test_ld_bc_nn() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.pc = 0xC000;
        mmu.set_word(0xC001 as usize, 0x0001);
        (&LD_BC_NN.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x01));
        assert_eq!(cpu.reg.bc(), 0x0001);
    }

    #[test]
    pub fn test_get_instruction_inc_bc() {
        let instruction = get_instruction(&0x03).unwrap();
        assert_eq!(instruction, &INC_BC);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_inc_bc() {
        let mut cpu = Cpu::new();
        cpu.reg.set_bc(0x0000);
        (&INC_BC.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x03));
        assert_eq!(cpu.reg.bc(), 0x0001);
    }

    #[test]
    pub fn test_get_instruction_inc_b() {
        let instruction = get_instruction(&0x04).unwrap();
        assert_eq!(instruction, &INC_B);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_inc_b() {
        let mut cpu = Cpu::new();
        cpu.reg.b = 0x00;
        (&INC_B.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x04));
        assert_eq!(cpu.reg.b, 0x01);
    }

    #[test]
    pub fn test_get_instruction_dec_b() {
        let instruction = get_instruction(&0x05).unwrap();
        assert_eq!(instruction, &DEC_B);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_dec_b() {
        let mut cpu = Cpu::new();
        cpu.reg.b = 0x01;
        (&DEC_B.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x05));
        assert_eq!(cpu.reg.b, 0x00);
    }

    #[test]
    pub fn test_get_instruction_ld_b_n() {
        let instruction = get_instruction(&0x06).unwrap();
        assert_eq!(instruction, &LD_B_N);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_ld_b_n() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.pc = 0xC000;
        mmu.set_byte(0xC001 as usize, 0x01);
        (&LD_B_N.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x06));
        assert_eq!(cpu.reg.b, 0x01);
    }

    #[test]
    pub fn test_get_instruction_ld_nn_sp() {
        let instruction = get_instruction(&0x08).unwrap();
        assert_eq!(instruction, &LD_NN_SP);
        assert_eq!(instruction.length, 3);
        assert_eq!(instruction.clock_cycles, 20);
    }

    #[test]
    pub fn test_ld_nn_sp() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.sp = 0x0000;
        cpu.reg.pc = 0xC000;
        mmu.set_word(0xC001 as usize, 0xC000);
        (&LD_NN_SP.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x08));
        assert_eq!(mmu.get_word(0xC000 as usize), 0x0000);
    }

    #[test]
    pub fn test_get_instruction_add_hl_bc() {
        let instruction = get_instruction(&0x09).unwrap();
        assert_eq!(instruction, &ADD_HL_BC);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_add_hl_bc() {
        let mut cpu = Cpu::new();
        cpu.reg.set_hl(0x0000);
        cpu.reg.set_bc(0x0001);
        (&ADD_HL_BC.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x09));
        assert_eq!(cpu.reg.hl(), 0x0001);
    }

    #[test]
    pub fn test_get_instruction_dec_bc() {
        let instruction = get_instruction(&0x0B).unwrap();
        assert_eq!(instruction, &DEC_BC);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_dec_bc() {
        let mut cpu = Cpu::new();
        cpu.reg.set_bc(0x0001);
        (&DEC_BC.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x0B));
        assert_eq!(cpu.reg.bc(), 0x0000);
    }

    #[test]
    pub fn test_dec_bc_underflow() {
        let mut cpu = Cpu::new();
        cpu.reg.set_bc(0x0000);
        (&DEC_BC.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x0B));
        assert_eq!(cpu.reg.bc(), 0xFFFF);
    }

    #[test]
    pub fn test_get_instruction_inc_c() {
        let instruction = get_instruction(&0x0C).unwrap();
        assert_eq!(instruction, &INC_C);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_inc_c() {
        let mut cpu = Cpu::new();
        cpu.reg.c = 0x00;
        (&INC_C.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x0C));
        assert_eq!(cpu.reg.c, 0x01);
    }

    #[test]
    pub fn test_get_instruction_dec_c() {
        let instruction = get_instruction(&0x0D).unwrap();
        assert_eq!(instruction, &DEC_C);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_dec_c() {
        let mut cpu = Cpu::new();
        cpu.reg.c = 0x01;
        (&DEC_C.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x0D));
        assert_eq!(cpu.reg.c, 0x00);
    }

    #[test]
    pub fn test_get_instruction_ld_c_n() {
        let instruction = get_instruction(&0x0E).unwrap();
        assert_eq!(instruction, &LD_C_N);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_ld_c_n() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.pc = 0xC000;
        mmu.set_byte(0xC001 as usize, 0x01);
        (&LD_C_N.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x0E));
        assert_eq!(cpu.reg.c, 0x01);
    }

    #[test]
    pub fn test_get_instruction_ld_de_nn() {
        let instruction = get_instruction(&0x11).unwrap();
        assert_eq!(instruction, &LD_DE_NN);
        assert_eq!(instruction.length, 3);
        assert_eq!(instruction.clock_cycles, 12);
    }

    #[test]
    pub fn test_ld_de_nn() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.pc = 0xC000;
        mmu.set_word(0xC001 as usize, 0x0001);
        (&LD_DE_NN.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x11));
        assert_eq!(cpu.reg.de(), 0x0001);
    }

    #[test]
    pub fn test_get_instruction_ld_de_a() {
        let instruction = get_instruction(&0x12).unwrap();
        assert_eq!(instruction, &LD_DE_A);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_ld_de_a() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.a = 0x01;
        cpu.reg.set_de(0xC000);
        (&LD_DE_A.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x12));
        assert_eq!(mmu.get_byte(0xC000 as usize), 0x01);
    }

    #[test]
    pub fn test_get_instruction_inc_de() {
        let instruction = get_instruction(&0x13).unwrap();
        assert_eq!(instruction, &INC_DE);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_inc_de() {
        let mut cpu = Cpu::new();
        cpu.reg.set_de(0x0000);
        (&INC_DE.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x13));
        assert_eq!(cpu.reg.de(), 0x0001);
    }

    #[test]
    pub fn test_get_instruction_inc_d() {
        let instruction = get_instruction(&0x14).unwrap();
        assert_eq!(instruction, &INC_D);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_inc_d() {
        let mut cpu = Cpu::new();
        cpu.reg.d = 0x00;
        (&INC_D.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x0C));
        assert_eq!(cpu.reg.d, 0x01);
    }

    #[test]
    pub fn test_get_instruction_dec_d() {
        let instruction = get_instruction(&0x15).unwrap();
        assert_eq!(instruction, &DEC_D);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_dec_d() {
        let mut cpu = Cpu::new();
        cpu.reg.d = 0x01;
        (&DEC_D.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x15));
        assert_eq!(cpu.reg.d, 0x00);
    }

    #[test]
    pub fn test_get_instruction_ld_d_n() {
        let instruction = get_instruction(&0x16).unwrap();
        assert_eq!(instruction, &LD_D_N);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_ld_d_n() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.pc = 0xC000;
        mmu.set_byte(0xC001 as usize, 0x01);
        (&LD_D_N.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x16));
        assert_eq!(cpu.reg.d, 0x01);
    }

    #[test]
    pub fn test_get_instruction_rla() {
        let instruction = get_instruction(&0x17).unwrap();
        assert_eq!(instruction, &RLA);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_rla() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x80;
        cpu.reg.clear_flag(Flag::Carry);
        (&RLA.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x17));
        assert_eq!(cpu.reg.a, 0x00);
        assert!(!cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_get_instruction_jr_n() {
        let instruction = get_instruction(&0x18).unwrap();
        assert_eq!(instruction, &JR_N);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 12);
    }

    #[test]
    pub fn test_jr_n() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.pc = 0xC00A;
        mmu.set_byte(0xC00B as usize, 0xFB);
        (&JR_N.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x18));
        assert_eq!(cpu.reg.pc, 0xC007);
    }

    #[test]
    pub fn test_get_instruction_add_hl_de() {
        let instruction = get_instruction(&0x19).unwrap();
        assert_eq!(instruction, &ADD_HL_DE);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_add_hl_de() {
        let mut cpu = Cpu::new();
        cpu.reg.set_hl(0x0000);
        cpu.reg.set_de(0x0001);
        (&ADD_HL_DE.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x19));
        assert_eq!(cpu.reg.hl(), 0x0001);
    }

    #[test]
    pub fn test_get_instruction_ld_a_de() {
        let instruction = get_instruction(&0x1A).unwrap();
        assert_eq!(instruction, &LD_A_DE);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_ld_a_de() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.set_de(0xC000);
        mmu.set_byte(0xC000 as usize, 0x01);
        (&LD_A_DE.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x1A));
        assert_eq!(cpu.reg.a, 0x01);
    }

    #[test]
    pub fn test_get_instruction_dec_de() {
        let instruction = get_instruction(&0x1B).unwrap();
        assert_eq!(instruction, &DEC_DE);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_dec_de() {
        let mut cpu = Cpu::new();
        cpu.reg.set_de(0x0001);
        (&DEC_DE.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x1B));
        assert_eq!(cpu.reg.de(), 0x0000);
    }

    #[test]
    pub fn test_dec_de_underflow() {
        let mut cpu = Cpu::new();
        cpu.reg.set_de(0x0000);
        (&DEC_DE.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x1B));
        assert_eq!(cpu.reg.de(), 0xFFFF);
    }

    #[test]
    pub fn test_get_instruction_inc_e() {
        let instruction = get_instruction(&0x1C).unwrap();
        assert_eq!(instruction, &INC_E);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_inc_e() {
        let mut cpu = Cpu::new();
        cpu.reg.e = 0x00;
        (&INC_E.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x1C));
        assert_eq!(cpu.reg.e, 0x01);
    }

    #[test]
    pub fn test_get_instruction_dec_e() {
        let instruction = get_instruction(&0x1D).unwrap();
        assert_eq!(instruction, &DEC_E);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_dec_e() {
        let mut cpu = Cpu::new();
        cpu.reg.e = 0x01;
        (&DEC_E.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x0D));
        assert_eq!(cpu.reg.e, 0x00);
    }

    #[test]
    pub fn test_get_instruction_ld_e_n() {
        let instruction = get_instruction(&0x1E).unwrap();
        assert_eq!(instruction, &LD_E_N);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_ld_e_n() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.pc = 0xC000;
        mmu.set_byte(0xC001 as usize, 0x01);
        (&LD_E_N.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x0E));
        assert_eq!(cpu.reg.e, 0x01);
    }

    #[test]
    pub fn test_get_instruction_rra() {
        let instruction = get_instruction(&0x1F).unwrap();
        assert_eq!(instruction, &RRA);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_rra() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x01;
        cpu.reg.clear_flag(Flag::Carry);
        (&RRA.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x1F));
        assert_eq!(cpu.reg.a, 0x00);
        assert!(!cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_get_instruction_jr_nz_n() {
        let instruction = get_instruction(&0x20).unwrap();
        assert_eq!(instruction, &JR_NZ_N);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
        assert_eq!(instruction.clock_cycles_condition.unwrap(), 12);
    }

    #[test]
    pub fn test_jr_nz_n_zero() {
        let mut cpu = Cpu::new();
        cpu.reg.set_flag(Flag::Zero);
        cpu.reg.pc = 0xC000;
        (&JR_NZ_N.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x20));
        assert_eq!(cpu.reg.pc, 0xC000);
    }

    #[test]
    pub fn test_jr_nz_n_not_zero() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.clear_flag(Flag::Zero);
        cpu.reg.pc = 0xC003;
        mmu.set_byte(0xC004 as usize, 0xFB);
        (&JR_NZ_N.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x20));
        assert_eq!(cpu.reg.pc, 0xC000);
    }

    #[test]
    pub fn test_get_instruction_ld_hl_nn() {
        let instruction = get_instruction(&0x21).unwrap();
        assert_eq!(instruction, &LD_HL_NN);
        assert_eq!(instruction.length, 3);
        assert_eq!(instruction.clock_cycles, 12);
    }

    #[test]
    pub fn test_ld_hl_nn() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.pc = 0xC000;
        mmu.set_word(0xC001 as usize, 0x0001);
        (&LD_HL_NN.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x21));
        assert_eq!(cpu.reg.hl(), 0x0001);
    }

    #[test]
    pub fn test_get_instruction_ld_hl_inc_a() {
        let instruction = get_instruction(&0x22).unwrap();
        assert_eq!(instruction, &LD_HL_INC_A);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_ld_hl_inc_a() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.a = 0x01;
        cpu.reg.set_hl(0xC000);
        (&LD_HL_INC_A.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x22));
        assert_eq!(cpu.reg.hl(), 0xC001);
        assert_eq!(mmu.get_byte(0xC000 as usize), 0x01);
    }

    #[test]
    pub fn test_get_instruction_inc_hl() {
        let instruction = get_instruction(&0x23).unwrap();
        assert_eq!(instruction, &INC_HL);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_inc_hl() {
        let mut cpu = Cpu::new();
        cpu.reg.set_hl(0x0000);
        (&INC_HL.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x23));
        assert_eq!(cpu.reg.hl(), 0x0001);
    }

    #[test]
    pub fn test_get_instruction_inc_h() {
        let instruction = get_instruction(&0x24).unwrap();
        assert_eq!(instruction, &INC_H);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_inc_h() {
        let mut cpu = Cpu::new();
        cpu.reg.h = 0x00;
        (&INC_H.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x0C));
        assert_eq!(cpu.reg.h, 0x01);
    }

    #[test]
    pub fn test_get_instruction_dec_h() {
        let instruction = get_instruction(&0x25).unwrap();
        assert_eq!(instruction, &DEC_H);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_dec_h() {
        let mut cpu = Cpu::new();
        cpu.reg.h = 0x01;
        (&DEC_H.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x15));
        assert_eq!(cpu.reg.h, 0x00);
    }

    #[test]
    pub fn test_get_instruction_ld_h_n() {
        let instruction = get_instruction(&0x26).unwrap();
        assert_eq!(instruction, &LD_H_N);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_ld_h_n() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.pc = 0xC000;
        mmu.set_byte(0xC001 as usize, 0x01);
        (&LD_H_N.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x16));
        assert_eq!(cpu.reg.h, 0x01);
    }

    #[test]
    pub fn test_get_instruction_daa() {
        let instruction = get_instruction(&0x27).unwrap();
        assert_eq!(instruction, &DAA);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_daa() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x01;
        cpu.reg.clear_all_flags();
        cpu.reg.set_flag(Flag::Negative);
        (&DAA.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x27));
        assert_eq!(cpu.reg.a, 0x01);
        assert!(!cpu.reg.check_flag(Flag::Zero));
        assert!(cpu.reg.check_flag(Flag::Negative));
        assert!(!cpu.reg.check_flag(Flag::HalfCarry));
        assert!(!cpu.reg.check_flag(Flag::Carry));
    }

    #[test]
    pub fn test_get_instruction_jr_z_n() {
        let instruction = get_instruction(&0x28).unwrap();
        assert_eq!(instruction, &JR_Z_N);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
        assert_eq!(instruction.clock_cycles_condition.unwrap(), 12);
    }

    #[test]
    pub fn test_jr_z_n_zero() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.pc = 0xC000;
        cpu.reg.set_flag(Flag::Zero);
        mmu.set_byte(0xC001 as usize, 0x01);
        (&JR_Z_N.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x28));
        assert_eq!(cpu.reg.pc, 0xC003);
    }

    #[test]
    pub fn test_jr_z_n_not_zero() {
        let mut cpu = Cpu::new();
        cpu.reg.pc = 0xC000;
        cpu.reg.clear_flag(Flag::Zero);
        (&JR_Z_N.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x28));
        assert_eq!(cpu.reg.pc, 0xC000);
    }

    #[test]
    pub fn test_get_instruction_add_hl_hl() {
        let instruction = get_instruction(&0x29).unwrap();
        assert_eq!(instruction, &ADD_HL_HL);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_add_hl_hl() {
        let mut cpu = Cpu::new();
        cpu.reg.set_hl(0x0001);
        (&ADD_HL_HL.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x29));
        assert_eq!(cpu.reg.hl(), 0x0002);
    }

    #[test]
    pub fn test_get_instruction_ld_a_hl_inc() {
        let instruction = get_instruction(&0x2A).unwrap();
        assert_eq!(instruction, &LD_A_HL_INC);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_ld_a_hl_inc() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.set_hl(0xC000);
        mmu.set_byte(0xC000 as usize, 0x01);
        (&LD_A_HL_INC.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x2A));
        assert_eq!(cpu.reg.hl(), 0xC001);
        assert_eq!(cpu.reg.a, 0x01);
    }

    #[test]
    pub fn test_get_instruction_dec_hl() {
        let instruction = get_instruction(&0x2B).unwrap();
        assert_eq!(instruction, &DEC_HL);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_dec_hl() {
        let mut cpu = Cpu::new();
        cpu.reg.set_hl(0x0001);
        (&DEC_HL.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x2B));
        assert_eq!(cpu.reg.hl(), 0x0000);
    }

    #[test]
    pub fn test_dec_hl_underflow() {
        let mut cpu = Cpu::new();
        cpu.reg.set_hl(0x0000);
        (&DEC_HL.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x2B));
        assert_eq!(cpu.reg.hl(), 0xFFFF);
    }

    #[test]
    pub fn test_get_instruction_inc_l() {
        let instruction = get_instruction(&0x2C).unwrap();
        assert_eq!(instruction, &INC_L);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_inc_l() {
        let mut cpu = Cpu::new();
        cpu.reg.l = 0x00;
        (&INC_L.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x2C));
        assert_eq!(cpu.reg.l, 0x01);
    }

    #[test]
    pub fn test_get_instruction_dec_l() {
        let instruction = get_instruction(&0x2D).unwrap();
        assert_eq!(instruction, &DEC_L);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_dec_l() {
        let mut cpu = Cpu::new();
        cpu.reg.l = 0x01;
        (&DEC_L.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x2D));
        assert_eq!(cpu.reg.l, 0x00);
    }

    #[test]
    pub fn test_get_instruction_ld_l_n() {
        let instruction = get_instruction(&0x2E).unwrap();
        assert_eq!(instruction, &LD_L_N);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_ld_l_n() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.pc = 0xC000;
        mmu.set_byte(0xC001 as usize, 0x01);
        (&LD_L_N.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x2E));
        assert_eq!(cpu.reg.l, 0x01);
    }

    #[test]
    pub fn test_get_instruction_jr_nc_n() {
        let instruction = get_instruction(&0x30).unwrap();
        assert_eq!(instruction, &JR_NC_N);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
        assert_eq!(instruction.clock_cycles_condition.unwrap(), 12);
    }

    #[test]
    pub fn test_jr_nc_n_carry() {
        let mut cpu = Cpu::new();
        cpu.reg.pc = 0xC000;
        cpu.reg.set_flag(Flag::Carry);
        (&JR_NC_N.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x30));
        assert_eq!(cpu.reg.pc, 0xC000);
    }

    #[test]
    pub fn test_jr_nc_n_not_carry() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.pc = 0xC000;
        cpu.reg.clear_flag(Flag::Carry);
        mmu.set_byte(0xC001 as usize, 0x01);
        (&JR_NC_N.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x30));
        assert_eq!(cpu.reg.pc, 0xC003);
    }

    #[test]
    pub fn test_get_instruction_ld_sp_nn() {
        let instruction = get_instruction(&0x31).unwrap();
        assert_eq!(instruction, &LD_SP_NN);
        assert_eq!(instruction.length, 3);
        assert_eq!(instruction.clock_cycles, 12);
    }

    #[test]
    pub fn test_ld_sp_nn() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.pc = 0xC000;
        mmu.set_word(0xC001 as usize, 0x0001);
        (&LD_SP_NN.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x31));
        assert_eq!(cpu.reg.sp, 0x0001);
    }

    #[test]
    pub fn test_get_instruction_ld_hl_n() {
        let instruction = get_instruction(&0x36).unwrap();
        assert_eq!(instruction, &LD_HL_N);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 12);
    }

    #[test]
    pub fn test_ld_hl_n() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.set_hl(0xC000);
        cpu.reg.pc = 0xC000;
        mmu.set_byte(0xC001 as usize, 0x01);
        (&LD_HL_N.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x36));
        assert_eq!(mmu.get_byte(0xC000 as usize), 0x01);
    }

    #[test]
    pub fn test_get_instruction_jr_c_n() {
        let instruction = get_instruction(&0x38).unwrap();
        assert_eq!(instruction, &JR_C_N);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
        assert_eq!(instruction.clock_cycles_condition.unwrap(), 12);
    }

    #[test]
    pub fn test_jr_c_n_carry() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.pc = 0xC000;
        cpu.reg.set_flag(Flag::Carry);
        mmu.set_byte(0xC001 as usize, 0x01);
        (&JR_C_N.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x38));
        assert_eq!(cpu.reg.pc, 0xC003);
    }

    #[test]
    pub fn test_jr_c_n_not_carry() {
        let mut cpu = Cpu::new();
        cpu.reg.pc = 0xC000;
        cpu.reg.clear_flag(Flag::Carry);
        (&JR_C_N.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x38));
        assert_eq!(cpu.reg.pc, 0xC000);
    }

    #[test]
    pub fn test_get_instruction_add_hl_sp() {
        let instruction = get_instruction(&0x39).unwrap();
        assert_eq!(instruction, &ADD_HL_SP);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_add_hl_sp() {
        let mut cpu = Cpu::new();
        cpu.reg.set_hl(0x0001);
        cpu.reg.sp = 0x0001;
        (&ADD_HL_SP.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x39));
        assert_eq!(cpu.reg.hl(), 0x0002);
    }

    #[test]
    pub fn test_get_instruction_dec_sp() {
        let instruction = get_instruction(&0x3B).unwrap();
        assert_eq!(instruction, &DEC_SP);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_dec_sp() {
        let mut cpu = Cpu::new();
        cpu.reg.sp = 0x0001;
        (&DEC_SP.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x3B));
        assert_eq!(cpu.reg.sp, 0x0000);
    }

    #[test]
    pub fn test_dec_sp_underflow() {
        let mut cpu = Cpu::new();
        cpu.reg.sp = 0x0000;
        (&DEC_SP.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x3B));
        assert_eq!(cpu.reg.sp, 0xFFFF);
    }

    #[test]
    pub fn test_get_instruction_inc_a() {
        let instruction = get_instruction(&0x3C).unwrap();
        assert_eq!(instruction, &INC_A);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_inc_a() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x00;
        (&INC_A.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x3C));
        assert_eq!(cpu.reg.a, 0x01);
    }

    #[test]
    pub fn test_get_instruction_ld_hl_dec_a() {
        let instruction = get_instruction(&0x32).unwrap();
        assert_eq!(instruction, &LD_HL_DEC_A);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_ld_hl_dec_a() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.a = 0x01;
        cpu.reg.set_hl(0xC000);
        (&LD_HL_DEC_A.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x32));
        assert_eq!(cpu.reg.hl(), 0xBFFF);
        assert_eq!(mmu.get_byte(0xC000 as usize), 0x01);
    }

    #[test]
    pub fn test_get_instruction_inc_sp() {
        let instruction = get_instruction(&0x33).unwrap();
        assert_eq!(instruction, &INC_SP);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_inc_sp() {
        let mut cpu = Cpu::new();
        cpu.reg.sp = 0x0000;
        (&INC_SP.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x33));
        assert_eq!(cpu.reg.sp, 0x0001);
    }

    #[test]
    pub fn test_inc_sp_overflow() {
        let mut cpu = Cpu::new();
        cpu.reg.sp = 0xFFFF;
        (&INC_SP.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x33));
        assert_eq!(cpu.reg.sp, 0x0000);
    }

    #[test]
    pub fn test_get_instruction_dec_hl_ptr() {
        let instruction = get_instruction(&0x35).unwrap();
        assert_eq!(instruction, &DEC_HL_PTR);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 12);
    }

    #[test]
    pub fn test_dec_hl_ptr() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.set_hl(0xC000);
        mmu.set_byte(0xC000 as usize, 0x01);
        (&DEC_HL_PTR.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x35));
        assert_eq!(mmu.get_byte(0xC000 as usize), 0x00);
    }

    #[test]
    pub fn test_get_instruction_dec_a() {
        let instruction = get_instruction(&0x3D).unwrap();
        assert_eq!(instruction, &DEC_A);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_dec_a() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x01;
        (&DEC_A.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x3D));
        assert_eq!(cpu.reg.a, 0x00);
    }

    #[test]
    pub fn test_get_instruction_ld_a_n() {
        let instruction = get_instruction(&0x3E).unwrap();
        assert_eq!(instruction, &LD_A_N);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_ld_a_n() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.pc = 0xC000;
        mmu.set_byte(0xC001 as usize, 0x01);
        (&LD_A_N.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x3E));
        assert_eq!(cpu.reg.a, 0x01);
    }

    #[test]
    pub fn test_get_instruction_ld_b_b() {
        let instruction = get_instruction(&0x40).unwrap();
        assert_eq!(instruction, &LD_B_B);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_ld_b_b() {
        let mut cpu = Cpu::new();
        cpu.reg.b = 0x01;
        (&LD_B_B.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x40));
        assert_eq!(cpu.reg.b, 0x01);
    }

    #[test]
    pub fn test_get_instruction_ld_b_c() {
        let instruction = get_instruction(&0x41).unwrap();
        assert_eq!(instruction, &LD_B_C);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_ld_b_c() {
        let mut cpu = Cpu::new();
        cpu.reg.c = 0x01;
        (&LD_B_C.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x41));
        assert_eq!(cpu.reg.b, 0x01);
    }

    #[test]
    pub fn test_get_instruction_ld_b_d() {
        let instruction = get_instruction(&0x42).unwrap();
        assert_eq!(instruction, &LD_B_D);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_ld_b_d() {
        let mut cpu = Cpu::new();
        cpu.reg.d = 0x01;
        (&LD_B_D.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x42));
        assert_eq!(cpu.reg.b, 0x01);
    }

    #[test]
    pub fn test_get_instruction_ld_b_e() {
        let instruction = get_instruction(&0x43).unwrap();
        assert_eq!(instruction, &LD_B_E);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_ld_b_e() {
        let mut cpu = Cpu::new();
        cpu.reg.e = 0x01;
        (&LD_B_E.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x43));
        assert_eq!(cpu.reg.b, 0x01);
    }

    #[test]
    pub fn test_get_instruction_ld_b_h() {
        let instruction = get_instruction(&0x44).unwrap();
        assert_eq!(instruction, &LD_B_H);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_ld_b_h() {
        let mut cpu = Cpu::new();
        cpu.reg.h = 0x01;
        (&LD_B_H.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x44));
        assert_eq!(cpu.reg.b, 0x01);
    }

    #[test]
    pub fn test_get_instruction_ld_b_l() {
        let instruction = get_instruction(&0x45).unwrap();
        assert_eq!(instruction, &LD_B_L);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_ld_b_l() {
        let mut cpu = Cpu::new();
        cpu.reg.l = 0x01;
        (&LD_B_L.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x45));
        assert_eq!(cpu.reg.b, 0x01);
    }

    #[test]
    pub fn test_get_instruction_ld_b_hl() {
        let instruction = get_instruction(&0x46).unwrap();
        assert_eq!(instruction, &LD_B_HL);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_ld_b_hl() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.set_hl(0xC000);
        mmu.set_byte(0xC000 as usize, 0x01);
        (&LD_B_HL.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x46));
        assert_eq!(cpu.reg.b, 0x01);
    }

    #[test]
    pub fn test_get_instruction_ld_b_a() {
        let instruction = get_instruction(&0x47).unwrap();
        assert_eq!(instruction, &LD_B_A);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_ld_b_a() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x01;
        (&LD_B_A.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x47));
        assert_eq!(cpu.reg.b, 0x01);
    }

    #[test]
    pub fn test_get_instruction_ld_c_b() {
        let instruction = get_instruction(&0x48).unwrap();
        assert_eq!(instruction, &LD_C_B);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_ld_c_b() {
        let mut cpu = Cpu::new();
        cpu.reg.b = 0x01;
        (&LD_C_B.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x48));
        assert_eq!(cpu.reg.c, 0x01);
    }

    #[test]
    pub fn test_get_instruction_ld_c_c() {
        let instruction = get_instruction(&0x49).unwrap();
        assert_eq!(instruction, &LD_C_C);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_ld_c_c() {
        let mut cpu = Cpu::new();
        cpu.reg.c = 0x01;
        (&LD_C_C.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x49));
        assert_eq!(cpu.reg.c, 0x01);
    }

    #[test]
    pub fn test_get_instruction_ld_c_d() {
        let instruction = get_instruction(&0x4A).unwrap();
        assert_eq!(instruction, &LD_C_D);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_ld_c_d() {
        let mut cpu = Cpu::new();
        cpu.reg.d = 0x01;
        (&LD_C_D.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x4A));
        assert_eq!(cpu.reg.c, 0x01);
    }

    #[test]
    pub fn test_get_instruction_ld_c_e() {
        let instruction = get_instruction(&0x4B).unwrap();
        assert_eq!(instruction, &LD_C_E);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_ld_c_e() {
        let mut cpu = Cpu::new();
        cpu.reg.e = 0x01;
        (&LD_C_E.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x4B));
        assert_eq!(cpu.reg.c, 0x01);
    }

    #[test]
    pub fn test_get_instruction_ld_c_h() {
        let instruction = get_instruction(&0x4C).unwrap();
        assert_eq!(instruction, &LD_C_H);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_ld_c_h() {
        let mut cpu = Cpu::new();
        cpu.reg.h = 0x01;
        (&LD_C_H.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x4C));
        assert_eq!(cpu.reg.c, 0x01);
    }

    #[test]
    pub fn test_get_instruction_ld_c_l() {
        let instruction = get_instruction(&0x4D).unwrap();
        assert_eq!(instruction, &LD_C_L);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_ld_c_l() {
        let mut cpu = Cpu::new();
        cpu.reg.l = 0x01;
        (&LD_C_L.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x4D));
        assert_eq!(cpu.reg.c, 0x01);
    }

    #[test]
    pub fn test_get_instruction_ld_c_hl() {
        let instruction = get_instruction(&0x4E).unwrap();
        assert_eq!(instruction, &LD_C_HL);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_ld_c_hl() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.set_hl(0xC000);
        mmu.set_byte(0xC000 as usize, 0x01);
        (&LD_C_HL.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x4E));
        assert_eq!(cpu.reg.c, 0x01);
    }

    #[test]
    pub fn test_get_instruction_ld_c_a() {
        let instruction = get_instruction(&0x4F).unwrap();
        assert_eq!(instruction, &LD_C_A);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_ld_c_a() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x01;
        (&LD_C_A.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x4F));
        assert_eq!(cpu.reg.c, 0x01);
    }

    #[test]
    pub fn test_get_instruction_ld_d_b() {
        let instruction = get_instruction(&0x50).unwrap();
        assert_eq!(instruction, &LD_D_B);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_ld_d_b() {
        let mut cpu = Cpu::new();
        cpu.reg.b = 0x01;
        (&LD_D_B.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x50));
        assert_eq!(cpu.reg.d, 0x01);
    }

    #[test]
    pub fn test_get_instruction_ld_d_c() {
        let instruction = get_instruction(&0x51).unwrap();
        assert_eq!(instruction, &LD_D_C);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_ld_d_c() {
        let mut cpu = Cpu::new();
        cpu.reg.c = 0x01;
        (&LD_D_C.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x51));
        assert_eq!(cpu.reg.d, 0x01);
    }

    #[test]
    pub fn test_get_instruction_ld_d_d() {
        let instruction = get_instruction(&0x52).unwrap();
        assert_eq!(instruction, &LD_D_D);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_ld_d_d() {
        let mut cpu = Cpu::new();
        cpu.reg.d = 0x01;
        (&LD_D_D.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x52));
        assert_eq!(cpu.reg.d, 0x01);
    }

    #[test]
    pub fn test_get_instruction_ld_d_e() {
        let instruction = get_instruction(&0x53).unwrap();
        assert_eq!(instruction, &LD_D_E);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_ld_d_e() {
        let mut cpu = Cpu::new();
        cpu.reg.e = 0x01;
        (&LD_D_E.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x53));
        assert_eq!(cpu.reg.d, 0x01);
    }

    #[test]
    pub fn test_get_instruction_ld_d_h() {
        let instruction = get_instruction(&0x54).unwrap();
        assert_eq!(instruction, &LD_D_H);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_ld_d_h() {
        let mut cpu = Cpu::new();
        cpu.reg.h = 0x01;
        (&LD_D_H.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x54));
        assert_eq!(cpu.reg.d, 0x01);
    }

    #[test]
    pub fn test_get_instruction_ld_d_l() {
        let instruction = get_instruction(&0x55).unwrap();
        assert_eq!(instruction, &LD_D_L);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_ld_d_l() {
        let mut cpu = Cpu::new();
        cpu.reg.l = 0x01;
        (&LD_D_L.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x55));
        assert_eq!(cpu.reg.d, 0x01);
    }

    #[test]
    pub fn test_get_instruction_ld_d_hl() {
        let instruction = get_instruction(&0x56).unwrap();
        assert_eq!(instruction, &LD_D_HL);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_ld_d_hl() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.set_hl(0xC000);
        mmu.set_byte(0xC000 as usize, 0x01);
        (&LD_D_HL.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x56));
        assert_eq!(cpu.reg.d, 0x01);
    }

    #[test]
    pub fn test_get_instruction_ld_d_a() {
        let instruction = get_instruction(&0x57).unwrap();
        assert_eq!(instruction, &LD_D_A);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_ld_d_a() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x01;
        (&LD_D_A.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x57));
        assert_eq!(cpu.reg.d, 0x01);
    }

    #[test]
    pub fn test_get_instruction_ld_e_b() {
        let instruction = get_instruction(&0x58).unwrap();
        assert_eq!(instruction, &LD_E_B);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_ld_e_b() {
        let mut cpu = Cpu::new();
        cpu.reg.b = 0x01;
        (&LD_E_B.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x58));
        assert_eq!(cpu.reg.e, 0x01);
    }

    #[test]
    pub fn test_get_instruction_ld_e_c() {
        let instruction = get_instruction(&0x59).unwrap();
        assert_eq!(instruction, &LD_E_C);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_ld_e_c() {
        let mut cpu = Cpu::new();
        cpu.reg.c = 0x01;
        (&LD_E_C.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x59));
        assert_eq!(cpu.reg.e, 0x01);
    }

    #[test]
    pub fn test_get_instruction_ld_e_d() {
        let instruction = get_instruction(&0x5A).unwrap();
        assert_eq!(instruction, &LD_E_D);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_ld_e_d() {
        let mut cpu = Cpu::new();
        cpu.reg.d = 0x01;
        (&LD_E_D.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x5A));
        assert_eq!(cpu.reg.e, 0x01);
    }

    #[test]
    pub fn test_get_instruction_ld_e_e() {
        let instruction = get_instruction(&0x5B).unwrap();
        assert_eq!(instruction, &LD_E_E);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_ld_e_e() {
        let mut cpu = Cpu::new();
        cpu.reg.e = 0x01;
        (&LD_E_E.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x5B));
        assert_eq!(cpu.reg.e, 0x01);
    }

    #[test]
    pub fn test_get_instruction_ld_e_h() {
        let instruction = get_instruction(&0x5C).unwrap();
        assert_eq!(instruction, &LD_E_H);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_ld_e_h() {
        let mut cpu = Cpu::new();
        cpu.reg.h = 0x01;
        (&LD_E_H.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x5C));
        assert_eq!(cpu.reg.e, 0x01);
    }

    #[test]
    pub fn test_get_instruction_ld_e_l() {
        let instruction = get_instruction(&0x5D).unwrap();
        assert_eq!(instruction, &LD_E_L);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_ld_e_l() {
        let mut cpu = Cpu::new();
        cpu.reg.l = 0x01;
        (&LD_E_L.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x5D));
        assert_eq!(cpu.reg.e, 0x01);
    }

    #[test]
    pub fn test_get_instruction_ld_e_hl() {
        let instruction = get_instruction(&0x5E).unwrap();
        assert_eq!(instruction, &LD_E_HL);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_ld_e_hl() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.set_hl(0xC000);
        mmu.set_byte(0xC000 as usize, 0x01);
        (&LD_E_HL.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x5E));
        assert_eq!(cpu.reg.e, 0x01);
    }

    #[test]
    pub fn test_get_instruction_ld_e_a() {
        let instruction = get_instruction(&0x5F).unwrap();
        assert_eq!(instruction, &LD_E_A);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_ld_e_a() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x01;
        (&LD_E_A.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x5F));
        assert_eq!(cpu.reg.e, 0x01);
    }

    #[test]
    pub fn test_get_instruction_ld_h_b() {
        let instruction = get_instruction(&0x60).unwrap();
        assert_eq!(instruction, &LD_H_B);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_ld_h_b() {
        let mut cpu = Cpu::new();
        cpu.reg.b = 0x01;
        (&LD_H_B.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x60));
        assert_eq!(cpu.reg.h, 0x01);
    }

    #[test]
    pub fn test_get_instruction_ld_h_c() {
        let instruction = get_instruction(&0x61).unwrap();
        assert_eq!(instruction, &LD_H_C);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_ld_h_c() {
        let mut cpu = Cpu::new();
        cpu.reg.c = 0x01;
        (&LD_H_C.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x61));
        assert_eq!(cpu.reg.h, 0x01);
    }

    #[test]
    pub fn test_get_instruction_ld_h_d() {
        let instruction = get_instruction(&0x62).unwrap();
        assert_eq!(instruction, &LD_H_D);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_ld_h_d() {
        let mut cpu = Cpu::new();
        cpu.reg.d = 0x01;
        (&LD_H_D.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x62));
        assert_eq!(cpu.reg.h, 0x01);
    }

    #[test]
    pub fn test_get_instruction_ld_h_e() {
        let instruction = get_instruction(&0x63).unwrap();
        assert_eq!(instruction, &LD_H_E);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_ld_h_e() {
        let mut cpu = Cpu::new();
        cpu.reg.e = 0x01;
        (&LD_H_E.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x63));
        assert_eq!(cpu.reg.h, 0x01);
    }

    #[test]
    pub fn test_get_instruction_ld_h_h() {
        let instruction = get_instruction(&0x64).unwrap();
        assert_eq!(instruction, &LD_H_H);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_ld_h_h() {
        let mut cpu = Cpu::new();
        cpu.reg.h = 0x01;
        (&LD_H_H.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x64));
        assert_eq!(cpu.reg.h, 0x01);
    }

    #[test]
    pub fn test_get_instruction_ld_h_l() {
        let instruction = get_instruction(&0x65).unwrap();
        assert_eq!(instruction, &LD_H_L);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_ld_h_l() {
        let mut cpu = Cpu::new();
        cpu.reg.l = 0x01;
        (&LD_H_L.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x65));
        assert_eq!(cpu.reg.h, 0x01);
    }

    #[test]
    pub fn test_get_instruction_ld_h_hl() {
        let instruction = get_instruction(&0x66).unwrap();
        assert_eq!(instruction, &LD_H_HL);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_ld_h_hl() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.set_hl(0xC000);
        mmu.set_byte(0xC000 as usize, 0x01);
        (&LD_H_HL.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x66));
        assert_eq!(cpu.reg.h, 0x01);
    }

    #[test]
    pub fn test_get_instruction_ld_h_a() {
        let instruction = get_instruction(&0x67).unwrap();
        assert_eq!(instruction, &LD_H_A);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_ld_h_a() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x01;
        (&LD_H_A.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x67));
        assert_eq!(cpu.reg.h, 0x01);
    }

    #[test]
    pub fn test_get_instruction_ld_l_b() {
        let instruction = get_instruction(&0x68).unwrap();
        assert_eq!(instruction, &LD_L_B);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_ld_l_b() {
        let mut cpu = Cpu::new();
        cpu.reg.b = 0x01;
        (&LD_L_B.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x68));
        assert_eq!(cpu.reg.l, 0x01);
    }

    #[test]
    pub fn test_get_instruction_ld_l_c() {
        let instruction = get_instruction(&0x69).unwrap();
        assert_eq!(instruction, &LD_L_C);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_ld_l_c() {
        let mut cpu = Cpu::new();
        cpu.reg.c = 0x01;
        (&LD_L_C.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x69));
        assert_eq!(cpu.reg.l, 0x01);
    }

    #[test]
    pub fn test_get_instruction_ld_l_d() {
        let instruction = get_instruction(&0x6A).unwrap();
        assert_eq!(instruction, &LD_L_D);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_ld_l_d() {
        let mut cpu = Cpu::new();
        cpu.reg.d = 0x01;
        (&LD_L_D.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x6A));
        assert_eq!(cpu.reg.l, 0x01);
    }

    #[test]
    pub fn test_get_instruction_ld_l_e() {
        let instruction = get_instruction(&0x6B).unwrap();
        assert_eq!(instruction, &LD_L_E);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_ld_l_e() {
        let mut cpu = Cpu::new();
        cpu.reg.e = 0x01;
        (&LD_L_E.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x6B));
        assert_eq!(cpu.reg.l, 0x01);
    }

    #[test]
    pub fn test_get_instruction_ld_l_h() {
        let instruction = get_instruction(&0x6C).unwrap();
        assert_eq!(instruction, &LD_L_H);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_ld_l_h() {
        let mut cpu = Cpu::new();
        cpu.reg.h = 0x01;
        (&LD_L_H.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x6C));
        assert_eq!(cpu.reg.l, 0x01);
    }

    #[test]
    pub fn test_get_instruction_ld_l_l() {
        let instruction = get_instruction(&0x6D).unwrap();
        assert_eq!(instruction, &LD_L_L);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_ld_l_l() {
        let mut cpu = Cpu::new();
        cpu.reg.l = 0x01;
        (&LD_L_L.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x6D));
        assert_eq!(cpu.reg.l, 0x01);
    }

    #[test]
    pub fn test_get_instruction_ld_l_hl() {
        let instruction = get_instruction(&0x6E).unwrap();
        assert_eq!(instruction, &LD_L_HL);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_ld_l_hl() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.set_hl(0xC000);
        mmu.set_byte(0xC000 as usize, 0x01);
        (&LD_L_HL.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x6E));
        assert_eq!(cpu.reg.l, 0x01);
    }

    #[test]
    pub fn test_get_instruction_ld_l_a() {
        let instruction = get_instruction(&0x6F).unwrap();
        assert_eq!(instruction, &LD_L_A);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_ld_l_a() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x01;
        (&LD_L_A.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x6F));
        assert_eq!(cpu.reg.l, 0x01);
    }

    #[test]
    pub fn test_get_instruction_ld_hl_b() {
        let instruction = get_instruction(&0x70).unwrap();
        assert_eq!(instruction, &LD_HL_B);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_ld_hl_b() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.b = 0x01;
        cpu.reg.set_hl(0xC000);
        (&LD_HL_B.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x70));
        assert_eq!(mmu.get_byte(0xC000 as usize), 0x01);
    }

    #[test]
    pub fn test_get_instruction_ld_hl_c() {
        let instruction = get_instruction(&0x71).unwrap();
        assert_eq!(instruction, &LD_HL_C);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_ld_hl_c() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.c = 0x01;
        cpu.reg.set_hl(0xC000);
        (&LD_HL_C.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x71));
        assert_eq!(mmu.get_byte(0xC000 as usize), 0x01);
    }

    #[test]
    pub fn test_get_instruction_ld_hl_d() {
        let instruction = get_instruction(&0x72).unwrap();
        assert_eq!(instruction, &LD_HL_D);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_ld_hl_d() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.d = 0x01;
        cpu.reg.set_hl(0xC000);
        (&LD_HL_D.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x72));
        assert_eq!(mmu.get_byte(0xC000 as usize), 0x01);
    }

    #[test]
    pub fn test_get_instruction_ld_hl_e() {
        let instruction = get_instruction(&0x73).unwrap();
        assert_eq!(instruction, &LD_HL_E);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_ld_hl_e() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.e = 0x01;
        cpu.reg.set_hl(0xC000);
        (&LD_HL_E.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x73));
        assert_eq!(mmu.get_byte(0xC000 as usize), 0x01);
    }

    #[test]
    pub fn test_get_instruction_ld_hl_h() {
        let instruction = get_instruction(&0x74).unwrap();
        assert_eq!(instruction, &LD_HL_H);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_ld_hl_h() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.set_hl(0xC000);
        (&LD_HL_H.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x74));
        assert_eq!(mmu.get_byte(0xC000 as usize), 0xC0);
    }

    #[test]
    pub fn test_get_instruction_ld_hl_l() {
        let instruction = get_instruction(&0x75).unwrap();
        assert_eq!(instruction, &LD_HL_L);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_ld_hl_l() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.set_hl(0xC000);
        (&LD_HL_L.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x75));
        assert_eq!(mmu.get_byte(0xC000 as usize), 0x00);
    }

    #[test]
    pub fn test_get_instruction_ld_hl_a() {
        let instruction = get_instruction(&0x77).unwrap();
        assert_eq!(instruction, &LD_HL_A);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_ld_hl_a() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.a = 0x01;
        cpu.reg.set_hl(0xC000);
        (&LD_HL_A.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x77));
        assert_eq!(mmu.get_byte(0xC000 as usize), 0x01);
    }

    #[test]
    pub fn test_get_instruction_ld_a_b() {
        let instruction = get_instruction(&0x78).unwrap();
        assert_eq!(instruction, &LD_A_B);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_ld_a_b() {
        let mut cpu = Cpu::new();
        cpu.reg.b = 0x01;
        (&LD_A_B.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x78));
        assert_eq!(cpu.reg.a, 0x01);
    }

    #[test]
    pub fn test_get_instruction_ld_a_c() {
        let instruction = get_instruction(&0x79).unwrap();
        assert_eq!(instruction, &LD_A_C);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_ld_a_c() {
        let mut cpu = Cpu::new();
        cpu.reg.c = 0x01;
        (&LD_A_C.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x79));
        assert_eq!(cpu.reg.a, 0x01);
    }

    #[test]
    pub fn test_get_instruction_ld_a_d() {
        let instruction = get_instruction(&0x7A).unwrap();
        assert_eq!(instruction, &LD_A_D);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_ld_a_d() {
        let mut cpu = Cpu::new();
        cpu.reg.d = 0x01;
        (&LD_A_D.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x7A));
        assert_eq!(cpu.reg.a, 0x01);
    }

    #[test]
    pub fn test_get_instruction_ld_a_e() {
        let instruction = get_instruction(&0x7B).unwrap();
        assert_eq!(instruction, &LD_A_E);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_ld_a_e() {
        let mut cpu = Cpu::new();
        cpu.reg.e = 0x01;
        (&LD_A_E.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x7B));
        assert_eq!(cpu.reg.a, 0x01);
    }

    #[test]
    pub fn test_get_instruction_ld_a_h() {
        let instruction = get_instruction(&0x7C).unwrap();
        assert_eq!(instruction, &LD_A_H);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_ld_a_h() {
        let mut cpu = Cpu::new();
        cpu.reg.h = 0x01;
        (&LD_A_H.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x7C));
        assert_eq!(cpu.reg.a, 0x01);
    }

    #[test]
    pub fn test_get_instruction_ld_a_l() {
        let instruction = get_instruction(&0x7D).unwrap();
        assert_eq!(instruction, &LD_A_L);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_ld_a_l() {
        let mut cpu = Cpu::new();
        cpu.reg.l = 0x01;
        (&LD_A_L.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x7D));
        assert_eq!(cpu.reg.a, 0x01);
    }

    #[test]
    pub fn test_get_instruction_ld_a_hl() {
        let instruction = get_instruction(&0x7E).unwrap();
        assert_eq!(instruction, &LD_A_HL);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_ld_a_hl() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.set_hl(0xC000);
        mmu.set_byte(0xC000 as usize, 0x01);
        (&LD_A_HL.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x7E));
        assert_eq!(cpu.reg.a, 0x01);
    }

    #[test]
    pub fn test_get_instruction_ld_a_a() {
        let instruction = get_instruction(&0x7F).unwrap();
        assert_eq!(instruction, &LD_A_A);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_ld_a_a() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x01;
        (&LD_A_A.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x7F));
        assert_eq!(cpu.reg.a, 0x01);
    }

    #[test]
    pub fn test_get_instruction_add_hl() {
        let instruction = get_instruction(&0x86).unwrap();
        assert_eq!(instruction, &ADD_HL);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_add_hl() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.a = 0x01;
        cpu.reg.set_hl(0xC000);
        mmu.set_byte(0xC000 as usize, 0x01);
        (&ADD_HL.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0x86));
        assert_eq!(cpu.reg.a, 0x0002);
    }

    #[test]
    pub fn test_get_instruction_sub_b() {
        let instruction = get_instruction(&0x90).unwrap();
        assert_eq!(instruction, &SUB_B);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_sub_b() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x02;
        cpu.reg.b = 0x01;
        (&SUB_B.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0x90));
        assert_eq!(cpu.reg.a, 0x01);
    }

    #[test]
    pub fn test_get_instruction_xor_c() {
        let instruction = get_instruction(&0xA9).unwrap();
        assert_eq!(instruction, &XOR_C);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_xor_c() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x03;
        cpu.reg.c = 0x01;
        (&XOR_C.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0xA9));
        assert_eq!(cpu.reg.a, 0x02);
    }

    #[test]
    pub fn test_get_instruction_xor_l() {
        let instruction = get_instruction(&0xAD).unwrap();
        assert_eq!(instruction, &XOR_L);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_xor_l() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x03;
        cpu.reg.l = 0x01;
        (&XOR_L.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0xAD));
        assert_eq!(cpu.reg.a, 0x02);
    }

    #[test]
    pub fn test_get_instruction_xor_hl() {
        let instruction = get_instruction(&0xAE).unwrap();
        assert_eq!(instruction, &XOR_HL);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_xor_hl() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.a = 0x03;
        cpu.reg.set_hl(0xC000);
        mmu.set_byte(0xC000 as usize, 0x01);
        (&XOR_HL.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xAE));
        assert_eq!(cpu.reg.a, 0x02);
    }

    #[test]
    pub fn test_get_instruction_xor_a() {
        let instruction = get_instruction(&0xAF).unwrap();
        assert_eq!(instruction, &XOR_A);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_xor_a() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x01;
        (&XOR_A.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0xAF));
        assert_eq!(cpu.reg.a, 0x00);
    }

    #[test]
    pub fn test_get_instruction_or_b() {
        let instruction = get_instruction(&0xB0).unwrap();
        assert_eq!(instruction, &OR_B);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_or_b() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x02;
        cpu.reg.b = 0x01;
        (&OR_B.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0xB0));
        assert_eq!(cpu.reg.a, 0x03);
    }

    #[test]
    pub fn test_get_instruction_or_c() {
        let instruction = get_instruction(&0xB1).unwrap();
        assert_eq!(instruction, &OR_C);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_or_c() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x02;
        cpu.reg.c = 0x01;
        (&OR_C.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0xB1));
        assert_eq!(cpu.reg.a, 0x03);
    }

    #[test]
    pub fn test_get_instruction_or_hl() {
        let instruction = get_instruction(&0xB6).unwrap();
        assert_eq!(instruction, &OR_HL);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_or_hl() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.a = 0x02;
        cpu.reg.set_hl(0xC000);
        mmu.set_byte(0xC000 as usize, 0x01);
        (&OR_HL.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xB6));
        assert_eq!(cpu.reg.a, 0x03);
    }

    #[test]
    pub fn test_get_instruction_or_a() {
        let instruction = get_instruction(&0xB7).unwrap();
        assert_eq!(instruction, &OR_A);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_or_a() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x01;
        (&OR_A.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0xB1));
        assert_eq!(cpu.reg.a, 0x01);
    }

    #[test]
    pub fn test_get_instruction_cp_b() {
        let instruction = get_instruction(&0xB8).unwrap();
        assert_eq!(instruction, &CP_B);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_cp_b() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x01;
        cpu.reg.b = 0x01;
        (&CP_B.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0xB8));
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_get_instruction_cp_c() {
        let instruction = get_instruction(&0xB9).unwrap();
        assert_eq!(instruction, &CP_C);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_cp_c() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x01;
        cpu.reg.c = 0x01;
        (&CP_C.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0xB9));
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_get_instruction_cp_d() {
        let instruction = get_instruction(&0xBA).unwrap();
        assert_eq!(instruction, &CP_D);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_cp_d() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x01;
        cpu.reg.d = 0x01;
        (&CP_D.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0xBA));
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_get_instruction_cp_e() {
        let instruction = get_instruction(&0xBB).unwrap();
        assert_eq!(instruction, &CP_E);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_cp_e() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x01;
        cpu.reg.e = 0x01;
        (&CP_E.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0xBB));
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_get_instruction_cp_hl() {
        let instruction = get_instruction(&0xBE).unwrap();
        assert_eq!(instruction, &CP_HL);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_cp_hl() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.a = 0x01;
        cpu.reg.set_hl(0xC000);
        mmu.set_byte(0xC000 as usize, 0x02);
        (&CP_HL.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xBE));
        assert!(cpu.reg.check_flag(Flag::Carry));
    }

    #[test]
    pub fn test_get_instruction_ret_nz() {
        let instruction = get_instruction(&0xC0).unwrap();
        assert_eq!(instruction, &RET_NZ);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 8);
        assert_eq!(instruction.clock_cycles_condition.unwrap(), 20);
    }

    #[test]
    pub fn test_ret_nz_zero() {
        let mut cpu = Cpu::new();
        cpu.reg.pc = 0xC000;
        cpu.reg.set_flag(Flag::Zero);
        (&RET_NZ.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0xC0));
        assert_eq!(cpu.reg.pc, 0xC000);
    }

    #[test]
    pub fn test_ret_nz_not_zero() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.clear_flag(Flag::Zero);
        cpu.reg.sp = 0xC000;
        mmu.set_word(0xC000 as usize, 0x0001);
        (&RET_NZ.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xC0));
        assert_eq!(cpu.reg.pc, 0x0001);
    }

    #[test]
    pub fn test_get_instruction_pop_bc() {
        let instruction = get_instruction(&0xC1).unwrap();
        assert_eq!(instruction, &POP_BC);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 12);
    }

    #[test]
    pub fn test_pop_bc() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.sp = 0xC000;
        mmu.set_word(0xC000 as usize, 0x0001);
        (&POP_BC.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xC1));
        assert_eq!(cpu.reg.bc(), 0x0001);
        assert_eq!(cpu.reg.sp, 0xC002);
    }

    #[test]
    pub fn test_get_instruction_jp_nz_nn() {
        let instruction = get_instruction(&0xC2).unwrap();
        assert_eq!(instruction, &JP_NZ_NN);
        assert_eq!(instruction.length, 3);
        assert_eq!(instruction.clock_cycles, 12);
        assert_eq!(instruction.clock_cycles_condition.unwrap(), 16);
    }

    #[test]
    pub fn test_jp_nz_nn_zero() {
        let mut cpu = Cpu::new();
        cpu.reg.pc = 0xC000;
        cpu.reg.set_flag(Flag::Zero);
        (&JP_NZ_NN.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0xC2));
        assert_eq!(cpu.reg.pc, 0xC000);
    }

    #[test]
    pub fn test_jp_nz_nn_not_zero() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.pc = 0xC000;
        cpu.reg.clear_flag(Flag::Zero);
        mmu.set_word(0xC001 as usize, 0x0001);
        (&JP_NZ_NN.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xC2));
        assert_eq!(cpu.reg.pc, 0x0001);
    }

    #[test]
    pub fn test_get_instruction_jp_nn() {
        let instruction = get_instruction(&0xC3).unwrap();
        assert_eq!(instruction, &JP_NN);
        assert_eq!(instruction.length, 3);
        assert_eq!(instruction.clock_cycles, 16);
    }

    #[test]
    pub fn test_jp_nn() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.pc = 0xC000;
        mmu.set_word(0xC001 as usize, 0x0001);
        (&JP_NN.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xC3));
        assert_eq!(cpu.reg.pc, 0x0001);
    }

    #[test]
    pub fn test_get_instruction_call_nz_nn() {
        let instruction = get_instruction(&0xC4).unwrap();
        assert_eq!(instruction, &CALL_NZ_NN);
        assert_eq!(instruction.length, 3);
        assert_eq!(instruction.clock_cycles, 12);
        assert_eq!(instruction.clock_cycles_condition.unwrap(), 24);
    }

    #[test]
    pub fn test_call_nz_nn_zero() {
        let mut cpu = Cpu::new();
        cpu.reg.pc = 0xC000;
        cpu.reg.set_flag(Flag::Zero);
        (&CALL_NZ_NN.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0xC4));
        assert_eq!(cpu.reg.pc, 0xC000);
    }

    #[test]
    pub fn test_call_nz_nn_not_zero() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.pc = 0xC000;
        cpu.reg.clear_flag(Flag::Zero);
        mmu.set_word(0xC001 as usize, 0x0001);
        (&CALL_NZ_NN.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xC4));
        assert_eq!(cpu.reg.pc, 0x0001);
    }

    #[test]
    pub fn test_get_instruction_push_bc() {
        let instruction = get_instruction(&0xC5).unwrap();
        assert_eq!(instruction, &PUSH_BC);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 16);
    }

    #[test]
    pub fn test_push_bc() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.set_bc(0x0001);
        cpu.reg.sp = 0xC002;
        (&PUSH_BC.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xC5));
        assert_eq!(mmu.get_word(0xC000 as usize), 0x0001);
        assert_eq!(cpu.reg.sp, 0xC000);
    }

    #[test]
    pub fn test_get_instruction_add_n() {
        let instruction = get_instruction(&0xC6).unwrap();
        assert_eq!(instruction, &ADD_N);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_add_n() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.a = 0x01;
        cpu.reg.pc = 0xC000;
        mmu.set_byte(0xC001 as usize, 0x01);
        (&ADD_N.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xC6));
        assert_eq!(cpu.reg.a, 0x02);
    }

    #[test]
    pub fn test_get_instruction_rst_00() {
        let instruction = get_instruction(&0xC7).unwrap();
        assert_eq!(instruction, &RST_00);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 16);
    }

    #[test]
    pub fn test_rst_00() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.pc = 0xC000;
        cpu.reg.sp = 0xC002;
        (&RST_00.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xC7));
        assert_eq!(mmu.get_word(0xC000 as usize), 0xC001);
        assert_eq!(cpu.reg.sp, 0xC000);
        assert_eq!(cpu.reg.pc, 0x0000);
    }

    #[test]
    pub fn test_get_instruction_ret_z() {
        let instruction = get_instruction(&0xC8).unwrap();
        assert_eq!(instruction, &RET_Z);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 8);
        assert_eq!(instruction.clock_cycles_condition.unwrap(), 20);
    }

    #[test]
    pub fn test_ret_z_zero() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.sp = 0xC000;
        cpu.reg.pc = 0xC000;
        mmu.set_word(0xC000 as usize, 0x0001);
        cpu.reg.set_flag(Flag::Zero);
        (&RET_Z.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xC8));
        assert_eq!(cpu.reg.pc, 0x0001);
        assert_eq!(cpu.reg.sp, 0xC002);
    }

    #[test]
    pub fn test_ret_z_not_zero() {
        let mut cpu = Cpu::new();
        cpu.reg.pc = 0xC000;
        cpu.reg.sp = 0xC000;
        cpu.reg.clear_flag(Flag::Zero);
        (&RET_Z.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0xC8));
        assert_eq!(cpu.reg.pc, 0xC000);
        assert_eq!(cpu.reg.sp, 0xC000);
    }

    #[test]
    pub fn test_get_instruction_ret() {
        let instruction = get_instruction(&0xC9).unwrap();
        assert_eq!(instruction, &RET);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 16);
    }

    #[test]
    pub fn test_ret() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.sp = 0xC000;
        cpu.reg.pc = 0xC000;
        mmu.set_word(0xC000 as usize, 0x0001);
        (&RET.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xC9));
        assert_eq!(cpu.reg.pc, 0x0001);
        assert_eq!(cpu.reg.sp, 0xC002);
    }

    #[test]
    pub fn test_get_instruction_jp_z_nn() {
        let instruction = get_instruction(&0xCA).unwrap();
        assert_eq!(instruction, &JP_Z_NN);
        assert_eq!(instruction.length, 3);
        assert_eq!(instruction.clock_cycles, 12);
        assert_eq!(instruction.clock_cycles_condition.unwrap(), 16);
    }

    #[test]
    pub fn test_jp_z_nn_zero() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.pc = 0xC000;
        cpu.reg.set_flag(Flag::Zero);
        mmu.set_word(0xC001 as usize, 0x0001);
        (&JP_Z_NN.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xCA));
        assert_eq!(cpu.reg.pc, 0x0001);
    }

    #[test]
    pub fn test_jp_z_nn_not_zero() {
        let mut cpu = Cpu::new();
        cpu.reg.pc = 0xC000;
        cpu.reg.clear_flag(Flag::Zero);
        (&JP_Z_NN.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0xCA));
        assert_eq!(cpu.reg.pc, 0xC000);
    }

    #[test]
    pub fn test_get_instruction_call_z_nn() {
        let instruction = get_instruction(&0xCC).unwrap();
        assert_eq!(instruction, &CALL_Z_NN);
        assert_eq!(instruction.length, 3);
        assert_eq!(instruction.clock_cycles, 12);
        assert_eq!(instruction.clock_cycles_condition.unwrap(), 24);
    }

    #[test]
    pub fn test_call_z_nn_zero() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.pc = 0xC000;
        cpu.reg.set_flag(Flag::Zero);
        mmu.set_word(0xC001 as usize, 0x0001);
        (&CALL_Z_NN.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xCC));
        assert_eq!(cpu.reg.pc, 0x0001);
    }

    #[test]
    pub fn test_call_z_nn_not_zero() {
        let mut cpu = Cpu::new();
        cpu.reg.pc = 0xC000;
        cpu.reg.clear_flag(Flag::Zero);
        (&CALL_Z_NN.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0xCC));
        assert_eq!(cpu.reg.pc, 0xC000);
    }

    #[test]
    pub fn test_get_instruction_call_nn() {
        let instruction = get_instruction(&0xCD).unwrap();
        assert_eq!(instruction, &CALL_NN);
        assert_eq!(instruction.length, 3);
        assert_eq!(instruction.clock_cycles, 24);
    }

    #[test]
    pub fn test_call_nn() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.pc = 0xC000;
        mmu.set_word(0xC001 as usize, 0x0001);
        (&CALL_NN.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xCD));
        assert_eq!(cpu.reg.pc, 0x0001);
    }

    #[test]
    pub fn test_get_instruction_adc_n() {
        let instruction = get_instruction(&0xCE).unwrap();
        assert_eq!(instruction, &ADC_N);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_adc_n() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.a = 0x01;
        cpu.reg.pc = 0xC000;
        cpu.reg.set_flag(Flag::Carry);
        mmu.set_byte(0xC001 as usize, 0x01);
        (&ADC_N.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xCE));
        assert_eq!(cpu.reg.a, 0x03);
    }

    #[test]
    pub fn test_get_instruction_rst_08() {
        let instruction = get_instruction(&0xCF).unwrap();
        assert_eq!(instruction, &RST_08);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 16);
    }

    #[test]
    pub fn test_rst_08() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.pc = 0xC000;
        cpu.reg.sp = 0xC002;
        (&RST_08.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xCF));
        assert_eq!(mmu.get_word(0xC000 as usize), 0xC001);
        assert_eq!(cpu.reg.sp, 0xC000);
        assert_eq!(cpu.reg.pc, 0x0008);
    }

    #[test]
    pub fn test_get_instruction_ret_nc() {
        let instruction = get_instruction(&0xD0).unwrap();
        assert_eq!(instruction, &RET_NC);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 8);
        assert_eq!(instruction.clock_cycles_condition.unwrap(), 20);
    }

    #[test]
    pub fn test_ret_nc_carry() {
        let mut cpu = Cpu::new();
        cpu.reg.sp = 0xC000;
        cpu.reg.pc = 0xC000;
        cpu.reg.set_flag(Flag::Carry);
        (&RET_NC.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0xD0));
        assert_eq!(cpu.reg.pc, 0xC000);
        assert_eq!(cpu.reg.sp, 0xC000);
    }

    #[test]
    pub fn test_ret_nc_not_carry() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.sp = 0xC000;
        cpu.reg.pc = 0xC000;
        mmu.set_word(0xC000 as usize, 0x0001);
        cpu.reg.clear_flag(Flag::Carry);
        (&RET_NC.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xD0));
        assert_eq!(cpu.reg.pc, 0x0001);
        assert_eq!(cpu.reg.sp, 0xC002);
    }

    #[test]
    pub fn test_get_instruction_pop_de() {
        let instruction = get_instruction(&0xD1).unwrap();
        assert_eq!(instruction, &POP_DE);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 12);
    }

    #[test]
    pub fn test_pop_de() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.sp = 0xC000;
        mmu.set_word(0xC000 as usize, 0x0001);
        (&POP_DE.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xD1));
        assert_eq!(cpu.reg.de(), 0x0001);
        assert_eq!(cpu.reg.sp, 0xC002);
    }

    #[test]
    pub fn test_get_instruction_jp_nc_nn() {
        let instruction = get_instruction(&0xD2).unwrap();
        assert_eq!(instruction, &JP_NC_NN);
        assert_eq!(instruction.length, 3);
        assert_eq!(instruction.clock_cycles, 12);
        assert_eq!(instruction.clock_cycles_condition.unwrap(), 16);
    }

    #[test]
    pub fn test_jp_nc_nn_carry() {
        let mut cpu = Cpu::new();
        cpu.reg.pc = 0xC000;
        cpu.reg.set_flag(Flag::Carry);
        (&JP_NC_NN.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0xD2));
        assert_eq!(cpu.reg.pc, 0xC000);
    }

    #[test]
    pub fn test_jp_nc_nn_not_carry() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.pc = 0xC000;
        cpu.reg.clear_flag(Flag::Carry);
        mmu.set_word(0xC001 as usize, 0x0001);
        (&JP_NC_NN.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xD2));
        assert_eq!(cpu.reg.pc, 0x0001);
    }

    #[test]
    pub fn test_get_instruction_call_nc_nn() {
        let instruction = get_instruction(&0xD4).unwrap();
        assert_eq!(instruction, &CALL_NC_NN);
        assert_eq!(instruction.length, 3);
        assert_eq!(instruction.clock_cycles, 12);
        assert_eq!(instruction.clock_cycles_condition.unwrap(), 24);
    }

    #[test]
    pub fn test_call_nc_nn_carry() {
        let mut cpu = Cpu::new();
        cpu.reg.pc = 0xC000;
        cpu.reg.set_flag(Flag::Carry);
        (&CALL_NC_NN.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0xD4));
        assert_eq!(cpu.reg.pc, 0xC000);
    }

    #[test]
    pub fn test_call_nc_nn_not_carry() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.pc = 0xC000;
        cpu.reg.clear_flag(Flag::Carry);
        mmu.set_word(0xC001 as usize, 0x0001);
        (&CALL_NC_NN.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xD4));
        assert_eq!(cpu.reg.pc, 0x0001);
    }

    #[test]
    pub fn test_get_instruction_push_de() {
        let instruction = get_instruction(&0xD5).unwrap();
        assert_eq!(instruction, &PUSH_DE);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 16);
    }

    #[test]
    pub fn test_push_de() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.set_de(0x0001);
        cpu.reg.sp = 0xC002;
        (&PUSH_DE.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xD5));
        assert_eq!(mmu.get_word(0xC000 as usize), 0x0001);
        assert_eq!(cpu.reg.sp, 0xC000);
    }

    #[test]
    pub fn test_get_instruction_sub_n() {
        let instruction = get_instruction(&0xD6).unwrap();
        assert_eq!(instruction, &SUB_N);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_sub_n() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.a = 0x02;
        cpu.reg.pc = 0xC000;
        mmu.set_byte(0xC001 as usize, 0x01);
        (&SUB_N.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xD6));
        assert_eq!(cpu.reg.a, 0x01);
    }

    #[test]
    pub fn test_get_instruction_rst_10() {
        let instruction = get_instruction(&0xD7).unwrap();
        assert_eq!(instruction, &RST_10);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 16);
    }

    #[test]
    pub fn test_rst_10() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.pc = 0xC000;
        cpu.reg.sp = 0xC002;
        (&RST_10.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xD7));
        assert_eq!(mmu.get_word(0xC000 as usize), 0xC001);
        assert_eq!(cpu.reg.sp, 0xC000);
        assert_eq!(cpu.reg.pc, 0x0010);
    }

    #[test]
    pub fn test_get_instruction_ret_c() {
        let instruction = get_instruction(&0xD8).unwrap();
        assert_eq!(instruction, &RET_C);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 8);
        assert_eq!(instruction.clock_cycles_condition.unwrap(), 20);
    }

    #[test]
    pub fn test_ret_c_carry() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.sp = 0xC000;
        cpu.reg.pc = 0xC000;
        mmu.set_word(0xC000 as usize, 0x0001);
        cpu.reg.set_flag(Flag::Carry);
        (&RET_C.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xD8));
        assert_eq!(cpu.reg.pc, 0x0001);
        assert_eq!(cpu.reg.sp, 0xC002);
    }

    #[test]
    pub fn test_ret_c_not_carry() {
        let mut cpu = Cpu::new();
        cpu.reg.pc = 0xC000;
        cpu.reg.sp = 0xC000;
        cpu.reg.clear_flag(Flag::Carry);
        (&RET_C.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0xD8));
        assert_eq!(cpu.reg.pc, 0xC000);
        assert_eq!(cpu.reg.sp, 0xC000);
    }

    #[test]
    pub fn test_get_instruction_reti() {
        let instruction = get_instruction(&0xD9).unwrap();
        assert_eq!(instruction, &RETI);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 16);
    }

    #[test]
    pub fn test_reti() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.sp = 0xC000;
        cpu.reg.pc = 0xC000;
        mmu.set_word(0xC000 as usize, 0x0001);
        (&RETI.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xD9));
        assert_eq!(cpu.reg.pc, 0x0001);
        assert_eq!(cpu.reg.sp, 0xC002);
        assert!(cpu.ime);
    }

    #[test]
    pub fn test_get_instruction_jp_c_nn() {
        let instruction = get_instruction(&0xDA).unwrap();
        assert_eq!(instruction, &JP_C_NN);
        assert_eq!(instruction.length, 3);
        assert_eq!(instruction.clock_cycles, 12);
        assert_eq!(instruction.clock_cycles_condition.unwrap(), 16);
    }

    #[test]
    pub fn test_jp_c_nn_carry() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.pc = 0xC000;
        cpu.reg.set_flag(Flag::Carry);
        mmu.set_word(0xC001 as usize, 0x0001);
        (&JP_C_NN.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xDA));
        assert_eq!(cpu.reg.pc, 0x0001);
    }

    #[test]
    pub fn test_jp_c_nn_not_carry() {
        let mut cpu = Cpu::new();
        cpu.reg.pc = 0xC000;
        cpu.reg.clear_flag(Flag::Carry);
        (&JP_C_NN.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0xDA));
        assert_eq!(cpu.reg.pc, 0xC000);
    }

    #[test]
    pub fn test_get_instruction_call_c_nn() {
        let instruction = get_instruction(&0xDC).unwrap();
        assert_eq!(instruction, &CALL_C_NN);
        assert_eq!(instruction.length, 3);
        assert_eq!(instruction.clock_cycles, 12);
        assert_eq!(instruction.clock_cycles_condition.unwrap(), 24);
    }

    #[test]
    pub fn test_call_c_nn_carry() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.pc = 0xC000;
        cpu.reg.set_flag(Flag::Carry);
        mmu.set_word(0xC001 as usize, 0x0001);
        (&CALL_C_NN.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xDC));
        assert_eq!(cpu.reg.pc, 0x0001);
    }

    #[test]
    pub fn test_call_c_nn_not_carry() {
        let mut cpu = Cpu::new();
        cpu.reg.pc = 0xC000;
        cpu.reg.clear_flag(Flag::Carry);
        (&CALL_C_NN.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0xDC));
        assert_eq!(cpu.reg.pc, 0xC000);
    }

    #[test]
    pub fn test_get_instruction_sbc_n() {
        let instruction = get_instruction(&0xDE).unwrap();
        assert_eq!(instruction, &SBC_N);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_sbc_n() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.a = 0x02;
        cpu.reg.pc = 0xC000;
        cpu.reg.set_flag(Flag::Carry);
        mmu.set_byte(0xC001 as usize, 0x01);
        (&SBC_N.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xDE));
        assert_eq!(cpu.reg.a, 0x00);
    }

    #[test]
    pub fn test_get_instruction_rst_18() {
        let instruction = get_instruction(&0xDF).unwrap();
        assert_eq!(instruction, &RST_18);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 16);
    }

    #[test]
    pub fn test_rst_18() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.pc = 0xC000;
        cpu.reg.sp = 0xC002;
        (&RST_18.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xDF));
        assert_eq!(mmu.get_word(0xC000 as usize), 0xC001);
        assert_eq!(cpu.reg.sp, 0xC000);
        assert_eq!(cpu.reg.pc, 0x0018);
    }

    #[test]
    pub fn test_get_instruction_ldh_n_a() {
        let instruction = get_instruction(&0xE0).unwrap();
        assert_eq!(instruction, &LDH_N_A);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 12);
    }

    #[test]
    pub fn test_ldh_n_a() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.a = 0x01;
        cpu.reg.pc = 0xC000;
        mmu.set_byte(0xC001 as usize, 0x01);
        (&LDH_N_A.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xE0));
        assert_eq!(mmu.get_byte(0xFF01 as usize), 0x01);
    }

    #[test]
    pub fn test_get_instruction_pop_hl() {
        let instruction = get_instruction(&0xE1).unwrap();
        assert_eq!(instruction, &POP_HL);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 12);
    }

    #[test]
    pub fn test_pop_hl() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.sp = 0xC000;
        mmu.set_word(0xC000 as usize, 0x0001);
        (&POP_HL.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xE1));
        assert_eq!(cpu.reg.hl(), 0x0001);
        assert_eq!(cpu.reg.sp, 0xC002);
    }

    #[test]
    pub fn test_get_instruction_ldh_c_a() {
        let instruction = get_instruction(&0xE2).unwrap();
        assert_eq!(instruction, &LDH_C_A);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_ldh_c_a() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.a = 0x01;
        cpu.reg.c = 0x01;
        (&LDH_C_A.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xE2));
        assert_eq!(mmu.get_byte(0xFF01 as usize), 0x01);
    }

    #[test]
    pub fn test_get_instruction_push_hl() {
        let instruction = get_instruction(&0xE5).unwrap();
        assert_eq!(instruction, &PUSH_HL);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 16);
    }

    #[test]
    pub fn test_push_hl() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.set_hl(0x0001);
        cpu.reg.sp = 0xC002;
        (&PUSH_HL.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xE5));
        assert_eq!(mmu.get_word(0xC000 as usize), 0x0001);
        assert_eq!(cpu.reg.sp, 0xC000);
    }

    #[test]
    pub fn test_get_instruction_and_n() {
        let instruction = get_instruction(&0xE6).unwrap();
        assert_eq!(instruction, &AND_N);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_and_n() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.a = 0x03;
        cpu.reg.pc = 0xC000;
        mmu.set_byte(0xC001 as usize, 0x01);
        (&AND_N.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xE6));
        assert_eq!(cpu.reg.a, 0x01);
    }

    #[test]
    pub fn test_get_instruction_rst_20() {
        let instruction = get_instruction(&0xE7).unwrap();
        assert_eq!(instruction, &RST_20);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 16);
    }

    #[test]
    pub fn test_rst_20() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.pc = 0xC000;
        cpu.reg.sp = 0xC002;
        (&RST_20.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xE7));
        assert_eq!(mmu.get_word(0xC000 as usize), 0xC001);
        assert_eq!(cpu.reg.sp, 0xC000);
        assert_eq!(cpu.reg.pc, 0x0020);
    }

    #[test]
    pub fn test_get_instruction_add_sp_n() {
        let instruction = get_instruction(&0xE8).unwrap();
        assert_eq!(instruction, &ADD_SP_N);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 16);
    }

    #[test]
    pub fn test_add_sp_n() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.sp = 0x0001;
        cpu.reg.pc = 0xC000;
        mmu.set_byte(0xC001 as usize, 0x01);
        (&ADD_SP_N.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xE8));
        assert_eq!(cpu.reg.sp, 0x0002);
    }

    #[test]
    pub fn test_get_instruction_jp_hl() {
        let instruction = get_instruction(&0xE9).unwrap();
        assert_eq!(instruction, &JP_HL);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_jp_hl() {
        let mut cpu = Cpu::new();
        cpu.reg.set_hl(0x0001);
        (&JP_HL.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0xE9));
        assert_eq!(cpu.reg.pc, 0x0001);
    }

    #[test]
    pub fn test_get_instruction_ld_nn_a() {
        let instruction = get_instruction(&0xEA).unwrap();
        assert_eq!(instruction, &LD_NN_A);
        assert_eq!(instruction.length, 3);
        assert_eq!(instruction.clock_cycles, 16);
    }

    #[test]
    pub fn test_ld_nn_a() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.a = 0x01;
        cpu.reg.pc = 0xC000;
        mmu.set_word(0xC001 as usize, 0xFF01);
        (&LD_NN_A.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xEA));
        assert_eq!(mmu.get_byte(0xFF01 as usize), 0x01);
    }

    #[test]
    pub fn test_get_instruction_xor_n() {
        let instruction = get_instruction(&0xEE).unwrap();
        assert_eq!(instruction, &XOR_N);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_xor_n() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.a = 0x03;
        cpu.reg.pc = 0xC000;
        mmu.set_byte(0xC001 as usize, 0x01);
        (&XOR_N.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xEE));
        assert_eq!(cpu.reg.a, 0x02);
    }

    #[test]
    pub fn test_get_instruction_rst_28() {
        let instruction = get_instruction(&0xEF).unwrap();
        assert_eq!(instruction, &RST_28);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 16);
    }

    #[test]
    pub fn test_rst_28() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.pc = 0xC000;
        cpu.reg.sp = 0xC002;
        (&RST_28.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xEF));
        assert_eq!(mmu.get_word(0xC000 as usize), 0xC001);
        assert_eq!(cpu.reg.sp, 0xC000);
        assert_eq!(cpu.reg.pc, 0x0028);
    }

    #[test]
    pub fn test_get_instruction_ldh_a_n() {
        let instruction = get_instruction(&0xF0).unwrap();
        assert_eq!(instruction, &LDH_A_N);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 12);
    }

    #[test]
    pub fn test_ldh_a_n() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.pc = 0xC000;
        mmu.set_byte(0xC001 as usize, 0x01);
        mmu.set_byte(0xFF01 as usize, 0x01);
        (&LDH_A_N.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xF0));
        assert_eq!(cpu.reg.a, 0x01);
    }

    #[test]
    pub fn test_get_instruction_pop_af() {
        let instruction = get_instruction(&0xF1).unwrap();
        assert_eq!(instruction, &POP_AF);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 12);
    }

    #[test]
    pub fn test_pop_af() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.sp = 0xC000;
        mmu.set_word(0xC000 as usize, 0x0110);
        (&POP_AF.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xF1));
        assert_eq!(cpu.reg.af(), 0x0110);
        assert_eq!(cpu.reg.sp, 0xC002);
    }

    #[test]
    pub fn test_get_instruction_ldh_a_c() {
        let instruction = get_instruction(&0xF2).unwrap();
        assert_eq!(instruction, &LDH_A_C);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_ldh_a_c() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.c = 0x01;
        mmu.set_byte(0xFF01 as usize, 0x01);
        (&LDH_A_C.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xF2));
        assert_eq!(cpu.reg.a, 0x01);
    }

    #[test]
    pub fn test_get_instruction_di() {
        let instruction = get_instruction(&0xF3).unwrap();
        assert_eq!(instruction, &DI);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 4);
    }

    #[test]
    pub fn test_di() {
        let mut cpu = Cpu::new();
        cpu.ime = true;
        (&DI.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0xF3));
        assert_eq!(cpu.ime, false);
    }

    #[test]
    pub fn test_get_instruction_push_af() {
        let instruction = get_instruction(&0xF5).unwrap();
        assert_eq!(instruction, &PUSH_AF);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 16);
    }

    #[test]
    pub fn test_push_af() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.set_af(0x0110);
        cpu.reg.sp = 0xC002;
        (&PUSH_AF.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xF5));
        assert_eq!(mmu.get_word(0xC000 as usize), 0x0110);
        assert_eq!(cpu.reg.sp, 0xC000);
    }

    #[test]
    pub fn test_get_instruction_or_n() {
        let instruction = get_instruction(&0xF6).unwrap();
        assert_eq!(instruction, &OR_N);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_or_n() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.a = 0x02;
        cpu.reg.pc = 0xC000;
        mmu.set_byte(0xC001 as usize, 0x01);
        (&OR_N.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xF6));
        assert_eq!(cpu.reg.a, 0x03);
    }

    #[test]
    pub fn test_get_instruction_rst_30() {
        let instruction = get_instruction(&0xF7).unwrap();
        assert_eq!(instruction, &RST_30);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 16);
    }

    #[test]
    pub fn test_rst_30() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.pc = 0xC000;
        cpu.reg.sp = 0xC002;
        (&RST_30.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xF7));
        assert_eq!(mmu.get_word(0xC000 as usize), 0xC001);
        assert_eq!(cpu.reg.sp, 0xC000);
        assert_eq!(cpu.reg.pc, 0x0030);
    }

    #[test]
    pub fn test_get_instruction_ld_hl_sp_n() {
        let instruction = get_instruction(&0xF8).unwrap();
        assert_eq!(instruction, &LD_HL_SP_N);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 12);
    }

    #[test]
    pub fn test_ld_hl_sp_n() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.sp = 0x0001;
        cpu.reg.pc = 0xC000;
        mmu.set_byte(0xC001 as usize, 0x01);
        (&LD_HL_SP_N.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xF8));
        assert_eq!(cpu.reg.hl(), 0x0002);
    }

    #[test]
    pub fn test_get_instruction_ld_sp_hl() {
        let instruction = get_instruction(&0xF9).unwrap();
        assert_eq!(instruction, &LD_SP_HL);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_ld_sp_hl() {
        let mut cpu = Cpu::new();
        cpu.reg.set_hl(0x0001);
        (&LD_SP_HL.handler)(&mut cpu, &mut Memory::new(), &OpCode::Regular(0xF9));
        assert_eq!(cpu.reg.sp, 0x0001);
    }

    #[test]
    pub fn test_get_instruction_ld_a_nn() {
        let instruction = get_instruction(&0xFA).unwrap();
        assert_eq!(instruction, &LD_A_NN);
        assert_eq!(instruction.length, 3);
        assert_eq!(instruction.clock_cycles, 16);
    }

    #[test]
    pub fn test_ld_a_nn() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.pc = 0xC000;
        mmu.set_word(0xC001 as usize, 0xFF01);
        mmu.set_byte(0xFF01 as usize, 0x01);
        (&LD_A_NN.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xFA));
        assert_eq!(cpu.reg.a, 0x01);
    }

    #[test]
    pub fn test_get_instruction_cp_n() {
        let instruction = get_instruction(&0xFE).unwrap();
        assert_eq!(instruction, &CP_N);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_cp_n() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.a = 0x01;
        cpu.reg.pc = 0xC000;
        mmu.set_byte(0xC001 as usize, 0x02);
        (&CP_N.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xFE));
        assert!(cpu.reg.check_flag(Flag::Carry));
    }

    #[test]
    pub fn test_get_instruction_rst_38() {
        let instruction = get_instruction(&0xFF).unwrap();
        assert_eq!(instruction, &RST_38);
        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.clock_cycles, 16);
    }

    #[test]
    pub fn test_rst_38() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.pc = 0xC000;
        cpu.reg.sp = 0xC002;
        (&RST_38.handler)(&mut cpu, &mut mmu, &OpCode::Regular(0xFF));
        assert_eq!(mmu.get_word(0xC000 as usize), 0xC001);
        assert_eq!(cpu.reg.sp, 0xC000);
        assert_eq!(cpu.reg.pc, 0x0038);
    }
}
