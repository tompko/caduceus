use std::io::{self, Read};
use std::fs::File;
use std::path::Path;

pub struct Cartridge {
    rom: Box<[u8]>,
}

impl Cartridge {
    pub fn load<P: AsRef<Path>>(file_name: P) -> io::Result<Cartridge> {
        let mut file = File::open(file_name)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        let cart = Cartridge::from_bytes(&buffer);

        println!("Loaded {:0x} bytes of cart", buffer.len());

        Ok(cart)
    }

    pub fn from_bytes(bytes: &[u8]) -> Cartridge {
        let bytes_copy = bytes.to_vec();

        Cartridge {
            rom: bytes_copy.into_boxed_slice(),
        }
    }

    pub fn read_u8(&self, addr: u16) -> u8 {
        self.rom[addr as usize]
    }
}
