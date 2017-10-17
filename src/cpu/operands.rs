use super::io::{Src8, Src16, Dst8, Dst16};
use super::state::{State, Flags};
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
            AF => ((state.a as u16) << 8) | (state.f.bits() as u16),
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
                state.f = Flags::from_bits(value as u8).unwrap();
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
    ImmediateExtended,

    ZeroPage,

    BC,
    DE,
    HL,
    // IX,
    // IY,
}

impl Address {
    pub fn indirect(&self, state: &mut State, bus: &mut Bus) -> u16 {
        use self::Address::*;

        match *self {
            Direct => state.next16(bus),
            ZeroPage => state.next8(bus) as u16,
            ImmediateExtended => state.next16(bus),
            BC => state.bc(),
            DE => state.de(),
            HL => state.hl(),
        }
    }
}

impl Src8 for Address {
    fn src8(&self, state: &mut State, bus: &mut Bus) -> u8 {
        let addr = self.indirect(state, bus);

        bus.read8(addr)
    }
}

impl Dst8 for Address {
    fn dst8(&self, state: &mut State, bus: &mut Bus, val: u8) {
        let addr = self.indirect(state, bus);

        bus.write8(addr, val);
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

pub trait Condition {
    fn check(&self, state: &State) -> bool;
}

impl Condition for () {
    fn check(&self, _: &State) -> bool {
        true
    }
}

pub mod condition {
    #![allow(non_camel_case_types)]
    use super::{Flags, State, Condition};

    pub struct CARRY;

    impl Condition for CARRY {
        fn check(&self, state: &State) -> bool {
            state.f.contains(Flags::C)
        }
    }

    pub struct NON_CARRY;

    impl Condition for NON_CARRY {
        fn check(&self, state: &State) -> bool {
            !state.f.contains(Flags::C)
        }
    }

    pub struct ZERO;

    impl Condition for ZERO {
        fn check(&self, state: &State) -> bool {
            state.f.contains(Flags::Z)
        }
    }

    pub struct NON_ZERO;

    impl Condition for NON_ZERO {
        fn check(&self, state: &State) -> bool {
            !state.f.contains(Flags::Z)
        }
    }

}
