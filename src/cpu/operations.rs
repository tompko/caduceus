use super::io::{Src16, Dst16};
use super::operands::{Register16, Immediate16};

pub trait Operations {
    fn read_opcode(&mut self) -> u8;
    fn read_extended_opcode(&mut self) -> u8;

    fn load16<S: Src16, D: Dst16>(&mut self, dst: D, src: S);

    fn disable_interrupts(&mut self);
    fn set_interrupt_mode(&mut self, interrupt_mode: u8);
}

pub fn visit<O: Operations>(mut ops: O) {
    use self::Register16::*;

    let opcode = ops.read_opcode();

    match opcode {
        // 16-bit load group
        0x01 => ops.load16(BC, Immediate16),
        0x11 => ops.load16(DE, Immediate16),
        0x21 => ops.load16(HL, Immediate16),
        0x31 => ops.load16(SP, Immediate16),

        0xed => visit_ed(ops),
        0xf3 => ops.disable_interrupts(),
        _ => panic!("Unrecognised opcode 0x{:02x}", opcode),
    }
}

pub fn visit_ed<O: Operations>(mut ops: O) {
    let opcode = ops.read_extended_opcode();

    match opcode {
        0x46 => ops.set_interrupt_mode(0),
        0x56 => ops.set_interrupt_mode(1),
        0x5E => ops.set_interrupt_mode(2),
        _ => panic!("Unrecognised extended ED opcode 0x{:02x}", opcode),
    }
}
