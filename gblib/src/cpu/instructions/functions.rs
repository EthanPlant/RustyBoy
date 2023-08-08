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
    let result = byte << 7 | carry;
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
        let mut mem = Memory::new();
        mem.set_byte(0x0000 as u16, 0x01);
        cpu.reg.pc = 0x0000;
        let op = get_op8(&cpu, &mem, 0);
        assert_eq!(op, 0x01);
    }

    #[test]
    fn test_get_op16() {
        let mut cpu = Cpu::new();
        let mut mem = Memory::new();
        mem.set_word(0x0001 as u16, 0x0102);
        cpu.reg.pc = 0x0000;
        let op = get_op16(&cpu, &mem);
        assert_eq!(op, 0x0102);
    }

    #[test]
    fn test_get_hl() {
        let mut cpu = Cpu::new();
        let mut mem = Memory::new();
        mem.set_byte(0x0000 as u16, 0x01);
        cpu.reg.pc = 0x0000;
        cpu.reg.set_hl(0x0000);
        let op = get_hl(&cpu, &mem);
        assert_eq!(op, 0x01);
    }

    #[test]
    fn test_set_hl() {
        let mut cpu = Cpu::new();
        let mut mem = Memory::new();
        cpu.reg.pc = 0x0000;
        cpu.reg.set_hl(0x0000);
        set_hl(&mut cpu, &mut mem, 0x01);
        assert_eq!(mem.get_byte(0x0000 as u16), 0x01);
    }

    #[test]
    fn test_push() {
        let mut cpu = Cpu::new();
        let mut mem = Memory::new();
        cpu.reg.pc = 0x0000;
        cpu.reg.sp = 0xFFFE;
        push(&mut cpu, &mut mem, 0x0102);
        assert_eq!(mem.get_byte(0xFFFC as u16), 0x02);
        assert_eq!(mem.get_byte(0xFFFD as u16), 0x01);
        assert_eq!(cpu.reg.sp, 0xFFFC);
    }

    #[test]
    fn test_pop() {
        let mut cpu = Cpu::new();
        let mut mem = Memory::new();
        cpu.reg.pc = 0x0000;
        cpu.reg.sp = 0xFFFC;
        mem.set_byte(0xFFFC as u16, 0x02);
        mem.set_byte(0xFFFD as u16, 0x01);
        let op = pop(&mut cpu, &mut mem);
        assert_eq!(op, 0x0102);
        assert_eq!(cpu.reg.sp, 0xFFFE);
    }

    #[test]
    fn test_call() {
        let mut cpu = Cpu::new();
        let mut mem = Memory::new();
        cpu.reg.pc = 0x0000;
        cpu.reg.sp = 0xFFFE;
        mem.set_byte(0x0001 as u16, 0x02);
        mem.set_byte(0x0002 as u16, 0x01);
        call(&mut cpu, &mut mem);
        assert_eq!(cpu.reg.pc, 0x0102);
        assert_eq!(cpu.reg.sp, 0xFFFC);
        assert_eq!(mem.get_byte(0xFFFC as u16), 0x03);
        assert_eq!(mem.get_byte(0xFFFE as u16), 0x00);
    }

    #[test]
    fn test_jr() {
        let mut cpu = Cpu::new();
        let mut mem = Memory::new();
        mem.set_byte(0x000B as u16, 0xFB);
        cpu.reg.pc = 0x000A;
        jr(&mut cpu, &mut mem);
        assert_eq!(cpu.reg.pc, 0x0007);
    }

    #[test]
    fn test_add() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x01;
        add(&mut cpu, 0x01);
        assert_eq!(cpu.reg.a, 0x02);
        assert!(!cpu.reg.check_flag(Flag::Zero));
        assert!(!cpu.reg.check_flag(Flag::Subtract));
        assert!(!cpu.reg.check_flag(Flag::HalfCarry));
        assert!(!cpu.reg.check_flag(Flag::Carry));
    }

    #[test]
    fn test_add_zero() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0xFF;
        add(&mut cpu, 0x01);
        assert_eq!(cpu.reg.a, 0x00);
        assert!(cpu.reg.check_flag(Flag::Zero));
        assert!(!cpu.reg.check_flag(Flag::Subtract));
        assert!(cpu.reg.check_flag(Flag::HalfCarry));
        assert!(cpu.reg.check_flag(Flag::Carry));
    }

    #[test]
    fn test_add_half_carry() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x0F;
        add(&mut cpu, 0x01);
        assert_eq!(cpu.reg.a, 0x10);
        assert!(!cpu.reg.check_flag(Flag::Zero));
        assert!(!cpu.reg.check_flag(Flag::Subtract));
        assert!(cpu.reg.check_flag(Flag::HalfCarry));
        assert!(!cpu.reg.check_flag(Flag::Carry));
    }

    #[test]
    fn test_add_carry() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0xFF;
        add(&mut cpu, 0x01);
        assert_eq!(cpu.reg.a, 0x00);
        assert!(cpu.reg.check_flag(Flag::Zero));
        assert!(!cpu.reg.check_flag(Flag::Subtract));
        assert!(cpu.reg.check_flag(Flag::HalfCarry));
        assert!(cpu.reg.check_flag(Flag::Carry));
    }

    #[test]
    fn test_dec() {
        let mut cpu = Cpu::new();
        let res = dec(&mut cpu, 0x02);
        assert_eq!(res, 0x01);
        assert!(!cpu.reg.check_flag(Flag::Zero));
        assert!(cpu.reg.check_flag(Flag::Subtract));
        assert!(!cpu.reg.check_flag(Flag::HalfCarry));
        assert!(!cpu.reg.check_flag(Flag::Carry));
    }

    #[test]
    fn test_dec_zero() {
        let mut cpu = Cpu::new();
        let res = dec(&mut cpu, 0x01);
        assert_eq!(res, 0x00);
        assert!(cpu.reg.check_flag(Flag::Zero));
        assert!(cpu.reg.check_flag(Flag::Subtract));
        assert!(!cpu.reg.check_flag(Flag::HalfCarry));
        assert!(!cpu.reg.check_flag(Flag::Carry));
    }

    #[test]
    fn test_dec_half_carry() {
        let mut cpu = Cpu::new();
        let res = dec(&mut cpu, 0x10);
        assert_eq!(res, 0x0F);
        assert!(!cpu.reg.check_flag(Flag::Zero));
        assert!(cpu.reg.check_flag(Flag::Subtract));
        assert!(cpu.reg.check_flag(Flag::HalfCarry));
        assert!(!cpu.reg.check_flag(Flag::Carry));
    }

    #[test]
    fn test_inc() {
        let mut cpu = Cpu::new();
        let res = inc(&mut cpu, 0x00);
        assert_eq!(res, 0x01);
        assert!(!cpu.reg.check_flag(Flag::Zero));
        assert!(!cpu.reg.check_flag(Flag::Subtract));
        assert!(!cpu.reg.check_flag(Flag::HalfCarry));
        assert!(!cpu.reg.check_flag(Flag::Carry));
    }

    #[test]
    fn test_inc_zero() {
        let mut cpu = Cpu::new();
        let res = inc(&mut cpu, 0xFF);
        assert_eq!(res, 0x00);
        assert!(cpu.reg.check_flag(Flag::Zero));
        assert!(!cpu.reg.check_flag(Flag::Subtract));
        assert!(cpu.reg.check_flag(Flag::HalfCarry));
        assert!(!cpu.reg.check_flag(Flag::Carry));
    }

    #[test]
    fn test_inc_half_carry() {
        let mut cpu = Cpu::new();
        let res = inc(&mut cpu, 0x0F);
        assert_eq!(res, 0x10);
        assert!(!cpu.reg.check_flag(Flag::Zero));
        assert!(!cpu.reg.check_flag(Flag::Subtract));
        assert!(cpu.reg.check_flag(Flag::HalfCarry));
        assert!(!cpu.reg.check_flag(Flag::Carry));
    }

    #[test]
    fn test_sub() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x02;
        sub(&mut cpu, 0x01);
        assert_eq!(cpu.reg.a, 0x01);
        assert!(!cpu.reg.check_flag(Flag::Zero));
        assert!(cpu.reg.check_flag(Flag::Subtract));
        assert!(!cpu.reg.check_flag(Flag::HalfCarry));
        assert!(!cpu.reg.check_flag(Flag::Carry));
    }

    #[test]
    fn test_sub_zero() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x01;
        sub(&mut cpu, 0x01);
        assert_eq!(cpu.reg.a, 0x00);
        assert!(cpu.reg.check_flag(Flag::Zero));
        assert!(cpu.reg.check_flag(Flag::Subtract));
        assert!(!cpu.reg.check_flag(Flag::HalfCarry));
        assert!(!cpu.reg.check_flag(Flag::Carry));
    }

    #[test]
    fn test_sub_half_carry() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x10;
        sub(&mut cpu, 0x01);
        assert_eq!(cpu.reg.a, 0x0F);
        assert!(!cpu.reg.check_flag(Flag::Zero));
        assert!(cpu.reg.check_flag(Flag::Subtract));
        assert!(cpu.reg.check_flag(Flag::HalfCarry));
        assert!(!cpu.reg.check_flag(Flag::Carry));
    }

    #[test]
    fn test_sub_carry() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x00;
        sub(&mut cpu, 0x01);
        assert_eq!(cpu.reg.a, 0xFF);
        assert!(!cpu.reg.check_flag(Flag::Zero));
        assert!(cpu.reg.check_flag(Flag::Subtract));
        assert!(cpu.reg.check_flag(Flag::HalfCarry));
        assert!(cpu.reg.check_flag(Flag::Carry));
    }

    #[test]
    fn test_xor_bytes() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x0F;
        xor_bytes(&mut cpu, 0x0F, 0x0F);
        assert_eq!(cpu.reg.a, 0x00);
        assert!(cpu.reg.check_flag(Flag::Zero));
        assert!(!cpu.reg.check_flag(Flag::Subtract));
        assert!(!cpu.reg.check_flag(Flag::HalfCarry));
        assert!(!cpu.reg.check_flag(Flag::Carry));
    }

    #[test]
    fn test_xor_bytes_not_zero() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x0F;
        xor_bytes(&mut cpu, 0x0F, 0x0E);
        assert_eq!(cpu.reg.a, 0x01);
        assert!(!cpu.reg.check_flag(Flag::Zero));
        assert!(!cpu.reg.check_flag(Flag::Subtract));
        assert!(!cpu.reg.check_flag(Flag::HalfCarry));
        assert!(!cpu.reg.check_flag(Flag::Carry));
    }

    #[test]
    fn test_bit() {
        let mut cpu = Cpu::new();
        bit(&mut cpu, 0x0F, 7);
        assert!(cpu.reg.check_flag(Flag::Zero));
        assert!(!cpu.reg.check_flag(Flag::Subtract));
        assert!(cpu.reg.check_flag(Flag::HalfCarry));
        assert!(!cpu.reg.check_flag(Flag::Carry));
    }

    #[test]
    fn test_bit_not_zero() {
        let mut cpu = Cpu::new();
        bit(&mut cpu, 0x0F, 0);
        assert!(!cpu.reg.check_flag(Flag::Zero));
        assert!(!cpu.reg.check_flag(Flag::Subtract));
        assert!(cpu.reg.check_flag(Flag::HalfCarry));
        assert!(!cpu.reg.check_flag(Flag::Carry));
    }

    #[test]
    fn test_rl() {
        let mut cpu = Cpu::new();
        let res = rl(&mut cpu, 0x80);
        assert_eq!(res, 0x01);
        assert!(!cpu.reg.check_flag(Flag::Zero));
        assert!(!cpu.reg.check_flag(Flag::Subtract));
        assert!(!cpu.reg.check_flag(Flag::HalfCarry));
        assert!(cpu.reg.check_flag(Flag::Carry));
    }

    #[test]
    fn test_rl_zero() {
        let mut cpu = Cpu::new();
        let res = rl(&mut cpu, 0x00);
        assert_eq!(res, 0x00);
        assert!(cpu.reg.check_flag(Flag::Zero));
        assert!(!cpu.reg.check_flag(Flag::Subtract));
        assert!(!cpu.reg.check_flag(Flag::HalfCarry));
        assert!(!cpu.reg.check_flag(Flag::Carry));
    }
}
