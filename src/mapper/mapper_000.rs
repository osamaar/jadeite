use core::panic;

use super::Mapper;
use crate::Cart;

pub struct Mapper000 {

}

impl Mapper000 {
    pub fn new() -> Self { Self {  } }
}


impl Mapper for Mapper000 {
    fn cpu_read(&self, cart: &Cart, addr: u16) -> u8 {
        let mask = match cart.prg_rom_page_count {
            2 => 0x7fff,        // 16kb
            1 => 0x3fff,        // 32kb
            _ => panic!(
                "Mapping failed: {} PRG pages not supported by this mapper",
                cart.prg_rom_page_count
            ),
        };

        cart.prg_rom[(addr & mask) as usize]
    }

    fn cpu_write(&self, _cart: &mut Cart, _addr: u16, _value: u8) {

    }

    fn ppu_read(&self, cart: &Cart, addr: u16) -> u8 {
        cart.chr_rom[(addr & 0x1fff) as usize]
    }

    fn ppu_write(&self, cart: &mut Cart, addr: u16, value: u8) {
        if cart.chr_rom_page_count == 0 {
            cart.chr_ram[(addr & 0x1fff) as usize] = value;
        }
    }

    fn id(&self) -> u16 {
        0
    }

    fn name(&self) -> String {
        "NROM".to_owned()
    }
}