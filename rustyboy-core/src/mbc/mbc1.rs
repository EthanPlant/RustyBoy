use crate::cartridge::Cartridge;
use crate::mbc::Mbc;

/// Banking modes supported by MBC1
#[derive(Debug)]
enum BankingMode {
    /// 0000-3FFF is always bank 0 of ROM, A000-BFFF is bank 0 of RAM.
    Simple,
    /// 0000-3FFF and A000-BFFF can be banked
    Advanced,
}

pub struct Mbc1 {
    /// The cartridge that this MBC1 is managing.
    cart: Cartridge,
    /// Whether or not the RAM is enabled.
    ram_enabled: bool,
    /// The current ROM bank that is mapped into the 0x4000-0x7FFF range.
    rom_bank: u8,
    /// The current RAM bank that is mapped into the 0xA000-0xBFFF range.
    ram_bank: u8,
    /// The current banking mode.
    banking_mode: BankingMode,
}

impl Mbc1 {
    pub fn new(cart: Cartridge) -> Self {
        Mbc1 {
            cart,
            ram_enabled: false,
            rom_bank: 1,
            ram_bank: 0,
            banking_mode: BankingMode::Simple,
        }
    }

    fn read_rom_bank(&self, bank: u8, offset: usize) -> u8 {
        self.cart.rom[(bank as usize * 0x4000) + offset]
    }

    fn ram_bank_offset(&self) -> u16 {
        match self.banking_mode {
            BankingMode::Simple => 0,
            BankingMode::Advanced => self.ram_bank as u16 * 0x2000,
        }
    }
}

impl Mbc for Mbc1 {
    fn read_byte_from_rom(&self, addr: usize) -> u8 {
        match addr {
            0x0000..=0x3FFF => match self.banking_mode {
                // 0000-3FFF is always bank 0 of ROM in simple mode
                // or bank 32, 64, or 96 in advanced mode
                BankingMode::Simple => self.cart.rom[addr],
                BankingMode::Advanced => {
                    let bits = self.rom_bank & 0x60;
                    self.read_rom_bank(bits * 0x20, addr)
                }
            },
            0x4000..=0x7FFF => self.read_rom_bank(self.rom_bank, addr - 0x4000),
            _ => 0,
        }
    }

    fn write_byte_to_rom(&mut self, addr: usize, value: u8) {
        match addr {
            0x0000..=0x1FFF => {
                self.ram_enabled = (value & 0x0F) == 0x0A;
                if self.ram_enabled {
                    log::trace!("MBC1: RAM enabled");
                } else {
                    log::trace!("MBC1: RAM disabled");
                }
            }
            0x2000..=0x3FFF => {
                self.rom_bank = value & 0x1F;
                if self.rom_bank == 0 {
                    self.rom_bank = 1;
                }
                log::trace!("MBC1: Switching to ROM bank {}", self.rom_bank);
            }
            0x4000..=0x5FFF => {
                self.ram_bank = value & 0x03;
                log::trace!("MBC1: Switching to RAM bank {}", self.ram_bank);
            }
            0x6000..=0x7FFF => {
                self.banking_mode = match value & 0x01 {
                    0 => BankingMode::Simple,
                    1 => BankingMode::Advanced,
                    _ => unreachable!(),
                };
                log::trace!("MBC1: Switching to banking mode {:?}", self.banking_mode);
            }
            _ => panic!(
                "MBC1: Attempted to write to ROM at invalid address {:04X}",
                addr
            ),
        }
    }

    fn read_byte_from_ram(&self, addr: usize) -> u8 {
        if self.ram_enabled {
            self.cart.ram[self.ram_bank_offset() as usize + addr]
        } else {
            log::warn!("MBC1: Attempted to read from RAM when it is disabled");
            0xFF
        }
    }

    fn write_byte_to_ram(&mut self, addr: usize, value: u8) {
        if self.ram_enabled {
            let offset = self.ram_bank_offset();
            self.cart.ram[offset as usize + addr] = value;
        } else {
            log::warn!("MBC1: Attempted to write to RAM when it is disabled");
        }
    }
}
