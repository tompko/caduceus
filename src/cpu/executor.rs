use super::io::{Src16, Dst16};
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
        self.1.read8(pc)
    }

    fn load16<S: Src16, D: Dst16>(&mut self, dst: D, src: S) {
        let val = src.src16(self.0, self.1);
        dst.dst16(self.0, self.1, val);
    }

    fn read_extended_opcode(&mut self) -> u8 {
        let pc = self.0.pc;
        self.0.pc += 1;
        self.1.read8(pc)
    }

    fn disable_interrupts(&mut self) {
        self.0.iff1 = false;
        self.0.iff2 = false;
    }

    fn set_interrupt_mode(&mut self, interrupt_mode: u8) {
        self.0.interrupt_mode = interrupt_mode;
    }
}
