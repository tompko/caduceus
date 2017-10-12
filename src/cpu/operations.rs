use super::io::{Src8, Src16, Dst8, Dst16};
use super::operands::{Register8, Register16, Immediate8, Immediate16, Address, PortAddress, Condition, condition};

pub trait Operations {
    fn read_opcode(&mut self) -> u8;
    fn read_extended_opcode(&mut self) -> u8;

    fn load8<S: Src8, D: Dst8>(&mut self, dst: D, src: S);
    fn load16<S: Src16, D: Dst16>(&mut self, dst: D, src: S);
    fn push16<S: Src16>(&mut self, src: S);
    fn pop16<D: Dst16>(&mut self, dst: D);

    fn ldi(&mut self);
    fn ldir(&mut self);

    fn cp<S: Src8>(&mut self, src: S);

    fn disable_interrupts(&mut self);
    fn set_interrupt_mode(&mut self, interrupt_mode: u8);

    fn inc16(&mut self, r: Register16);

    fn jump<C: Condition>(&mut self, addr: Address, cond: C);
    fn jr<C: Condition>(&mut self, cond: C);
    fn call<C: Condition>(&mut self, addr: Address, cond: C);
    fn ret(&mut self);

    fn out<S: Src8>(&mut self, addr: PortAddress, src: S);
}

pub fn visit<O: Operations>(mut ops: O) {
    use self::Register8::*;
    use self::Register16::*;

    let opcode = ops.read_opcode();

    match opcode {
        // 8-bit load group
        0x3e => ops.load8(A, Immediate8),
        0x06 => ops.load8(B, Immediate8),
        0x0a => ops.load8(A, Address::BC),
        0x0e => ops.load8(C, Immediate8),
        0x16 => ops.load8(D, Immediate8),
        0x1a => ops.load8(A, Address::DE),
        0x1e => ops.load8(E, Immediate8),
        0x26 => ops.load8(H, Immediate8),
        0x2e => ops.load8(L, Immediate8),
        0x40 => ops.load8(B, B),
        0x41 => ops.load8(B, C),
        0x42 => ops.load8(B, D),
        0x43 => ops.load8(B, E),
        0x44 => ops.load8(B, H),
        0x45 => ops.load8(B, L),
        0x47 => ops.load8(B, A),
        0x48 => ops.load8(C, B),
        0x49 => ops.load8(C, C),
        0x4A => ops.load8(C, D),
        0x4B => ops.load8(C, E),
        0x4C => ops.load8(C, H),
        0x4D => ops.load8(C, L),
        0x4F => ops.load8(C, A),
        0x50 => ops.load8(D, B),
        0x51 => ops.load8(D, C),
        0x52 => ops.load8(D, D),
        0x53 => ops.load8(D, E),
        0x54 => ops.load8(D, H),
        0x55 => ops.load8(D, L),
        0x57 => ops.load8(D, A),
        0x58 => ops.load8(E, B),
        0x59 => ops.load8(E, C),
        0x5A => ops.load8(E, D),
        0x5B => ops.load8(E, E),
        0x5E => ops.load8(C, H),
        0x5D => ops.load8(E, L),
        0x5F => ops.load8(E, A),
        0x60 => ops.load8(H, B),
        0x61 => ops.load8(H, C),
        0x62 => ops.load8(H, D),
        0x63 => ops.load8(H, E),
        0x64 => ops.load8(H, H),
        0x65 => ops.load8(H, L),
        0x67 => ops.load8(H, A),
        0x68 => ops.load8(L, B),
        0x69 => ops.load8(L, C),
        0x6A => ops.load8(L, D),
        0x6B => ops.load8(L, E),
        0x6E => ops.load8(L, H),
        0x6D => ops.load8(L, L),
        0x6F => ops.load8(L, A),
        0x78 => ops.load8(A, B),
        0x79 => ops.load8(A, C),
        0x7A => ops.load8(A, D),
        0x7B => ops.load8(A, E),
        0x7E => ops.load8(A, H),
        0x7D => ops.load8(A, L),
        0x7F => ops.load8(A, A),

        // 16-bit load group
        0x01 => ops.load16(BC, Immediate16),
        0x11 => ops.load16(DE, Immediate16),
        0x21 => ops.load16(HL, Immediate16),
        0x31 => ops.load16(SP, Immediate16),
        0xc5 => ops.push16(BC),
        0xd5 => ops.push16(DE),
        0xe5 => ops.push16(HL),
        0xf5 => ops.push16(AF),
        0xc1 => ops.pop16(BC),
        0xd1 => ops.pop16(DE),
        0xe1 => ops.pop16(HL),
        0xf1 => ops.pop16(AF),

        // 8-bit arithmetic group
        0xb8 => ops.cp(B),
        0xb9 => ops.cp(C),
        0xba => ops.cp(D),
        0xbb => ops.cp(E),
        0xbc => ops.cp(H),
        0xbd => ops.cp(L),
        0xbf => ops.cp(A),
        0xfe => ops.cp(Immediate8),

        // General purpose arithmetic and CPU control group
        0xf3 => ops.disable_interrupts(),

        // 16-bit arithmetic group
        0x03 => ops.inc16(BC),
        0x13 => ops.inc16(DE),
        0x23 => ops.inc16(HL),
        0x33 => ops.inc16(SP),

        // jump and call group
        0xc3 => ops.jump(Address::Direct, ()),
        0xc2 => ops.jump(Address::Direct, condition::NON_ZERO),
        0xca => ops.jump(Address::Direct, condition::ZERO),
        0x18 => ops.jr(()),
        0x20 => ops.jr(condition::NON_ZERO),
        0x28 => ops.jr(condition::ZERO),
        0x30 => ops.jr(condition::NON_CARRY),
        0x38 => ops.jr(condition::CARRY),
        0xcc => ops.call(Address::Direct, condition::ZERO),
        0xc4 => ops.call(Address::Direct, condition::NON_ZERO),
        0xcd => ops.call(Address::Direct, ()),
        0xc9 => ops.ret(),

        // Input and output group
        0xd3 => ops.out(PortAddress::Immediate, A),

        // extended instructions
        0xed => visit_ed(ops),
        _ => panic!("Unrecognised opcode 0x{:02x}", opcode),
    }
}

pub fn visit_ed<O: Operations>(mut ops: O) {
    let opcode = ops.read_extended_opcode();

    match opcode {
        // Exchange, block transfer, and search group
        0xa0 => ops.ldi(),
        0xb0 => ops.ldir(),

        // General purpose arithmetic and CPU control group
        0x46 => ops.set_interrupt_mode(0),
        0x56 => ops.set_interrupt_mode(1),
        0x5E => ops.set_interrupt_mode(2),
        _ => panic!("Unrecognised extended ED opcode 0x{:02x}", opcode),
    }
}
