pub trait Operations {
    fn read_opcode(&mut self) -> u8;
}

pub fn visit<O: Operations>(mut ops: O) {
    let opcode = ops.read_opcode();

    match opcode {
        _ => panic!("Unrecognised opcode 0x{:02x}", opcode),
    }
}
