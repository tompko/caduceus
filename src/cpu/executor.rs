use super::operations::Operations;
use super::state::State;
use super::super::bus::Bus;

pub struct Executor<'a> (pub &'a mut State, pub &'a mut Bus);

impl<'a> Executor<'a> {
}

impl<'a> Operations for Executor<'a> {
    fn read_opcode(&mut self) -> u8 {
        let pc = self.0.pc;
        self.0.pc += 1;
        self.1.read_u8(pc)
    }
}
