use std::fmt::{Debug, Write};

use crate::Cart;

pub struct Bus <'a> {
    ram: Box<[u8]>,
    cart: Option<&'a mut Cart>,
}

impl <'a> Bus <'a> {
    pub fn new() -> Self {
        Self {
            ram: vec![0u8; 0x800].into_boxed_slice(),
            cart: None,
        }
    }

    pub fn attach_cart(&mut self, cart: &'a mut Cart) {
        self.cart = Some(cart);
    }

    pub fn read(&self, _addr: u16) -> u8 {
        0
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        match addr {
            0x00..=0xff => {
                self.ram[addr as usize] = val;
            },
            _ => {
                todo!("Implement full memory map");
            }
        }
    }

    pub fn print_ram<T: Write>(&self, target: &mut T) -> std::fmt::Result {
        write!(target, "{:6}", " ")?;

        for col in 0..0x10 {
            write!(target, "{:2}{:#04X}", " ", col)?;
        }

        write!(target, "\n")?;
        write!(target, "\n")?;

        for row in 0..0x10 {
            write!(target, "{:#06X}", row*0x10)?;
            for col in 0..0x10 {
                write!(target, "{:4}{:02x}", " ", self.ram[row*0x10+col])?;
            }
            write!(target, "\n")?;
        }

        Ok(())
    }
}

impl <'a> Debug for Bus <'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Bus")
        .field("ram", &format!("RAM: {} bytes of memory", self.ram.len()))
        .field("cart", &self.cart)
        .finish()
    }
}