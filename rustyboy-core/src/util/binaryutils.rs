pub fn is_bit_set(byte: &u8, bit: u8) -> bool {
    (byte & (1 << bit)) != 0
}
