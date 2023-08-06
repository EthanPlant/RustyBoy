mod mmu;

use mmu::Memory;

/// The Gameboy emulator
pub struct Gameboy {
    mmu: Memory,
}

impl Gameboy {
    /// Initialize the emulator
    pub fn new(cart_data: Vec<u8>) -> Self {
        let mut mmu = Memory::new();
        for (i, b) in cart_data.iter().enumerate() {
            mmu.set_byte(i, *b);
        }
        Self {
            mmu,
        }
    }
}