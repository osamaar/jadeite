mod mapper_000;

pub use self::mapper_000::Mapper000;

use crate::CartData;

pub trait Mapper {
    fn id(&self) -> u16;
    fn name(&self) -> String;
    fn cpu_read(&self, cart: &CartData, addr: u16) -> u8;
    fn cpu_write(&self, cart: &mut CartData, addr: u16, value: u8);
    fn ppu_read(&self, cart: &CartData, addr: u16) -> u8;
    fn ppu_write(&self, cart: &mut CartData, addr: u16, value: u8);
}