use super::super::bus::Bus;

pub struct State {
    pub a: u8,
    pub f: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,

    pub a_: u8,
    pub f_: u8,
    pub b_: u8,
    pub c_: u8,
    pub d_: u8,
    pub e_: u8,
    pub h_: u8,
    pub l_: u8,

    pub i: u8,
    pub r: u8,
    pub ix: u16,
    pub iy: u16,
    pub sp: u16,
    pub pc: u16,

    // Interrupt enable flip-flops
    pub iff1: bool,
    pub iff2: bool,
    pub interrupt_mode: u8,
}

impl Default for State {
    fn default() -> State {
        State {
            // TODO - check initial values
            a: 0,
            f: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,

            a_: 0,
            f_: 0,
            b_: 0,
            c_: 0,
            d_: 0,
            e_: 0,
            h_: 0,
            l_: 0,

            i: 0,
            r: 0,
            ix: 0,
            iy: 0,
            sp: 0,
            pc: 0,

            iff1: false,
            iff2: false,
            interrupt_mode: 0,
        }
    }
}

impl State {
    pub fn bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }

    pub fn set_bc(&mut self, val: u16) {
        self.b = (val >> 8) as u8;
        self.c = val as u8;
    }

    pub fn de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }

    pub fn set_de(&mut self, val: u16) {
        self.d = (val >> 8) as u8;
        self.e = val as u8;
    }

    pub fn hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }

    pub fn set_hl(&mut self, val: u16) {
        self.h = (val >> 8) as u8;
        self.l = val as u8;
    }

    pub fn next8(&mut self, bus: &Bus) -> u8 {
        let addr = self.pc;
        self.pc = self.pc.wrapping_add(1);

        bus.read8(addr)
    }

    pub fn next16(&mut self, bus: &Bus) -> u16 {
        let address = self.pc;
        self.pc = self.pc.wrapping_add(2);

        let lb = bus.read8(address) as u16;
        let hb = bus.read8(address+1) as u16;

        (hb << 8) | lb
    }

    pub fn push8(&mut self, bus: &mut Bus, val: u8) {
        self.sp = self.sp.wrapping_sub(1);

        bus.write8(self.sp, val);
    }

    pub fn pop8(&mut self, bus: &mut Bus) -> u8 {
        let val = bus.read8(self.sp);
        self.sp = self.sp.wrapping_add(1);

        val
    }

    pub fn push16(&mut self, bus: &mut Bus, val: u16) {
        let lsb = val as u8;
        let msb = (val >> 8) as u8;

        self.push8(bus, msb);
        self.push8(bus, lsb);
    }

    pub fn pop16(&mut self, bus: &mut Bus) -> u16 {
        let lsb = self.pop8(bus) as u16;
        let msb = self.pop8(bus) as u16;

        (msb << 8) | lsb
    }
}
