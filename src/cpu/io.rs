use super::super::bus::Bus;
use super::State;

pub trait In8: Copy {
    fn read8<B: Bus>(&self, state: &mut State, bus: &mut B) -> u8;
}

pub trait Out8: Copy {
    fn write8<B: Bus>(&self, state: &mut State, bus: &mut B, value: u8);
}

pub trait In16: Copy {
    fn read16<B: Bus>(&self, state: &mut State, bus: &mut B) -> u16;
}

pub trait Out16: Copy {
    fn write16<B: Bus>(&self, state: &mut State, bus: &mut B, value: u16);
}
