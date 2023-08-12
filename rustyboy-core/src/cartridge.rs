const TITLE_START_ADDR: u16 = 0x0134;
const TITLE_END_ADDR: u16 = 0x0143;
const CART_TYPE_ADDR: u16 = 0x0147;
const ROM_SIZE_ADDR: u16 = 0x0148;
const RAM_SIZE_ADDR: u16 = 0x0149;

#[derive(PartialEq)]
pub enum CartridgeType {
    RomOnly = 0x00,
    Mbc1 = 0x01,
    Mbc1Ram = 0x02,
    Mbc1RamBattery = 0x03,
    Mbc2 = 0x05,
    Mbc2Battery = 0x06,
    RomRam = 0x08,
    RomRamBattery = 0x09,
    Mmm01 = 0x0B,
    Mmm01Ram = 0x0C,
    Mmm01RamBattery = 0x0D,
    Mbc3TimerBattery = 0x0F,
    Mbc3TimerRamBattery = 0x10,
    Mbc3 = 0x11,
    Mbc3Ram = 0x12,
    Mbc3RamBattery = 0x13,
    Mbc5 = 0x19,
    Mbc5Ram = 0x1A,
    Mbc5RamBattery = 0x1B,
    Mbc5Rumble = 0x1C,
    Mbc5RumbleRam = 0x1D,
    Mbc5RumbleRamBattery = 0x1E,
    Mbc6 = 0x20,
    Mbc7SensorRumbleRamBattery = 0x22,
    PocketCamera = 0xFC,
    BandaiTama5 = 0xFD,
    HuC3 = 0xFE,
    HuC1RamBattery = 0xFF,
}

impl std::fmt::Display for CartridgeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RomOnly => write!(f, "ROM ONLY"),
            Self::Mbc1 => write!(f, "MBC1"),
            Self::Mbc1Ram => write!(f, "MBC1+RAM"),
            Self::Mbc1RamBattery => write!(f, "MBC1+RAM+BATTERY"),
            Self::Mbc2 => write!(f, "MBC2"),
            Self::Mbc2Battery => write!(f, "MBC2+BATTERY"),
            Self::RomRam => write!(f, "ROM+RAM"),
            Self::RomRamBattery => write!(f, "ROM+RAM+BATTERY"),
            Self::Mmm01 => write!(f, "MMM01"),
            Self::Mmm01Ram => write!(f, "MMM01+RAM"),
            Self::Mmm01RamBattery => write!(f, "MMM01+RAM+BATTERY"),
            Self::Mbc3TimerBattery => write!(f, "MBC3+TIMER+BATTERY"),
            Self::Mbc3TimerRamBattery => write!(f, "MBC3+TIMER+RAM+BATTERY"),
            Self::Mbc3 => write!(f, "MBC3"),
            Self::Mbc3Ram => write!(f, "MBC3+RAM"),
            Self::Mbc3RamBattery => write!(f, "MBC3+RAM+BATTERY"),
            Self::Mbc5 => write!(f, "MBC5"),
            Self::Mbc5Ram => write!(f, "MBC5+RAM"),
            Self::Mbc5RamBattery => write!(f, "MBC5+RAM+BATTERY"),
            Self::Mbc5Rumble => write!(f, "MBC5+RUMBLE"),
            Self::Mbc5RumbleRam => write!(f, "MBC5+RUMBLE+RAM"),
            Self::Mbc5RumbleRamBattery => write!(f, "MBC5+RUMBLE+RAM+BATTERY"),
            Self::Mbc6 => write!(f, "MBC6"),
            Self::Mbc7SensorRumbleRamBattery => write!(f, "MBC7+SENSOR+RUMBLE+RAM+BATTERY"),
            Self::PocketCamera => write!(f, "POCKET CAMERA"),
            Self::BandaiTama5 => write!(f, "BANDAI TAMA5"),
            Self::HuC3 => write!(f, "HuC3"),
            Self::HuC1RamBattery => write!(f, "HuC1+RAM+BATTERY"),
        }
    }
}

impl TryFrom<u8> for CartridgeType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            x if x == 0x00 => Ok(Self::RomOnly),
            x if x == 0x01 => Ok(Self::Mbc1),
            x if x == 0x02 => Ok(Self::Mbc1Ram),
            x if x == 0x03 => Ok(Self::Mbc1RamBattery),
            x if x == 0x05 => Ok(Self::Mbc2),
            x if x == 0x06 => Ok(Self::Mbc2Battery),
            x if x == 0x08 => Ok(Self::RomRam),
            x if x == 0x09 => Ok(Self::RomRamBattery),
            x if x == 0x0B => Ok(Self::Mmm01),
            x if x == 0x0C => Ok(Self::Mmm01Ram),
            x if x == 0x0D => Ok(Self::Mmm01RamBattery),
            x if x == 0x0F => Ok(Self::Mbc3TimerBattery),
            x if x == 0x10 => Ok(Self::Mbc3TimerRamBattery),
            x if x == 0x11 => Ok(Self::Mbc3),
            x if x == 0x12 => Ok(Self::Mbc3Ram),
            x if x == 0x13 => Ok(Self::Mbc3RamBattery),
            x if x == 0x19 => Ok(Self::Mbc5),
            x if x == 0x1A => Ok(Self::Mbc5Ram),
            x if x == 0x1B => Ok(Self::Mbc5RamBattery),
            x if x == 0x1C => Ok(Self::Mbc5Rumble),
            x if x == 0x1D => Ok(Self::Mbc5RumbleRam),
            x if x == 0x1E => Ok(Self::Mbc5RumbleRamBattery),
            x if x == 0x20 => Ok(Self::Mbc6),
            x if x == 0x22 => Ok(Self::Mbc7SensorRumbleRamBattery),
            x if x == 0xFC => Ok(Self::PocketCamera),
            x if x == 0xFD => Ok(Self::BandaiTama5),
            x if x == 0xFE => Ok(Self::HuC3),
            x if x == 0xFF => Ok(Self::HuC1RamBattery),
            _ => Err(()),
        }
    }
}

