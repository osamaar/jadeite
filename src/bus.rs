use std::fmt::{Debug, Write};

use crate::Cart;

pub struct Bus <'a> {
    ram: Box<[u8]>,
    cart: Option<&'a mut Cart>,
}

impl <'a> Bus <'a> {
    pub fn new() -> Self {
        Self {
            ram: vec![0x0u8; 0x800].into_boxed_slice(),
            cart: None,
        }
    }

    pub fn attach_cart(&mut self, cart: &'a mut Cart) {
        self.cart = Some(cart);
    }

    pub fn read(&self, addr: u16) -> u8 {
        let cart = self.cart.as_ref().unwrap();

        match addr {
            // RAM
            0x00..=0x7ff => {
                self.ram[addr as usize]
            },

            // Cartridge
            0x8000..=0xffff => {
                cart.cpu_read(addr)
            },

            a => panic!("Bus: Adressing nowhere: {:#06X}", a),
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        let cart = self.cart.as_mut().unwrap();

        match addr {
            // RAM
            0x00..=0x7ff => {
                self.ram[addr as usize] = value;
            },

            // Cartridge
            0x8000..=0xffff => {
                cart.cpu_write(addr, value);
            },

            _ => unimplemented!()
        }
    }

    /// Print a full memory page to target.
    pub fn print_page<T: Write>(&self, target: &mut T, base: u16) -> std::fmt::Result {
        write!(target, "{:6}", " ")?;

        for col in 0..0x10 {
            write!(target, "{:2}{:#04X}", " ", col)?;
        }

        write!(target, "\n")?;
        write!(target, "\n")?;

        for row in 0..0x10 {
            write!(target, "{:#06X}", base+row*0x10)?;
            for col in 0..0x10 {
                let addr = base + row*0x10 + col;
                write!(target, "{:4}{:02x}", " ", self.read(addr))?;
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