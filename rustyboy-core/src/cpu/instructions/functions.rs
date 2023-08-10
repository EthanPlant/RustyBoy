use crate::cpu::cpu::Cpu;
use crate::cpu::registers::Flag;
use crate::mmu::Memory;

/// Get the n'th 8 bit instruction operand
pub fn get_op8(cpu: &Cpu, mmu: &Memory, n: u8) -> u8 {
    mmu.get_byte(cpu.reg.pc + n as u16)
}

/// Get a 16 bit instruction operand
pub fn get_op16(cpu: &Cpu, mmu: &Memory) -> u16 {
    mmu.get_word(cpu.reg.pc + 1 as u16)
}

/// Get the value in memory pointed to by HL
pub fn get_hl(cpu: &Cpu, mmu: &Memory) -> u8 {
    mmu.get_byte(cpu.reg.hl())
}

/// Set the value in memory pointed to by HL
pub fn set_hl(cpu: &mut Cpu, mmu: &mut Memory, value: u8) {
    mmu.set_byte(cpu.reg.hl(), value);
}

/// Push a value to the stack
pub fn push(cpu: &mut Cpu, mmu: &mut Memory, value: u16) {
    cpu.reg.sp = cpu.reg.sp.wrapping_sub(2);
    mmu.set_word(cpu.reg.sp, value);
}

/// Pop a value from the stack
pub fn pop(cpu: &mut Cpu, mmu: &mut Memory) -> u16 {
    let value = mmu.get_word(cpu.reg.sp);
    cpu.reg.sp = cpu.reg.sp.wrapping_add(2);
    value
}

/// Call a subroutine.
/// Pushes the address of the next instruction to the stack.
/// Sets the program counter to the address of the subroutine.
pub fn call(cpu: &mut Cpu, mmu: &mut Memory) {
    let addr = get_op16(cpu, mmu);
    push(cpu, mmu, cpu.reg.pc + 3);
    cpu.reg.pc = addr;
}

/// Jump relative to the next instruction.
/// The operand is treated as a signed byte, added to the memory address of the next instruction.
pub fn jr(cpu: &mut Cpu, mmu: &mut Memory) {
    let op = get_op8(cpu, mmu, 1) as i8;
    cpu.reg.pc = ((cpu.reg.pc + 2) as i16 + op as i16) as u16;
}

/// Add a byte to the A register.
/// Sets the zero flag if the result is zero.
/// Sets the carry flag if the result overflows.
/// Sets the half carry flag if the lower nibble of the A register overflows.
/// Clears the subtract flag.
pub fn add(cpu: &mut Cpu, byte: u8) {
    let result = cpu.reg.a.wrapping_add(byte);
    cpu.reg.clear_all_flags();
    if result == 0 {
        cpu.reg.set_flag(Flag::Zero);
    }
    if (cpu.reg.a as u16) + (byte as u16) > 0xFF {
        cpu.reg.set_flag(Flag::Carry);
    }
    if (cpu.reg.a & 0x0F) + (byte & 0x0F) > 0x0F {
        cpu.reg.set_flag(Flag::HalfCarry);
    }
    cpu.reg.a = result;
}

/// Compare a byte with the A register.
/// Sets the zero flag if the result is zero.
/// Sets the subtract flag.
/// Sets the carry flag if the A register is smaller than the byte.
/// Sets the half carry flag if the lower nibble of the A register is smaller than the lower nibble of the byte.
pub fn cp(cpu: &mut Cpu, byte: u8) {
    cpu.reg.clear_all_flags();
    if cpu.reg.a == byte {
        cpu.reg.set_flag(Flag::Zero);
    }
    if cpu.reg.a < byte {
        cpu.reg.set_flag(Flag::Carry);
    }
    if (cpu.reg.a & 0x0F) < (byte & 0x0F) {
        cpu.reg.set_flag(Flag::HalfCarry);
    }
    cpu.reg.set_flag(Flag::Subtract);
}

/// Decrement a byte.
/// Sets the zero flag if the result is zero.
/// Sets the subtract flag.
/// Sets the half carry flag if the result underflows.
pub fn dec(cpu: &mut Cpu, byte: u8) -> u8 {
    let result = byte.wrapping_sub(1);
    cpu.reg.clear_all_flags();
    if result == 0 {
        cpu.reg.set_flag(Flag::Zero);
    }
    if byte & 0x0F == 0x00 {
        cpu.reg.set_flag(Flag::HalfCarry);
    }
    cpu.reg.set_flag(Flag::Subtract);
    result
}

