use cartridge::Cartridge;

pub struct Bus {
    cart: Cartridge,
    ram: Box<[u8]>,
}

impl Bus {
    pub fn new(cart: Cartridge) -> Self {
        Bus {
            cart: cart,
            ram: vec![0; 0x2000].into_boxed_slice(),
        }
    }

    pub fn read8(&self, addr: u16) -> u8 {
        match addr {
            0...0xbfff => self.cart.read_u8(addr),
            0xc000...0xdfff => self.ram[addr as usize - 0xc000],
            _ => panic!("Read from unrecognised address 0x{:04x}", addr),
        }
    }

    pub fn write8(&mut self, addr: u16, val: u8) {
        match addr {
            0xc000...0xdfff => self.ram[addr as usize - 0xc000] = val,
            _ => panic!("Write to unrecognised address 0x{:04x}", addr),
        }
    }

    pub fn out8(&mut self, addr: u8, val: u8) {
        match addr {
            0xfd => print!("{}", val as char),
            _ => println!("Write to port {:02x} = {:02x}", addr, val),
        }
    }
}
