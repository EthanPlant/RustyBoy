/// The system clock

/// The amount of clock cycles per frame
pub const CYCLES_PER_FRAME: u32 = 70221;

pub struct Clock {
    /// The amount of CPU clock cycles that have passed since the emulator started running
    pub clock_cycles_passed: u32,
    /// The amount of machine cycles that have passed since the emulator started running
    pub machine_cycles_passed: u32,
}

impl Clock {
    /// Create a new clock
    pub fn new() -> Self {
        Clock {
            clock_cycles_passed: 0,
            machine_cycles_passed: 0,
        }
    }

    /// Add cycles to the clock
    pub fn cycle(&mut self, cycles: u8) {
        self.clock_cycles_passed += cycles as u32;
        self.machine_cycles_passed += (cycles / 4) as u32;
    }

    /// Reset the clock
    pub fn reset(&mut self) {
        self.clock_cycles_passed = 0;
        self.machine_cycles_passed = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let clock = Clock::new();
        assert_eq!(clock.clock_cycles_passed, 0);
        assert_eq!(clock.machine_cycles_passed, 0);
    }

    #[test]
    fn test_cycle() {
        let mut clock = Clock::new();
        clock.cycle(4);
        assert_eq!(clock.clock_cycles_passed, 4);
        assert_eq!(clock.machine_cycles_passed, 1);
    }

    #[test]
    fn test_reset() {
        let mut clock = Clock::new();
        clock.cycle(4);
        clock.reset();
        assert_eq!(clock.clock_cycles_passed, 0);
        assert_eq!(clock.machine_cycles_passed, 0);
    }
}
