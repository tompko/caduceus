use super::super::bus::Bus;
use super::State;

pub trait Src8: Copy {
    fn src8(&self, state: &mut State, bus: &mut Bus) -> u8;
}

pub trait Dst8: Copy {
    fn dst8(&self, state: &mut State, bus: &mut Bus, value: u8);
}

pub trait Src16: Copy {
    fn src16(&self, state: &mut State, bus: &mut Bus) -> u16;
}

pub trait Dst16: Copy {
    fn dst16(&self, state: &mut State, bus: &mut Bus, value: u16);
}
