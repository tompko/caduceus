use super::io::{Src8, Src16, Dst8, Dst16};
use super::operands::{Register16, Address, PortAddress, Condition};
use super::operations::Operations;
use super::state::{State, Flags};
use super::super::bus::Bus;

pub struct Executor<'a> (pub &'a mut State, pub &'a mut Bus);

// TODO - timings
impl<'a> Operations for Executor<'a> {
    fn read_opcode(&mut self) -> u8 {
        let pc = self.0.pc;
        self.0.pc += 1;
        let op = self.1.read8(pc);

        // println!("{:04x} {:02x}", pc, op);

        op
    }

    fn read_extended_opcode(&mut self) -> u8 {
        let pc = self.0.pc;
        self.0.pc += 1;
        self.1.read8(pc)
    }

    fn load8<S: Src8, D: Dst8>(&mut self, dst: D, src: S) {
        let val = src.src8(self.0, self.1);
        dst.dst8(self.0, self.1, val);
    }

    fn load16<S: Src16, D: Dst16>(&mut self, dst: D, src: S) {
        let val = src.src16(self.0, self.1);
        dst.dst16(self.0, self.1, val);
    }

    fn push16<S: Src16>(&mut self, src: S) {
        let val = src.src16(self.0, self.1);
        self.0.push16(self.1, val);
    }

    fn pop16<D: Dst16>(&mut self, dst: D) {
        let val = self.0.pop16(self.1);
        dst.dst16(self.0, self.1, val);
    }

    fn ldi(&mut self) {
        // (DE) ← (HL), DE ← DE + 1, HL ← HL + 1, BC ← BC – 1
        let src_addr = self.0.hl();
        let val = self.1.read8(src_addr);
        let dst_addr = self.0.de();
        let bc = self.0.bc();

        self.1.write8(dst_addr, val);

        self.0.set_de(dst_addr.wrapping_add(1));
        self.0.set_hl(src_addr.wrapping_add(1));
        self.0.set_bc(bc.wrapping_sub(1));
    }

    fn ldir(&mut self) {
        self.ldi();

        if self.0.bc() != 0 {
            self.0.pc = self.0.pc.wrapping_sub(2);
        }
    }

    fn cp<S: Src8>(&mut self, src:S) {
        let val = src.src8(self.0, self.1);

        self.subc_impl(val, false);
    }

    fn disable_interrupts(&mut self) {
        self.0.iff1 = false;
        self.0.iff2 = false;
    }

    fn set_interrupt_mode(&mut self, interrupt_mode: u8) {
        self.0.interrupt_mode = interrupt_mode;
    }

    fn inc16(&mut self, r: Register16) {
        let val = r.src16(self.0, self.1);
        r.dst16(self.0, self.1, val.wrapping_add(1));
    }

    fn jump<C: Condition>(&mut self, addr: Address, cond: C) {
        let addr = addr.indirect(self.0, self.1);
        if cond.check(self.0) {
            self.0.pc = addr;
        }
    }

    fn jr<C: Condition>(&mut self, c: C) {
        let offset = self.0.next8(self.1);

        if c.check(self.0) {
            self.0.pc = self.0.pc.wrapping_add(offset as i8 as u16);
        }
    }

    fn call<C: Condition>(&mut self, addr: Address, cond: C) {
        let addr = addr.indirect(self.0, self.1);

        if cond.check(self.0) {
            let pc = self.0.pc;

            self.0.push16(self.1, pc);

            self.0.pc = addr;
        }
    }

    fn ret(&mut self) {
        let pc = self.0.pop16(self.1);
        self.0.pc = pc;
    }

    fn out<S: Src8>(&mut self, addr: PortAddress, src: S) {
        let val = src.src8(self.0, self.1);
        let addr = addr.indirect(self.0, self.1);

        self.1.out8(addr, val);
    }
}

impl<'a> Executor<'a> {
    fn subc_impl(&mut self, val: u8, carry: bool) -> u8 {
        let carry = if carry { 1 } else { 0 };
        let (tmp, underflow) = self.0.a.overflowing_sub(val);
        let (r, underflow_c) = tmp.overflowing_sub(carry);

        let a_sign = self.0.a >> 7;
        let v_sign = val >> 7;
        let r_sign = self.0.r >> 7;

        self.0.f.set(Flags::S, r_sign == 1);
        self.0.f.set(Flags::H, ((val & 0xf) + carry) > (self.0.a & 0xf));
        self.0.f.set(Flags::Z, r == 0);
        self.0.f.set(Flags::N, true);
        self.0.f.set(Flags::P, (a_sign == v_sign) && (a_sign != r_sign));
        self.0.f.set(Flags::C, underflow || underflow_c);

        r
    }
}
