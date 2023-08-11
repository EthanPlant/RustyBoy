use crate::cpu::cpu::Cpu;
use crate::cpu::instructions::{functions, Instruction, InstructionType, OpCode};
use crate::mmu::Memory;

/// 0x00 - RLC B
const RLC_B: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RLC B",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.b = functions::rlc(cpu, cpu.reg.b);
        InstructionType::ActionTaken
    },
};

/// 0x01 - RLC C
const RLC_C: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RLC C",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.c = functions::rlc(cpu, cpu.reg.c);
        InstructionType::ActionTaken
    },
};

/// 0x02 - RLC D
const RLC_D: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RLC D",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.d = functions::rlc(cpu, cpu.reg.d);
        InstructionType::ActionTaken
    },
};

/// 0x03 - RLC E
const RLC_E: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RLC E",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.e = functions::rlc(cpu, cpu.reg.e);
        InstructionType::ActionTaken
    },
};

/// 0x04 - RLC H
const RLC_H: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RLC H",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.h = functions::rlc(cpu, cpu.reg.h);
        InstructionType::ActionTaken
    },
};

/// 0x05 - RLC L
const RLC_L: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RLC L",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.l = functions::rlc(cpu, cpu.reg.l);
        InstructionType::ActionTaken
    },
};

/// 0x06 - RLC (HL)
const RLC_HL: Instruction = Instruction {
    length: 2,
    clock_cycles: 16,
    clock_cycles_condition: None,
    description: "RLC (HL)",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        let value = functions::get_hl(cpu, mmu);
        let result = functions::rlc(cpu, value);
        mmu.set_byte(cpu.reg.hl(), result);
        InstructionType::ActionTaken
    },
};

/// 0x07 - RLC A
const RLC_A: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RLC A",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.a = functions::rlc(cpu, cpu.reg.a);
        InstructionType::ActionTaken
    },
};

/// 0x08 - RRC B
const RRC_B: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RRC B",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.b = functions::rrc(cpu, cpu.reg.b);
        InstructionType::ActionTaken
    },
};

/// 0x09 - RRC C
const RRC_C: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RRC C",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.c = functions::rrc(cpu, cpu.reg.c);
        InstructionType::ActionTaken
    },
};

/// 0x0A - RRC D
const RRC_D: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RRC D",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.d = functions::rrc(cpu, cpu.reg.d);
        InstructionType::ActionTaken
    },
};

/// 0x0B - RRC E
const RRC_E: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RRC E",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.e = functions::rrc(cpu, cpu.reg.e);
        InstructionType::ActionTaken
    },
};

/// 0x0C - RRC H
const RRC_H: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RRC H",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.h = functions::rrc(cpu, cpu.reg.h);
        InstructionType::ActionTaken
    },
};

/// 0x0D - RRC L
const RRC_L: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RRC L",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.l = functions::rrc(cpu, cpu.reg.l);
        InstructionType::ActionTaken
    },
};

/// 0x0E - RRC (HL)
const RRC_HL: Instruction = Instruction {
    length: 2,
    clock_cycles: 16,
    clock_cycles_condition: None,
    description: "RRC (HL)",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        let value = functions::get_hl(cpu, mmu);
        let result = functions::rrc(cpu, value);
        mmu.set_byte(cpu.reg.hl(), result);
        InstructionType::ActionTaken
    },
};

/// 0x0F - RRC A
const RRC_A: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RRC A",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.a = functions::rrc(cpu, cpu.reg.a);
        InstructionType::ActionTaken
    },
};

/// 0x10 - RL B
const RL_B: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RL B",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.b = functions::rl(cpu, cpu.reg.b);
        InstructionType::ActionTaken
    },
};

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

/// 0x12 - RL D
const RL_D: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RL D",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.d = functions::rl(cpu, cpu.reg.d);
        InstructionType::ActionTaken
    },
};

/// 0x13 - RL E
const RL_E: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RL E",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.e = functions::rl(cpu, cpu.reg.e);
        InstructionType::ActionTaken
    },
};

/// 0x14 - RL H
const RL_H: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RL H",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.h = functions::rl(cpu, cpu.reg.h);
        InstructionType::ActionTaken
    },
};

/// 0x15 - RL L
const RL_L: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RL L",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.l = functions::rl(cpu, cpu.reg.l);
        InstructionType::ActionTaken
    },
};

/// 0x16 - RL (HL)
const RL_HL: Instruction = Instruction {
    length: 2,
    clock_cycles: 16,
    clock_cycles_condition: None,
    description: "RL (HL)",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        let value = functions::get_hl(cpu, mmu);
        let result = functions::rl(cpu, value);
        mmu.set_byte(cpu.reg.hl(), result);
        InstructionType::ActionTaken
    },
};

/// 0x17 - RL A
const RL_A: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RL A",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.a = functions::rl(cpu, cpu.reg.a);
        InstructionType::ActionTaken
    },
};

/// 0x18 - RR B
const RR_B: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RR B",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.b = functions::rr(cpu, cpu.reg.b);
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

/// 0x1B - RR E
const RR_E: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RR E",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.e = functions::rr(cpu, cpu.reg.e);
        InstructionType::ActionTaken
    },
};

/// 0x1C - RR H
const RR_H: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RR H",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.h = functions::rr(cpu, cpu.reg.h);
        InstructionType::ActionTaken
    },
};

/// 0x1D - RR L
const RR_L: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RR L",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.l = functions::rr(cpu, cpu.reg.l);
        InstructionType::ActionTaken
    },
};

/// 0x1E - RR (HL)
const RR_HL: Instruction = Instruction {
    length: 2,
    clock_cycles: 16,
    clock_cycles_condition: None,
    description: "RR (HL)",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        let value = functions::get_hl(cpu, mmu);
        let result = functions::rr(cpu, value);
        mmu.set_byte(cpu.reg.hl(), result);
        InstructionType::ActionTaken
    },
};

/// 0x1F - RR A
const RR_A: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RR A",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.a = functions::rr(cpu, cpu.reg.a);
        InstructionType::ActionTaken
    },
};

/// 0x20 - SLA B
const SLA_B: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SLA B",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.b = functions::sla(cpu, cpu.reg.b);
        InstructionType::ActionTaken
    },
};

/// 0x21 - SLA C
const SLA_C: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SLA C",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.c = functions::sla(cpu, cpu.reg.c);
        InstructionType::ActionTaken
    },
};

/// 0x22 - SLA D
const SLA_D: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SLA D",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.d = functions::sla(cpu, cpu.reg.d);
        InstructionType::ActionTaken
    },
};

/// 0x23 - SLA E
const SLA_E: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SLA E",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.e = functions::sla(cpu, cpu.reg.e);
        InstructionType::ActionTaken
    },
};

/// 0x24 - SLA H
const SLA_H: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SLA H",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.h = functions::sla(cpu, cpu.reg.h);
        InstructionType::ActionTaken
    },
};

/// 0x25 - SLA L
const SLA_L: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SLA L",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.l = functions::sla(cpu, cpu.reg.l);
        InstructionType::ActionTaken
    },
};

/// 0x26 - SLA (HL)
const SLA_HL: Instruction = Instruction {
    length: 2,
    clock_cycles: 16,
    clock_cycles_condition: None,
    description: "SLA (HL)",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        let value = functions::get_hl(cpu, mmu);
        let result = functions::sla(cpu, value);
        mmu.set_byte(cpu.reg.hl(), result);
        InstructionType::ActionTaken
    },
};

/// 0x27 - SLA A
const SLA_A: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SLA A",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.a = functions::sla(cpu, cpu.reg.a);
        InstructionType::ActionTaken
    },
};

/// 0x28 - SRA B
const SRA_B: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SRA B",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.b = functions::sra(cpu, cpu.reg.b);
        InstructionType::ActionTaken
    },
};

/// 0x29 - SRA C
const SRA_C: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SRA C",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.c = functions::sra(cpu, cpu.reg.c);
        InstructionType::ActionTaken
    },
};

/// 0x2A - SRA D
const SRA_D: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SRA D",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.d = functions::sra(cpu, cpu.reg.d);
        InstructionType::ActionTaken
    },
};

/// 0x2B - SRA E
const SRA_E: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SRA E",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.e = functions::sra(cpu, cpu.reg.e);
        InstructionType::ActionTaken
    },
};

/// 0x2C - SRA H
const SRA_H: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SRA H",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.h = functions::sra(cpu, cpu.reg.h);
        InstructionType::ActionTaken
    },
};

/// 0x2D - SRA L
const SRA_L: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SRA L",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.l = functions::sra(cpu, cpu.reg.l);
        InstructionType::ActionTaken
    },
};

/// 0x2E - SRA (HL)
const SRA_HL: Instruction = Instruction {
    length: 2,
    clock_cycles: 16,
    clock_cycles_condition: None,
    description: "SRA (HL)",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        let value = functions::get_hl(cpu, mmu);
        let result = functions::sra(cpu, value);
        mmu.set_byte(cpu.reg.hl(), result);
        InstructionType::ActionTaken
    },
};

/// 0x2F - SRA A
const SRA_A: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SRA A",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.a = functions::sra(cpu, cpu.reg.a);
        InstructionType::ActionTaken
    },
};

/// 0x30 - SWAP B
const SWAP_B: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SWAP B",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.b = functions::swap(cpu, cpu.reg.b);
        InstructionType::ActionTaken
    },
};

/// 0x31 - SWAP C
const SWAP_C: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SWAP C",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.c = functions::swap(cpu, cpu.reg.c);
        InstructionType::ActionTaken
    },
};

/// 0x32 - SWAP D
const SWAP_D: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SWAP D",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.d = functions::swap(cpu, cpu.reg.d);
        InstructionType::ActionTaken
    },
};

/// 0x33 - SWAP E
const SWAP_E: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SWAP E",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.e = functions::swap(cpu, cpu.reg.e);
        InstructionType::ActionTaken
    },
};

/// 0x34 - SWAP H
const SWAP_H: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SWAP H",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.h = functions::swap(cpu, cpu.reg.h);
        InstructionType::ActionTaken
    },
};

/// 0x35 - SWAP L
const SWAP_L: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SWAP L",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.l = functions::swap(cpu, cpu.reg.l);
        InstructionType::ActionTaken
    },
};

/// 0x36 - SWAP (HL)
const SWAP_HL: Instruction = Instruction {
    length: 2,
    clock_cycles: 16,
    clock_cycles_condition: None,
    description: "SWAP (HL)",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        let value = functions::get_hl(cpu, mmu);
        let result = functions::swap(cpu, value);
        mmu.set_byte(cpu.reg.hl(), result);
        InstructionType::ActionTaken
    },
};

/// 0x37 - SWAP A
const SWAP_A: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SWAP A",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.a = functions::swap(cpu, cpu.reg.a);
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

/// 0x39 - SRL C
const SRL_C: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SRL C",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.c = functions::srl(cpu, cpu.reg.c);
        InstructionType::ActionTaken
    },
};

/// 0x3A - SRL D
const SRL_D: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SRL D",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.d = functions::srl(cpu, cpu.reg.d);
        InstructionType::ActionTaken
    },
};

/// 0x3B - SRL E
const SRL_E: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SRL E",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.e = functions::srl(cpu, cpu.reg.e);
        InstructionType::ActionTaken
    },
};

/// 0x3C - SRL H
const SRL_H: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SRL H",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.h = functions::srl(cpu, cpu.reg.h);
        InstructionType::ActionTaken
    },
};

/// 0x3D - SRL L
const SRL_L: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SRL L",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.l = functions::srl(cpu, cpu.reg.l);
        InstructionType::ActionTaken
    },
};

/// 0x3E - SRL (HL)
const SRL_HL: Instruction = Instruction {
    length: 2,
    clock_cycles: 16,
    clock_cycles_condition: None,
    description: "SRL (HL)",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        let value = functions::get_hl(cpu, mmu);
        let result = functions::srl(cpu, value);
        mmu.set_byte(cpu.reg.hl(), result);
        InstructionType::ActionTaken
    },
};

/// 0x3F - SRL A
const SRL_A: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SRL A",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.a = functions::srl(cpu, cpu.reg.a);
        InstructionType::ActionTaken
    },
};

/// 0x40 - BIT 0, B
const BIT_0_B: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "BIT 0 B",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        functions::bit(cpu, cpu.reg.b, 0);
        InstructionType::ActionTaken
    },
};

/// 0x41 - BIT 0, C
const BIT_0_C: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "BIT 0 C",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        functions::bit(cpu, cpu.reg.c, 0);
        InstructionType::ActionTaken
    },
};

/// 0x42 - BIT 0, D
const BIT_0_D: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "BIT 0 D",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        functions::bit(cpu, cpu.reg.d, 0);
        InstructionType::ActionTaken
    },
};

/// 0x43 - BIT 0, E
const BIT_0_E: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "BIT 0 E",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        functions::bit(cpu, cpu.reg.e, 0);
        InstructionType::ActionTaken
    },
};

/// 0x44 - BIT 0, H
const BIT_0_H: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "BIT 0 H",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        functions::bit(cpu, cpu.reg.h, 0);
        InstructionType::ActionTaken
    },
};

/// 0x45 - BIT 0, L
const BIT_0_L: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "BIT 0 L",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        functions::bit(cpu, cpu.reg.l, 0);
        InstructionType::ActionTaken
    },
};

/// 0x46 - BIT 0, (HL)
const BIT_0_HL: Instruction = Instruction {
    length: 2,
    clock_cycles: 16,
    clock_cycles_condition: None,
    description: "BIT 0 (HL)",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        functions::bit(cpu, functions::get_hl(cpu, mmu), 0);
        InstructionType::ActionTaken
    },
};

/// 0x47 - BIT 0, A
const BIT_0_A: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "BIT 0 A",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        functions::bit(cpu, cpu.reg.a, 0);
        InstructionType::ActionTaken
    },
};

/// 0x48 - BIT 1, B
const BIT_1_B: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "BIT 1 B",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        functions::bit(cpu, cpu.reg.b, 1);
        InstructionType::ActionTaken
    },
};

/// 0x49 - BIT 1, C
const BIT_1_C: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "BIT 1 C",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        functions::bit(cpu, cpu.reg.c, 1);
        InstructionType::ActionTaken
    },
};

/// 0x4A - BIT 1, D
const BIT_1_D: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "BIT 1 D",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        functions::bit(cpu, cpu.reg.d, 1);
        InstructionType::ActionTaken
    },
};

/// 0x4B - BIT 1, E
const BIT_1_E: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "BIT 1 E",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        functions::bit(cpu, cpu.reg.e, 1);
        InstructionType::ActionTaken
    },
};

/// 0x4C - BIT 1, H
const BIT_1_H: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "BIT 1 H",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        functions::bit(cpu, cpu.reg.h, 1);
        InstructionType::ActionTaken
    },
};

/// 0x4D - BIT 1, L
const BIT_1_L: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "BIT 1 L",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        functions::bit(cpu, cpu.reg.l, 1);
        InstructionType::ActionTaken
    },
};

/// 0x4E - BIT 1, (HL)
const BIT_1_HL: Instruction = Instruction {
    length: 2,
    clock_cycles: 16,
    clock_cycles_condition: None,
    description: "BIT 1 (HL)",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        functions::bit(cpu, functions::get_hl(cpu, mmu), 1);
        InstructionType::ActionTaken
    },
};

/// 0x4F - BIT 1, A
const BIT_1_A: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "BIT 1 A",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        functions::bit(cpu, cpu.reg.a, 1);
        InstructionType::ActionTaken
    },
};

/// 0x50 - BIT 2, B
const BIT_2_B: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "BIT 2 B",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        functions::bit(cpu, cpu.reg.b, 2);
        InstructionType::ActionTaken
    },
};

/// 0x51 - BIT 2, C
const BIT_2_C: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "BIT 2 C",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        functions::bit(cpu, cpu.reg.c, 2);
        InstructionType::ActionTaken
    },
};

/// 0x52 - BIT 2, D
const BIT_2_D: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "BIT 2 D",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        functions::bit(cpu, cpu.reg.d, 2);
        InstructionType::ActionTaken
    },
};

/// 0x53 - BIT 2, E
const BIT_2_E: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "BIT 2 E",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        functions::bit(cpu, cpu.reg.e, 2);
        InstructionType::ActionTaken
    },
};

/// 0x54 - BIT 2, H
const BIT_2_H: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "BIT 2 H",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        functions::bit(cpu, cpu.reg.h, 2);
        InstructionType::ActionTaken
    },
};

/// 0x55 - BIT 2, L
const BIT_2_L: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "BIT 2 L",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        functions::bit(cpu, cpu.reg.l, 2);
        InstructionType::ActionTaken
    },
};

/// 0x56 - BIT 2, (HL)
const BIT_2_HL: Instruction = Instruction {
    length: 2,
    clock_cycles: 16,
    clock_cycles_condition: None,
    description: "BIT 2 (HL)",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        functions::bit(cpu, functions::get_hl(cpu, mmu), 2);
        InstructionType::ActionTaken
    },
};

/// 0x57 - BIT 2, A
const BIT_2_A: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "BIT 2 A",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        functions::bit(cpu, cpu.reg.a, 2);
        InstructionType::ActionTaken
    },
};

/// 0x58 - BIT 3, B
const BIT_3_B: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "BIT 3 B",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        functions::bit(cpu, cpu.reg.b, 3);
        InstructionType::ActionTaken
    },
};

/// 0x59 - BIT 3, C
const BIT_3_C: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "BIT 3 C",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        functions::bit(cpu, cpu.reg.c, 3);
        InstructionType::ActionTaken
    },
};

/// 0x5A - BIT 3, D
const BIT_3_D: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "BIT 3 D",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        functions::bit(cpu, cpu.reg.d, 3);
        InstructionType::ActionTaken
    },
};

/// 0x5B - BIT 3, E
const BIT_3_E: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "BIT 3 E",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        functions::bit(cpu, cpu.reg.e, 3);
        InstructionType::ActionTaken
    },
};

/// 0x5C - BIT 3, H
const BIT_3_H: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "BIT 3 H",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        functions::bit(cpu, cpu.reg.h, 3);
        InstructionType::ActionTaken
    },
};

/// 0x5D - BIT 3, L
const BIT_3_L: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "BIT 3 L",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        functions::bit(cpu, cpu.reg.l, 3);
        InstructionType::ActionTaken
    },
};

/// 0x5E - BIT 3, (HL)
const BIT_3_HL: Instruction = Instruction {
    length: 2,
    clock_cycles: 16,
    clock_cycles_condition: None,
    description: "BIT 3 (HL)",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        functions::bit(cpu, functions::get_hl(cpu, mmu), 3);
        InstructionType::ActionTaken
    },
};

/// 0x5F - BIT 3, A
const BIT_3_A: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "BIT 3 A",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        functions::bit(cpu, cpu.reg.a, 3);
        InstructionType::ActionTaken
    },
};

/// 0x60 - BIT 4, B
const BIT_4_B: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "BIT 4 B",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        functions::bit(cpu, cpu.reg.b, 4);
        InstructionType::ActionTaken
    },
};

/// 0x61 - BIT 4, C
const BIT_4_C: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "BIT 4 C",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        functions::bit(cpu, cpu.reg.c, 4);
        InstructionType::ActionTaken
    },
};

/// 0x62 - BIT 4, D
const BIT_4_D: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "BIT 4 D",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        functions::bit(cpu, cpu.reg.d, 4);
        InstructionType::ActionTaken
    },
};

/// 0x63 - BIT 4, E
const BIT_4_E: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "BIT 4 E",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        functions::bit(cpu, cpu.reg.e, 4);
        InstructionType::ActionTaken
    },
};

