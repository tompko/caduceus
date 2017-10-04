use super::operations::Operations;
use super::state::State;
use super::super::bus::Bus;

pub struct Executor<'a> (pub &'a mut State, pub &'a mut Bus);

impl<'a> Executor<'a> {
}

// TODO - timings
impl<'a> Operations for Executor<'a> {
    fn read_opcode(&mut self) -> u8 {
        let pc = self.0.pc;
        self.0.pc += 1;
        self.1.read_u8(pc)
    }

    fn read_extended_opcode(&mut self) -> u8 {
        let pc = self.0.pc;
        self.0.pc += 1;
        self.1.read_u8(pc)
    }

    fn disable_interrupts(&mut self) {
        self.0.iff1 = false;
        self.0.iff2 = false;
    }

    fn set_interrupt_mode(&mut self, interrupt_mode: u8) {
        self.0.interrupt_mode = interrupt_mode;
    }
}
