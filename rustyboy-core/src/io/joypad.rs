use crate::util::binaryutils::is_bit_set;

pub enum Key {
    A,
    B,
    Left,
    Right,
    Up,
    Down,
    Start,
    Select,
}

pub struct Joypad {
    direction_keys: u8,
    button_keys: u8,
}

impl Joypad {
    pub fn new() -> Self {
        Self {
            direction_keys: 0x0F,
            button_keys: 0x0F,
        }
    }

    pub fn push_key(&mut self, key: Key) {
        match key {
            Key::A => self.button_keys &= 0x01 ^ 0x0F,
            Key::B => self.button_keys &= 0x02 ^ 0x0F,
            Key::Select => self.button_keys &= 0x04 ^ 0x0F,
            Key::Start => self.button_keys &= 0x08 ^ 0x0F,
            Key::Right => self.direction_keys &= 0x01 ^ 0x0F,
            Key::Left => self.direction_keys &= 0x02 ^ 0x0F,
            Key::Up => self.direction_keys &= 0x04 ^ 0x0F,
            Key::Down => self.direction_keys &= 0x08 ^ 0x0F,
        }
    }

    pub fn release_key(&mut self, key: Key) {
        match key {
            Key::A => self.button_keys |= 0x01,
            Key::B => self.button_keys |= 0x02,
            Key::Select => self.button_keys |= 0x04,
            Key::Start => self.button_keys |= 0x08,
            Key::Right => self.direction_keys |= 0x01,
            Key::Left => self.direction_keys |= 0x02,
            Key::Up => self.direction_keys |= 0x04,
            Key::Down => self.direction_keys |= 0x08,
        }
    }

    pub fn read_value(&self, value: u8) -> u8 {
        if !is_bit_set(&value, 4) {
            return self.direction_keys;
        } else if !is_bit_set(&value, 5) {
            return self.button_keys;
        }

        0xFF
    }
}