/// 0x64 - BIT 4, H
const BIT_4_H: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "BIT 4 H",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        functions::bit(cpu, cpu.reg.h, 4);
        InstructionType::ActionTaken
    },
};

/// 0x65 - BIT 4, L
const BIT_4_L: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "BIT 4 L",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        functions::bit(cpu, cpu.reg.l, 4);
        InstructionType::ActionTaken
    },
};

/// 0x66 - BIT 4, (HL)
const BIT_4_HL: Instruction = Instruction {
    length: 2,
    clock_cycles: 16,
    clock_cycles_condition: None,
    description: "BIT 4 (HL)",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        functions::bit(cpu, functions::get_hl(cpu, mmu), 4);
        InstructionType::ActionTaken
    },
};

/// 0x67 - BIT 4, A
const BIT_4_A: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "BIT 4 A",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        functions::bit(cpu, cpu.reg.a, 4);
        InstructionType::ActionTaken
    },
};

/// 0x68 - BIT 5, B
const BIT_5_B: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "BIT 5 B",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        functions::bit(cpu, cpu.reg.b, 5);
        InstructionType::ActionTaken
    },
};

/// 0x69 - BIT 5, C
const BIT_5_C: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "BIT 5 C",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        functions::bit(cpu, cpu.reg.c, 5);
        InstructionType::ActionTaken
    },
};

/// 0x6A - BIT 5, D
const BIT_5_D: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "BIT 5 D",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        functions::bit(cpu, cpu.reg.d, 5);
        InstructionType::ActionTaken
    },
};

/// 0x6B - BIT 5, E
const BIT_5_E: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "BIT 5 E",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        functions::bit(cpu, cpu.reg.e, 5);
        InstructionType::ActionTaken
    },
};

/// 0x6C - BIT 5, H
const BIT_5_H: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "BIT 5 H",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        functions::bit(cpu, cpu.reg.h, 5);
        InstructionType::ActionTaken
    },
};

/// 0x6D - BIT 5, L
const BIT_5_L: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "BIT 5 L",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        functions::bit(cpu, cpu.reg.l, 5);
        InstructionType::ActionTaken
    },
};

/// 0x6E - BIT 5, (HL)
const BIT_5_HL: Instruction = Instruction {
    length: 2,
    clock_cycles: 16,
    clock_cycles_condition: None,
    description: "BIT 5 (HL)",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        functions::bit(cpu, functions::get_hl(cpu, mmu), 5);
        InstructionType::ActionTaken
    },
};

/// 0x6F - BIT 5, A
const BIT_5_A: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "BIT 5 A",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        functions::bit(cpu, cpu.reg.a, 5);
        InstructionType::ActionTaken
    },
};

/// 0x70 - BIT 6, B
const BIT_6_B: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "BIT 6 B",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        functions::bit(cpu, cpu.reg.b, 6);
        InstructionType::ActionTaken
    },
};

/// 0x71 - BIT 6, C
const BIT_6_C: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "BIT 6 C",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        functions::bit(cpu, cpu.reg.c, 6);
        InstructionType::ActionTaken
    },
};

/// 0x72 - BIT 6, D
const BIT_6_D: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "BIT 6 D",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        functions::bit(cpu, cpu.reg.d, 6);
        InstructionType::ActionTaken
    },
};

/// 0x73 - BIT 6, E
const BIT_6_E: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "BIT 6 E",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        functions::bit(cpu, cpu.reg.e, 6);
        InstructionType::ActionTaken
    },
};

/// 0x74 - BIT 6, H
const BIT_6_H: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "BIT 6 H",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        functions::bit(cpu, cpu.reg.h, 6);
        InstructionType::ActionTaken
    },
};

/// 0x75 - BIT 6, L
const BIT_6_L: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "BIT 6 L",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        functions::bit(cpu, cpu.reg.l, 6);
        InstructionType::ActionTaken
    },
};

/// 0x76 - BIT 6, (HL)
const BIT_6_HL: Instruction = Instruction {
    length: 2,
    clock_cycles: 16,
    clock_cycles_condition: None,
    description: "BIT 6 (HL)",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        functions::bit(cpu, functions::get_hl(cpu, mmu), 6);
        InstructionType::ActionTaken
    },
};

/// 0x77 - BIT 6, A
const BIT_6_A: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "BIT 6 A",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        functions::bit(cpu, cpu.reg.a, 6);
        InstructionType::ActionTaken
    },
};

/// 0x78 - BIT 7, B
const BIT_7_B: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "BIT 7 B",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        functions::bit(cpu, cpu.reg.b, 7);
        InstructionType::ActionTaken
    },
};

/// 0x79 - BIT 7, C
const BIT_7_C: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "BIT 7 C",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        functions::bit(cpu, cpu.reg.c, 7);
        InstructionType::ActionTaken
    },
};

/// 0x7A - BIT 7, D
const BIT_7_D: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "BIT 7 D",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        functions::bit(cpu, cpu.reg.d, 7);
        InstructionType::ActionTaken
    },
};

/// 0x7B - BIT 7, E
const BIT_7_E: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "BIT 7 E",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        functions::bit(cpu, cpu.reg.e, 7);
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

/// 0x7D - BIT 7, L
const BIT_7_L: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "BIT 7 L",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        functions::bit(cpu, cpu.reg.l, 7);
        InstructionType::ActionTaken
    },
};

/// 0x7E - BIT 7, (HL)
const BIT_7_HL: Instruction = Instruction {
    length: 2,
    clock_cycles: 16,
    clock_cycles_condition: None,
    description: "BIT 7 (HL)",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        functions::bit(cpu, functions::get_hl(cpu, mmu), 7);
        InstructionType::ActionTaken
    },
};

/// 0x7F - BIT 7, A
const BIT_7_A: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "BIT 7 A",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        functions::bit(cpu, cpu.reg.a, 7);
        InstructionType::ActionTaken
    },
};

/// 0x80 - RES 0, B
const RES_0_B: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RES 0 B",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.b = functions::res(cpu.reg.b, 0);
        InstructionType::ActionTaken
    },
};

/// 0x81 - RES 0, C
const RES_0_C: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RES 0 C",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.c = functions::res(cpu.reg.c, 0);
        InstructionType::ActionTaken
    },
};

/// 0x82 - RES 0, D
const RES_0_D: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RES 0 D",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.d = functions::res(cpu.reg.d, 0);
        InstructionType::ActionTaken
    },
};

/// 0x83 - RES 0, E
const RES_0_E: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RES 0 E",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.e = functions::res(cpu.reg.e, 0);
        InstructionType::ActionTaken
    },
};

/// 0x84 - RES 0, H
const RES_0_H: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RES 0 H",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.h = functions::res(cpu.reg.h, 0);
        InstructionType::ActionTaken
    },
};

/// 0x85 - RES 0, L
const RES_0_L: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RES 0 L",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.l = functions::res(cpu.reg.l, 0);
        InstructionType::ActionTaken
    },
};

/// 0x86 - RES 0, (HL)
const RES_0_HL: Instruction = Instruction {
    length: 2,
    clock_cycles: 16,
    clock_cycles_condition: None,
    description: "RES 0 (HL)",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        let hl = cpu.reg.hl();
        let value = mmu.get_byte(hl);
        mmu.set_byte(hl, functions::res(value, 0));
        InstructionType::ActionTaken
    },
};

/// 0x87 - RES 0, A
const RES_0_A: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RES 0 A",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.a = functions::res(cpu.reg.a, 0);
        InstructionType::ActionTaken
    },
};

/// 0x88 - RES 1, B
const RES_1_B: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RES 1 B",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.b = functions::res(cpu.reg.b, 1);
        InstructionType::ActionTaken
    },
};

/// 0x89 - RES 1, C
const RES_1_C: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RES 1 C",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.c = functions::res(cpu.reg.c, 1);
        InstructionType::ActionTaken
    },
};

/// 0x8A - RES 1, D
const RES_1_D: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RES 1 D",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.d = functions::res(cpu.reg.d, 1);
        InstructionType::ActionTaken
    },
};

/// 0x8B - RES 1, E
const RES_1_E: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RES 1 E",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.e = functions::res(cpu.reg.e, 1);
        InstructionType::ActionTaken
    },
};

/// 0x8C - RES 1, H
const RES_1_H: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RES 1 H",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.h = functions::res(cpu.reg.h, 1);
        InstructionType::ActionTaken
    },
};

/// 0x8D - RES 1, L
const RES_1_L: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RES 1 L",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.l = functions::res(cpu.reg.l, 1);
        InstructionType::ActionTaken
    },
};

/// 0x8E - RES 1, (HL)
const RES_1_HL: Instruction = Instruction {
    length: 2,
    clock_cycles: 16,
    clock_cycles_condition: None,
    description: "RES 1 (HL)",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        let hl = cpu.reg.hl();
        let value = mmu.get_byte(hl);
        mmu.set_byte(hl, functions::res(value, 1));
        InstructionType::ActionTaken
    },
};

/// 0x8F - RES 1, A
const RES_1_A: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RES 1 A",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.a = functions::res(cpu.reg.a, 1);
        InstructionType::ActionTaken
    },
};

/// 0x90 - RES 2, B
const RES_2_B: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RES 2 B",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.b = functions::res(cpu.reg.b, 2);
        InstructionType::ActionTaken
    },
};

/// 0x91 - RES 2, C
const RES_2_C: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RES 2 C",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.c = functions::res(cpu.reg.c, 2);
        InstructionType::ActionTaken
    },
};

/// 0x92 - RES 2, D
const RES_2_D: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RES 2 D",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.d = functions::res(cpu.reg.d, 2);
        InstructionType::ActionTaken
    },
};

/// 0x93 - RES 2, E
const RES_2_E: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RES 2 E",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.e = functions::res(cpu.reg.e, 2);
        InstructionType::ActionTaken
    },
};

/// 0x94 - RES 2, H
const RES_2_H: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RES 2 H",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.h = functions::res(cpu.reg.h, 2);
        InstructionType::ActionTaken
    },
};

/// 0x95 - RES 2, L
const RES_2_L: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RES 2 L",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.l = functions::res(cpu.reg.l, 2);
        InstructionType::ActionTaken
    },
};

/// 0x96 - RES 2, (HL)
const RES_2_HL: Instruction = Instruction {
    length: 2,
    clock_cycles: 16,
    clock_cycles_condition: None,
    description: "RES 2 (HL)",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        let hl = cpu.reg.hl();
        let value = mmu.get_byte(hl);
        mmu.set_byte(hl, functions::res(value, 2));
        InstructionType::ActionTaken
    },
};

/// 0x97 - RES 2, A
const RES_2_A: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RES 2 A",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.a = functions::res(cpu.reg.a, 2);
        InstructionType::ActionTaken
    },
};

/// 0x98 - RES 3, B
const RES_3_B: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RES 3 B",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.b = functions::res(cpu.reg.b, 3);
        InstructionType::ActionTaken
    },
};

/// 0x99 - RES 3, C
const RES_3_C: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RES 3 C",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.c = functions::res(cpu.reg.c, 3);
        InstructionType::ActionTaken
    },
};

/// 0x9A - RES 3, D
const RES_3_D: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RES 3 D",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.d = functions::res(cpu.reg.d, 3);
        InstructionType::ActionTaken
    },
};

/// 0x9B - RES 3, E
const RES_3_E: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RES 3 E",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.e = functions::res(cpu.reg.e, 3);
        InstructionType::ActionTaken
    },
};

/// 0x9C - RES 3, H
const RES_3_H: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RES 3 H",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.h = functions::res(cpu.reg.h, 3);
        InstructionType::ActionTaken
    },
};

/// 0x9D - RES 3, L
const RES_3_L: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RES 3 L",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.l = functions::res(cpu.reg.l, 3);
        InstructionType::ActionTaken
    },
};

/// 0x9E - RES 3, (HL)
const RES_3_HL: Instruction = Instruction {
    length: 2,
    clock_cycles: 16,
    clock_cycles_condition: None,
    description: "RES 3 (HL)",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        let hl = cpu.reg.hl();
        let value = mmu.get_byte(hl);
        mmu.set_byte(hl, functions::res(value, 3));
        InstructionType::ActionTaken
    },
};

/// 0x9F - RES 3, A
const RES_3_A: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RES 3 A",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.a = functions::res(cpu.reg.a, 3);
        InstructionType::ActionTaken
    },
};

/// 0xA0 - RES 4, B
const RES_4_B: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RES 4 B",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.b = functions::res(cpu.reg.b, 4);
        InstructionType::ActionTaken
    },
};

/// 0xA1 - RES 4, C
const RES_4_C: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RES 4 C",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.c = functions::res(cpu.reg.c, 4);
        InstructionType::ActionTaken
    },
};

/// 0xA2 - RES 4, D
const RES_4_D: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RES 4 D",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.d = functions::res(cpu.reg.d, 4);
        InstructionType::ActionTaken
    },
};

/// 0xA3 - RES 4, E
const RES_4_E: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RES 4 E",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.e = functions::res(cpu.reg.e, 4);
        InstructionType::ActionTaken
    },
};

/// 0xA4 - RES 4, H
const RES_4_H: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RES 4 H",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.h = functions::res(cpu.reg.h, 4);
        InstructionType::ActionTaken
    },
};

/// 0xA5 - RES 4, L
const RES_4_L: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RES 4 L",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.l = functions::res(cpu.reg.l, 4);
        InstructionType::ActionTaken
    },
};

/// 0xA6 - RES 4, (HL)
const RES_4_HL: Instruction = Instruction {
    length: 2,
    clock_cycles: 16,
    clock_cycles_condition: None,
    description: "RES 4 (HL)",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        let hl = cpu.reg.hl();
        let value = mmu.get_byte(hl);
        mmu.set_byte(hl, functions::res(value, 4));
        InstructionType::ActionTaken
    },
};

/// 0xA7 - RES 4, A
const RES_4_A: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RES 4 A",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.a = functions::res(cpu.reg.a, 4);
        InstructionType::ActionTaken
    },
};

/// 0xA8 - RES 5, B
const RES_5_B: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RES 5 B",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.b = functions::res(cpu.reg.b, 5);
        InstructionType::ActionTaken
    },
};

/// 0xA9 - RES 5, C
const RES_5_C: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RES 5 C",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.c = functions::res(cpu.reg.c, 5);
        InstructionType::ActionTaken
    },
};

/// 0xAA - RES 5, D
const RES_5_D: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RES 5 D",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.d = functions::res(cpu.reg.d, 5);
        InstructionType::ActionTaken
    },
};

/// 0xAB - RES 5, E
const RES_5_E: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RES 5 E",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.e = functions::res(cpu.reg.e, 5);
        InstructionType::ActionTaken
    },
};

/// 0xAC - RES 5, H
const RES_5_H: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RES 5 H",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.h = functions::res(cpu.reg.h, 5);
        InstructionType::ActionTaken
    },
};

/// 0xAD - RES 5, L
const RES_5_L: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RES 5 L",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.l = functions::res(cpu.reg.l, 5);
        InstructionType::ActionTaken
    },
};

/// 0xAE - RES 5, (HL)
const RES_5_HL: Instruction = Instruction {
    length: 2,
    clock_cycles: 16,
    clock_cycles_condition: None,
    description: "RES 5 (HL)",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        let hl = cpu.reg.hl();
        let value = mmu.get_byte(hl);
        mmu.set_byte(hl, functions::res(value, 5));
        InstructionType::ActionTaken
    },
};

/// 0xAF - RES 5, A
const RES_5_A: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RES 5 A",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.a = functions::res(cpu.reg.a, 5);
        InstructionType::ActionTaken
    },
};

/// 0xB0 - RES 6, B
const RES_6_B: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RES 6 B",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.b = functions::res(cpu.reg.b, 6);
        InstructionType::ActionTaken
    },
};

/// 0xB1 - RES 6, C
const RES_6_C: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RES 6 C",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.c = functions::res(cpu.reg.c, 6);
        InstructionType::ActionTaken
    },
};

/// 0xB2 - RES 6, D
const RES_6_D: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RES 6 D",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.d = functions::res(cpu.reg.d, 6);
        InstructionType::ActionTaken
    },
};

/// 0xB3 - RES 6, E
const RES_6_E: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RES 6 E",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.e = functions::res(cpu.reg.e, 6);
        InstructionType::ActionTaken
    },
};

/// 0xB4 - RES 6, H
const RES_6_H: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RES 6 H",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.h = functions::res(cpu.reg.h, 6);
        InstructionType::ActionTaken
    },
};

/// 0xB5 - RES 6, L
const RES_6_L: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RES 6 L",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.l = functions::res(cpu.reg.l, 6);
        InstructionType::ActionTaken
    },
};

/// 0xB6 - RES 6, (HL)
const RES_6_HL: Instruction = Instruction {
    length: 2,
    clock_cycles: 16,
    clock_cycles_condition: None,
    description: "RES 6 (HL)",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        let hl = cpu.reg.hl();
        let value = mmu.get_byte(hl);
        mmu.set_byte(hl, functions::res(value, 6));
        InstructionType::ActionTaken
    },
};

/// 0xB7 - RES 6, A
const RES_6_A: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RES 6 A",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.a = functions::res(cpu.reg.a, 6);
        InstructionType::ActionTaken
    },
};

/// 0xB8 - RES 7, B
const RES_7_B: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RES 7 B",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.b = functions::res(cpu.reg.b, 7);
        InstructionType::ActionTaken
    },
};

/// 0xB9 - RES 7, C
const RES_7_C: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RES 7 C",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.c = functions::res(cpu.reg.c, 7);
        InstructionType::ActionTaken
    },
};

/// 0xBA - RES 7, D
const RES_7_D: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RES 7 D",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.d = functions::res(cpu.reg.d, 7);
        InstructionType::ActionTaken
    },
};

/// 0xBB - RES 7, E
const RES_7_E: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RES 7 E",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.e = functions::res(cpu.reg.e, 7);
        InstructionType::ActionTaken
    },
};

/// 0xBC - RES 7, H
const RES_7_H: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RES 7 H",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.h = functions::res(cpu.reg.h, 7);
        InstructionType::ActionTaken
    },
};

/// 0xBD - RES 7, L
const RES_7_L: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RES 7 L",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.l = functions::res(cpu.reg.l, 7);
        InstructionType::ActionTaken
    },
};

/// 0xBE - RES 7, (HL)
const RES_7_HL: Instruction = Instruction {
    length: 2,
    clock_cycles: 16,
    clock_cycles_condition: None,
    description: "RES 7 (HL)",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        let hl = cpu.reg.hl();
        let value = mmu.get_byte(hl);
        mmu.set_byte(hl, functions::res(value, 7));
        InstructionType::ActionTaken
    },
};

/// 0xBF - RES 7, A
const RES_7_A: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "RES 7 A",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.a = functions::res(cpu.reg.a, 7);
        InstructionType::ActionTaken
    },
};

/// 0xC0 - SET 0, B
const SET_0_B: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SET 0 B",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.b = functions::set(cpu.reg.b, 0);
        InstructionType::ActionTaken
    },
};

/// 0xC1 - SET 0, C
const SET_0_C: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SET 0 C",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.c = functions::set(cpu.reg.c, 0);
        InstructionType::ActionTaken
    },
};

/// 0xC2 - SET 0, D
const SET_0_D: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SET 0 D",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.d = functions::set(cpu.reg.d, 0);
        InstructionType::ActionTaken
    },
};

/// 0xC3 - SET 0, E
const SET_0_E: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SET 0 E",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.e = functions::set(cpu.reg.e, 0);
        InstructionType::ActionTaken
    },
};

/// 0xC4 - SET 0, H
const SET_0_H: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SET 0 H",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.h = functions::set(cpu.reg.h, 0);
        InstructionType::ActionTaken
    },
};

