use crate::cpu::cpu::Cpu;
use crate::mmu::Memory;

pub const INTERRUPT_ENABLE_ADDR: usize = 0xFFFF;
pub const INTERRUPT_FLAG_ADDR: usize = 0xFF0F;

const VBLANK_ISR: u16 = 0x0040;
const LCD_STAT_ISR: u16 = 0x0048;
const TIMER_ISR: u16 = 0x0050;
const SERIAL_ISR: u16 = 0x0058;
const JOYPAD_ISR: u16 = 0x0060;

/// The types of interrupts the Gameboy can handle
#[derive(Copy, Clone, Debug)]
pub enum Interrupt {
    VBlank = 0x01,
    LcdStat = 0x02,
    Timer = 0x04,
    Serial = 0x08,
    Joypad = 0x10,
}

/// The state of the interrupt registers at a given time
pub struct InterruptState {
    pub enabled_interrupts: u8,
    pub requested_interrupts: u8,
}

impl InterruptState {
    pub fn new() -> Self {
        InterruptState {
            enabled_interrupts: 0x00,
            requested_interrupts: 0xE1,
        }
    }

    /// Clear the flag for a given interrupt
    pub fn clear_interrupt(&mut self, interrupt: &Interrupt) {
        let interrupt_value = *interrupt as u8;
        self.requested_interrupts &= interrupt_value ^ 0xFF;
    }

    pub fn interrupt_fired(&self, interrupt: &Interrupt) -> bool {
        let interrupt_value = *interrupt as u8;

        self.requested_interrupts & interrupt_value == interrupt_value
            && self.enabled_interrupts & interrupt_value == interrupt_value
    }
}

/// Check if there are any pending interrupts
pub fn pending_interrupt(mmu: &Memory) -> bool {
    let interrupt_flag = mmu.get_byte(INTERRUPT_FLAG_ADDR);
    let interrupt_enable = mmu.get_byte(INTERRUPT_ENABLE_ADDR);

    (interrupt_flag & interrupt_enable) != 0
}

/// Handle all interrupts
pub fn handle_interrupts(cpu: &mut Cpu, mmu: &mut Memory) -> Option<u8> {
    if handle_interrupt(cpu, mmu, &Interrupt::VBlank, VBLANK_ISR) {
        return Some(12);
    }

    if handle_interrupt(cpu, mmu, &Interrupt::LcdStat, LCD_STAT_ISR) {
        return Some(12);
    }

    if handle_interrupt(cpu, mmu, &Interrupt::Timer, TIMER_ISR) {
        return Some(12);
    }

    if handle_interrupt(cpu, mmu, &Interrupt::Joypad, JOYPAD_ISR) {
        return Some(12);
    }

    if handle_interrupt(cpu, mmu, &Interrupt::Serial, SERIAL_ISR) {
        return Some(12);
    }

    return None;
}

/// Handle a specific interrupt
fn handle_interrupt(cpu: &mut Cpu, mmu: &mut Memory, interrupt: &Interrupt, isr_addr: u16) -> bool {
    if !mmu.interrupts.interrupt_fired(interrupt) {
        return false;
    }

    log::trace!("Handling {:?} interrupt", interrupt);

    cpu.ime = false;
    cpu.reg.sp -= 2;
    mmu.set_word(cpu.reg.sp, cpu.reg.pc);
    cpu.reg.pc = isr_addr;
    mmu.interrupts.clear_interrupt(interrupt);
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let interrupt_state = InterruptState::new();
        assert_eq!(interrupt_state.enabled_interrupts, 0x00);
        assert_eq!(interrupt_state.requested_interrupts, 0xE1);
    }

    #[test]
    fn test_clear_interrupt() {
        let mut interrupt_state = InterruptState::new();
        interrupt_state.requested_interrupts = 0xFF;
        interrupt_state.clear_interrupt(&Interrupt::VBlank);
        assert_eq!(interrupt_state.requested_interrupts, 0xFE);
    }

    #[test]
    fn test_interrupt_fired() {
        let mut interrupt_state = InterruptState::new();
        interrupt_state.requested_interrupts = 0xFF;
        interrupt_state.enabled_interrupts = 0xFF;
        assert_eq!(interrupt_state.interrupt_fired(&Interrupt::VBlank), true);
    }

    #[test]
    fn test_interrupt_not_fired() {
        let mut interrupt_state = InterruptState::new();
        interrupt_state.requested_interrupts = 0xFF;
        interrupt_state.enabled_interrupts = 0x00;
        assert_eq!(interrupt_state.interrupt_fired(&Interrupt::VBlank), false);
    }

    #[test]
    fn test_handle_interrupts() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        mmu.interrupts.requested_interrupts = 0xFF;
        mmu.interrupts.enabled_interrupts = 0xFF;
        handle_interrupts(&mut cpu, &mut mmu);
        assert_eq!(cpu.ime, false);
        assert_eq!(cpu.reg.sp, 0xFFFC);
        assert_eq!(cpu.reg.pc, 0x0040);
        assert_eq!(mmu.interrupts.requested_interrupts, 0xFE);
    }

    #[test]
    fn test_handle_interrupts_no_interrupts() {
        let mut cpu = Cpu::new();
        let mut mmu = Memory::new();
        mmu.interrupts.requested_interrupts = 0x00;
        mmu.interrupts.enabled_interrupts = 0x00;
        handle_interrupts(&mut cpu, &mut mmu);
        assert_eq!(cpu.ime, false);
        assert_eq!(cpu.reg.sp, 0xFFFE);
        assert_eq!(cpu.reg.pc, 0x0100);
        assert_eq!(mmu.interrupts.requested_interrupts, 0x00);
    }
}
