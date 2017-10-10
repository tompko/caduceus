use super::io::{Src8, Src16, Dst8, Dst16};
use super::state::State;
use super::super::bus::Bus;

#[derive(Debug, Clone, Copy)]
pub enum Register8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

impl Src8 for Register8 {
    fn src8(&self, state: &mut State, _: &mut Bus) -> u8 {
        use self::Register8::*;

        match *self {
            A => state.a,
            B => state.b,
            C => state.c,
            D => state.d,
            E => state.e,
            H => state.h,
            L => state.l,
        }
    }
}

impl Dst8 for Register8 {
    fn dst8(&self, state: &mut State, _: &mut Bus, val: u8) {
        use self::Register8::*;

        match *self {
            A => state.a = val,
            B => state.b = val,
            C => state.c = val,
            D => state.d = val,
            E => state.e = val,
            H => state.h = val,
            L => state.l = val,
        }
    }
}


#[derive(Debug, Clone, Copy)]
pub enum Register16 {
    AF,
    BC,
    DE,
    HL,
    SP,
}

impl Src16 for Register16 {
    fn src16(&self, state: &mut State, _: &mut Bus) -> u16 {
        use self::Register16::*;

        match *self {
            AF => ((state.a as u16) << 8) | (state.f as u16),
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
            AF => {
                state.a = (value >> 8) as u8;
                state.f = value as u8;
            }
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
pub struct Immediate8;

impl Src8 for Immediate8 {
    fn src8(&self, state: &mut State, bus: &mut Bus) -> u8 {
        state.next8(bus)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Immediate16;

impl Src16 for Immediate16 {
    fn src16(&self, state: &mut State, bus: &mut Bus) -> u16 {
        state.next16(bus)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Address {
    Direct,
    // Relative,

    ZeroPage,

    // HL,
    // IX,
    // IY,
}

impl Address {
    pub fn indirect(&self, state: &mut State, bus: &mut Bus) -> u16 {
        use self::Address::*;

        match *self {
            Direct => state.next16(bus),
            ZeroPage => state.next8(bus) as u16,
        }
    }
}

pub enum PortAddress {
    Immediate,
    Indirect,
}

impl PortAddress {
    pub fn indirect(&self, state: &mut State, bus: &mut Bus) -> u8 {
        use self::PortAddress::*;

        match *self {
            Immediate => state.next8(bus),
            Indirect => state.c,
        }
    }
}