/// 0xC5 - SET 0, L
const SET_0_L: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SET 0 L",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.l = functions::set(cpu.reg.l, 0);
        InstructionType::ActionTaken
    },
};

/// 0xC6 - SET 0, (HL)
const SET_0_HL: Instruction = Instruction {
    length: 2,
    clock_cycles: 16,
    clock_cycles_condition: None,
    description: "SET 0 (HL)",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        let hl = cpu.reg.hl();
        let value = mmu.get_byte(hl);
        mmu.set_byte(hl, functions::set(value, 0));
        InstructionType::ActionTaken
    },
};

/// 0xC7 - SET 0, A
const SET_0_A: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SET 0 A",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.a = functions::set(cpu.reg.a, 0);
        InstructionType::ActionTaken
    },
};

/// 0xC8 - SET 1, B
const SET_1_B: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SET 1 B",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.b = functions::set(cpu.reg.b, 1);
        InstructionType::ActionTaken
    },
};

/// 0xC9 - SET 1, C
const SET_1_C: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SET 1 C",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.c = functions::set(cpu.reg.c, 1);
        InstructionType::ActionTaken
    },
};

/// 0xCA - SET 1, D
const SET_1_D: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SET 1 D",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.d = functions::set(cpu.reg.d, 1);
        InstructionType::ActionTaken
    },
};

/// 0xCB - SET 1, E
const SET_1_E: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SET 1 E",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.e = functions::set(cpu.reg.e, 1);
        InstructionType::ActionTaken
    },
};

/// 0xCC - SET 1, H
const SET_1_H: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SET 1 H",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.h = functions::set(cpu.reg.h, 1);
        InstructionType::ActionTaken
    },
};

/// 0xCD - SET 1, L
const SET_1_L: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SET 1 L",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.l = functions::set(cpu.reg.l, 1);
        InstructionType::ActionTaken
    },
};

/// 0xCE - SET 1, (HL)
const SET_1_HL: Instruction = Instruction {
    length: 2,
    clock_cycles: 16,
    clock_cycles_condition: None,
    description: "SET 1 (HL)",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        let hl = cpu.reg.hl();
        let value = mmu.get_byte(hl);
        mmu.set_byte(hl, functions::set(value, 1));
        InstructionType::ActionTaken
    },
};

/// 0xCF - SET 1, A
const SET_1_A: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SET 1 A",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.a = functions::set(cpu.reg.a, 1);
        InstructionType::ActionTaken
    },
};

/// 0xD0 - SET 2, B
const SET_2_B: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SET 2 B",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.b = functions::set(cpu.reg.b, 2);
        InstructionType::ActionTaken
    },
};

/// 0xD1 - SET 2, C
const SET_2_C: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SET 2 C",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.c = functions::set(cpu.reg.c, 2);
        InstructionType::ActionTaken
    },
};

/// 0xD2 - SET 2, D
const SET_2_D: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SET 2 D",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.d = functions::set(cpu.reg.d, 2);
        InstructionType::ActionTaken
    },
};

/// 0xD3 - SET 2, E
const SET_2_E: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SET 2 E",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.e = functions::set(cpu.reg.e, 2);
        InstructionType::ActionTaken
    },
};

/// 0xD4 - SET 2, H
const SET_2_H: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SET 2 H",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.h = functions::set(cpu.reg.h, 2);
        InstructionType::ActionTaken
    },
};

/// 0xD5 - SET 2, L
const SET_2_L: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SET 2 L",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.l = functions::set(cpu.reg.l, 2);
        InstructionType::ActionTaken
    },
};

/// 0xD6 - SET 2, (HL)
const SET_2_HL: Instruction = Instruction {
    length: 2,
    clock_cycles: 16,
    clock_cycles_condition: None,
    description: "SET 2 (HL)",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        let hl = cpu.reg.hl();
        let value = mmu.get_byte(hl);
        mmu.set_byte(hl, functions::set(value, 2));
        InstructionType::ActionTaken
    },
};

/// 0xD7 - SET 2, A
const SET_2_A: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SET 2 A",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.a = functions::set(cpu.reg.a, 2);
        InstructionType::ActionTaken
    },
};

/// 0xD8 - SET 3, B
const SET_3_B: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SET 3 B",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.b = functions::set(cpu.reg.b, 3);
        InstructionType::ActionTaken
    },
};

/// 0xD9 - SET 3, C
const SET_3_C: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SET 3 C",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.c = functions::set(cpu.reg.c, 3);
        InstructionType::ActionTaken
    },
};

/// 0xDA - SET 3, D
const SET_3_D: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SET 3 D",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.d = functions::set(cpu.reg.d, 3);
        InstructionType::ActionTaken
    },
};

/// 0xDB - SET 3, E
const SET_3_E: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SET 3 E",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.e = functions::set(cpu.reg.e, 3);
        InstructionType::ActionTaken
    },
};

/// 0xDC - SET 3, H
const SET_3_H: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SET 3 H",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.h = functions::set(cpu.reg.h, 3);
        InstructionType::ActionTaken
    },
};

/// 0xDD - SET 3, L
const SET_3_L: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SET 3 L",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.l = functions::set(cpu.reg.l, 3);
        InstructionType::ActionTaken
    },
};

/// 0xDE - SET 3, (HL)
const SET_3_HL: Instruction = Instruction {
    length: 2,
    clock_cycles: 16,
    clock_cycles_condition: None,
    description: "SET 3 (HL)",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        let hl = cpu.reg.hl();
        let value = mmu.get_byte(hl);
        mmu.set_byte(hl, functions::set(value, 3));
        InstructionType::ActionTaken
    },
};

/// 0xDF - SET 3, A
const SET_3_A: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SET 3 A",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.a = functions::set(cpu.reg.a, 3);
        InstructionType::ActionTaken
    },
};

/// 0xE0 - SET 4, B
const SET_4_B: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SET 4 B",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.b = functions::set(cpu.reg.b, 4);
        InstructionType::ActionTaken
    },
};

/// 0xE1 - SET 4, C
const SET_4_C: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SET 4 C",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.c = functions::set(cpu.reg.c, 4);
        InstructionType::ActionTaken
    },
};

/// 0xE2 - SET 4, D
const SET_4_D: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SET 4 D",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.d = functions::set(cpu.reg.d, 4);
        InstructionType::ActionTaken
    },
};

/// 0xE3 - SET 4, E
const SET_4_E: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SET 4 E",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.e = functions::set(cpu.reg.e, 4);
        InstructionType::ActionTaken
    },
};

/// 0xE4 - SET 4, H
const SET_4_H: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SET 4 H",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.h = functions::set(cpu.reg.h, 4);
        InstructionType::ActionTaken
    },
};

/// 0xE5 - SET 4, L
const SET_4_L: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SET 4 L",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.l = functions::set(cpu.reg.l, 4);
        InstructionType::ActionTaken
    },
};

/// 0xE6 - SET 4, (HL)
const SET_4_HL: Instruction = Instruction {
    length: 2,
    clock_cycles: 16,
    clock_cycles_condition: None,
    description: "SET 4 (HL)",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        let hl = cpu.reg.hl();
        let value = mmu.get_byte(hl);
        mmu.set_byte(hl, functions::set(value, 4));
        InstructionType::ActionTaken
    },
};

/// 0xE7 - SET 4, A
const SET_4_A: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SET 4 A",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.a = functions::set(cpu.reg.a, 4);
        InstructionType::ActionTaken
    },
};

/// 0xE8 - SET 5, B
const SET_5_B: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SET 5 B",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.b = functions::set(cpu.reg.b, 5);
        InstructionType::ActionTaken
    },
};

/// 0xE9 - SET 5, C
const SET_5_C: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SET 5 C",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.c = functions::set(cpu.reg.c, 5);
        InstructionType::ActionTaken
    },
};

/// 0xEA - SET 5, D
const SET_5_D: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SET 5 D",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.d = functions::set(cpu.reg.d, 5);
        InstructionType::ActionTaken
    },
};

/// 0xEB - SET 5, E
const SET_5_E: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SET 5 E",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.e = functions::set(cpu.reg.e, 5);
        InstructionType::ActionTaken
    },
};

/// 0xEC - SET 5, H
const SET_5_H: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SET 5 H",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.h = functions::set(cpu.reg.h, 5);
        InstructionType::ActionTaken
    },
};

/// 0xED - SET 5, L
const SET_5_L: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SET 5 L",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.l = functions::set(cpu.reg.l, 5);
        InstructionType::ActionTaken
    },
};

/// 0xEE - SET 5, (HL)
const SET_5_HL: Instruction = Instruction {
    length: 2,
    clock_cycles: 16,
    clock_cycles_condition: None,
    description: "SET 5 (HL)",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        let hl = cpu.reg.hl();
        let value = mmu.get_byte(hl);
        mmu.set_byte(hl, functions::set(value, 5));
        InstructionType::ActionTaken
    },
};

/// 0xEF - SET 5, A
const SET_5_A: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SET 5 A",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.a = functions::set(cpu.reg.a, 5);
        InstructionType::ActionTaken
    },
};

/// 0xF0 - SET 6, B
const SET_6_B: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SET 6 B",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.b = functions::set(cpu.reg.b, 6);
        InstructionType::ActionTaken
    },
};

/// 0xF1 - SET 6, C
const SET_6_C: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SET 6 C",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.c = functions::set(cpu.reg.c, 6);
        InstructionType::ActionTaken
    },
};

/// 0xF2 - SET 6, D
const SET_6_D: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SET 6 D",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.d = functions::set(cpu.reg.d, 6);
        InstructionType::ActionTaken
    },
};

/// 0xF3 - SET 6, E
const SET_6_E: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SET 6 E",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.e = functions::set(cpu.reg.e, 6);
        InstructionType::ActionTaken
    },
};

/// 0xF4 - SET 6, H
const SET_6_H: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SET 6 H",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.h = functions::set(cpu.reg.h, 6);
        InstructionType::ActionTaken
    },
};

/// 0xF5 - SET 6, L
const SET_6_L: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SET 6 L",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.l = functions::set(cpu.reg.l, 6);
        InstructionType::ActionTaken
    },
};

/// 0xF6 - SET 6, (HL)
const SET_6_HL: Instruction = Instruction {
    length: 2,
    clock_cycles: 16,
    clock_cycles_condition: None,
    description: "SET 6 (HL)",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        let hl = cpu.reg.hl();
        let value = mmu.get_byte(hl);
        mmu.set_byte(hl, functions::set(value, 6));
        InstructionType::ActionTaken
    },
};

/// 0xF7 - SET 6, A
const SET_6_A: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SET 6 A",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.a = functions::set(cpu.reg.a, 6);
        InstructionType::ActionTaken
    },
};

/// 0xF8 - SET 7, B
const SET_7_B: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SET 7 B",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.b = functions::set(cpu.reg.b, 7);
        InstructionType::ActionTaken
    },
};

/// 0xF9 - SET 7, C
const SET_7_C: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SET 7 C",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.c = functions::set(cpu.reg.c, 7);
        InstructionType::ActionTaken
    },
};

/// 0xFA - SET 7, D
const SET_7_D: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SET 7 D",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.d = functions::set(cpu.reg.d, 7);
        InstructionType::ActionTaken
    },
};

/// 0xFB - SET 7, E
const SET_7_E: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SET 7 E",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.e = functions::set(cpu.reg.e, 7);
        InstructionType::ActionTaken
    },
};

/// 0xFC - SET 7, H
const SET_7_H: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SET 7 H",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.h = functions::set(cpu.reg.h, 7);
        InstructionType::ActionTaken
    },
};

/// 0xFD - SET 7, L
const SET_7_L: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SET 7 L",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.l = functions::set(cpu.reg.l, 7);
        InstructionType::ActionTaken
    },
};

/// 0xFE - SET 7, (HL)
const SET_7_HL: Instruction = Instruction {
    length: 2,
    clock_cycles: 16,
    clock_cycles_condition: None,
    description: "SET 7 (HL)",
    handler: |cpu: &mut Cpu, mmu: &mut Memory, _: &OpCode| {
        let hl = cpu.reg.hl();
        let value = mmu.get_byte(hl);
        mmu.set_byte(hl, functions::set(value, 7));
        InstructionType::ActionTaken
    },
};

/// 0xFF - SET 7, A
const SET_7_A: Instruction = Instruction {
    length: 2,
    clock_cycles: 8,
    clock_cycles_condition: None,
    description: "SET 7 A",
    handler: |cpu: &mut Cpu, _: &mut Memory, _: &OpCode| {
        cpu.reg.a = functions::set(cpu.reg.a, 7);
        InstructionType::ActionTaken
    },
};

