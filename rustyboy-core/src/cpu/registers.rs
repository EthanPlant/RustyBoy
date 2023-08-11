/// The CPU flags
pub enum Flag {
    /// Set when an arithmetic operation results in a zero value
    Zero = 0x80,
    /// Set when an arithmetic operation results in a negative value
    Subtract = 0x40,
    /// Set when an arithmetic operation results in a carry from bit 3 to bit 4
    HalfCarry = 0x20,
    /// Set when an arithmetic operation results in a carry from bit 7 to bit 8
    Carry = 0x10,
}

/// The CPU registers
#[derive(Clone, Debug, PartialEq)]
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    /// The flag register
    pub f: u8,
    pub h: u8,
    pub l: u8,
    /// Program Counter
    pub pc: u16,
    /// Stack Pointer
    pub sp: u16,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            a: 0x00,
            b: 0x00,
            c: 0x00,
            d: 0x00,
            e: 0x00,
            f: 0x00,
            h: 0x00,
            l: 0x00,
            pc: 0x0000,
            sp: 0x0000,
        }
    }

    /// Set a given flag
    pub fn set_flag(&mut self, flag: Flag) {
        self.f |= flag as u8;
    }

    /// Clear a given flag
    pub fn clear_flag(&mut self, flag: Flag) {
        self.f &= !(flag as u8);
    }

    /// Clear all flags
    pub fn clear_all_flags(&mut self) {
        self.f = 0x00;
    }

    /// Check if a given flag is set
    pub fn check_flag(&self, flag: Flag) -> bool {
        self.f & (flag as u8) != 0
    }

    /// Get the AF register pair
    pub fn af(&self) -> u16 {
        ((self.a as u16) << 8) | (self.f as u16)
    }

    /// Get the BC register pair
    pub fn bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }

    /// Get the DE register pair
    pub fn de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }

    /// Get the HL register pair
    pub fn hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }

    /// Set the AF register pair
    /// Ensure that the lower four bytes of the F register are zero
    pub fn set_af(&mut self, value: u16) {
        self.a = ((value & 0xFF00) >> 8) as u8;
        self.f = (value & 0x00FF) as u8;
        self.f &= 0xF0;
    }

    /// Set the BC register pair
    pub fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0x00FF) as u8;
    }

    /// Set the DE register pair
    pub fn set_de(&mut self, value: u16) {
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0x00FF) as u8;
    }

    /// Set the HL register pair
    pub fn set_hl(&mut self, value: u16) {
        self.h = ((value & 0xFF00) >> 8) as u8;
        self.l = (value & 0x00FF) as u8;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_zero_flag() {
        let mut regs = Registers::new();
        regs.set_flag(Flag::Zero);
        assert_eq!(regs.f, 0x80);
    }

    #[test]
    fn test_set_subtract_flag() {
        let mut regs = Registers::new();
        regs.set_flag(Flag::Subtract);
        assert_eq!(regs.f, 0x40);
    }

    #[test]
    fn test_set_half_carry_flag() {
        let mut regs = Registers::new();
        regs.set_flag(Flag::HalfCarry);
        assert_eq!(regs.f, 0x20);
    }

    #[test]
    fn test_set_carry_flag() {
        let mut regs = Registers::new();
        regs.set_flag(Flag::Carry);
        assert_eq!(regs.f, 0x10);
    }

    #[test]
    fn test_clear_zero_flag() {
        let mut regs = Registers::new();
        regs.f = 0xFF;
        regs.clear_flag(Flag::Zero);
        assert_eq!(regs.f, 0x7F);
    }

    #[test]
    fn test_clear_subtract_flag() {
        let mut regs = Registers::new();
        regs.f = 0xFF;
        regs.clear_flag(Flag::Subtract);
        assert_eq!(regs.f, 0xBF);
    }

    #[test]
    fn test_clear_half_carry_flag() {
        let mut regs = Registers::new();
        regs.f = 0xFF;
        regs.clear_flag(Flag::HalfCarry);
        assert_eq!(regs.f, 0xDF);
    }

    #[test]
    fn test_clear_carry_flag() {
        let mut regs = Registers::new();
        regs.f = 0xFF;
        regs.clear_flag(Flag::Carry);
        assert_eq!(regs.f, 0xEF);
    }

    #[test]
    fn test_clear_all_flags() {
        let mut regs = Registers::new();
        regs.f = 0xFF;
        regs.clear_all_flags();
        assert_eq!(regs.f, 0x00);
    }

    #[test]
    fn test_check_flag() {
        let mut regs = Registers::new();
        regs.f = 0xFF;
        assert_eq!(regs.check_flag(Flag::Zero), true);
        assert_eq!(regs.check_flag(Flag::Subtract), true);
        assert_eq!(regs.check_flag(Flag::HalfCarry), true);
        assert_eq!(regs.check_flag(Flag::Carry), true);
    }

    #[test]
    fn test_check_flag_not_set() {
        let mut regs = Registers::new();
        regs.f = 0x00;
        assert_eq!(regs.check_flag(Flag::Zero), false);
        assert_eq!(regs.check_flag(Flag::Subtract), false);
        assert_eq!(regs.check_flag(Flag::HalfCarry), false);
        assert_eq!(regs.check_flag(Flag::Carry), false);
    }

    #[test]
    fn test_af() {
        let mut regs = Registers::new();
        regs.a = 0x12;
        regs.f = 0x34;
        assert_eq!(regs.af(), 0x1234);
    }

    #[test]
    fn test_bc() {
        let mut regs = Registers::new();
        regs.b = 0x12;
        regs.c = 0x34;
        assert_eq!(regs.bc(), 0x1234);
    }

    #[test]
    fn test_de() {
        let mut regs = Registers::new();
        regs.d = 0x12;
        regs.e = 0x34;
        assert_eq!(regs.de(), 0x1234);
    }

    #[test]
    fn test_hl() {
        let mut regs = Registers::new();
        regs.h = 0x12;
        regs.l = 0x34;
        assert_eq!(regs.hl(), 0x1234);
    }

    #[test]
    fn test_set_af() {
        let mut regs = Registers::new();
        regs.set_af(0x1230);
        assert_eq!(regs.a, 0x12);
        assert_eq!(regs.f, 0x30);
    }

    #[test]
    fn test_set_af_f_lower_nibble_non_zero() {
        let mut regs = Registers::new();
        regs.set_af(0x1234);
        assert_eq!(regs.a, 0x12);
        assert_eq!(regs.f, 0x30);
    }

    #[test]
    fn test_set_bc() {
        let mut regs = Registers::new();
        regs.set_bc(0x1234);
        assert_eq!(regs.b, 0x12);
        assert_eq!(regs.c, 0x34);
    }

    #[test]
    fn test_set_de() {
        let mut regs = Registers::new();
        regs.set_de(0x1234);
        assert_eq!(regs.d, 0x12);
        assert_eq!(regs.e, 0x34);
    }

    #[test]
    fn test_set_hl() {
        let mut regs = Registers::new();
        regs.set_hl(0x1234);
        assert_eq!(regs.h, 0x12);
        assert_eq!(regs.l, 0x34);
    }
}
