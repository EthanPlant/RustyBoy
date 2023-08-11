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

/// Add a byte to the A register with the carry flag
/// Sets the zero flag if the result is zero.
/// Sets the carry flag if the result overflows.
/// Sets the half carry flag if the lower nibble of the A register overflows.
/// Clears the subtract flag.
pub fn adc(cpu: &mut Cpu, byte: u8) {
    let carry = cpu.reg.check_flag(Flag::Carry) as u8;
    let result = cpu.reg.a.wrapping_add(byte).wrapping_add(carry);
    cpu.reg.clear_all_flags();
    if result == 0 {
        cpu.reg.set_flag(Flag::Zero);
    }
    if (cpu.reg.a as u16) + (byte as u16) + (carry as u16) > 0xFF {
        cpu.reg.set_flag(Flag::Carry);
    }
    if (cpu.reg.a & 0x0F) + (byte & 0x0F) + carry > 0x0F {
        cpu.reg.set_flag(Flag::HalfCarry);
    }
    cpu.reg.a = result;
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

/// Add a word to the HL register.
/// Sets the carry flag if the result overflows.
/// Sets the half carry flag if overflow from bit 11
/// Clears the subtract flag.
pub fn add16(cpu: &mut Cpu, word: u16) {
    let result = cpu.reg.hl().wrapping_add(word);
    cpu.reg.clear_flag(Flag::Subtract);
    if (cpu.reg.hl() & 0x0FFF) + (word & 0x0FFF) > 0x0FFF {
        cpu.reg.set_flag(Flag::HalfCarry);
    } else {
        cpu.reg.clear_flag(Flag::HalfCarry);
    }

    if (cpu.reg.hl() as u32) + (word as u32) > 0xFFFF {
        cpu.reg.set_flag(Flag::Carry);
    } else {
        cpu.reg.clear_flag(Flag::Carry);
    }
    cpu.reg.set_hl(result);
}

/// Logical AND on a byte and the A register.
/// Sets the zero flag if the result is zero.
/// Sets the half carry flag.
/// Clears all other flags.
/// Stores the result in the A register.
pub fn and(cpu: &mut Cpu, byte: u8) {
    let result = cpu.reg.a & byte;
    cpu.reg.clear_all_flags();
    cpu.reg.set_flag(Flag::HalfCarry);
    if result == 0 {
        cpu.reg.set_flag(Flag::Zero);
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
    if result == 0 {
        cpu.reg.set_flag(Flag::Zero);
    } else {
        cpu.reg.clear_flag(Flag::Zero);
    }
    if byte & 0x0F == 0x00 {
        cpu.reg.set_flag(Flag::HalfCarry);
    } else {
        cpu.reg.clear_flag(Flag::HalfCarry);
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
    if result == 0 {
        cpu.reg.set_flag(Flag::Zero);
    } else {
        cpu.reg.clear_flag(Flag::Zero);
    }
    if byte & 0x0F == 0x0F {
        cpu.reg.set_flag(Flag::HalfCarry);
    } else {
        cpu.reg.clear_flag(Flag::HalfCarry)
    }
    result
}

/// Logical OR on a byte and the A register.
/// Sets the zero flag if the result is zero.
/// Clears all other flags.
/// Stores the result in the A register.
pub fn or(cpu: &mut Cpu, byte: u8) {
    let result = cpu.reg.a | byte;
    cpu.reg.clear_all_flags();
    if result == 0 {
        cpu.reg.set_flag(Flag::Zero);
    }
    cpu.reg.a = result;
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
    if byte & (1 << bit) == 0 {
        cpu.reg.set_flag(Flag::Zero);
    } else {
        cpu.reg.clear_flag(Flag::Zero);
    }
    cpu.reg.clear_flag(Flag::Subtract);
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

/// Rotates a byte right through the carry flag.
/// Sets the zero flag if the result is zero.
/// Sets the carry flag to the value of bit 0.
/// Clears all other flags.
pub fn rr(cpu: &mut Cpu, byte: u8) -> u8 {
    let carry: u8 = byte & 1;
    let result = (byte >> 1) | (cpu.reg.check_flag(Flag::Carry) as u8) << 7;
    cpu.reg.clear_all_flags();
    if result == 0 {
        cpu.reg.set_flag(Flag::Zero);
    }
    if carry == 1 {
        cpu.reg.set_flag(Flag::Carry);
    }
    result
}

/// Logical shift to the right
/// Sets the zero flag if the result is zero.
/// Sets the carry flag to the value of bit 0.
/// Clears all other flags.
pub fn srl(cpu: &mut Cpu, byte: u8) -> u8 {
    let carry: u8 = byte & 1;
    let result = byte >> 1;
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
    pub fn test_get_op8() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.pc = 0xC000;
        mmu.set_byte(0xC001 as usize, 0x01);
        assert_eq!(get_op8(&cpu, &mmu, 1), 0x01);
    }

    #[test]
    pub fn test_get_op16() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.pc = 0xC000;
        mmu.set_word(0xC001 as usize, 0x0001);
        assert_eq!(get_op16(&cpu, &mmu), 0x0001);
    }

    #[test]
    pub fn test_get_hl() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.h = 0xC0;
        cpu.reg.l = 0x00;
        mmu.set_byte(0xC000 as usize, 0x01);
        assert_eq!(get_hl(&cpu, &mmu), 0x01);
    }

    #[test]
    pub fn test_set_hl() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.h = 0xC0;
        cpu.reg.l = 0x00;
        set_hl(&mut cpu, &mut mmu, 0x01);
        assert_eq!(mmu.get_byte(0xC000 as usize), 0x01);
    }

    #[test]
    pub fn test_push() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.sp = 0xFFFE;
        push(&mut cpu, &mut mmu, 0x0001);
        assert_eq!(cpu.reg.sp, 0xFFFC);
        assert_eq!(mmu.get_byte(0xFFFC as usize), 0x0001);
    }

    #[test]
    pub fn test_pop() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.sp = 0xFFFC;
        mmu.set_word(0xFFFC as usize, 0x0001);
        assert_eq!(pop(&mut cpu, &mut mmu), 0x0001);
    }

    #[test]
    pub fn test_call() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.sp = 0xFFFE;
        cpu.reg.pc = 0xC000;
        mmu.set_word(0xC001 as usize, 0x0001);
        call(&mut cpu, &mut mmu);
        assert_eq!(cpu.reg.sp, 0xFFFC);
        assert_eq!(cpu.reg.pc, 0x0001);
        assert_eq!(mmu.get_word(0xFFFC as usize), 0xC003);
    }

    #[test]
    pub fn test_jr_positive() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.pc = 0xC000;
        mmu.set_byte(0xC001 as usize, 0x01);
        jr(&mut cpu, &mut mmu);
        assert_eq!(cpu.reg.pc, 0xC003);
    }

    #[test]
    pub fn test_jr_negative() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        cpu.reg.pc = 0xC000;
        mmu.set_byte(0xC001 as usize, 0xFE);
        jr(&mut cpu, &mut mmu);
        assert_eq!(cpu.reg.pc, 0xC000);
    }

    #[test]
    pub fn test_adc_carry_not_set() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x01;
        cpu.reg.clear_flag(Flag::Carry);
        adc(&mut cpu, 0x01);
        assert_eq!(cpu.reg.a, 0x02);
        assert!(!cpu.reg.check_flag(Flag::Zero));
        assert!(!cpu.reg.check_flag(Flag::Subtract));
        assert!(!cpu.reg.check_flag(Flag::HalfCarry));
        assert!(!cpu.reg.check_flag(Flag::Carry));
    }

    #[test]
    pub fn test_adc_carry_set() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x01;
        cpu.reg.set_flag(Flag::Carry);
        adc(&mut cpu, 0x01);
        assert_eq!(cpu.reg.a, 0x03);
        assert!(!cpu.reg.check_flag(Flag::Zero));
        assert!(!cpu.reg.check_flag(Flag::Subtract));
        assert!(!cpu.reg.check_flag(Flag::HalfCarry));
        assert!(!cpu.reg.check_flag(Flag::Carry));
    }

    #[test]
    pub fn test_adc_zero() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x00;
        cpu.reg.clear_flag(Flag::Carry);
        adc(&mut cpu, 0x00);
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_adc_halfcarry() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x0F;
        cpu.reg.clear_flag(Flag::Carry);
        adc(&mut cpu, 0x01);
        assert!(cpu.reg.check_flag(Flag::HalfCarry));
    }

    #[test]
    pub fn test_adc_carry() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0xFF;
        cpu.reg.clear_flag(Flag::Carry);
        adc(&mut cpu, 0x02);
        assert_eq!(cpu.reg.a, 0x01);
        assert!(cpu.reg.check_flag(Flag::Carry));
    }

    #[test]
    pub fn test_add() {
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
    pub fn test_add_zero() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x00;
        add(&mut cpu, 0x00);
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_add_halfcarry() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x0F;
        add(&mut cpu, 0x01);
        assert!(cpu.reg.check_flag(Flag::HalfCarry));
    }

    #[test]
    pub fn test_add_carry() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0xFF;
        add(&mut cpu, 0x02);
        assert_eq!(cpu.reg.a, 0x01);
        assert!(cpu.reg.check_flag(Flag::Carry));
    }

    #[test]
    pub fn test_add16() {
        let mut cpu = Cpu::new();
        cpu.reg.set_hl(0x0001);
        add16(&mut cpu, 0x0001);
        assert_eq!(cpu.reg.hl(), 0x0002);
        assert!(!cpu.reg.check_flag(Flag::Subtract));
        assert!(!cpu.reg.check_flag(Flag::HalfCarry));
        assert!(!cpu.reg.check_flag(Flag::Carry));
    }

    #[test]
    pub fn test_add16_halfcarry() {
        let mut cpu = Cpu::new();
        cpu.reg.set_hl(0x0FFF);
        add16(&mut cpu, 0x0001);
        assert_eq!(cpu.reg.hl(), 0x1000);
        assert!(cpu.reg.check_flag(Flag::HalfCarry));
    }

    #[test]
    pub fn test_add16_carry() {
        let mut cpu = Cpu::new();
        cpu.reg.set_hl(0xFFFF);
        add16(&mut cpu, 0x0001);
        assert_eq!(cpu.reg.hl(), 0x0000);
        assert!(cpu.reg.check_flag(Flag::Carry));
    }

    #[test]
    pub fn test_and() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x01;
        and(&mut cpu, 0x01);
        assert_eq!(cpu.reg.a, 0x01);
        assert!(!cpu.reg.check_flag(Flag::Zero));
        assert!(!cpu.reg.check_flag(Flag::Subtract));
        assert!(cpu.reg.check_flag(Flag::HalfCarry));
        assert!(!cpu.reg.check_flag(Flag::Carry));
    }

    #[test]
    pub fn test_and_zero() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x01;
        and(&mut cpu, 0x00);
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_cp() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x02;
        cp(&mut cpu, 0x01);
        assert_eq!(cpu.reg.a, 0x02);
        assert!(!cpu.reg.check_flag(Flag::Zero));
        assert!(cpu.reg.check_flag(Flag::Subtract));
        assert!(!cpu.reg.check_flag(Flag::HalfCarry));
        assert!(!cpu.reg.check_flag(Flag::Carry));
    }

    #[test]
    pub fn test_cp_zero() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x01;
        cp(&mut cpu, 0x01);
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_cp_halfcarry() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x10;
        cp(&mut cpu, 0x01);
        assert!(cpu.reg.check_flag(Flag::HalfCarry));
    }

    #[test]
    pub fn test_cp_carry() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x00;
        cp(&mut cpu, 0x01);
        assert!(cpu.reg.check_flag(Flag::Carry));
    }

    #[test]
    pub fn test_dec() {
        let mut cpu = Cpu::new();
        assert_eq!(dec(&mut cpu, 0x02), 0x01);
        assert!(!cpu.reg.check_flag(Flag::Zero));
        assert!(cpu.reg.check_flag(Flag::Subtract));
        assert!(!cpu.reg.check_flag(Flag::HalfCarry));
    }

    #[test]
    pub fn test_dec_zero() {
        let mut cpu = Cpu::new();
        dec(&mut cpu, 0x01);
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_dec_halfcarry() {
        let mut cpu = Cpu::new();
        dec(&mut cpu, 0x10);
        assert!(cpu.reg.check_flag(Flag::HalfCarry));
    }

    #[test]
    pub fn test_dec_underflow() {
        let mut cpu = Cpu::new();
        assert_eq!(dec(&mut cpu, 0x00), 0xFF);
    }

    #[test]
    pub fn test_inc() {
        let mut cpu = Cpu::new();
        assert_eq!(inc(&mut cpu, 0x01), 0x02);
        assert!(!cpu.reg.check_flag(Flag::Zero));
        assert!(!cpu.reg.check_flag(Flag::Subtract));
        assert!(!cpu.reg.check_flag(Flag::HalfCarry))
    }

    #[test]
    pub fn test_inc_zero() {
        let mut cpu = Cpu::new();
        assert_eq!(inc(&mut cpu, 0xFF), 0x00);
        assert!(cpu.reg.check_flag(Flag::Zero))
    }

    #[test]
    pub fn test_inc_halfcarry() {
        let mut cpu = Cpu::new();
        inc(&mut cpu, 0x0F);
        assert!(cpu.reg.check_flag(Flag::HalfCarry))
    }

    #[test]
    pub fn test_or() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x01;
        or(&mut cpu, 0x02);
        assert_eq!(cpu.reg.a, 0x03);
        assert!(!cpu.reg.check_flag(Flag::Zero));
        assert!(!cpu.reg.check_flag(Flag::Subtract));
        assert!(!cpu.reg.check_flag(Flag::HalfCarry));
        assert!(!cpu.reg.check_flag(Flag::Carry));
    }

    #[test]
    pub fn test_or_zero() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x00;
        or(&mut cpu, 0x00);
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_sub() {
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
    pub fn test_sub_zero() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x01;
        sub(&mut cpu, 0x01);
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_sub_halfcarry() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x10;
        sub(&mut cpu, 0x01);
        assert!(cpu.reg.check_flag(Flag::HalfCarry));
    }

    #[test]
    pub fn test_sub_carry() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x00;
        sub(&mut cpu, 0x01);
        assert!(cpu.reg.check_flag(Flag::Carry));
    }

    #[test]
    pub fn test_xor() {
        let mut cpu = Cpu::new();
        xor_bytes(&mut cpu, 0x01, 0x03);
        assert_eq!(cpu.reg.a, 0x02);
        assert!(!cpu.reg.check_flag(Flag::Zero));
        assert!(!cpu.reg.check_flag(Flag::Subtract));
        assert!(!cpu.reg.check_flag(Flag::HalfCarry));
        assert!(!cpu.reg.check_flag(Flag::Carry));
    }

    #[test]
    pub fn test_xor_zero() {
        let mut cpu = Cpu::new();
        xor_bytes(&mut cpu, 0x01, 0x01);
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_bit() {
        let mut cpu = Cpu::new();
        bit(&mut cpu, 0x01, 0);
        assert!(!cpu.reg.check_flag(Flag::Zero));
        assert!(!cpu.reg.check_flag(Flag::Subtract));
        assert!(cpu.reg.check_flag(Flag::HalfCarry))
    }

    #[test]
    pub fn test_bit_zero() {
        let mut cpu = Cpu::new();
        bit(&mut cpu, 0x00, 0);
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_rl_bit_seven_clear_carry_clear() {
        let mut cpu = Cpu::new();
        cpu.reg.clear_flag(Flag::Carry);
        assert_eq!(rl(&mut cpu, 0x7F), 0xFE);
        assert!(!cpu.reg.check_flag(Flag::Zero));
        assert!(!cpu.reg.check_flag(Flag::Subtract));
        assert!(!cpu.reg.check_flag(Flag::HalfCarry));
        assert!(!cpu.reg.check_flag(Flag::Carry));
    }

    #[test]
    pub fn test_rl_bit_seven_set_carry_clear() {
        let mut cpu = Cpu::new();
        cpu.reg.clear_flag(Flag::Carry);
        assert_eq!(rl(&mut cpu, 0xFF), 0xFE);
        assert!(cpu.reg.check_flag(Flag::Carry));
    }

    #[test]
    pub fn test_rl_bit_seven_clear_carry_set() {
        let mut cpu = Cpu::new();
        cpu.reg.set_flag(Flag::Carry);
        assert_eq!(rl(&mut cpu, 0x7F), 0xFF);
        assert!(!cpu.reg.check_flag(Flag::Carry));
    }

    #[test]
    pub fn test_rl_bit_seven_set_carry_set() {
        let mut cpu = Cpu::new();
        cpu.reg.set_flag(Flag::Carry);
        assert_eq!(rl(&mut cpu, 0xFF), 0xFF);
        assert!(cpu.reg.check_flag(Flag::Carry));
    }

    #[test]
    pub fn test_rl_zero() {
        let mut cpu = Cpu::new();
        cpu.reg.clear_flag(Flag::Carry);
        rl(&mut cpu, 0x00);
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_rr_bit_zero_clear_carry_clear() {
        let mut cpu = Cpu::new();
        cpu.reg.clear_flag(Flag::Carry);
        assert_eq!(rr(&mut cpu, 0xFE), 0x7F);
        assert!(!cpu.reg.check_flag(Flag::Zero));
        assert!(!cpu.reg.check_flag(Flag::Subtract));
        assert!(!cpu.reg.check_flag(Flag::HalfCarry));
        assert!(!cpu.reg.check_flag(Flag::Carry));
    }

    #[test]
    pub fn test_rr_bit_zero_set_carry_clear() {
        let mut cpu = Cpu::new();
        cpu.reg.clear_flag(Flag::Carry);
        assert_eq!(rr(&mut cpu, 0xFF), 0x7F);
        assert!(cpu.reg.check_flag(Flag::Carry));
    }

    #[test]
    pub fn test_rr_bit_zero_clear_carry_set() {
        let mut cpu = Cpu::new();
        cpu.reg.set_flag(Flag::Carry);
        assert_eq!(rr(&mut cpu, 0xFE), 0xFF);
        assert!(!cpu.reg.check_flag(Flag::Carry));
    }

    #[test]
    pub fn test_rr_bit_zero_set_carry_set() {
        let mut cpu = Cpu::new();
        cpu.reg.set_flag(Flag::Carry);
        assert_eq!(rr(&mut cpu, 0xFF), 0xFF);
        assert!(cpu.reg.check_flag(Flag::Carry));
    }

    #[test]
    pub fn test_rr_zero() {
        let mut cpu = Cpu::new();
        cpu.reg.clear_flag(Flag::Carry);
        rr(&mut cpu, 0x00);
        assert!(cpu.reg.check_flag(Flag::Zero));
    }

    #[test]
    pub fn test_srl_bit_zero_clear() {
        let mut cpu = Cpu::new();
        assert_eq!(srl(&mut cpu, 0xFE), 0x7F);
        assert!(!cpu.reg.check_flag(Flag::Zero));
        assert!(!cpu.reg.check_flag(Flag::Subtract));
        assert!(!cpu.reg.check_flag(Flag::HalfCarry));
        assert!(!cpu.reg.check_flag(Flag::Carry));
    }

    #[test]
    pub fn test_srl_bit_zero_set() {
        let mut cpu = Cpu::new();
        assert_eq!(srl(&mut cpu, 0xFF), 0x7F);
        assert!(cpu.reg.check_flag(Flag::Carry));
    }

    #[test]
    pub fn test_srl_zero() {
        let mut cpu = Cpu::new();
        srl(&mut cpu, 0x01);
        assert!(cpu.reg.check_flag(Flag::Zero));
    }
}
