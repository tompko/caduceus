use cartridge::Cartridge;

pub struct Bus {
    cart: Cartridge,
}

impl Bus {
    pub fn new(cart: Cartridge) -> Self {
        Bus {
            cart: cart,
        }
    }

    pub fn read_u8(&self, addr: u16) -> u8 {
        match addr {
            0...0xbfff => self.cart.read_u8(addr),
            _ => panic!("Read from unrecognised address 0x{:04x}", addr),
        }
    }
}