enum RomSize {
    K32 = 0x00,
    K64 = 0x01,
    K128 = 0x02,
    K256 = 0x03,
    K512 = 0x04,
    M1 = 0x05,
    M2 = 0x06,
    M4 = 0x07,
    M8 = 0x08,
    M1_1 = 0x52,
    M1_2 = 0x53,
    M1_5 = 0x54,
}

impl std::fmt::Display for RomSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::K32 => write!(f, "32 KB"),
            Self::K64 => write!(f, "64 KB"),
            Self::K128 => write!(f, "128 KB"),
            Self::K256 => write!(f, "256 KB"),
            Self::K512 => write!(f, "512 KB"),
            Self::M1 => write!(f, "1 MB"),
            Self::M2 => write!(f, "2 MB"),
            Self::M4 => write!(f, "4 MB"),
            Self::M8 => write!(f, "8 MB"),
            Self::M1_1 => write!(f, "1.1 MB"),
            Self::M1_2 => write!(f, "1.2 MB"),
            Self::M1_5 => write!(f, "1.5 MB"),
        }
    }
}

impl TryFrom<u8> for RomSize {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            x if x == 0x00 => Ok(Self::K32),
            x if x == 0x01 => Ok(Self::K64),
            x if x == 0x02 => Ok(Self::K128),
            x if x == 0x03 => Ok(Self::K256),
            x if x == 0x04 => Ok(Self::K512),
            x if x == 0x05 => Ok(Self::M1),
            x if x == 0x06 => Ok(Self::M2),
            x if x == 0x07 => Ok(Self::M4),
            x if x == 0x08 => Ok(Self::M8),
            x if x == 0x52 => Ok(Self::M1_1),
            x if x == 0x53 => Ok(Self::M1_2),
            x if x == 0x54 => Ok(Self::M1_5),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy)]
enum RamSize {
    None = 0x00,
    K2 = 0x01,
    K8 = 0x02,
    K32 = 0x03,
    K128 = 0x04,
    K64 = 0x05,
}

impl std::fmt::Display for RamSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::K2 => write!(f, "2 KB"),
            Self::K8 => write!(f, "8 KB"),
            Self::K32 => write!(f, "32 KB"),
            Self::K128 => write!(f, "128 KB"),
            Self::K64 => write!(f, "64 KB"),
        }
    }
}

impl TryFrom<u8> for RamSize {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            x if x == 0x00 => Ok(Self::None),
            x if x == 0x01 => Ok(Self::K2),
            x if x == 0x02 => Ok(Self::K8),
            x if x == 0x03 => Ok(Self::K32),
            x if x == 0x04 => Ok(Self::K128),
            x if x == 0x05 => Ok(Self::K64),
            _ => Err(()),
        }
    }
}

pub struct Cartridge {
    pub cart_type: CartridgeType,
    pub rom: Vec<u8>,
    pub ram: Vec<u8>,
}

impl Cartridge {
    /// Create a new Cartridge
    pub fn new() -> Self {
        Cartridge {
            cart_type: CartridgeType::RomOnly,
            rom: vec![0xFF; 0x8000],
            ram: vec![0xFF; 0x2000],
        }
    }

    /// Create a new Cartridge from a ROM file
    pub fn new_from_rom(rom_name: &str) -> Self {
        let rom = std::fs::read(rom_name).expect("Failed to open ROM file");
        let cart_type = CartridgeType::try_from(rom[CART_TYPE_ADDR as usize]).unwrap();
        let rom_size = RomSize::try_from(rom[ROM_SIZE_ADDR as usize]).unwrap();
        let ram_size = RamSize::try_from(rom[RAM_SIZE_ADDR as usize]).unwrap();
        let ram = vec![0; ram_size as usize];

        log::info!("Loaded ROM from {}", rom_name);
        log::debug!("Title: {}", Self::get_title(&rom));
        log::debug!("Cartridge Type: {}", cart_type);
        log::debug!("ROM Size: {}", rom_size);
        log::debug!("RAM Size: {}\n", ram_size);

        Cartridge {
            cart_type,
            rom,
            ram,
        }
    }

    /// Get the title of the ROM from the header
    fn get_title(rom: &Vec<u8>) -> String {
        let mut title = String::new();
        for i in TITLE_START_ADDR..=TITLE_END_ADDR {
            let c = rom[i as usize];
            if c == 0 {
                break;
            }
            title.push(c as char);
        }
        title
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let cart = Cartridge::new();
        assert_eq!(cart.rom.len(), 0x8000);
        assert_eq!(cart.ram.len(), 0x2000);
    }

    #[test]
    fn test_new_from_rom() {
        let cart = Cartridge::new_from_rom("resources/test-rom.gb");
        assert_eq!(cart.rom.len(), 0x8000);
        assert_eq!(cart.ram.len(), 0);
    }
}
