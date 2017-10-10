use super::io::{Src8, Src16, Dst8, Dst16};
use super::operands::{Register8, Register16, Immediate8, Immediate16, Address, PortAddress};

pub trait Operations {
    fn read_opcode(&mut self) -> u8;
    fn read_extended_opcode(&mut self) -> u8;

    fn load8<S: Src8, D: Dst8>(&mut self, dst: D, src: S);
    fn load16<S: Src16, D: Dst16>(&mut self, dst: D, src: S);
    fn push16<S: Src16>(&mut self, src: S);

    fn ldi(&mut self);
    fn ldir(&mut self);

    fn disable_interrupts(&mut self);
    fn set_interrupt_mode(&mut self, interrupt_mode: u8);

    fn jump(&mut self, addr: Address);
    fn call(&mut self, addr: Address);

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
        0x0e => ops.load8(C, Immediate8),
        0x16 => ops.load8(D, Immediate8),
        0x1e => ops.load8(E, Immediate8),
        0x26 => ops.load8(H, Immediate8),
        0x2e => ops.load8(L, Immediate8),

        // 16-bit load group
        0x01 => ops.load16(BC, Immediate16),
        0x11 => ops.load16(DE, Immediate16),
        0x21 => ops.load16(HL, Immediate16),
        0x31 => ops.load16(SP, Immediate16),
        0xc5 => ops.push16(BC),
        0xd5 => ops.push16(DE),
        0xe5 => ops.push16(HL),
        0xf5 => ops.push16(AF),

        // General purpose arithmetic and CPU control group
        0xf3 => ops.disable_interrupts(),

        // jump and call group
        0xc3 => ops.jump(Address::Direct),
        0xcd => ops.call(Address::Direct),

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
