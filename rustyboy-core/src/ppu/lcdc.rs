#[derive(Copy, Clone)]
pub struct Lcdc {
    /// Bit 0 - BG and Window Display (0=Off, 1=On)
    pub background_enabled: bool,
    /// Bit 1 - OBJ (Sprite) Display Enable (0=Off, 1=On)
    pub objects_enabled: bool,
    /// Bit 2 - OBJ (Sprite) Size (0=8x8, 1=8x16)
    pub objects_size: bool,
    /// Bit 3 - BG Tile Map Display Select (0=9800-9BFF, 1=9C00-9FFF)
    pub background_tile_map: bool,
    /// Bit 4 - BG & Window Tile Data Select (0=8800-97FF, 1=8000-8FFF)
    pub background_tile_data: bool,
    /// Bit 5 - Window Display Enable (0=Off, 1=On)
    pub window_enabled: bool,
    /// Bit 6 - Window Tile Map Display Select (0=9800-9BFF, 1=9C00-9FFF)
    pub window_tile_map: bool,
    /// Bit 7 - LCD Display Enable (0=Off, 1=On)
    pub enabled: bool,
}

impl Lcdc {
    pub fn new() -> Self {
        Lcdc {
            background_enabled: true,
            objects_enabled: false,
            objects_size: false,
            background_tile_map: false,
            background_tile_data: true,
            window_enabled: false,
            window_tile_map: false,
            enabled: true,
        }
    }

    pub fn set(&mut self, value: u8) {
        self.background_enabled = value & 0b0000_0001 != 0;
        self.objects_enabled = value & 0b0000_0010 != 0;
        self.objects_size = value & 0b0000_0100 != 0;
        self.background_tile_map = value & 0b0000_1000 != 0;
        self.background_tile_data = value & 0b0001_0000 != 0;
        self.window_enabled = value & 0b0010_0000 != 0;
        self.window_tile_map = value & 0b0100_0000 != 0;
        self.enabled = value & 0b1000_0000 != 0;
    }
}

impl Into<u8> for Lcdc {
    fn into(self) -> u8 {
        let mut value = 0;
        if self.background_enabled {
            value |= 0b0000_0001;
        }
        if self.objects_enabled {
            value |= 0b0000_0010;
        }
        if self.objects_size {
            value |= 0b0000_0100;
        }
        if self.background_tile_map {
            value |= 0b0000_1000;
        }
        if self.background_tile_data {
            value |= 0b0001_0000;
        }
        if self.window_enabled {
            value |= 0b0010_0000;
        }
        if self.window_tile_map {
            value |= 0b0100_0000;
        }
        if self.enabled {
            value |= 0b1000_0000;
        }
        value
    }
}