/// Get a CB instruction by opcode
pub fn get_instruction(op_code: &u8) -> Option<&Instruction> {
    match op_code {
        0x00 => Some(&RLC_B),
        0x01 => Some(&RLC_C),
        0x02 => Some(&RLC_D),
        0x03 => Some(&RLC_E),
        0x04 => Some(&RLC_H),
        0x05 => Some(&RLC_L),
        0x06 => Some(&RLC_HL),
        0x07 => Some(&RLC_A),
        0x08 => Some(&RRC_B),
        0x09 => Some(&RRC_C),
        0x0A => Some(&RRC_D),
        0x0B => Some(&RRC_E),
        0x0C => Some(&RRC_H),
        0x0D => Some(&RRC_L),
        0x0E => Some(&RRC_HL),
        0x0F => Some(&RRC_A),

        0x10 => Some(&RL_B),
        0x11 => Some(&RL_C),
        0x12 => Some(&RL_D),
        0x13 => Some(&RL_E),
        0x14 => Some(&RL_H),
        0x15 => Some(&RL_L),
        0x16 => Some(&RL_HL),
        0x17 => Some(&RL_A),
        0x18 => Some(&RR_B),
        0x19 => Some(&RR_C),
        0x1A => Some(&RR_D),
        0x1B => Some(&RR_E),
        0x1C => Some(&RR_H),
        0x1D => Some(&RR_L),
        0x1E => Some(&RR_HL),
        0x1F => Some(&RR_A),

        0x20 => Some(&SLA_B),
        0x21 => Some(&SLA_C),
        0x22 => Some(&SLA_D),
        0x23 => Some(&SLA_E),
        0x24 => Some(&SLA_H),
        0x25 => Some(&SLA_L),
        0x26 => Some(&SLA_HL),
        0x27 => Some(&SLA_A),
        0x28 => Some(&SRA_B),
        0x29 => Some(&SRA_C),
        0x2A => Some(&SRA_D),
        0x2B => Some(&SRA_E),
        0x2C => Some(&SRA_H),
        0x2D => Some(&SRA_L),
        0x2E => Some(&SRA_HL),
        0x2F => Some(&SRA_A),

        0x30 => Some(&SWAP_B),
        0x31 => Some(&SWAP_C),
        0x32 => Some(&SWAP_D),
        0x33 => Some(&SWAP_E),
        0x34 => Some(&SWAP_H),
        0x35 => Some(&SWAP_L),
        0x36 => Some(&SWAP_HL),
        0x37 => Some(&SWAP_A),
        0x38 => Some(&SRL_B),
        0x39 => Some(&SRL_C),
        0x3A => Some(&SRL_D),
        0x3B => Some(&SRL_E),
        0x3C => Some(&SRL_H),
        0x3D => Some(&SRL_L),
        0x3E => Some(&SRL_HL),
        0x3F => Some(&SRL_A),

        0x40 => Some(&BIT_0_B),
        0x41 => Some(&BIT_0_C),
        0x42 => Some(&BIT_0_D),
        0x43 => Some(&BIT_0_E),
        0x44 => Some(&BIT_0_H),
        0x45 => Some(&BIT_0_L),
        0x46 => Some(&BIT_0_HL),
        0x47 => Some(&BIT_0_A),
        0x48 => Some(&BIT_1_B),
        0x49 => Some(&BIT_1_C),
        0x4A => Some(&BIT_1_D),
        0x4B => Some(&BIT_1_E),
        0x4C => Some(&BIT_1_H),
        0x4D => Some(&BIT_1_L),
        0x4E => Some(&BIT_1_HL),
        0x4F => Some(&BIT_1_A),

        0x50 => Some(&BIT_2_B),
        0x51 => Some(&BIT_2_C),
        0x52 => Some(&BIT_2_D),
        0x53 => Some(&BIT_2_E),
        0x54 => Some(&BIT_2_H),
        0x55 => Some(&BIT_2_L),
        0x56 => Some(&BIT_2_HL),
        0x57 => Some(&BIT_2_A),
        0x58 => Some(&BIT_3_B),
        0x59 => Some(&BIT_3_C),
        0x5A => Some(&BIT_3_D),
        0x5B => Some(&BIT_3_E),
        0x5C => Some(&BIT_3_H),
        0x5D => Some(&BIT_3_L),
        0x5E => Some(&BIT_3_HL),
        0x5F => Some(&BIT_3_A),

        0x60 => Some(&BIT_4_B),
        0x61 => Some(&BIT_4_C),
        0x62 => Some(&BIT_4_D),
        0x63 => Some(&BIT_4_E),
        0x64 => Some(&BIT_4_H),
        0x65 => Some(&BIT_4_L),
        0x66 => Some(&BIT_4_HL),
        0x67 => Some(&BIT_4_A),
        0x68 => Some(&BIT_5_B),
        0x69 => Some(&BIT_5_C),
        0x6A => Some(&BIT_5_D),
        0x6B => Some(&BIT_5_E),
        0x6C => Some(&BIT_5_H),
        0x6D => Some(&BIT_5_L),
        0x6E => Some(&BIT_5_HL),
        0x6F => Some(&BIT_5_A),

        0x70 => Some(&BIT_6_B),
        0x71 => Some(&BIT_6_C),
        0x72 => Some(&BIT_6_D),
        0x73 => Some(&BIT_6_E),
        0x74 => Some(&BIT_6_H),
        0x75 => Some(&BIT_6_L),
        0x76 => Some(&BIT_6_HL),
        0x77 => Some(&BIT_6_A),
        0x78 => Some(&BIT_7_B),
        0x79 => Some(&BIT_7_C),
        0x7A => Some(&BIT_7_D),
        0x7B => Some(&BIT_7_E),
        0x7C => Some(&BIT_7_H),
        0x7D => Some(&BIT_7_L),
        0x7E => Some(&BIT_7_HL),
        0x7F => Some(&BIT_7_A),

        0x80 => Some(&RES_0_B),
        0x81 => Some(&RES_0_C),
        0x82 => Some(&RES_0_D),
        0x83 => Some(&RES_0_E),
        0x84 => Some(&RES_0_H),
        0x85 => Some(&RES_0_L),
        0x86 => Some(&RES_0_HL),
        0x87 => Some(&RES_0_A),
        0x88 => Some(&RES_1_B),
        0x89 => Some(&RES_1_C),
        0x8A => Some(&RES_1_D),
        0x8B => Some(&RES_1_E),
        0x8C => Some(&RES_1_H),
        0x8D => Some(&RES_1_L),
        0x8E => Some(&RES_1_HL),
        0x8F => Some(&RES_1_A),

        0x90 => Some(&RES_2_B),
        0x91 => Some(&RES_2_C),
        0x92 => Some(&RES_2_D),
        0x93 => Some(&RES_2_E),
        0x94 => Some(&RES_2_H),
        0x95 => Some(&RES_2_L),
        0x96 => Some(&RES_2_HL),
        0x97 => Some(&RES_2_A),
        0x98 => Some(&RES_3_B),
        0x99 => Some(&RES_3_C),
        0x9A => Some(&RES_3_D),
        0x9B => Some(&RES_3_E),
        0x9C => Some(&RES_3_H),
        0x9D => Some(&RES_3_L),
        0x9E => Some(&RES_3_HL),
        0x9F => Some(&RES_3_A),

        0xA0 => Some(&RES_4_B),
        0xA1 => Some(&RES_4_C),
        0xA2 => Some(&RES_4_D),
        0xA3 => Some(&RES_4_E),
        0xA4 => Some(&RES_4_H),
        0xA5 => Some(&RES_4_L),
        0xA6 => Some(&RES_4_HL),
        0xA7 => Some(&RES_4_A),
        0xA8 => Some(&RES_5_B),
        0xA9 => Some(&RES_5_C),
        0xAA => Some(&RES_5_D),
        0xAB => Some(&RES_5_E),
        0xAC => Some(&RES_5_H),
        0xAD => Some(&RES_5_L),
        0xAE => Some(&RES_5_HL),
        0xAF => Some(&RES_5_A),

        0xB0 => Some(&RES_6_B),
        0xB1 => Some(&RES_6_C),
        0xB2 => Some(&RES_6_D),
        0xB3 => Some(&RES_6_E),
        0xB4 => Some(&RES_6_H),
        0xB5 => Some(&RES_6_L),
        0xB6 => Some(&RES_6_HL),
        0xB7 => Some(&RES_6_A),
        0xB8 => Some(&RES_7_B),
        0xB9 => Some(&RES_7_C),
        0xBA => Some(&RES_7_D),
        0xBB => Some(&RES_7_E),
        0xBC => Some(&RES_7_H),
        0xBD => Some(&RES_7_L),
        0xBE => Some(&RES_7_HL),
        0xBF => Some(&RES_7_A),

        0xC0 => Some(&SET_0_B),
        0xC1 => Some(&SET_0_C),
        0xC2 => Some(&SET_0_D),
        0xC3 => Some(&SET_0_E),
        0xC4 => Some(&SET_0_H),
        0xC5 => Some(&SET_0_L),
        0xC6 => Some(&SET_0_HL),
        0xC7 => Some(&SET_0_A),
        0xC8 => Some(&SET_1_B),
        0xC9 => Some(&SET_1_C),
        0xCA => Some(&SET_1_D),
        0xCB => Some(&SET_1_E),
        0xCC => Some(&SET_1_H),
        0xCD => Some(&SET_1_L),
        0xCE => Some(&SET_1_HL),
        0xCF => Some(&SET_1_A),

        0xD0 => Some(&SET_2_B),
        0xD1 => Some(&SET_2_C),
        0xD2 => Some(&SET_2_D),
        0xD3 => Some(&SET_2_E),
        0xD4 => Some(&SET_2_H),
        0xD5 => Some(&SET_2_L),
        0xD6 => Some(&SET_2_HL),
        0xD7 => Some(&SET_2_A),
        0xD8 => Some(&SET_3_B),
        0xD9 => Some(&SET_3_C),
        0xDA => Some(&SET_3_D),
        0xDB => Some(&SET_3_E),
        0xDC => Some(&SET_3_H),
        0xDD => Some(&SET_3_L),
        0xDE => Some(&SET_3_HL),
        0xDF => Some(&SET_3_A),

        0xE0 => Some(&SET_4_B),
        0xE1 => Some(&SET_4_C),
        0xE2 => Some(&SET_4_D),
        0xE3 => Some(&SET_4_E),
        0xE4 => Some(&SET_4_H),
        0xE5 => Some(&SET_4_L),
        0xE6 => Some(&SET_4_HL),
        0xE7 => Some(&SET_4_A),
        0xE8 => Some(&SET_5_B),
        0xE9 => Some(&SET_5_C),
        0xEA => Some(&SET_5_D),
        0xEB => Some(&SET_5_E),
        0xEC => Some(&SET_5_H),
        0xED => Some(&SET_5_L),
        0xEE => Some(&SET_5_HL),
        0xEF => Some(&SET_5_A),

        0xF0 => Some(&SET_6_B),
        0xF1 => Some(&SET_6_C),
        0xF2 => Some(&SET_6_D),
        0xF3 => Some(&SET_6_E),
        0xF4 => Some(&SET_6_H),
        0xF5 => Some(&SET_6_L),
        0xF6 => Some(&SET_6_HL),
        0xF7 => Some(&SET_6_A),
        0xF8 => Some(&SET_7_B),
        0xF9 => Some(&SET_7_C),
        0xFA => Some(&SET_7_D),
        0xFB => Some(&SET_7_E),
        0xFC => Some(&SET_7_H),
        0xFD => Some(&SET_7_L),
        0xFE => Some(&SET_7_HL),
        0xFF => Some(&SET_7_A),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::cpu::registers::Flag;

    #[test]
    pub fn test_get_instruction_rlc_b() {
        let instruction = get_instruction(&0x00).unwrap();
        assert_eq!(instruction, &RLC_B);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_rlc_b() {
        let mut cpu = Cpu::new();
        cpu.reg.b = 0x80;
        cpu.reg.clear_flag(Flag::Carry);
        (&RLC_B.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x00));
        assert_eq!(cpu.reg.b, 0x01);
    }

    #[test]
    pub fn test_get_instruction_rlc_c() {
        let instruction = get_instruction(&0x01).unwrap();
        assert_eq!(instruction, &RLC_C);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_rlc_c() {
        let mut cpu = Cpu::new();
        cpu.reg.c = 0x80;
        cpu.reg.clear_flag(Flag::Carry);
        (&RLC_C.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x01));
        assert_eq!(cpu.reg.c, 0x01);
    }

    #[test]
    pub fn test_get_instruction_rlc_d() {
        let instruction = get_instruction(&0x02).unwrap();
        assert_eq!(instruction, &RLC_D);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_rlc_d() {
        let mut cpu = Cpu::new();
        cpu.reg.d = 0x80;
        cpu.reg.clear_flag(Flag::Carry);
        (&RLC_D.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x02));
        assert_eq!(cpu.reg.d, 0x01);
    }

    #[test]
    pub fn test_get_instruction_rlc_e() {
        let instruction = get_instruction(&0x03).unwrap();
        assert_eq!(instruction, &RLC_E);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_rlc_e() {
        let mut cpu = Cpu::new();
        cpu.reg.e = 0x80;
        cpu.reg.clear_flag(Flag::Carry);
        (&RLC_E.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x03));
        assert_eq!(cpu.reg.e, 0x01);
    }

    #[test]
    pub fn test_get_instruction_rlc_h() {
        let instruction = get_instruction(&0x04).unwrap();
        assert_eq!(instruction, &RLC_H);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_rlc_h() {
        let mut cpu = Cpu::new();
        cpu.reg.h = 0x80;
        cpu.reg.clear_flag(Flag::Carry);
        (&RLC_H.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x04));
        assert_eq!(cpu.reg.h, 0x01);
    }

    #[test]
    pub fn test_get_instruction_rlc_l() {
        let instruction = get_instruction(&0x05).unwrap();
        assert_eq!(instruction, &RLC_L);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_rlc_l() {
        let mut cpu = Cpu::new();
        cpu.reg.l = 0x80;
        cpu.reg.clear_flag(Flag::Carry);
        (&RLC_L.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x05));
        assert_eq!(cpu.reg.l, 0x01);
    }

    #[test]
    pub fn test_get_instruction_rlc_hl() {
        let instruction = get_instruction(&0x06).unwrap();
        assert_eq!(instruction, &RLC_HL);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 16);
    }

    #[test]
    pub fn test_rlc_hl() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        mmu.set_byte(0xC000 as usize, 0x80);
        cpu.reg.set_hl(0xC000);
        cpu.reg.clear_flag(Flag::Carry);
        (&RLC_HL.handler)(&mut cpu, &mut mmu, &OpCode::CB(0x06));
        assert_eq!(mmu.get_byte(0xC000 as usize), 0x01);
    }

    #[test]
    pub fn test_get_instruction_rlc_a() {
        let instruction = get_instruction(&0x07).unwrap();
        assert_eq!(instruction, &RLC_A);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_rlc_a() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x80;
        cpu.reg.clear_flag(Flag::Carry);
        (&RLC_A.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x07));
        assert_eq!(cpu.reg.a, 0x01);
    }

    #[test]
    pub fn test_get_instruction_rrc_b() {
        let instruction = get_instruction(&0x08).unwrap();
        assert_eq!(instruction, &RRC_B);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_rrc_b() {
        let mut cpu = Cpu::new();
        cpu.reg.b = 0x80;
        cpu.reg.clear_flag(Flag::Carry);
        (&RRC_B.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x08));
        assert_eq!(cpu.reg.b, 0x40);
    }

    #[test]
    pub fn test_get_instruction_rrc_c() {
        let instruction = get_instruction(&0x09).unwrap();
        assert_eq!(instruction, &RRC_C);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_rrc_c() {
        let mut cpu = Cpu::new();
        cpu.reg.c = 0x80;
        cpu.reg.clear_flag(Flag::Carry);
        (&RRC_C.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x09));
        assert_eq!(cpu.reg.c, 0x40);
    }

    #[test]
    pub fn test_get_instruction_rrc_d() {
        let instruction = get_instruction(&0x0A).unwrap();
        assert_eq!(instruction, &RRC_D);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_rrc_d() {
        let mut cpu = Cpu::new();
        cpu.reg.d = 0x80;
        cpu.reg.clear_flag(Flag::Carry);
        (&RRC_D.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x0A));
        assert_eq!(cpu.reg.d, 0x40);
    }

    #[test]
    pub fn test_get_instruction_rrc_e() {
        let instruction = get_instruction(&0x0B).unwrap();
        assert_eq!(instruction, &RRC_E);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_rrc_e() {
        let mut cpu = Cpu::new();
        cpu.reg.e = 0x80;
        cpu.reg.clear_flag(Flag::Carry);
        (&RRC_E.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x0B));
        assert_eq!(cpu.reg.e, 0x40);
    }

    #[test]
    pub fn test_get_instruction_rrc_h() {
        let instruction = get_instruction(&0x0C).unwrap();
        assert_eq!(instruction, &RRC_H);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_rrc_h() {
        let mut cpu = Cpu::new();
        cpu.reg.h = 0x80;
        cpu.reg.clear_flag(Flag::Carry);
        (&RRC_H.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x0C));
        assert_eq!(cpu.reg.h, 0x40);
    }

    #[test]
    pub fn test_get_instruction_rrc_l() {
        let instruction = get_instruction(&0x0D).unwrap();
        assert_eq!(instruction, &RRC_L);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_rrc_l() {
        let mut cpu = Cpu::new();
        cpu.reg.l = 0x80;
        cpu.reg.clear_flag(Flag::Carry);
        (&RRC_L.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x0D));
        assert_eq!(cpu.reg.l, 0x40);
    }

    #[test]
    pub fn test_get_instruction_rrc_hl() {
        let instruction = get_instruction(&0x0E).unwrap();
        assert_eq!(instruction, &RRC_HL);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 16);
    }

    #[test]
    pub fn test_rrc_hl() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        mmu.set_byte(0xC000 as usize, 0x80);
        cpu.reg.set_hl(0xC000);
        cpu.reg.clear_flag(Flag::Carry);
        (&RRC_HL.handler)(&mut cpu, &mut mmu, &OpCode::CB(0x0E));
        assert_eq!(mmu.get_byte(0xC000 as usize), 0x40);
    }

    #[test]
    pub fn test_get_instruction_rrc_a() {
        let instruction = get_instruction(&0x0F).unwrap();
        assert_eq!(instruction, &RRC_A);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_rrc_a() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x80;
        cpu.reg.clear_flag(Flag::Carry);
        (&RRC_A.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x0F));
        assert_eq!(cpu.reg.a, 0x40);
    }

    #[test]
    pub fn test_get_instruction_rl_b() {
        let instruction = get_instruction(&0x10).unwrap();
        assert_eq!(instruction, &RL_B);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_rl_b() {
        let mut cpu = Cpu::new();
        cpu.reg.b = 0x80;
        cpu.reg.clear_flag(Flag::Carry);
        (&RL_B.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x10));
        assert_eq!(cpu.reg.b, 0x00);
    }

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
    pub fn test_get_instruction_rl_d() {
        let instruction = get_instruction(&0x12).unwrap();
        assert_eq!(instruction, &RL_D);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_rl_d() {
        let mut cpu = Cpu::new();
        cpu.reg.d = 0x80;
        cpu.reg.clear_flag(Flag::Carry);
        (&RL_D.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x12));
        assert_eq!(cpu.reg.d, 0x00);
    }

    #[test]
    pub fn test_get_instruction_rl_e() {
        let instruction = get_instruction(&0x13).unwrap();
        assert_eq!(instruction, &RL_E);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_rl_e() {
        let mut cpu = Cpu::new();
        cpu.reg.e = 0x80;
        cpu.reg.clear_flag(Flag::Carry);
        (&RL_E.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x13));
        assert_eq!(cpu.reg.e, 0x00);
    }

    #[test]
    pub fn test_get_instruction_rl_h() {
        let instruction = get_instruction(&0x14).unwrap();
        assert_eq!(instruction, &RL_H);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_rl_h() {
        let mut cpu = Cpu::new();
        cpu.reg.h = 0x80;
        cpu.reg.clear_flag(Flag::Carry);
        (&RL_H.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x14));
        assert_eq!(cpu.reg.h, 0x00);
    }

    #[test]
    pub fn test_get_instruction_rl_l() {
        let instruction = get_instruction(&0x15).unwrap();
        assert_eq!(instruction, &RL_L);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_rl_l() {
        let mut cpu = Cpu::new();
        cpu.reg.l = 0x80;
        cpu.reg.clear_flag(Flag::Carry);
        (&RL_L.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x15));
        assert_eq!(cpu.reg.l, 0x00);
    }

    #[test]
    pub fn test_get_instruction_rl_hl() {
        let instruction = get_instruction(&0x16).unwrap();
        assert_eq!(instruction, &RL_HL);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 16);
    }

    #[test]
    pub fn test_rl_hl() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        mmu.set_byte(0xC000 as usize, 0x80);
        cpu.reg.set_hl(0xC000);
        cpu.reg.clear_flag(Flag::Carry);
        (&RL_HL.handler)(&mut cpu, &mut mmu, &OpCode::CB(0x16));
        assert_eq!(mmu.get_byte(0xC000 as usize), 0x00);
    }

    #[test]
    pub fn test_get_instruction_rl_a() {
        let instruction = get_instruction(&0x17).unwrap();
        assert_eq!(instruction, &RL_A);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_rl_a() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x80;
        cpu.reg.clear_flag(Flag::Carry);
        (&RL_A.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x17));
        assert_eq!(cpu.reg.a, 0x00);
    }

    #[test]
    pub fn test_get_instruction_rr_b() {
        let instruction = get_instruction(&0x18).unwrap();
        assert_eq!(instruction, &RR_B);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_rr_b() {
        let mut cpu = Cpu::new();
        cpu.reg.b = 0x80;
        cpu.reg.clear_flag(Flag::Carry);
        (&RR_B.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x18));
        assert_eq!(cpu.reg.b, 0x40);
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
    pub fn test_get_instruction_rr_e() {
        let instruction = get_instruction(&0x1B).unwrap();
        assert_eq!(instruction, &RR_E);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_rr_e() {
        let mut cpu = Cpu::new();
        cpu.reg.e = 0x80;
        cpu.reg.clear_flag(Flag::Carry);
        (&RR_E.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x1B));
        assert_eq!(cpu.reg.e, 0x40);
    }

    #[test]
    pub fn test_get_instruction_rr_h() {
        let instruction = get_instruction(&0x1C).unwrap();
        assert_eq!(instruction, &RR_H);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_rr_h() {
        let mut cpu = Cpu::new();
        cpu.reg.h = 0x80;
        cpu.reg.clear_flag(Flag::Carry);
        (&RR_H.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x1C));
        assert_eq!(cpu.reg.h, 0x40);
    }

    #[test]
    pub fn test_get_instruction_rr_l() {
        let instruction = get_instruction(&0x1D).unwrap();
        assert_eq!(instruction, &RR_L);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_rr_l() {
        let mut cpu = Cpu::new();
        cpu.reg.l = 0x80;
        cpu.reg.clear_flag(Flag::Carry);
        (&RR_L.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x1D));
        assert_eq!(cpu.reg.l, 0x40);
    }

    #[test]
    pub fn test_get_instruction_rr_hl() {
        let instruction = get_instruction(&0x1E).unwrap();
        assert_eq!(instruction, &RR_HL);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 16);
    }

    #[test]
    pub fn test_rr_hl() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        mmu.set_byte(0xC000 as usize, 0x80);
        cpu.reg.set_hl(0xC000);
        cpu.reg.clear_flag(Flag::Carry);
        (&RR_HL.handler)(&mut cpu, &mut mmu, &OpCode::CB(0x1E));
        assert_eq!(mmu.get_byte(0xC000 as usize), 0x40);
    }

    #[test]
    pub fn test_get_instruction_rr_a() {
        let instruction = get_instruction(&0x1F).unwrap();
        assert_eq!(instruction, &RR_A);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_rr_a() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x80;
        cpu.reg.clear_flag(Flag::Carry);
        (&RR_A.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x1F));
        assert_eq!(cpu.reg.a, 0x40);
    }

    #[test]
    pub fn test_get_instruction_sla_b() {
        let instruction = get_instruction(&0x20).unwrap();
        assert_eq!(instruction, &SLA_B);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_sla_b() {
        let mut cpu = Cpu::new();
        cpu.reg.b = 0x80;
        (&SLA_B.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x20));
        assert_eq!(cpu.reg.b, 0x00);
    }

    #[test]
    pub fn test_get_instruction_sla_c() {
        let instruction = get_instruction(&0x21).unwrap();
        assert_eq!(instruction, &SLA_C);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_sla_c() {
        let mut cpu = Cpu::new();
        cpu.reg.c = 0x80;
        (&SLA_C.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x21));
        assert_eq!(cpu.reg.c, 0x00);
    }

    #[test]
    pub fn test_get_instruction_sla_d() {
        let instruction = get_instruction(&0x22).unwrap();
        assert_eq!(instruction, &SLA_D);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_sla_d() {
        let mut cpu = Cpu::new();
        cpu.reg.d = 0x80;
        (&SLA_D.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x22));
        assert_eq!(cpu.reg.d, 0x00);
    }

    #[test]
    pub fn test_get_instruction_sla_e() {
        let instruction = get_instruction(&0x23).unwrap();
        assert_eq!(instruction, &SLA_E);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_sla_e() {
        let mut cpu = Cpu::new();
        cpu.reg.e = 0x80;
        (&SLA_E.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x23));
        assert_eq!(cpu.reg.e, 0x00);
    }

    #[test]
    pub fn test_get_instruction_sla_h() {
        let instruction = get_instruction(&0x24).unwrap();
        assert_eq!(instruction, &SLA_H);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_sla_h() {
        let mut cpu = Cpu::new();
        cpu.reg.h = 0x80;
        (&SLA_H.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x24));
        assert_eq!(cpu.reg.h, 0x00);
    }

    #[test]
    pub fn test_get_instruction_sla_l() {
        let instruction = get_instruction(&0x25).unwrap();
        assert_eq!(instruction, &SLA_L);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_sla_l() {
        let mut cpu = Cpu::new();
        cpu.reg.l = 0x80;
        (&SLA_L.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x25));
        assert_eq!(cpu.reg.l, 0x00);
    }

    #[test]
    pub fn test_get_instruction_sla_hl() {
        let instruction = get_instruction(&0x26).unwrap();
        assert_eq!(instruction, &SLA_HL);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 16);
    }

    #[test]
    pub fn test_sla_hl() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        mmu.set_byte(0xC000 as usize, 0x80);
        cpu.reg.set_hl(0xC000);
        (&SLA_HL.handler)(&mut cpu, &mut mmu, &OpCode::CB(0x26));
        assert_eq!(mmu.get_byte(0xC000 as usize), 0x00);
    }

    #[test]
    pub fn test_get_instruction_sla_a() {
        let instruction = get_instruction(&0x27).unwrap();
        assert_eq!(instruction, &SLA_A);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_sla_a() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x80;
        (&SLA_A.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x27));
        assert_eq!(cpu.reg.a, 0x00);
    }

    #[test]
    pub fn test_get_instruction_sra_b() {
        let instruction = get_instruction(&0x28).unwrap();
        assert_eq!(instruction, &SRA_B);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_sra_b() {
        let mut cpu = Cpu::new();
        cpu.reg.b = 0x81;
        (&SRA_B.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x28));
        assert_eq!(cpu.reg.b, 0xC0);
    }

    #[test]
    pub fn test_get_instruction_sra_c() {
        let instruction = get_instruction(&0x29).unwrap();
        assert_eq!(instruction, &SRA_C);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_sra_c() {
        let mut cpu = Cpu::new();
        cpu.reg.c = 0x81;
        (&SRA_C.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x29));
        assert_eq!(cpu.reg.c, 0xC0);
    }

    #[test]
    pub fn test_get_instruction_sra_d() {
        let instruction = get_instruction(&0x2A).unwrap();
        assert_eq!(instruction, &SRA_D);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_sra_d() {
        let mut cpu = Cpu::new();
        cpu.reg.d = 0x81;
        (&SRA_D.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x2A));
        assert_eq!(cpu.reg.d, 0xC0);
    }

    #[test]
    pub fn test_get_instruction_sra_e() {
        let instruction = get_instruction(&0x2B).unwrap();
        assert_eq!(instruction, &SRA_E);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_sra_e() {
        let mut cpu = Cpu::new();
        cpu.reg.e = 0x81;
        (&SRA_E.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x2B));
        assert_eq!(cpu.reg.e, 0xC0);
    }

    #[test]
    pub fn test_get_instruction_sra_h() {
        let instruction = get_instruction(&0x2C).unwrap();
        assert_eq!(instruction, &SRA_H);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_sra_h() {
        let mut cpu = Cpu::new();
        cpu.reg.h = 0x81;
        (&SRA_H.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x2C));
        assert_eq!(cpu.reg.h, 0xC0);
    }

    #[test]
    pub fn test_get_instruction_sra_l() {
        let instruction = get_instruction(&0x2D).unwrap();
        assert_eq!(instruction, &SRA_L);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_sra_l() {
        let mut cpu = Cpu::new();
        cpu.reg.l = 0x81;
        (&SRA_L.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x2D));
        assert_eq!(cpu.reg.l, 0xC0);
    }

    #[test]
    pub fn test_get_instruction_sra_hl() {
        let instruction = get_instruction(&0x2E).unwrap();
        assert_eq!(instruction, &SRA_HL);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 16);
    }

    #[test]
    pub fn test_sra_hl() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        mmu.set_byte(0xC000 as usize, 0x81);
        cpu.reg.set_hl(0xC000);
        (&SRA_HL.handler)(&mut cpu, &mut mmu, &OpCode::CB(0x2E));
        assert_eq!(mmu.get_byte(0xC000 as usize), 0xC0);
    }

    #[test]
    pub fn test_get_instruction_sra_a() {
        let instruction = get_instruction(&0x2F).unwrap();
        assert_eq!(instruction, &SRA_A);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_sra_a() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x81;
        (&SRA_A.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x2F));
        assert_eq!(cpu.reg.a, 0xC0);
    }

    #[test]
    pub fn test_get_instruction_swap_b() {
        let instruction = get_instruction(&0x30).unwrap();
        assert_eq!(instruction, &SWAP_B);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_swap_b() {
        let mut cpu = Cpu::new();
        cpu.reg.b = 0x80;
        (&SWAP_B.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x30));
        assert_eq!(cpu.reg.b, 0x08);
    }

    #[test]
    pub fn test_get_instruction_swap_c() {
        let instruction = get_instruction(&0x31).unwrap();
        assert_eq!(instruction, &SWAP_C);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_swap_c() {
        let mut cpu = Cpu::new();
        cpu.reg.c = 0x80;
        (&SWAP_C.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x31));
        assert_eq!(cpu.reg.c, 0x08);
    }

    #[test]
    pub fn test_get_instruction_swap_d() {
        let instruction = get_instruction(&0x32).unwrap();
        assert_eq!(instruction, &SWAP_D);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_swap_d() {
        let mut cpu = Cpu::new();
        cpu.reg.d = 0x80;
        (&SWAP_D.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x32));
        assert_eq!(cpu.reg.d, 0x08);
    }

    #[test]
    pub fn test_get_instruction_swap_e() {
        let instruction = get_instruction(&0x33).unwrap();
        assert_eq!(instruction, &SWAP_E);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_swap_e() {
        let mut cpu = Cpu::new();
        cpu.reg.e = 0x80;
        (&SWAP_E.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x33));
        assert_eq!(cpu.reg.e, 0x08);
    }

    #[test]
    pub fn test_get_instruction_swap_h() {
        let instruction = get_instruction(&0x34).unwrap();
        assert_eq!(instruction, &SWAP_H);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_swap_h() {
        let mut cpu = Cpu::new();
        cpu.reg.h = 0x80;
        (&SWAP_H.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x34));
        assert_eq!(cpu.reg.h, 0x08);
    }

    #[test]
    pub fn test_get_instruction_swap_l() {
        let instruction = get_instruction(&0x35).unwrap();
        assert_eq!(instruction, &SWAP_L);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_swap_l() {
        let mut cpu = Cpu::new();
        cpu.reg.l = 0x80;
        (&SWAP_L.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x35));
        assert_eq!(cpu.reg.l, 0x08);
    }

    #[test]
    pub fn test_get_instruction_swap_hl() {
        let instruction = get_instruction(&0x36).unwrap();
        assert_eq!(instruction, &SWAP_HL);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 16);
    }

    #[test]
    pub fn test_swap_hl() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        mmu.set_byte(0xC000 as usize, 0x80);
        cpu.reg.set_hl(0xC000);
        (&SWAP_HL.handler)(&mut cpu, &mut mmu, &OpCode::CB(0x36));
        assert_eq!(mmu.get_byte(0xC000 as usize), 0x08);
    }

    #[test]
    pub fn test_get_instruction_swap_a() {
        let instruction = get_instruction(&0x37).unwrap();
        assert_eq!(instruction, &SWAP_A);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_swap_a() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x80;
        (&SWAP_A.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x37));
        assert_eq!(cpu.reg.a, 0x08);
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
    pub fn test_get_instruction_srl_c() {
        let instruction = get_instruction(&0x39).unwrap();
        assert_eq!(instruction, &SRL_C);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_srl_c() {
        let mut cpu = Cpu::new();
        cpu.reg.c = 0x80;
        (&SRL_C.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x39));
        assert_eq!(cpu.reg.c, 0x40);
    }

    #[test]
    pub fn test_get_instruction_srl_d() {
        let instruction = get_instruction(&0x3A).unwrap();
        assert_eq!(instruction, &SRL_D);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_srl_d() {
        let mut cpu = Cpu::new();
        cpu.reg.d = 0x80;
        (&SRL_D.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x3A));
        assert_eq!(cpu.reg.d, 0x40);
    }

    #[test]
    pub fn test_get_instruction_srl_e() {
        let instruction = get_instruction(&0x3B).unwrap();
        assert_eq!(instruction, &SRL_E);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_srl_e() {
        let mut cpu = Cpu::new();
        cpu.reg.e = 0x80;
        (&SRL_E.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x3B));
        assert_eq!(cpu.reg.e, 0x40);
    }

    #[test]
    pub fn test_get_instruction_srl_h() {
        let instruction = get_instruction(&0x3C).unwrap();
        assert_eq!(instruction, &SRL_H);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_srl_h() {
        let mut cpu = Cpu::new();
        cpu.reg.h = 0x80;
        (&SRL_H.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x3C));
        assert_eq!(cpu.reg.h, 0x40);
    }

    #[test]
    pub fn test_get_instruction_srl_l() {
        let instruction = get_instruction(&0x3D).unwrap();
        assert_eq!(instruction, &SRL_L);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_srl_l() {
        let mut cpu = Cpu::new();
        cpu.reg.l = 0x80;
        (&SRL_L.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x3D));
        assert_eq!(cpu.reg.l, 0x40);
    }

    #[test]
    pub fn test_get_instruction_srl_hl() {
        let instruction = get_instruction(&0x3E).unwrap();
        assert_eq!(instruction, &SRL_HL);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 16);
    }

    #[test]
    pub fn test_srl_hl() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        mmu.set_byte(0xC000 as usize, 0x80);
        cpu.reg.set_hl(0xC000);
        (&SRL_HL.handler)(&mut cpu, &mut mmu, &OpCode::CB(0x3E));
        assert_eq!(mmu.get_byte(0xC000 as usize), 0x40);
    }

    #[test]
    pub fn test_get_instruction_srl_a() {
        let instruction = get_instruction(&0x3F).unwrap();
        assert_eq!(instruction, &SRL_A);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_srl_a() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x80;
        (&SRL_A.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x3F));
        assert_eq!(cpu.reg.a, 0x40);
    }

    #[test]
    pub fn test_get_instruction_bit_0_b() {
        let instruction = get_instruction(&0x40).unwrap();
        assert_eq!(instruction, &BIT_0_B);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_bit_0_b() {
        let mut cpu = Cpu::new();
        cpu.reg.b = 0xFE;
        (&BIT_0_B.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x40));
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_get_instruction_bit_0_c() {
        let instruction = get_instruction(&0x41).unwrap();
        assert_eq!(instruction, &BIT_0_C);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_bit_0_c() {
        let mut cpu = Cpu::new();
        cpu.reg.c = 0xFE;
        (&BIT_0_C.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x41));
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_get_instruction_bit_0_d() {
        let instruction = get_instruction(&0x42).unwrap();
        assert_eq!(instruction, &BIT_0_D);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_bit_0_d() {
        let mut cpu = Cpu::new();
        cpu.reg.d = 0xFE;
        (&BIT_0_D.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x42));
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_get_instruction_bit_0_e() {
        let instruction = get_instruction(&0x43).unwrap();
        assert_eq!(instruction, &BIT_0_E);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_bit_0_e() {
        let mut cpu = Cpu::new();
        cpu.reg.e = 0xFE;
        (&BIT_0_E.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x43));
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_get_instruction_bit_0_h() {
        let instruction = get_instruction(&0x44).unwrap();
        assert_eq!(instruction, &BIT_0_H);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_bit_0_h() {
        let mut cpu = Cpu::new();
        cpu.reg.h = 0xFE;
        (&BIT_0_H.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x44));
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_get_instruction_bit_0_l() {
        let instruction = get_instruction(&0x45).unwrap();
        assert_eq!(instruction, &BIT_0_L);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_bit_0_l() {
        let mut cpu = Cpu::new();
        cpu.reg.l = 0xFE;
        (&BIT_0_L.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x45));
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_get_instruction_bit_0_hl() {
        let instruction = get_instruction(&0x46).unwrap();
        assert_eq!(instruction, &BIT_0_HL);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 16);
    }

    #[test]
    pub fn test_bit_0_hl() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.set_hl(0xC000);
        mmu.set_byte(0xC000 as usize, 0xFE);
        (&BIT_0_HL.handler)(&mut cpu, &mut mmu, &OpCode::CB(0x46));
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_get_instruction_bit_0_a() {
        let instruction = get_instruction(&0x47).unwrap();
        assert_eq!(instruction, &BIT_0_A);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_bit_0_a() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0xFE;
        (&BIT_0_A.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x47));
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_get_instruction_bit_1_b() {
        let instruction = get_instruction(&0x48).unwrap();
        assert_eq!(instruction, &BIT_1_B);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_bit_1_b() {
        let mut cpu = Cpu::new();
        cpu.reg.b = 0xFD;
        (&BIT_1_B.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x48));
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_get_instruction_bit_1_c() {
        let instruction = get_instruction(&0x49).unwrap();
        assert_eq!(instruction, &BIT_1_C);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_bit_1_c() {
        let mut cpu = Cpu::new();
        cpu.reg.c = 0xFD;
        (&BIT_1_C.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x49));
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_get_instruction_bit_1_d() {
        let instruction = get_instruction(&0x4A).unwrap();
        assert_eq!(instruction, &BIT_1_D);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_bit_1_d() {
        let mut cpu = Cpu::new();
        cpu.reg.d = 0xFD;
        (&BIT_1_D.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x4A));
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_get_instruction_bit_1_e() {
        let instruction = get_instruction(&0x4B).unwrap();
        assert_eq!(instruction, &BIT_1_E);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_bit_1_e() {
        let mut cpu = Cpu::new();
        cpu.reg.e = 0xFD;
        (&BIT_1_E.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x4B));
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_get_instruction_bit_1_h() {
        let instruction = get_instruction(&0x4C).unwrap();
        assert_eq!(instruction, &BIT_1_H);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_bit_1_h() {
        let mut cpu = Cpu::new();
        cpu.reg.h = 0xFD;
        (&BIT_1_H.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x4C));
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_get_instruction_bit_1_l() {
        let instruction = get_instruction(&0x4D).unwrap();
        assert_eq!(instruction, &BIT_1_L);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_bit_1_l() {
        let mut cpu = Cpu::new();
        cpu.reg.l = 0xFD;
        (&BIT_1_L.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x4D));
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_get_instruction_bit_1_hl() {
        let instruction = get_instruction(&0x4E).unwrap();
        assert_eq!(instruction, &BIT_1_HL);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 16);
    }

    #[test]
    pub fn test_bit_1_hl() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.set_hl(0xC000);
        mmu.set_byte(0xC000 as usize, 0xFD);
        (&BIT_1_HL.handler)(&mut cpu, &mut mmu, &OpCode::CB(0x4E));
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_get_instruction_bit_1_a() {
        let instruction = get_instruction(&0x4F).unwrap();
        assert_eq!(instruction, &BIT_1_A);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_bit_1_a() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0xFD;
        (&BIT_1_A.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x4F));
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_get_instruction_bit_2_b() {
        let instruction = get_instruction(&0x50).unwrap();
        assert_eq!(instruction, &BIT_2_B);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn get_instruction_bit_2_b() {
        let mut cpu = Cpu::new();
        cpu.reg.b = 0xFB;
        (&BIT_2_B.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x50));
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_get_instruction_bit_2_c() {
        let instruction = get_instruction(&0x51).unwrap();
        assert_eq!(instruction, &BIT_2_C);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn get_instruction_bit_2_c() {
        let mut cpu = Cpu::new();
        cpu.reg.c = 0xFB;
        (&BIT_2_C.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x51));
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_get_instruction_bit_2_d() {
        let instruction = get_instruction(&0x52).unwrap();
        assert_eq!(instruction, &BIT_2_D);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn get_instruction_bit_2_d() {
        let mut cpu = Cpu::new();
        cpu.reg.d = 0xFB;
        (&BIT_2_D.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x52));
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_get_instruction_bit_2_e() {
        let instruction = get_instruction(&0x53).unwrap();
        assert_eq!(instruction, &BIT_2_E);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn get_instruction_bit_2_e() {
        let mut cpu = Cpu::new();
        cpu.reg.e = 0xFB;
        (&BIT_2_E.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x53));
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_get_instruction_bit_2_h() {
        let instruction = get_instruction(&0x54).unwrap();
        assert_eq!(instruction, &BIT_2_H);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn get_instruction_bit_2_h() {
        let mut cpu = Cpu::new();
        cpu.reg.h = 0xFB;
        (&BIT_2_H.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x54));
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_get_instruction_bit_2_l() {
        let instruction = get_instruction(&0x55).unwrap();
        assert_eq!(instruction, &BIT_2_L);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn get_instruction_bit_2_l() {
        let mut cpu = Cpu::new();
        cpu.reg.l = 0xFB;
        (&BIT_2_L.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x55));
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_get_instruction_bit_2_hl() {
        let instruction = get_instruction(&0x56).unwrap();
        assert_eq!(instruction, &BIT_2_HL);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 16);
    }

    #[test]
    pub fn get_instruction_bit_2_hl() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.set_hl(0xC000);
        mmu.set_byte(0xC000 as usize, 0xFB);
        (&BIT_2_HL.handler)(&mut cpu, &mut mmu, &OpCode::CB(0x56));
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_get_instruction_bit_2_a() {
        let instruction = get_instruction(&0x57).unwrap();
        assert_eq!(instruction, &BIT_2_A);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn get_instruction_bit_2_a() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0xFB;
        (&BIT_2_A.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x57));
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_get_instruction_bit_3_b() {
        let instruction = get_instruction(&0x58).unwrap();
        assert_eq!(instruction, &BIT_3_B);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn get_instruction_bit_3_b() {
        let mut cpu = Cpu::new();
        cpu.reg.b = 0xF7;
        (&BIT_3_B.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x58));
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_get_instruction_bit_3_c() {
        let instruction = get_instruction(&0x59).unwrap();
        assert_eq!(instruction, &BIT_3_C);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn get_instruction_bit_3_c() {
        let mut cpu = Cpu::new();
        cpu.reg.c = 0xF7;
        (&BIT_3_C.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x59));
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_get_instruction_bit_3_d() {
        let instruction = get_instruction(&0x5A).unwrap();
        assert_eq!(instruction, &BIT_3_D);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn get_instruction_bit_3_d() {
        let mut cpu = Cpu::new();
        cpu.reg.d = 0xF7;
        (&BIT_3_D.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x5A));
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_get_instruction_bit_3_e() {
        let instruction = get_instruction(&0x5B).unwrap();
        assert_eq!(instruction, &BIT_3_E);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn get_instruction_bit_3_e() {
        let mut cpu = Cpu::new();
        cpu.reg.e = 0xF7;
        (&BIT_3_E.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x5B));
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_get_instruction_bit_3_h() {
        let instruction = get_instruction(&0x5C).unwrap();
        assert_eq!(instruction, &BIT_3_H);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn get_instruction_bit_3_h() {
        let mut cpu = Cpu::new();
        cpu.reg.h = 0xF7;
        (&BIT_3_H.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x5C));
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_get_instruction_bit_3_l() {
        let instruction = get_instruction(&0x5D).unwrap();
        assert_eq!(instruction, &BIT_3_L);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn get_instruction_bit_3_l() {
        let mut cpu = Cpu::new();
        cpu.reg.l = 0xF7;
        (&BIT_3_L.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x5D));
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_get_instruction_bit_3_hl() {
        let instruction = get_instruction(&0x5E).unwrap();
        assert_eq!(instruction, &BIT_3_HL);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 16);
    }

    #[test]
    pub fn get_instruction_bit_3_hl() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.set_hl(0xC000);
        mmu.set_byte(0xC000 as usize, 0xF7);
        (&BIT_3_HL.handler)(&mut cpu, &mut mmu, &OpCode::CB(0x5E));
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_get_instruction_bit_3_a() {
        let instruction = get_instruction(&0x5F).unwrap();
        assert_eq!(instruction, &BIT_3_A);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn get_instruction_bit_3_a() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0xF7;
        (&BIT_3_A.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x5F));
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_get_instruction_bit_4_b() {
        let instruction = get_instruction(&0x60).unwrap();
        assert_eq!(instruction, &BIT_4_B);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn get_instruction_bit_4_b() {
        let mut cpu = Cpu::new();
        cpu.reg.b = 0xEF;
        (&BIT_4_B.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x60));
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_get_instruction_bit_4_c() {
        let instruction = get_instruction(&0x61).unwrap();
        assert_eq!(instruction, &BIT_4_C);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn get_instruction_bit_4_c() {
        let mut cpu = Cpu::new();
        cpu.reg.c = 0xEF;
        (&BIT_4_C.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x61));
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_get_instruction_bit_4_d() {
        let instruction = get_instruction(&0x62).unwrap();
        assert_eq!(instruction, &BIT_4_D);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn get_instruction_bit_4_d() {
        let mut cpu = Cpu::new();
        cpu.reg.d = 0xEF;
        (&BIT_4_D.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x62));
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_get_instruction_bit_4_e() {
        let instruction = get_instruction(&0x63).unwrap();
        assert_eq!(instruction, &BIT_4_E);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn get_instruction_bit_4_e() {
        let mut cpu = Cpu::new();
        cpu.reg.e = 0xEF;
        (&BIT_4_E.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x63));
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_get_instruction_bit_4_h() {
        let instruction = get_instruction(&0x64).unwrap();
        assert_eq!(instruction, &BIT_4_H);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn get_instruction_bit_4_h() {
        let mut cpu = Cpu::new();
        cpu.reg.h = 0xEF;
        (&BIT_4_H.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x64));
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_get_instruction_bit_4_l() {
        let instruction = get_instruction(&0x65).unwrap();
        assert_eq!(instruction, &BIT_4_L);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn get_instruction_bit_4_l() {
        let mut cpu = Cpu::new();
        cpu.reg.l = 0xEF;
        (&BIT_4_L.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x65));
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_get_instruction_bit_4_hl() {
        let instruction = get_instruction(&0x66).unwrap();
        assert_eq!(instruction, &BIT_4_HL);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 16);
    }

    #[test]
    pub fn get_instruction_bit_4_hl() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.set_hl(0xC000);
        mmu.set_byte(0xC000 as usize, 0xEF);
        (&BIT_4_HL.handler)(&mut cpu, &mut mmu, &OpCode::CB(0x66));
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_get_instruction_bit_4_a() {
        let instruction = get_instruction(&0x67).unwrap();
        assert_eq!(instruction, &BIT_4_A);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn get_instruction_bit_4_a() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0xEF;
        (&BIT_4_A.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x67));
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_get_instruction_bit_5_b() {
        let instruction = get_instruction(&0x68).unwrap();
        assert_eq!(instruction, &BIT_5_B);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn get_instruction_bit_5_b() {
        let mut cpu = Cpu::new();
        cpu.reg.b = 0xDF;
        (&BIT_5_B.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x68));
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_get_instruction_bit_5_c() {
        let instruction = get_instruction(&0x69).unwrap();
        assert_eq!(instruction, &BIT_5_C);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn get_instruction_bit_5_c() {
        let mut cpu = Cpu::new();
        cpu.reg.c = 0xDF;
        (&BIT_5_C.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x69));
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_get_instruction_bit_5_d() {
        let instruction = get_instruction(&0x6A).unwrap();
        assert_eq!(instruction, &BIT_5_D);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn get_instruction_bit_5_d() {
        let mut cpu = Cpu::new();
        cpu.reg.d = 0xDF;
        (&BIT_5_D.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x6A));
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_get_instruction_bit_5_e() {
        let instruction = get_instruction(&0x6B).unwrap();
        assert_eq!(instruction, &BIT_5_E);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn get_instruction_bit_5_e() {
        let mut cpu = Cpu::new();
        cpu.reg.e = 0xDF;
        (&BIT_5_E.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x6B));
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_get_instruction_bit_5_h() {
        let instruction = get_instruction(&0x6C).unwrap();
        assert_eq!(instruction, &BIT_5_H);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn get_instruction_bit_5_h() {
        let mut cpu = Cpu::new();
        cpu.reg.h = 0xDF;
        (&BIT_5_H.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x6C));
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_get_instruction_bit_5_l() {
        let instruction = get_instruction(&0x6D).unwrap();
        assert_eq!(instruction, &BIT_5_L);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn get_instruction_bit_5_l() {
        let mut cpu = Cpu::new();
        cpu.reg.l = 0xDF;
        (&BIT_5_L.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x6D));
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_get_instruction_bit_5_hl() {
        let instruction = get_instruction(&0x6E).unwrap();
        assert_eq!(instruction, &BIT_5_HL);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 16);
    }

    #[test]
    pub fn get_instruction_bit_5_hl() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.set_hl(0xC000);
        mmu.set_byte(0xC000 as usize, 0xDF);
        (&BIT_5_HL.handler)(&mut cpu, &mut mmu, &OpCode::CB(0x6E));
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_get_instruction_bit_5_a() {
        let instruction = get_instruction(&0x6F).unwrap();
        assert_eq!(instruction, &BIT_5_A);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn get_instruction_bit_5_a() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0xDF;
        (&BIT_5_A.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x6F));
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_get_instruction_bit_6_b() {
        let instruction = get_instruction(&0x70).unwrap();
        assert_eq!(instruction, &BIT_6_B);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn get_instruction_bit_6_b() {
        let mut cpu = Cpu::new();
        cpu.reg.b = 0xBF;
        (&BIT_6_B.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x70));
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_get_instruction_bit_6_c() {
        let instruction = get_instruction(&0x71).unwrap();
        assert_eq!(instruction, &BIT_6_C);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn get_instruction_bit_6_c() {
        let mut cpu = Cpu::new();
        cpu.reg.c = 0xBF;
        (&BIT_6_C.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x71));
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_get_instruction_bit_6_d() {
        let instruction = get_instruction(&0x72).unwrap();
        assert_eq!(instruction, &BIT_6_D);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn get_instruction_bit_6_d() {
        let mut cpu = Cpu::new();
        cpu.reg.d = 0xBF;
        (&BIT_6_D.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x72));
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_get_instruction_bit_6_e() {
        let instruction = get_instruction(&0x73).unwrap();
        assert_eq!(instruction, &BIT_6_E);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn get_instruction_bit_6_e() {
        let mut cpu = Cpu::new();
        cpu.reg.e = 0xBF;
        (&BIT_6_E.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x73));
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_get_instruction_bit_6_h() {
        let instruction = get_instruction(&0x74).unwrap();
        assert_eq!(instruction, &BIT_6_H);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn get_instruction_bit_6_h() {
        let mut cpu = Cpu::new();
        cpu.reg.h = 0xBF;
        (&BIT_6_H.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x74));
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_get_instruction_bit_6_l() {
        let instruction = get_instruction(&0x75).unwrap();
        assert_eq!(instruction, &BIT_6_L);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn get_instruction_bit_6_l() {
        let mut cpu = Cpu::new();
        cpu.reg.l = 0xBF;
        (&BIT_6_L.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x75));
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_get_instruction_bit_6_hl() {
        let instruction = get_instruction(&0x76).unwrap();
        assert_eq!(instruction, &BIT_6_HL);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 16);
    }

    #[test]
    pub fn get_instruction_bit_6_hl() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.set_hl(0xC000);
        mmu.set_byte(0xC000 as usize, 0xBF);
        (&BIT_6_HL.handler)(&mut cpu, &mut mmu, &OpCode::CB(0x76));
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_get_instruction_bit_6_a() {
        let instruction = get_instruction(&0x77).unwrap();
        assert_eq!(instruction, &BIT_6_A);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn get_instruction_bit_6_a() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0xBF;
        (&BIT_6_A.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x77));
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_get_instruction_bit_7_b() {
        let instruction = get_instruction(&0x78).unwrap();
        assert_eq!(instruction, &BIT_7_B);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_bit_7_b() {
        let mut cpu = Cpu::new();
        cpu.reg.b = 0x7F;
        (&BIT_7_B.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x78));
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_get_instruction_bit_7_c() {
        let instruction = get_instruction(&0x79).unwrap();
        assert_eq!(instruction, &BIT_7_C);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_bit_7_c() {
        let mut cpu = Cpu::new();
        cpu.reg.c = 0x7F;
        (&BIT_7_C.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x79));
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_get_instruction_bit_7_d() {
        let instruction = get_instruction(&0x7A).unwrap();
        assert_eq!(instruction, &BIT_7_D);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_bit_7_d() {
        let mut cpu = Cpu::new();
        cpu.reg.d = 0x7F;
        (&BIT_7_D.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x7A));
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_get_instruction_bit_7_e() {
        let instruction = get_instruction(&0x7B).unwrap();
        assert_eq!(instruction, &BIT_7_E);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_bit_7_e() {
        let mut cpu = Cpu::new();
        cpu.reg.e = 0x7F;
        (&BIT_7_E.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x7B));
        assert!(cpu.reg.check_flag(Flag::Zero));
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

    #[test]
    pub fn test_get_instruction_bit_7_l() {
        let instruction = get_instruction(&0x7D).unwrap();
        assert_eq!(instruction, &BIT_7_L);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_bit_7_l() {
        let mut cpu = Cpu::new();
        cpu.reg.l = 0x7F;
        (&BIT_7_L.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x7D));
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_get_instruction_bit_7_hl() {
        let instruction = get_instruction(&0x7E).unwrap();
        assert_eq!(instruction, &BIT_7_HL);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 16);
    }

    #[test]
    pub fn test_bit_7_hl() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.set_hl(0xC000);
        mmu.set_byte(0xC000 as usize, 0x7F);
        (&BIT_7_HL.handler)(&mut cpu, &mut mmu, &OpCode::CB(0x7E));
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_get_instruction_bit_7_a() {
        let instruction = get_instruction(&0x7F).unwrap();
        assert_eq!(instruction, &BIT_7_A);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_bit_7_a() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x7F;
        (&BIT_7_A.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x7F));
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_get_instruction_res_0_b() {
        let instruction = get_instruction(&0x80).unwrap();
        assert_eq!(instruction, &RES_0_B);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_res_0_b() {
        let mut cpu = Cpu::new();
        cpu.reg.b = 0x01;
        (&RES_0_B.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x80));
        assert_eq!(cpu.reg.b, 0x00);
    }

    #[test]
    pub fn test_get_instruction_res_0_c() {
        let instruction = get_instruction(&0x81).unwrap();
        assert_eq!(instruction, &RES_0_C);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_res_0_c() {
        let mut cpu = Cpu::new();
        cpu.reg.c = 0x01;
        (&RES_0_C.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x81));
        assert_eq!(cpu.reg.c, 0x00);
    }

    #[test]
    pub fn test_get_instruction_res_0_d() {
        let instruction = get_instruction(&0x82).unwrap();
        assert_eq!(instruction, &RES_0_D);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_res_0_d() {
        let mut cpu = Cpu::new();
        cpu.reg.d = 0x01;
        (&RES_0_D.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x82));
        assert_eq!(cpu.reg.d, 0x00);
    }

    #[test]
    pub fn test_get_instruction_res_0_e() {
        let instruction = get_instruction(&0x83).unwrap();
        assert_eq!(instruction, &RES_0_E);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_res_0_e() {
        let mut cpu = Cpu::new();
        cpu.reg.e = 0x01;
        (&RES_0_E.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x83));
        assert_eq!(cpu.reg.e, 0x00);
    }

    #[test]
    pub fn test_get_instruction_res_0_h() {
        let instruction = get_instruction(&0x84).unwrap();
        assert_eq!(instruction, &RES_0_H);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn test_res_0_h() {
        let mut cpu = Cpu::new();
        cpu.reg.h = 0x01;
        (&RES_0_H.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x84));
        assert_eq!(cpu.reg.h, 0x00);
    }

    #[test]
    pub fn test_get_instruction_res_0_l() {
        let instruction = get_instruction(&0x85).unwrap();
        assert_eq!(instruction, &RES_0_L);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn res_0_l() {
        let mut cpu = Cpu::new();
        cpu.reg.l = 0x01;
        (&RES_0_L.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x85));
        assert_eq!(cpu.reg.l, 0x00);
    }

    #[test]
    pub fn test_get_instruction_res_0_hl() {
        let instruction = get_instruction(&0x86).unwrap();
        assert_eq!(instruction, &RES_0_HL);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 16);
    }

    #[test]
    pub fn test_res_0_hl() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.set_hl(0xC000);
        mmu.set_byte(0xC000 as usize, 0x01);
        (&RES_0_HL.handler)(&mut cpu, &mut mmu, &OpCode::CB(0x86));
        assert_eq!(mmu.get_byte(0xC000 as usize), 0x00);
    }

    #[test]
    pub fn test_get_instruction_res_0_a() {
        let instruction = get_instruction(&0x87).unwrap();
        assert_eq!(instruction, &RES_0_A);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn res_0_a() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x01;
        (&RES_0_A.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x87));
        assert_eq!(cpu.reg.a, 0x00);
    }

    #[test]
    pub fn test_get_instruction_res_1_b() {
        let instruction = get_instruction(&0x88).unwrap();
        assert_eq!(instruction, &RES_1_B);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn res_1_b() {
        let mut cpu = Cpu::new();
        cpu.reg.b = 0x02;
        (&RES_1_B.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x88));
        assert_eq!(cpu.reg.b, 0x00);
    }

    #[test]
    pub fn test_get_instruction_res_1_c() {
        let instruction = get_instruction(&0x89).unwrap();
        assert_eq!(instruction, &RES_1_C);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn res_1_c() {
        let mut cpu = Cpu::new();
        cpu.reg.c = 0x02;
        (&RES_1_C.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x89));
        assert_eq!(cpu.reg.c, 0x00);
    }

    #[test]
    pub fn test_get_instruction_res_1_d() {
        let instruction = get_instruction(&0x8A).unwrap();
        assert_eq!(instruction, &RES_1_D);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn res_1_d() {
        let mut cpu = Cpu::new();
        cpu.reg.d = 0x02;
        (&RES_1_D.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x8A));
        assert_eq!(cpu.reg.d, 0x00);
    }

    #[test]
    pub fn test_get_instruction_res_1_e() {
        let instruction = get_instruction(&0x8B).unwrap();
        assert_eq!(instruction, &RES_1_E);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn res_1_e() {
        let mut cpu = Cpu::new();
        cpu.reg.e = 0x02;
        (&RES_1_E.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x8B));
        assert_eq!(cpu.reg.e, 0x00);
    }

    #[test]
    pub fn test_get_instruction_res_1_h() {
        let instruction = get_instruction(&0x8C).unwrap();
        assert_eq!(instruction, &RES_1_H);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn res_1_h() {
        let mut cpu = Cpu::new();
        cpu.reg.h = 0x02;
        (&RES_1_H.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x8C));
        assert_eq!(cpu.reg.h, 0x00);
    }

    #[test]
    pub fn test_get_instruction_res_1_l() {
        let instruction = get_instruction(&0x8D).unwrap();
        assert_eq!(instruction, &RES_1_L);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn res_1_l() {
        let mut cpu = Cpu::new();
        cpu.reg.l = 0x02;
        (&RES_1_L.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x8D));
        assert_eq!(cpu.reg.l, 0x00);
    }

    #[test]
    pub fn test_get_instruction_res_1_hl() {
        let instruction = get_instruction(&0x8E).unwrap();
        assert_eq!(instruction, &RES_1_HL);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 16);
    }

    #[test]
    pub fn res_1_hl() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.set_hl(0xC000);
        mmu.set_byte(0xC000 as usize, 0x02);
        (&RES_1_HL.handler)(&mut cpu, &mut mmu, &OpCode::CB(0x8E));
        assert_eq!(mmu.get_byte(0xC000 as usize), 0x00);
    }

    #[test]
    pub fn test_get_instruction_res_1_a() {
        let instruction = get_instruction(&0x8F).unwrap();
        assert_eq!(instruction, &RES_1_A);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn res_1_a() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x02;
        (&RES_1_A.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x8F));
        assert_eq!(cpu.reg.a, 0x00);
    }

    #[test]
    pub fn test_get_instruction_res_2_b() {
        let instruction = get_instruction(&0x90).unwrap();
        assert_eq!(instruction, &RES_2_B);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn res_2_b() {
        let mut cpu = Cpu::new();
        cpu.reg.b = 0x04;
        (&RES_2_B.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x90));
        assert_eq!(cpu.reg.b, 0x00);
    }

    #[test]
    pub fn test_get_instruction_res_2_c() {
        let instruction = get_instruction(&0x91).unwrap();
        assert_eq!(instruction, &RES_2_C);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn res_2_c() {
        let mut cpu = Cpu::new();
        cpu.reg.c = 0x04;
        (&RES_2_C.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x91));
        assert_eq!(cpu.reg.c, 0x00);
    }

    #[test]
    pub fn test_get_instruction_res_2_d() {
        let instruction = get_instruction(&0x92).unwrap();
        assert_eq!(instruction, &RES_2_D);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn res_2_d() {
        let mut cpu = Cpu::new();
        cpu.reg.d = 0x04;
        (&RES_2_D.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x92));
        assert_eq!(cpu.reg.d, 0x00);
    }

    #[test]
    pub fn test_get_instruction_res_2_e() {
        let instruction = get_instruction(&0x93).unwrap();
        assert_eq!(instruction, &RES_2_E);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn res_2_e() {
        let mut cpu = Cpu::new();
        cpu.reg.e = 0x04;
        (&RES_2_E.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x93));
        assert_eq!(cpu.reg.e, 0x00);
    }

    #[test]
    pub fn test_get_instruction_res_2_h() {
        let instruction = get_instruction(&0x94).unwrap();
        assert_eq!(instruction, &RES_2_H);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn res_2_h() {
        let mut cpu = Cpu::new();
        cpu.reg.h = 0x04;
        (&RES_2_H.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x94));
        assert_eq!(cpu.reg.h, 0x00);
    }

    #[test]
    pub fn test_get_instruction_res_2_l() {
        let instruction = get_instruction(&0x95).unwrap();
        assert_eq!(instruction, &RES_2_L);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn res_2_l() {
        let mut cpu = Cpu::new();
        cpu.reg.l = 0x04;
        (&RES_2_L.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x95));
        assert_eq!(cpu.reg.l, 0x00);
    }

    #[test]
    pub fn test_get_instruction_res_2_hl() {
        let instruction = get_instruction(&0x96).unwrap();
        assert_eq!(instruction, &RES_2_HL);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 16);
    }

    #[test]
    pub fn res_2_hl() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.set_hl(0xC000);
        mmu.set_byte(0xC000 as usize, 0x04);
        (&RES_2_HL.handler)(&mut cpu, &mut mmu, &OpCode::CB(0x96));
        assert_eq!(mmu.get_byte(0xC000 as usize), 0x00);
    }

    #[test]
    pub fn test_get_instruction_res_2_a() {
        let instruction = get_instruction(&0x97).unwrap();
        assert_eq!(instruction, &RES_2_A);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn res_2_a() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x04;
        (&RES_2_A.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x97));
        assert_eq!(cpu.reg.a, 0x00);
    }

    #[test]
    pub fn test_get_instruction_res_3_b() {
        let instruction = get_instruction(&0x98).unwrap();
        assert_eq!(instruction, &RES_3_B);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn res_3_b() {
        let mut cpu = Cpu::new();
        cpu.reg.b = 0x08;
        (&RES_3_B.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x98));
        assert_eq!(cpu.reg.b, 0x00);
    }

    #[test]
    pub fn test_get_instruction_res_3_c() {
        let instruction = get_instruction(&0x99).unwrap();
        assert_eq!(instruction, &RES_3_C);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn res_3_c() {
        let mut cpu = Cpu::new();
        cpu.reg.c = 0x08;
        (&RES_3_C.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x99));
        assert_eq!(cpu.reg.c, 0x00);
    }

    #[test]
    pub fn test_get_instruction_res_3_d() {
        let instruction = get_instruction(&0x9A).unwrap();
        assert_eq!(instruction, &RES_3_D);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn res_3_d() {
        let mut cpu = Cpu::new();
        cpu.reg.d = 0x08;
        (&RES_3_D.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x9A));
        assert_eq!(cpu.reg.d, 0x00);
    }

    #[test]
    pub fn test_get_instruction_res_3_e() {
        let instruction = get_instruction(&0x9B).unwrap();
        assert_eq!(instruction, &RES_3_E);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn res_3_e() {
        let mut cpu = Cpu::new();
        cpu.reg.e = 0x08;
        (&RES_3_E.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x9B));
        assert_eq!(cpu.reg.e, 0x00);
    }

    #[test]
    pub fn test_get_instruction_res_3_h() {
        let instruction = get_instruction(&0x9C).unwrap();
        assert_eq!(instruction, &RES_3_H);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn res_3_h() {
        let mut cpu = Cpu::new();
        cpu.reg.h = 0x08;
        (&RES_3_H.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x9C));
        assert_eq!(cpu.reg.h, 0x00);
    }

    #[test]
    pub fn test_get_instruction_res_3_l() {
        let instruction = get_instruction(&0x9D).unwrap();
        assert_eq!(instruction, &RES_3_L);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn res_3_l() {
        let mut cpu = Cpu::new();
        cpu.reg.l = 0x08;
        (&RES_3_L.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x9D));
        assert_eq!(cpu.reg.l, 0x00);
    }

    #[test]
    pub fn test_get_instruction_res_3_hl() {
        let instruction = get_instruction(&0x9E).unwrap();
        assert_eq!(instruction, &RES_3_HL);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 16);
    }

    #[test]
    pub fn res_3_hl() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.set_hl(0xC000);
        mmu.set_byte(0xC000 as usize, 0x08);
        (&RES_3_HL.handler)(&mut cpu, &mut mmu, &OpCode::CB(0x9E));
        assert_eq!(mmu.get_byte(0xC000 as usize), 0x00);
    }

    #[test]
    pub fn test_get_instruction_res_3_a() {
        let instruction = get_instruction(&0x9F).unwrap();
        assert_eq!(instruction, &RES_3_A);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn res_3_a() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x08;
        (&RES_3_A.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0x9F));
        assert_eq!(cpu.reg.a, 0x00);
    }

    #[test]
    pub fn test_get_instruction_res_4_b() {
        let instruction = get_instruction(&0xA0).unwrap();
        assert_eq!(instruction, &RES_4_B);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn res_4_b() {
        let mut cpu = Cpu::new();
        cpu.reg.b = 0x10;
        (&RES_4_B.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xA0));
        assert_eq!(cpu.reg.b, 0x00);
    }

    #[test]
    pub fn test_get_instruction_res_4_c() {
        let instruction = get_instruction(&0xA1).unwrap();
        assert_eq!(instruction, &RES_4_C);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn res_4_c() {
        let mut cpu = Cpu::new();
        cpu.reg.c = 0x10;
        (&RES_4_C.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xA1));
        assert_eq!(cpu.reg.c, 0x00);
    }

    #[test]
    pub fn test_get_instruction_res_4_d() {
        let instruction = get_instruction(&0xA2).unwrap();
        assert_eq!(instruction, &RES_4_D);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn res_4_d() {
        let mut cpu = Cpu::new();
        cpu.reg.d = 0x10;
        (&RES_4_D.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xA2));
        assert_eq!(cpu.reg.d, 0x00);
    }

    #[test]
    pub fn test_get_instruction_res_4_e() {
        let instruction = get_instruction(&0xA3).unwrap();
        assert_eq!(instruction, &RES_4_E);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn res_4_e() {
        let mut cpu = Cpu::new();
        cpu.reg.e = 0x10;
        (&RES_4_E.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xA3));
        assert_eq!(cpu.reg.e, 0x00);
    }

    #[test]
    pub fn test_get_instruction_res_4_h() {
        let instruction = get_instruction(&0xA4).unwrap();
        assert_eq!(instruction, &RES_4_H);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn res_4_h() {
        let mut cpu = Cpu::new();
        cpu.reg.h = 0x10;
        (&RES_4_H.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xA4));
        assert_eq!(cpu.reg.h, 0x00);
    }

    #[test]
    pub fn test_get_instruction_res_4_l() {
        let instruction = get_instruction(&0xA5).unwrap();
        assert_eq!(instruction, &RES_4_L);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn res_4_l() {
        let mut cpu = Cpu::new();
        cpu.reg.l = 0x10;
        (&RES_4_L.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xA5));
        assert_eq!(cpu.reg.l, 0x00);
    }

    #[test]
    pub fn test_get_instruction_res_4_hl() {
        let instruction = get_instruction(&0xA6).unwrap();
        assert_eq!(instruction, &RES_4_HL);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 16);
    }

    #[test]
    pub fn res_4_hl() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.set_hl(0xC000);
        mmu.set_byte(0xC000 as usize, 0x10);
        (&RES_4_HL.handler)(&mut cpu, &mut mmu, &OpCode::CB(0xA6));
        assert_eq!(mmu.get_byte(0xC000 as usize), 0x00);
    }

    #[test]
    pub fn test_get_instruction_res_4_a() {
        let instruction = get_instruction(&0xA7).unwrap();
        assert_eq!(instruction, &RES_4_A);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn res_4_a() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x10;
        (&RES_4_A.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xA7));
        assert_eq!(cpu.reg.a, 0x00);
    }

    #[test]
    pub fn test_get_instruction_res_5_b() {
        let instruction = get_instruction(&0xA8).unwrap();
        assert_eq!(instruction, &RES_5_B);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn res_5_b() {
        let mut cpu = Cpu::new();
        cpu.reg.b = 0x20;
        (&RES_5_B.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xA8));
        assert_eq!(cpu.reg.b, 0x00);
    }

    #[test]
    pub fn test_get_instruction_res_5_c() {
        let instruction = get_instruction(&0xA9).unwrap();
        assert_eq!(instruction, &RES_5_C);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn res_5_c() {
        let mut cpu = Cpu::new();
        cpu.reg.c = 0x20;
        (&RES_5_C.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xA9));
        assert_eq!(cpu.reg.c, 0x00);
    }

    #[test]
    pub fn test_get_instruction_res_5_d() {
        let instruction = get_instruction(&0xAA).unwrap();
        assert_eq!(instruction, &RES_5_D);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn res_5_d() {
        let mut cpu = Cpu::new();
        cpu.reg.d = 0x20;
        (&RES_5_D.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xAA));
        assert_eq!(cpu.reg.d, 0x00);
    }

    #[test]
    pub fn test_get_instruction_res_5_e() {
        let instruction = get_instruction(&0xAB).unwrap();
        assert_eq!(instruction, &RES_5_E);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn res_5_e() {
        let mut cpu = Cpu::new();
        cpu.reg.e = 0x20;
        (&RES_5_E.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xAB));
        assert_eq!(cpu.reg.e, 0x00);
    }

    #[test]
    pub fn test_get_instruction_res_5_h() {
        let instruction = get_instruction(&0xAC).unwrap();
        assert_eq!(instruction, &RES_5_H);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn res_5_h() {
        let mut cpu = Cpu::new();
        cpu.reg.h = 0x20;
        (&RES_5_H.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xAC));
        assert_eq!(cpu.reg.h, 0x00);
    }

    #[test]
    pub fn test_get_instruction_res_5_l() {
        let instruction = get_instruction(&0xAD).unwrap();
        assert_eq!(instruction, &RES_5_L);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn res_5_l() {
        let mut cpu = Cpu::new();
        cpu.reg.l = 0x20;
        (&RES_5_L.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xAD));
        assert_eq!(cpu.reg.l, 0x00);
    }

    #[test]
    pub fn test_get_instruction_res_5_hl() {
        let instruction = get_instruction(&0xAE).unwrap();
        assert_eq!(instruction, &RES_5_HL);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 16);
    }

    #[test]
    pub fn res_5_hl() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.set_hl(0xC000);
        mmu.set_byte(0xC000 as usize, 0x20);
        (&RES_5_HL.handler)(&mut cpu, &mut mmu, &OpCode::CB(0xAE));
        assert_eq!(mmu.get_byte(0xC000 as usize), 0x00);
    }

    #[test]
    pub fn test_get_instruction_res_5_a() {
        let instruction = get_instruction(&0xAF).unwrap();
        assert_eq!(instruction, &RES_5_A);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn res_5_a() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x20;
        (&RES_5_A.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xAF));
        assert_eq!(cpu.reg.a, 0x00);
    }

    #[test]
    pub fn test_get_instruction_res_6_b() {
        let instruction = get_instruction(&0xB0).unwrap();
        assert_eq!(instruction, &RES_6_B);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn res_6_b() {
        let mut cpu = Cpu::new();
        cpu.reg.b = 0x40;
        (&RES_6_B.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xB0));
        assert_eq!(cpu.reg.b, 0x00);
    }

    #[test]
    pub fn test_get_instruction_res_6_c() {
        let instruction = get_instruction(&0xB1).unwrap();
        assert_eq!(instruction, &RES_6_C);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn res_6_c() {
        let mut cpu = Cpu::new();
        cpu.reg.c = 0x40;
        (&RES_6_C.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xB1));
        assert_eq!(cpu.reg.c, 0x00);
    }

    #[test]
    pub fn test_get_instruction_res_6_d() {
        let instruction = get_instruction(&0xB2).unwrap();
        assert_eq!(instruction, &RES_6_D);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn res_6_d() {
        let mut cpu = Cpu::new();
        cpu.reg.d = 0x40;
        (&RES_6_D.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xB2));
        assert_eq!(cpu.reg.d, 0x00);
    }

    #[test]
    pub fn test_get_instruction_res_6_e() {
        let instruction = get_instruction(&0xB3).unwrap();
        assert_eq!(instruction, &RES_6_E);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn res_6_e() {
        let mut cpu = Cpu::new();
        cpu.reg.e = 0x40;
        (&RES_6_E.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xB3));
        assert_eq!(cpu.reg.e, 0x00);
    }

    #[test]
    pub fn test_get_instruction_res_6_h() {
        let instruction = get_instruction(&0xB4).unwrap();
        assert_eq!(instruction, &RES_6_H);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn res_6_h() {
        let mut cpu = Cpu::new();
        cpu.reg.h = 0x40;
        (&RES_6_H.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xB4));
        assert_eq!(cpu.reg.h, 0x00);
    }

    #[test]
    pub fn test_get_instruction_res_6_l() {
        let instruction = get_instruction(&0xB5).unwrap();
        assert_eq!(instruction, &RES_6_L);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn res_6_l() {
        let mut cpu = Cpu::new();
        cpu.reg.l = 0x40;
        (&RES_6_L.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xB5));
        assert_eq!(cpu.reg.l, 0x00);
    }

    #[test]
    pub fn test_get_instruction_res_6_hl() {
        let instruction = get_instruction(&0xB6).unwrap();
        assert_eq!(instruction, &RES_6_HL);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 16);
    }

    #[test]
    pub fn res_6_hl() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.set_hl(0xC000);
        mmu.set_byte(0xC000 as usize, 0x40);
        (&RES_6_HL.handler)(&mut cpu, &mut mmu, &OpCode::CB(0xB6));
        assert_eq!(mmu.get_byte(0xC000 as usize), 0x00);
    }

    #[test]
    pub fn test_get_instruction_res_6_a() {
        let instruction = get_instruction(&0xB7).unwrap();
        assert_eq!(instruction, &RES_6_A);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn res_6_a() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x40;
        (&RES_6_A.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xB7));
        assert_eq!(cpu.reg.a, 0x00);
    }

    #[test]
    pub fn test_get_instruction_res_7_b() {
        let instruction = get_instruction(&0xB8).unwrap();
        assert_eq!(instruction, &RES_7_B);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn res_7_b() {
        let mut cpu = Cpu::new();
        cpu.reg.b = 0x80;
        (&RES_7_B.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xB8));
        assert_eq!(cpu.reg.b, 0x00);
    }

    #[test]
    pub fn test_get_instruction_res_7_c() {
        let instruction = get_instruction(&0xB9).unwrap();
        assert_eq!(instruction, &RES_7_C);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn res_7_c() {
        let mut cpu = Cpu::new();
        cpu.reg.c = 0x80;
        (&RES_7_C.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xB9));
        assert_eq!(cpu.reg.c, 0x00);
    }

    #[test]
    pub fn test_get_instruction_res_7_d() {
        let instruction = get_instruction(&0xBA).unwrap();
        assert_eq!(instruction, &RES_7_D);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn res_7_d() {
        let mut cpu = Cpu::new();
        cpu.reg.d = 0x80;
        (&RES_7_D.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xBA));
        assert_eq!(cpu.reg.d, 0x00);
    }

    #[test]
    pub fn test_get_instruction_res_7_e() {
        let instruction = get_instruction(&0xBB).unwrap();
        assert_eq!(instruction, &RES_7_E);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn res_7_e() {
        let mut cpu = Cpu::new();
        cpu.reg.e = 0x80;
        (&RES_7_E.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xBB));
        assert_eq!(cpu.reg.e, 0x00);
    }

    #[test]
    pub fn test_get_instruction_res_7_h() {
        let instruction = get_instruction(&0xBC).unwrap();
        assert_eq!(instruction, &RES_7_H);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn res_7_h() {
        let mut cpu = Cpu::new();
        cpu.reg.h = 0x80;
        (&RES_7_H.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xBC));
        assert_eq!(cpu.reg.h, 0x00);
    }

    #[test]
    pub fn test_get_instruction_res_7_l() {
        let instruction = get_instruction(&0xBD).unwrap();
        assert_eq!(instruction, &RES_7_L);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn res_7_l() {
        let mut cpu = Cpu::new();
        cpu.reg.l = 0x80;
        (&RES_7_L.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xBD));
        assert_eq!(cpu.reg.l, 0x00);
    }

    #[test]
    pub fn test_get_instruction_res_7_hl() {
        let instruction = get_instruction(&0xBE).unwrap();
        assert_eq!(instruction, &RES_7_HL);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 16);
    }

    #[test]
    pub fn res_7_hl() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.set_hl(0xC000);
        mmu.set_byte(0xC000 as usize, 0x80);
        (&RES_7_HL.handler)(&mut cpu, &mut mmu, &OpCode::CB(0xBE));
        assert_eq!(mmu.get_byte(0xC000 as usize), 0x00);
    }

    #[test]
    pub fn test_get_instruction_res_7_a() {
        let instruction = get_instruction(&0xBF).unwrap();
        assert_eq!(instruction, &RES_7_A);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn res_7_a() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x80;
        (&RES_7_A.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xBF));
        assert_eq!(cpu.reg.a, 0x00);
    }

    #[test]
    pub fn test_get_instruction_set_0_b() {
        let instruction = get_instruction(&0xC0).unwrap();
        assert_eq!(instruction, &SET_0_B);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn set_0_b() {
        let mut cpu = Cpu::new();
        cpu.reg.b = 0x00;
        (&SET_0_B.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xC0));
        assert_eq!(cpu.reg.b, 0x01);
    }

    #[test]
    pub fn test_get_instruction_set_0_c() {
        let instruction = get_instruction(&0xC1).unwrap();
        assert_eq!(instruction, &SET_0_C);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn set_0_c() {
        let mut cpu = Cpu::new();
        cpu.reg.c = 0x00;
        (&SET_0_C.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xC1));
        assert_eq!(cpu.reg.c, 0x01);
    }

    #[test]
    pub fn test_get_instruction_set_0_d() {
        let instruction = get_instruction(&0xC2).unwrap();
        assert_eq!(instruction, &SET_0_D);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn set_0_d() {
        let mut cpu = Cpu::new();
        cpu.reg.d = 0x00;
        (&SET_0_D.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xC2));
        assert_eq!(cpu.reg.d, 0x01);
    }

    #[test]
    pub fn test_get_instruction_set_0_e() {
        let instruction = get_instruction(&0xC3).unwrap();
        assert_eq!(instruction, &SET_0_E);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn set_0_e() {
        let mut cpu = Cpu::new();
        cpu.reg.e = 0x00;
        (&SET_0_E.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xC3));
        assert_eq!(cpu.reg.e, 0x01);
    }

    #[test]
    pub fn test_get_instruction_set_0_h() {
        let instruction = get_instruction(&0xC4).unwrap();
        assert_eq!(instruction, &SET_0_H);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn set_0_h() {
        let mut cpu = Cpu::new();
        cpu.reg.h = 0x00;
        (&SET_0_H.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xC4));
        assert_eq!(cpu.reg.h, 0x01);
    }

    #[test]
    pub fn test_get_instruction_set_0_l() {
        let instruction = get_instruction(&0xC5).unwrap();
        assert_eq!(instruction, &SET_0_L);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn set_0_l() {
        let mut cpu = Cpu::new();
        cpu.reg.l = 0x00;
        (&SET_0_L.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xC5));
        assert_eq!(cpu.reg.l, 0x01);
    }

    #[test]
    pub fn test_get_instruction_set_0_hl() {
        let instruction = get_instruction(&0xC6).unwrap();
        assert_eq!(instruction, &SET_0_HL);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 16);
    }

    #[test]
    pub fn set_0_hl() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.set_hl(0xC000);
        mmu.set_byte(0xC000 as usize, 0x00);
        (&SET_0_HL.handler)(&mut cpu, &mut mmu, &OpCode::CB(0xC6));
        assert_eq!(mmu.get_byte(0xC000 as usize), 0x01);
    }

    #[test]
    pub fn test_get_instruction_set_0_a() {
        let instruction = get_instruction(&0xC7).unwrap();
        assert_eq!(instruction, &SET_0_A);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn set_0_a() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x00;
        (&SET_0_A.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xC7));
        assert_eq!(cpu.reg.a, 0x01);
    }

    #[test]
    pub fn test_get_instruction_set_1_b() {
        let instruction = get_instruction(&0xC8).unwrap();
        assert_eq!(instruction, &SET_1_B);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn set_1_b() {
        let mut cpu = Cpu::new();
        cpu.reg.b = 0x00;
        (&SET_1_B.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xC8));
        assert_eq!(cpu.reg.b, 0x02);
    }

    #[test]
    pub fn test_get_instruction_set_1_c() {
        let instruction = get_instruction(&0xC9).unwrap();
        assert_eq!(instruction, &SET_1_C);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn set_1_c() {
        let mut cpu = Cpu::new();
        cpu.reg.c = 0x00;
        (&SET_1_C.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xC9));
        assert_eq!(cpu.reg.c, 0x02);
    }

    #[test]
    pub fn test_get_instruction_set_1_d() {
        let instruction = get_instruction(&0xCA).unwrap();
        assert_eq!(instruction, &SET_1_D);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn set_1_d() {
        let mut cpu = Cpu::new();
        cpu.reg.d = 0x00;
        (&SET_1_D.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xCA));
        assert_eq!(cpu.reg.d, 0x02);
    }

    #[test]
    pub fn test_get_instruction_set_1_e() {
        let instruction = get_instruction(&0xCB).unwrap();
        assert_eq!(instruction, &SET_1_E);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn set_1_e() {
        let mut cpu = Cpu::new();
        cpu.reg.e = 0x00;
        (&SET_1_E.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xCB));
        assert_eq!(cpu.reg.e, 0x02);
    }

    #[test]
    pub fn test_get_instruction_set_1_h() {
        let instruction = get_instruction(&0xCC).unwrap();
        assert_eq!(instruction, &SET_1_H);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn set_1_h() {
        let mut cpu = Cpu::new();
        cpu.reg.h = 0x00;
        (&SET_1_H.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xCC));
        assert_eq!(cpu.reg.h, 0x02);
    }

    #[test]
    pub fn test_get_instruction_set_1_l() {
        let instruction = get_instruction(&0xCD).unwrap();
        assert_eq!(instruction, &SET_1_L);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn set_1_l() {
        let mut cpu = Cpu::new();
        cpu.reg.l = 0x00;
        (&SET_1_L.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xCD));
        assert_eq!(cpu.reg.l, 0x02);
    }

    #[test]
    pub fn test_get_instruction_set_1_hl() {
        let instruction = get_instruction(&0xCE).unwrap();
        assert_eq!(instruction, &SET_1_HL);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 16);
    }

    #[test]
    pub fn set_1_hl() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.set_hl(0xC000);
        mmu.set_byte(0xC000 as usize, 0x00);
        (&SET_1_HL.handler)(&mut cpu, &mut mmu, &OpCode::CB(0xCE));
        assert_eq!(mmu.get_byte(0xC000 as usize), 0x02);
    }

    #[test]
    pub fn test_get_instruction_set_1_a() {
        let instruction = get_instruction(&0xCF).unwrap();
        assert_eq!(instruction, &SET_1_A);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn set_1_a() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x00;
        (&SET_1_A.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xCF));
        assert_eq!(cpu.reg.a, 0x02);
    }

    #[test]
    pub fn test_get_instruction_set_2_b() {
        let instruction = get_instruction(&0xD0).unwrap();
        assert_eq!(instruction, &SET_2_B);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn set_2_b() {
        let mut cpu = Cpu::new();
        cpu.reg.b = 0x00;
        (&SET_2_B.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xD0));
        assert_eq!(cpu.reg.b, 0x04);
    }

    #[test]
    pub fn test_get_instruction_set_2_c() {
        let instruction = get_instruction(&0xD1).unwrap();
        assert_eq!(instruction, &SET_2_C);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn set_2_c() {
        let mut cpu = Cpu::new();
        cpu.reg.c = 0x00;
        (&SET_2_C.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xD1));
        assert_eq!(cpu.reg.c, 0x04);
    }

    #[test]
    pub fn test_get_instruction_set_2_d() {
        let instruction = get_instruction(&0xD2).unwrap();
        assert_eq!(instruction, &SET_2_D);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn set_2_d() {
        let mut cpu = Cpu::new();
        cpu.reg.d = 0x00;
        (&SET_2_D.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xD2));
        assert_eq!(cpu.reg.d, 0x04);
    }

    #[test]
    pub fn test_get_instruction_set_2_e() {
        let instruction = get_instruction(&0xD3).unwrap();
        assert_eq!(instruction, &SET_2_E);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn set_2_e() {
        let mut cpu = Cpu::new();
        cpu.reg.e = 0x00;
        (&SET_2_E.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xD3));
        assert_eq!(cpu.reg.e, 0x04);
    }

    #[test]
    pub fn test_get_instruction_set_2_h() {
        let instruction = get_instruction(&0xD4).unwrap();
        assert_eq!(instruction, &SET_2_H);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn set_2_h() {
        let mut cpu = Cpu::new();
        cpu.reg.h = 0x00;
        (&SET_2_H.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xD4));
        assert_eq!(cpu.reg.h, 0x04);
    }

    #[test]
    pub fn test_get_instruction_set_2_l() {
        let instruction = get_instruction(&0xD5).unwrap();
        assert_eq!(instruction, &SET_2_L);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn set_2_l() {
        let mut cpu = Cpu::new();
        cpu.reg.l = 0x00;
        (&SET_2_L.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xD5));
        assert_eq!(cpu.reg.l, 0x04);
    }

    #[test]
    pub fn test_get_instruction_set_2_hl() {
        let instruction = get_instruction(&0xD6).unwrap();
        assert_eq!(instruction, &SET_2_HL);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 16);
    }

    #[test]
    pub fn set_2_hl() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.set_hl(0xC000);
        mmu.set_byte(0xC000 as usize, 0x00);
        (&SET_2_HL.handler)(&mut cpu, &mut mmu, &OpCode::CB(0xD6));
        assert_eq!(mmu.get_byte(0xC000 as usize), 0x04);
    }

    #[test]
    pub fn test_get_instruction_set_2_a() {
        let instruction = get_instruction(&0xD7).unwrap();
        assert_eq!(instruction, &SET_2_A);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn set_2_a() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x00;
        (&SET_2_A.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xD7));
        assert_eq!(cpu.reg.a, 0x04);
    }

    #[test]
    pub fn test_get_instruction_set_3_b() {
        let instruction = get_instruction(&0xD8).unwrap();
        assert_eq!(instruction, &SET_3_B);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn set_3_b() {
        let mut cpu = Cpu::new();
        cpu.reg.b = 0x00;
        (&SET_3_B.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xD8));
        assert_eq!(cpu.reg.b, 0x08);
    }

    #[test]
    pub fn test_get_instruction_set_3_c() {
        let instruction = get_instruction(&0xD9).unwrap();
        assert_eq!(instruction, &SET_3_C);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn set_3_c() {
        let mut cpu = Cpu::new();
        cpu.reg.c = 0x00;
        (&SET_3_C.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xD9));
        assert_eq!(cpu.reg.c, 0x08);
    }

    #[test]
    pub fn test_get_instruction_set_3_d() {
        let instruction = get_instruction(&0xDA).unwrap();
        assert_eq!(instruction, &SET_3_D);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn set_3_d() {
        let mut cpu = Cpu::new();
        cpu.reg.d = 0x00;
        (&SET_3_D.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xDA));
        assert_eq!(cpu.reg.d, 0x08);
    }

    #[test]
    pub fn test_get_instruction_set_3_e() {
        let instruction = get_instruction(&0xDB).unwrap();
        assert_eq!(instruction, &SET_3_E);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn set_3_e() {
        let mut cpu = Cpu::new();
        cpu.reg.e = 0x00;
        (&SET_3_E.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xDB));
        assert_eq!(cpu.reg.e, 0x08);
    }

    #[test]
    pub fn test_get_instruction_set_3_h() {
        let instruction = get_instruction(&0xDC).unwrap();
        assert_eq!(instruction, &SET_3_H);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn set_3_h() {
        let mut cpu = Cpu::new();
        cpu.reg.h = 0x00;
        (&SET_3_H.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xDC));
        assert_eq!(cpu.reg.h, 0x08);
    }

    #[test]
    pub fn test_get_instruction_set_3_l() {
        let instruction = get_instruction(&0xDD).unwrap();
        assert_eq!(instruction, &SET_3_L);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn set_3_l() {
        let mut cpu = Cpu::new();
        cpu.reg.l = 0x00;
        (&SET_3_L.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xDD));
        assert_eq!(cpu.reg.l, 0x08);
    }

    #[test]
    pub fn test_get_instruction_set_3_hl() {
        let instruction = get_instruction(&0xDE).unwrap();
        assert_eq!(instruction, &SET_3_HL);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 16);
    }

    #[test]
    pub fn set_3_hl() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.set_hl(0xC000);
        mmu.set_byte(0xC000 as usize, 0x00);
        (&SET_3_HL.handler)(&mut cpu, &mut mmu, &OpCode::CB(0xDE));
        assert_eq!(mmu.get_byte(0xC000 as usize), 0x08);
    }

    #[test]
    pub fn test_get_instruction_set_3_a() {
        let instruction = get_instruction(&0xDF).unwrap();
        assert_eq!(instruction, &SET_3_A);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn set_3_a() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x00;
        (&SET_3_A.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xDF));
        assert_eq!(cpu.reg.a, 0x08);
    }

    #[test]
    pub fn test_get_instruction_set_4_b() {
        let instruction = get_instruction(&0xE0).unwrap();
        assert_eq!(instruction, &SET_4_B);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn set_4_b() {
        let mut cpu = Cpu::new();
        cpu.reg.b = 0x00;
        (&SET_4_B.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xE0));
        assert_eq!(cpu.reg.b, 0x10);
    }

    #[test]
    pub fn test_get_instruction_set_4_c() {
        let instruction = get_instruction(&0xE1).unwrap();
        assert_eq!(instruction, &SET_4_C);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn set_4_c() {
        let mut cpu = Cpu::new();
        cpu.reg.c = 0x00;
        (&SET_4_C.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xE1));
        assert_eq!(cpu.reg.c, 0x10);
    }

    #[test]
    pub fn test_get_instruction_set_4_d() {
        let instruction = get_instruction(&0xE2).unwrap();
        assert_eq!(instruction, &SET_4_D);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn set_4_d() {
        let mut cpu = Cpu::new();
        cpu.reg.d = 0x00;
        (&SET_4_D.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xE2));
        assert_eq!(cpu.reg.d, 0x10);
    }

    #[test]
    pub fn test_get_instruction_set_4_e() {
        let instruction = get_instruction(&0xE3).unwrap();
        assert_eq!(instruction, &SET_4_E);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn set_4_e() {
        let mut cpu = Cpu::new();
        cpu.reg.e = 0x00;
        (&SET_4_E.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xE3));
        assert_eq!(cpu.reg.e, 0x10);
    }

    #[test]
    pub fn test_get_instruction_set_4_h() {
        let instruction = get_instruction(&0xE4).unwrap();
        assert_eq!(instruction, &SET_4_H);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn set_4_h() {
        let mut cpu = Cpu::new();
        cpu.reg.h = 0x00;
        (&SET_4_H.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xE4));
        assert_eq!(cpu.reg.h, 0x10);
    }

    #[test]
    pub fn test_get_instruction_set_4_l() {
        let instruction = get_instruction(&0xE5).unwrap();
        assert_eq!(instruction, &SET_4_L);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn set_4_l() {
        let mut cpu = Cpu::new();
        cpu.reg.l = 0x00;
        (&SET_4_L.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xE5));
        assert_eq!(cpu.reg.l, 0x10);
    }

    #[test]
    pub fn test_get_instruction_set_4_hl() {
        let instruction = get_instruction(&0xE6).unwrap();
        assert_eq!(instruction, &SET_4_HL);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 16);
    }

    #[test]
    pub fn set_4_hl() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.set_hl(0xC000);
        mmu.set_byte(0xC000 as usize, 0x00);
        (&SET_4_HL.handler)(&mut cpu, &mut mmu, &OpCode::CB(0xE6));
        assert_eq!(mmu.get_byte(0xC000 as usize), 0x10);
    }

    #[test]
    pub fn test_get_instruction_set_4_a() {
        let instruction = get_instruction(&0xE7).unwrap();
        assert_eq!(instruction, &SET_4_A);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn set_4_a() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x00;
        (&SET_4_A.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xE7));
        assert_eq!(cpu.reg.a, 0x10);
    }

    #[test]
    pub fn test_get_instruction_set_5_b() {
        let instruction = get_instruction(&0xE8).unwrap();
        assert_eq!(instruction, &SET_5_B);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn set_5_b() {
        let mut cpu = Cpu::new();
        cpu.reg.b = 0x00;
        (&SET_5_B.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xE8));
        assert_eq!(cpu.reg.b, 0x20);
    }

    #[test]
    pub fn test_get_instruction_set_5_c() {
        let instruction = get_instruction(&0xE9).unwrap();
        assert_eq!(instruction, &SET_5_C);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn set_5_c() {
        let mut cpu = Cpu::new();
        cpu.reg.c = 0x00;
        (&SET_5_C.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xE9));
        assert_eq!(cpu.reg.c, 0x20);
    }

    #[test]
    pub fn test_get_instruction_set_5_d() {
        let instruction = get_instruction(&0xEA).unwrap();
        assert_eq!(instruction, &SET_5_D);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn set_5_d() {
        let mut cpu = Cpu::new();
        cpu.reg.d = 0x00;
        (&SET_5_D.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xEA));
        assert_eq!(cpu.reg.d, 0x20);
    }

    #[test]
    pub fn test_get_instruction_set_5_e() {
        let instruction = get_instruction(&0xEB).unwrap();
        assert_eq!(instruction, &SET_5_E);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn set_5_e() {
        let mut cpu = Cpu::new();
        cpu.reg.e = 0x00;
        (&SET_5_E.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xEB));
        assert_eq!(cpu.reg.e, 0x20);
    }

    #[test]
    pub fn test_get_instruction_set_5_h() {
        let instruction = get_instruction(&0xEC).unwrap();
        assert_eq!(instruction, &SET_5_H);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn set_5_h() {
        let mut cpu = Cpu::new();
        cpu.reg.h = 0x00;
        (&SET_5_H.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xEC));
        assert_eq!(cpu.reg.h, 0x20);
    }

    #[test]
    pub fn test_get_instruction_set_5_l() {
        let instruction = get_instruction(&0xED).unwrap();
        assert_eq!(instruction, &SET_5_L);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn set_5_l() {
        let mut cpu = Cpu::new();
        cpu.reg.l = 0x00;
        (&SET_5_L.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xED));
        assert_eq!(cpu.reg.l, 0x20);
    }

    #[test]
    pub fn test_get_instruction_set_5_hl() {
        let instruction = get_instruction(&0xEE).unwrap();
        assert_eq!(instruction, &SET_5_HL);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 16);
    }

    #[test]
    pub fn set_5_hl() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.set_hl(0xC000);
        mmu.set_byte(0xC000 as usize, 0x00);
        (&SET_5_HL.handler)(&mut cpu, &mut mmu, &OpCode::CB(0xEE));
        assert_eq!(mmu.get_byte(0xC000 as usize), 0x20);
    }

    #[test]
    pub fn test_get_instruction_set_5_a() {
        let instruction = get_instruction(&0xEF).unwrap();
        assert_eq!(instruction, &SET_5_A);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn set_5_a() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x00;
        (&SET_5_A.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xEF));
        assert_eq!(cpu.reg.a, 0x20);
    }

    #[test]
    pub fn test_get_instruction_set_6_b() {
        let instruction = get_instruction(&0xF0).unwrap();
        assert_eq!(instruction, &SET_6_B);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn set_6_b() {
        let mut cpu = Cpu::new();
        cpu.reg.b = 0x00;
        (&SET_6_B.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xF0));
        assert_eq!(cpu.reg.b, 0x40);
    }

    #[test]
    pub fn test_get_instruction_set_6_c() {
        let instruction = get_instruction(&0xF1).unwrap();
        assert_eq!(instruction, &SET_6_C);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn set_6_c() {
        let mut cpu = Cpu::new();
        cpu.reg.c = 0x00;
        (&SET_6_C.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xF1));
        assert_eq!(cpu.reg.c, 0x40);
    }

    #[test]
    pub fn test_get_instruction_set_6_d() {
        let instruction = get_instruction(&0xF2).unwrap();
        assert_eq!(instruction, &SET_6_D);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn set_6_d() {
        let mut cpu = Cpu::new();
        cpu.reg.d = 0x00;
        (&SET_6_D.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xF2));
        assert_eq!(cpu.reg.d, 0x40);
    }

    #[test]
    pub fn test_get_instruction_set_6_e() {
        let instruction = get_instruction(&0xF3).unwrap();
        assert_eq!(instruction, &SET_6_E);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn set_6_e() {
        let mut cpu = Cpu::new();
        cpu.reg.e = 0x00;
        (&SET_6_E.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xF3));
        assert_eq!(cpu.reg.e, 0x40);
    }

    #[test]
    pub fn test_get_instruction_set_6_h() {
        let instruction = get_instruction(&0xF4).unwrap();
        assert_eq!(instruction, &SET_6_H);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn set_6_h() {
        let mut cpu = Cpu::new();
        cpu.reg.h = 0x00;
        (&SET_6_H.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xF4));
        assert_eq!(cpu.reg.h, 0x40);
    }

    #[test]
    pub fn test_get_instruction_set_6_l() {
        let instruction = get_instruction(&0xF5).unwrap();
        assert_eq!(instruction, &SET_6_L);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn set_6_l() {
        let mut cpu = Cpu::new();
        cpu.reg.l = 0x00;
        (&SET_6_L.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xF5));
        assert_eq!(cpu.reg.l, 0x40);
    }

    #[test]
    pub fn test_get_instruction_set_6_hl() {
        let instruction = get_instruction(&0xF6).unwrap();
        assert_eq!(instruction, &SET_6_HL);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 16);
    }

    #[test]
    pub fn set_6_hl() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.set_hl(0xC000);
        mmu.set_byte(0xC000 as usize, 0x00);
        (&SET_6_HL.handler)(&mut cpu, &mut mmu, &OpCode::CB(0xF6));
        assert_eq!(mmu.get_byte(0xC000 as usize), 0x40);
    }

    #[test]
    pub fn test_get_instruction_set_6_a() {
        let instruction = get_instruction(&0xF7).unwrap();
        assert_eq!(instruction, &SET_6_A);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn set_6_a() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x00;
        (&SET_6_A.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xF7));
        assert_eq!(cpu.reg.a, 0x40);
    }

    #[test]
    pub fn test_get_instruction_set_7_b() {
        let instruction = get_instruction(&0xF8).unwrap();
        assert_eq!(instruction, &SET_7_B);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn set_7_b() {
        let mut cpu = Cpu::new();
        cpu.reg.b = 0x00;
        (&SET_7_B.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xF8));
        assert_eq!(cpu.reg.b, 0x80);
    }

    #[test]
    pub fn test_get_instruction_set_7_c() {
        let instruction = get_instruction(&0xF9).unwrap();
        assert_eq!(instruction, &SET_7_C);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn set_7_c() {
        let mut cpu = Cpu::new();
        cpu.reg.c = 0x00;
        (&SET_7_C.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xF9));
        assert_eq!(cpu.reg.c, 0x80);
    }

    #[test]
    pub fn test_get_instruction_set_7_d() {
        let instruction = get_instruction(&0xFA).unwrap();
        assert_eq!(instruction, &SET_7_D);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn set_7_d() {
        let mut cpu = Cpu::new();
        cpu.reg.d = 0x00;
        (&SET_7_D.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xFA));
        assert_eq!(cpu.reg.d, 0x80);
    }

    #[test]
    pub fn test_get_instruction_set_7_e() {
        let instruction = get_instruction(&0xFB).unwrap();
        assert_eq!(instruction, &SET_7_E);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn set_7_e() {
        let mut cpu = Cpu::new();
        cpu.reg.e = 0x00;
        (&SET_7_E.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xFB));
        assert_eq!(cpu.reg.e, 0x80);
    }

    #[test]
    pub fn test_get_instruction_set_7_h() {
        let instruction = get_instruction(&0xFC).unwrap();
        assert_eq!(instruction, &SET_7_H);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn set_7_h() {
        let mut cpu = Cpu::new();
        cpu.reg.h = 0x00;
        (&SET_7_H.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xFC));
        assert_eq!(cpu.reg.h, 0x80);
    }

    #[test]
    pub fn test_get_instruction_set_7_l() {
        let instruction = get_instruction(&0xFD).unwrap();
        assert_eq!(instruction, &SET_7_L);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn set_7_l() {
        let mut cpu = Cpu::new();
        cpu.reg.l = 0x00;
        (&SET_7_L.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xFD));
        assert_eq!(cpu.reg.l, 0x80);
    }

    #[test]
    pub fn test_get_instruction_set_7_hl() {
        let instruction = get_instruction(&0xFE).unwrap();
        assert_eq!(instruction, &SET_7_HL);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 16);
    }

    #[test]
    pub fn set_7_hl() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.set_hl(0xC000);
        mmu.set_byte(0xC000 as usize, 0x00);
        (&SET_7_HL.handler)(&mut cpu, &mut mmu, &OpCode::CB(0xFE));
        assert_eq!(mmu.get_byte(0xC000 as usize), 0x80);
    }

    #[test]
    pub fn test_get_instruction_set_7_a() {
        let instruction = get_instruction(&0xFF).unwrap();
        assert_eq!(instruction, &SET_7_A);
        assert_eq!(instruction.length, 2);
        assert_eq!(instruction.clock_cycles, 8);
    }

    #[test]
    pub fn set_7_a() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x00;
        (&SET_7_A.handler)(&mut cpu, &mut Memory::new(), &OpCode::CB(0xFF));
        assert_eq!(cpu.reg.a, 0x80);
    }
}
