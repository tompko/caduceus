pub trait Operations {
    fn read_opcode(&mut self) -> u8;
    fn read_extended_opcode(&mut self) -> u8;

    fn disable_interrupts(&mut self);
    fn set_interrupt_mode(&mut self, interrupt_mode: u8);
}

pub fn visit<O: Operations>(mut ops: O) {
    let opcode = ops.read_opcode();

    match opcode {
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
