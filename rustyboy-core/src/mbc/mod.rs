use crate::cartridge::{Cartridge, CartridgeType};

mod mbc1;
pub mod rom_only;

pub fn from_cartridge(cart: Cartridge) -> Box<dyn Mbc> {
    match cart.cart_type {
        CartridgeType::RomOnly => {
            return Box::new(rom_only::RomOnly::new(cart));
        }
        // Do this for now just to get Blargg working
        CartridgeType::Mbc1 => {
            return Box::new(mbc1::Mbc1::new(cart));
        }
        CartridgeType::Mbc1Ram => {
            return Box::new(mbc1::Mbc1::new(cart));
        }
        CartridgeType::Mbc1RamBattery => {
            return Box::new(mbc1::Mbc1::new(cart));
        }
        _ => unimplemented!(),
    }
}

pub trait Mbc {
    fn read_byte_from_rom(&self, addr: usize) -> u8;
    fn write_byte_to_rom(&mut self, addr: usize, value: u8);
    fn read_byte_from_ram(&self, addr: usize) -> u8;
    fn write_byte_to_ram(&mut self, addr: usize, value: u8);
}