/// Increment a byte.
/// Sets the zero flag if the result is zero.
/// Clears the subtract flag.
/// Sets the half carry flag if the result overflows.
pub fn inc(cpu: &mut Cpu, byte: u8) -> u8 {
    let result = byte.wrapping_add(1);
    cpu.reg.clear_all_flags();
    if result == 0 {
        cpu.reg.set_flag(Flag::Zero);
    }
    if byte & 0x0F == 0x0F {
        cpu.reg.set_flag(Flag::HalfCarry);
    }
    result
}

/// Subtract a byte from the A register.
/// Sets the zero flag if the result is zero.
/// Sets the subtract flag.
/// Sets the carry flag if the A register is smaller than the byte.
/// Sets the half carry flag if the lower nibble of the A register is smaller than the lower nibble of the byte.
pub fn sub(cpu: &mut Cpu, byte: u8) {
    cpu.reg.clear_all_flags();
    if cpu.reg.a == byte {
        cpu.reg.set_flag(Flag::Zero);
    }
    if cpu.reg.a < byte {
        cpu.reg.set_flag(Flag::Carry);
    }
    if (cpu.reg.a & 0x0F) < (byte & 0x0F) {
        cpu.reg.set_flag(Flag::HalfCarry);
    }
    cpu.reg.set_flag(Flag::Subtract);
    cpu.reg.a = cpu.reg.a.wrapping_sub(byte);
}

/// Performs a bitwise XOR on two bytes and stores the result in the A register.
/// Sets the zero flag if the result is zero.
/// Clears all other flags.
pub fn xor_bytes(cpu: &mut Cpu, byte1: u8, byte2: u8) {
    let result = byte1 ^ byte2;
    cpu.reg.clear_all_flags();
    if result == 0 {
        cpu.reg.set_flag(Flag::Zero);
    }
    cpu.reg.a = result;
}

/// Checks if a bit is set in a byte.
/// Sets the zero flag if the bit is not set.
/// Sets the half carry flag.
pub fn bit(cpu: &mut Cpu, byte: u8, bit: u8) {
    cpu.reg.clear_all_flags();
    if byte & (1 << bit) == 0 {
        cpu.reg.set_flag(Flag::Zero);
    }
    cpu.reg.set_flag(Flag::HalfCarry);
}

