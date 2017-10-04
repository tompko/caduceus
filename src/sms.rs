use cartridge::Cartridge;
use bus::Bus;
use vm::VM;

#[derive(Default)]
pub struct SMS {
    cartridge: Option<String>,
}

impl SMS {
    pub fn with_cartridge(mut self, cart: Option<&str>) -> Self {
        self.cartridge = match cart {
            Some(s) => Some(s.to_owned()),
            None => None,
        };

        self
    }

    pub fn build(self) -> VM {
        let input_file = self.cartridge.unwrap();
        let cartridge = Cartridge::load(&input_file).unwrap();

        let bus = Bus::new(cartridge);

        VM::new(bus)
    }
}
