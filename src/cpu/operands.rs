use super::io::{Src16, Dst16};
use super::state::State;
use super::super::bus::Bus;

#[derive(Debug, Clone, Copy)]
pub enum Register16 {
    BC,
    DE,
    HL,
    SP,
}

impl Src16 for Register16 {
    fn src16(&self, state: &mut State, _: &mut Bus) -> u16 {
        use self::Register16::*;

        match *self {
            BC => ((state.b as u16) << 8) | (state.c as u16),
            DE => ((state.d as u16) << 8) | (state.e as u16),
            HL => ((state.h as u16) << 8) | (state.l as u16),
            SP => state.sp,
        }
    }
}

impl Dst16 for Register16 {
    fn dst16(&self, state: &mut State, _: &mut Bus, value: u16) {
        use self::Register16::*;

        match *self {
            BC => {
                state.b = (value >> 8) as u8;
                state.c = value as u8;
            },
            DE => {
                state.d = (value >> 8) as u8;
                state.e = value as u8;
            },
            HL => {
                state.h = (value >> 8) as u8;
                state.l = value as u8;
            },
            SP => state.sp = value,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Immediate16;

impl Src16 for Immediate16 {
    fn src16(&self, state: &mut State, bus: &mut Bus) -> u16 {
        state.next16(bus)
    }
}