/// Rotates a byte left through the carry flag.
/// Sets the zero flag if the result is zero.
/// Sets the carry flag to the value of bit 7.
/// Clears all other flags.
pub fn rl(cpu: &mut Cpu, byte: u8) -> u8 {
    let carry: u8 = (byte >> 7) & 1;
    let result = (byte << 1) | cpu.reg.check_flag(Flag::Carry) as u8;
    cpu.reg.clear_all_flags();
    if result == 0 {
        cpu.reg.set_flag(Flag::Zero);
    }
    if carry == 1 {
        cpu.reg.set_flag(Flag::Carry);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_op8() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.pc = 0xC000;
        mmu.set_byte(0xC001 as usize, 0x01);
        assert_eq!(get_op8(&cpu, &mmu, 1), 0x01);
    }

    #[test]
    fn test_get_op16() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.pc = 0xC000;
        mmu.set_word(0xC001 as usize, 0x0201);
        assert_eq!(get_op16(&cpu, &mmu), 0x0201);
    }

    #[test]
    fn test_get_hl() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.h = 0xC0;
        cpu.reg.l = 0x00;
        mmu.set_byte(0xC000 as usize, 0x03);
        assert_eq!(get_hl(&cpu, &mmu), 0x03);
    }

    #[test]
    fn test_set_hl() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.h = 0xC0;
        cpu.reg.l = 0x00;
        set_hl(&mut cpu, &mut mmu, 0x03);
        assert_eq!(mmu.get_byte(0xC000 as usize), 0x03);
    }

    #[test]
    fn test_push() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.sp = 0xFFFE;
        push(&mut cpu, &mut mmu, 0x0201);
        assert_eq!(mmu.get_word(0xFFFC as usize), 0x0201);
        assert_eq!(cpu.reg.sp, 0xFFFC);
    }

    #[test]
    fn test_pop() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.sp = 0xFFFC;
        mmu.set_word(0xFFFC as usize, 0x0201);
        assert_eq!(pop(&mut cpu, &mut mmu), 0x0201);
        assert_eq!(cpu.reg.sp, 0xFFFE);
    }

    #[test]
    fn test_call() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.pc = 0xC000;
        cpu.reg.sp = 0xFFFE;
        mmu.set_word(0xC001 as usize, 0x0201);
        call(&mut cpu, &mut mmu);
        assert_eq!(cpu.reg.pc, 0x0201);
        assert_eq!(cpu.reg.sp, 0xFFFC);
        assert_eq!(mmu.get_word(0xFFFC as usize), 0xC003);
    }

    #[test]
    fn test_jr_positive() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.pc = 0xC000;
        mmu.set_byte(0xC001 as usize, 0x01);
        jr(&mut cpu, &mut mmu);
        assert_eq!(cpu.reg.pc, 0xC003);
    }

    #[test]
    fn test_jr_negative() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.pc = 0xC000;
        mmu.set_byte(0xC001 as usize, 0xFE);
        jr(&mut cpu, &mut mmu);
        assert_eq!(cpu.reg.pc, 0xC000);
    }

    #[test]
    pub fn test_add() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0;
        add(&mut cpu, 0x01);
        assert_eq!(cpu.reg.a, 0x01);
        assert!(!cpu.reg.check_flag(Flag::Zero));
        assert!(!cpu.reg.check_flag(Flag::Carry));
        assert!(!cpu.reg.check_flag(Flag::HalfCarry));
        assert!(!cpu.reg.check_flag(Flag::Subtract));
    }

    #[test]
    pub fn test_add_zero() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0;
        add(&mut cpu, 0x00);
        assert_eq!(cpu.reg.a, 0x00);
        assert!(cpu.reg.check_flag(Flag::Zero));
        assert!(!cpu.reg.check_flag(Flag::Carry));
        assert!(!cpu.reg.check_flag(Flag::HalfCarry));
        assert!(!cpu.reg.check_flag(Flag::Subtract));
    }

    #[test]
    pub fn test_add_carry() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x80;
        add(&mut cpu, 0x81);
        assert_eq!(cpu.reg.a, 0x01);
        assert!(!cpu.reg.check_flag(Flag::Zero));
        assert!(cpu.reg.check_flag(Flag::Carry));
        assert!(!cpu.reg.check_flag(Flag::HalfCarry));
        assert!(!cpu.reg.check_flag(Flag::Subtract));
    }

    #[test]
    pub fn test_add_half_carry() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x0F;
        add(&mut cpu, 0x01);
        assert_eq!(cpu.reg.a, 0x10);
        assert!(!cpu.reg.check_flag(Flag::Zero));
        assert!(!cpu.reg.check_flag(Flag::Carry));
        assert!(cpu.reg.check_flag(Flag::HalfCarry));
        assert!(!cpu.reg.check_flag(Flag::Subtract));
    }

    #[test]
    pub fn test_cp_equal() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x01;
        cp(&mut cpu, 0x01);
        assert_eq!(cpu.reg.a, 0x01);
        assert!(cpu.reg.check_flag(Flag::Zero));
        assert!(!cpu.reg.check_flag(Flag::Carry));
        assert!(!cpu.reg.check_flag(Flag::HalfCarry));
        assert!(cpu.reg.check_flag(Flag::Subtract));
    }

    #[test]
    pub fn test_cp_greater() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x0F;
        cp(&mut cpu, 0x10);
        assert_eq!(cpu.reg.a, 0x0F);
        assert!(!cpu.reg.check_flag(Flag::Zero));
        assert!(cpu.reg.check_flag(Flag::Carry));
        assert!(!cpu.reg.check_flag(Flag::HalfCarry));
        assert!(cpu.reg.check_flag(Flag::Subtract));
    }

    #[test]
    pub fn test_cp_less() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0xFF;
        cp(&mut cpu, 0x00);
        assert_eq!(cpu.reg.a, 0xFF);
        assert!(!cpu.reg.check_flag(Flag::Zero));
        assert!(!cpu.reg.check_flag(Flag::Carry));
        assert!(!cpu.reg.check_flag(Flag::HalfCarry));
        assert!(cpu.reg.check_flag(Flag::Subtract));
    }

    #[test]
    pub fn test_cp_half_carry() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x10;
        cp(&mut cpu, 0x01);
        assert_eq!(cpu.reg.a, 0x10);
        assert!(!cpu.reg.check_flag(Flag::Zero));
        assert!(!cpu.reg.check_flag(Flag::Carry));
        assert!(cpu.reg.check_flag(Flag::HalfCarry));
        assert!(cpu.reg.check_flag(Flag::Subtract));
    }

    #[test]
    pub fn test_dec() {
        let mut cpu = Cpu::new();
        assert_eq!(dec(&mut cpu, 0x02), 0x01);
        assert!(!cpu.reg.check_flag(Flag::Zero));
        assert!(cpu.reg.check_flag(Flag::Subtract));
        assert!(!cpu.reg.check_flag(Flag::HalfCarry));
        assert!(!cpu.reg.check_flag(Flag::Carry));
    }

    #[test]
    pub fn test_dec_zero() {
        let mut cpu = Cpu::new();
        assert_eq!(dec(&mut cpu, 0x01), 0x00);
        assert!(cpu.reg.check_flag(Flag::Zero));
        assert!(cpu.reg.check_flag(Flag::Subtract));
        assert!(!cpu.reg.check_flag(Flag::HalfCarry));
        assert!(!cpu.reg.check_flag(Flag::Carry));
    }

    #[test]
    pub fn test_dec_half_carry() {
        let mut cpu = Cpu::new();
        assert_eq!(dec(&mut cpu, 0x10), 0x0F);
        assert!(!cpu.reg.check_flag(Flag::Zero));
        assert!(cpu.reg.check_flag(Flag::Subtract));
        assert!(cpu.reg.check_flag(Flag::HalfCarry));
        assert!(!cpu.reg.check_flag(Flag::Carry));
    }

    #[test]
    pub fn test_dec_underflow() {
        let mut cpu = Cpu::new();
        assert_eq!(dec(&mut cpu, 0x00), 0xFF);
        assert!(!cpu.reg.check_flag(Flag::Zero));
        assert!(cpu.reg.check_flag(Flag::Subtract));
        assert!(cpu.reg.check_flag(Flag::HalfCarry));
        assert!(!cpu.reg.check_flag(Flag::Carry));
    }

    #[test]
    pub fn test_inc() {
        let mut cpu = Cpu::new();
        assert_eq!(inc(&mut cpu, 0x01), 0x02);
        assert!(!cpu.reg.check_flag(Flag::Zero));
        assert!(!cpu.reg.check_flag(Flag::Subtract));
        assert!(!cpu.reg.check_flag(Flag::HalfCarry));
        assert!(!cpu.reg.check_flag(Flag::Carry));
    }

    #[test]
    pub fn test_inc_zero() {
        let mut cpu = Cpu::new();
        assert_eq!(inc(&mut cpu, 0xFF), 0x00);
        assert!(cpu.reg.check_flag(Flag::Zero));
        assert!(!cpu.reg.check_flag(Flag::Subtract));
        assert!(cpu.reg.check_flag(Flag::HalfCarry));
        assert!(!cpu.reg.check_flag(Flag::Carry));
    }

    #[test]
    pub fn test_inc_half_carry() {
        let mut cpu = Cpu::new();
        assert_eq!(inc(&mut cpu, 0x0F), 0x10);
        assert!(!cpu.reg.check_flag(Flag::Zero));
        assert!(!cpu.reg.check_flag(Flag::Subtract));
        assert!(cpu.reg.check_flag(Flag::HalfCarry));
        assert!(!cpu.reg.check_flag(Flag::Carry));
    }

    #[test]
    pub fn test_sub() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x02;
        sub(&mut cpu, 0x01);
        assert_eq!(cpu.reg.a, 0x01);
        assert!(!cpu.reg.check_flag(Flag::Zero));
        assert!(!cpu.reg.check_flag(Flag::Carry));
        assert!(!cpu.reg.check_flag(Flag::HalfCarry));
        assert!(cpu.reg.check_flag(Flag::Subtract));
    }

    #[test]
    pub fn test_sub_zero() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x01;
        sub(&mut cpu, 0x01);
        assert_eq!(cpu.reg.a, 0x00);
        assert!(cpu.reg.check_flag(Flag::Zero));
        assert!(!cpu.reg.check_flag(Flag::Carry));
        assert!(!cpu.reg.check_flag(Flag::HalfCarry));
        assert!(cpu.reg.check_flag(Flag::Subtract));
    }

    #[test]
    pub fn test_sub_carry() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x7F;
        sub(&mut cpu, 0xFF);
        assert_eq!(cpu.reg.a, 0x80);
        assert!(!cpu.reg.check_flag(Flag::Zero));
        assert!(cpu.reg.check_flag(Flag::Carry));
        assert!(!cpu.reg.check_flag(Flag::HalfCarry));
        assert!(cpu.reg.check_flag(Flag::Subtract));
    }

    #[test]
    pub fn test_sub_half_carry() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x10;
        sub(&mut cpu, 0x01);
        assert_eq!(cpu.reg.a, 0x0F);
        assert!(!cpu.reg.check_flag(Flag::Zero));
        assert!(!cpu.reg.check_flag(Flag::Carry));
        assert!(cpu.reg.check_flag(Flag::HalfCarry));
        assert!(cpu.reg.check_flag(Flag::Subtract));
    }

    #[test]
    pub fn test_xor_bytes() {
        let mut cpu = Cpu::new();
        xor_bytes(&mut cpu, 0x00, 0x01);
        assert_eq!(cpu.reg.a, 0x01);
        assert!(!cpu.reg.check_flag(Flag::Zero));
        assert!(!cpu.reg.check_flag(Flag::Carry));
        assert!(!cpu.reg.check_flag(Flag::HalfCarry));
        assert!(!cpu.reg.check_flag(Flag::Subtract));
    }

    #[test]
    pub fn test_xor_bytes_zero() {
        let mut cpu = Cpu::new();
        xor_bytes(&mut cpu, 0x01, 0x01);
        assert_eq!(cpu.reg.a, 0x00);
        assert!(cpu.reg.check_flag(Flag::Zero));
        assert!(!cpu.reg.check_flag(Flag::Carry));
        assert!(!cpu.reg.check_flag(Flag::HalfCarry));
        assert!(!cpu.reg.check_flag(Flag::Subtract));
    }

    #[test]
    pub fn test_bit_one() {
        let mut cpu = Cpu::new();
        bit(&mut cpu, 0x01, 0);
        assert!(!cpu.reg.check_flag(Flag::Zero));
        assert!(cpu.reg.check_flag(Flag::HalfCarry));
    }

    #[test]
    pub fn test_bit_zero() {
        let mut cpu = Cpu::new();
        bit(&mut cpu, 0x00, 0);
        assert!(cpu.reg.check_flag(Flag::Zero));
        assert!(cpu.reg.check_flag(Flag::HalfCarry));
    }

    #[test]
    pub fn test_rl_bit_seven_set() {
        let mut cpu = Cpu::new();
        cpu.reg.clear_flag(Flag::Carry);
        assert_eq!(rl(&mut cpu, 0x80), 0x00);
        assert!(cpu.reg.check_flag(Flag::Zero));
        assert!(cpu.reg.check_flag(Flag::Carry));
        assert!(!cpu.reg.check_flag(Flag::HalfCarry));
        assert!(!cpu.reg.check_flag(Flag::Subtract));
    }

    #[test]
    pub fn test_rl_bit_seven_clear() {
        let mut cpu = Cpu::new();
        cpu.reg.clear_flag(Flag::Carry);
        assert_eq!(rl(&mut cpu, 0x00), 0x00);
        assert!(cpu.reg.check_flag(Flag::Zero));
        assert!(!cpu.reg.check_flag(Flag::Carry));
        assert!(!cpu.reg.check_flag(Flag::HalfCarry));
        assert!(!cpu.reg.check_flag(Flag::Subtract));
    }

    #[test]
    pub fn test_rl_bit_seven_set_carry_set() {
        let mut cpu = Cpu::new();
        cpu.reg.set_flag(Flag::Carry);
        assert_eq!(rl(&mut cpu, 0x80), 0x01);
        assert!(!cpu.reg.check_flag(Flag::Zero));
        assert!(cpu.reg.check_flag(Flag::Carry));
        assert!(!cpu.reg.check_flag(Flag::HalfCarry));
        assert!(!cpu.reg.check_flag(Flag::Subtract));
    }

    #[test]
    pub fn test_rl_bit_seven_clear_carry_set() {
        let mut cpu = Cpu::new();
        cpu.reg.set_flag(Flag::Carry);
        assert_eq!(rl(&mut cpu, 0x00), 0x01);
        assert!(!cpu.reg.check_flag(Flag::Zero));
        assert!(!cpu.reg.check_flag(Flag::Carry));
        assert!(!cpu.reg.check_flag(Flag::HalfCarry));
        assert!(!cpu.reg.check_flag(Flag::Subtract));
    }
}
