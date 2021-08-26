use crate::Cart;

mod mapper_000;

pub use mapper_000::Mapper000;

pub trait Mapper {
    fn id(&self) -> u16;
    fn name(&self) -> String;
    fn cpu_read(&self, cart: &Cart, addr: u16) -> u8;
    fn cpu_write(&self, cart: &mut Cart, addr: u16, value: u8);
    fn ppu_read(&self, cart: &Cart, addr: u16) -> u8;
    fn ppu_write(&self, cart: &mut Cart, addr: u16, value: u8);
}