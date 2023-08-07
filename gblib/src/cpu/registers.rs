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
}