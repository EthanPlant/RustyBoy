use crate::cartridge::Cartridge;
use crate::mbc::Mbc;

pub struct RomOnly {
    pub cart: Cartridge,
}

impl RomOnly {
    pub fn new(cart: Cartridge) -> Self {
        Self { cart }
    }
}

impl Mbc for RomOnly {
    fn read_byte_from_rom(&self, addr: usize) -> u8 {
        self.cart.rom[addr]
    }

    fn write_byte_to_rom(&mut self, addr: usize, value: u8) {
        log::warn!(
            "Attempted to write to ROM at address {:#06X} with value {:#04X}",
            addr,
            value
        );
    }

    fn read_byte_from_ram(&self, addr: usize) -> u8 {
        self.cart.ram[addr]
    }

    fn write_byte_to_ram(&mut self, addr: usize, value: u8) {
        self.cart.ram[addr] = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rom_only() {
        let mut cart = Cartridge::new();
        cart.rom[0x0000] = 0x00;
        cart.rom[0x7FFF] = 0xFF;
        cart.ram[0x0000] = 0x00;
        cart.ram[0x1FFF] = 0xFF;

        let mut rom_only = RomOnly::new(cart);

        assert_eq!(rom_only.read_byte_from_rom(0x0000), 0x00);
        assert_eq!(rom_only.read_byte_from_rom(0x7FFF), 0xFF);
        assert_eq!(rom_only.read_byte_from_ram(0x0000), 0x00);
        assert_eq!(rom_only.read_byte_from_ram(0x1FFF), 0xFF);

        rom_only.write_byte_to_rom(0x0000, 0xFF);
        rom_only.write_byte_to_rom(0x7FFF, 0x00);
        rom_only.write_byte_to_ram(0x0000, 0xFF);
        rom_only.write_byte_to_ram(0x1FFF, 0x00);

        assert_eq!(rom_only.read_byte_from_rom(0x0000), 0x00);
        assert_eq!(rom_only.read_byte_from_rom(0x7FFF), 0xFF);
        assert_eq!(rom_only.read_byte_from_ram(0x0000), 0xFF);
        assert_eq!(rom_only.read_byte_from_ram(0x1FFF), 0x00);
    }
}
