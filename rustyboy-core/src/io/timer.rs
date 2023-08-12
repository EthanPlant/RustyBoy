use crate::util::binaryutils;

const DIV_CYCLES: u32 = 256;
const SPEED_ZERO_CYCLES: u32 = 1024;
const SPEED_ONE_CYCLES: u32 = 16;
const SPEED_TWO_CYCLES: u32 = 64;
const SPEED_THREE_CYCLES: u32 = 256;

pub const DIV_ADDR: usize = 0xFF04;
pub const TIMA_ADDR: usize = 0xFF05;
pub const TMA_ADDR: usize = 0xFF06;
pub const TAC_ADDR: usize = 0xFF07;

pub struct Timer {
    /// DIV register
    pub divider: u8,
    /// TIMA register
    pub counter: u8,
    /// TMA register
    pub modulo: u8,
    /// TAC register
    pub control: u8,
    /// Has an interrupt been fired
    pub interrupt_fired: bool,
    /// The amount of clock cycles that have passed since the last divider update
    clock_cycles_div: u32,
    /// The amount of clock cycles that have passed since the last TIMA update
    clock_cycles_tima: u32,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            divider: 0x18,
            counter: 0x00,
            modulo: 0x00,
            control: 0xF8,
            interrupt_fired: false,
            clock_cycles_div: 0,
            clock_cycles_tima: 0,
        }
    }

    pub fn step(&mut self, clock_cycles: u8) {
        // Update the divider register
        self.clock_cycles_div += clock_cycles as u32;
        if self.clock_cycles_div >= DIV_CYCLES {
            self.divider = self.divider.wrapping_add(1);
            self.clock_cycles_div -= DIV_CYCLES;
        }

        let speed = self.control << 6;
        if !binaryutils::is_bit_set(&self.control, 2) {
            return;
        }

        self.clock_cycles_tima += clock_cycles as u32;

        let timer_cycles = match speed {
            0x00 => SPEED_ZERO_CYCLES,
            0x40 => SPEED_ONE_CYCLES,
            0x80 => SPEED_TWO_CYCLES,
            0xC0 => SPEED_THREE_CYCLES,
            _ => panic!("Unknown timer speed! {:02X}", speed),
        };

        while self.clock_cycles_tima >= timer_cycles {
            if self.counter == 0xFF {
                self.counter = self.modulo;
                self.interrupt_fired = true;
                return;
            }

            self.counter += 1;
            self.clock_cycles_tima -= timer_cycles;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let timer = Timer::new();
        assert_eq!(timer.divider, 0x18);
        assert_eq!(timer.counter, 0x00);
        assert_eq!(timer.modulo, 0x00);
        assert_eq!(timer.control, 0xF8);
        assert_eq!(timer.interrupt_fired, false);
        assert_eq!(timer.clock_cycles_div, 0);
        assert_eq!(timer.clock_cycles_tima, 0);
    }

    #[test]
    fn test_step() {
        let mut timer = Timer::new();
        timer.step(4);
        assert_eq!(timer.divider, 0x18);
        assert_eq!(timer.counter, 0x00);
        assert_eq!(timer.modulo, 0x00);
        assert_eq!(timer.control, 0xF8);
        assert_eq!(timer.interrupt_fired, false);
        assert_eq!(timer.clock_cycles_div, 4);
        assert_eq!(timer.clock_cycles_tima, 0);
    }

    #[test]
    fn test_step_divider() {
        let mut timer = Timer::new();
        timer.step(0xFF);
        timer.step(0xFF);
        assert_eq!(timer.divider, 0x19);
        assert_eq!(timer.counter, 0x00);
        assert_eq!(timer.modulo, 0x00);
        assert_eq!(timer.control, 0xF8);
        assert_eq!(timer.interrupt_fired, false);
        assert_eq!(timer.clock_cycles_div, 0xFE);
        assert_eq!(timer.clock_cycles_tima, 0);
    }
}
