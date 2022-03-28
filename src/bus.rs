use std::{cell::RefCell, fmt::{Debug, Write}, rc::Rc};

use crate::{Cart, Ppu};

pub struct Bus <'a> {
    ram: Box<[u8]>,
    cart: Option<&'a mut Cart>,
    ppu: Rc<RefCell<Ppu>>,
}

impl <'a> Bus <'a> {
    pub fn new(ppu: Rc<RefCell<Ppu>>) -> Self {
        Self {
            ram: vec![0x0u8; 0x800].into_boxed_slice(),
            cart: None,
            ppu,
        }
    }

    pub fn attach_cart(&mut self, cart: &'a mut Cart) {
        self.cart = Some(cart);
    }

    pub fn read(&self, addr: u16) -> u8 {
        let cart = self.cart.as_ref().unwrap();

        let value = match addr {
            // RAM & Mirros
            0x0000..=0x01fff => {
                self.ram[(addr&0x07ff) as usize]
            },

            // PPU Registers & Mirrors
            0x2000..=0x3fff => {
                let addr_adj = 0x2000 | (addr&0x0007);
                let value = (*self.ppu).borrow_mut().read(addr_adj);
                // println!("= Read: @{:04X} (ADJ: {:04X}) = {:02X}", addr, addr_adj, value);
                value
            },

            // Cartridge ROM
            0x8000..=0xffff => {
                cart.cpu_read(addr)
            },

            // APU & IO Registers
            0x4000..=0x4017 => {
                // TODO: Implement APU & IO.
                0
            },

            a => panic!("Bus: Adressing nowhere: {:#06X}", a),
        };

        // println!("= Read: @{:04X} = {:02X}", addr, value);
        value
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        let cart = self.cart.as_mut().unwrap();
        // println!("+ WRITE: @{:04X} = {:02X}", addr, value);

        match addr {
            // RAM & Mirros
            0x0000..=0x01fff => {
                self.ram[(addr&0x07ff) as usize] = value;
            },

            // PPU Registers & Mirrors
            0x2000..=0x3fff => {
                let addr_adj = 0x2000 | (addr&0x0007);
                (*self.ppu).borrow_mut().write(addr_adj, value);
                // println!("= Write: @{:04X} (ADJ: {:04X}) = {:02X}", addr, addr_adj, value);
            },

            // Cartridge
            0x8000..=0xffff => {
                cart.cpu_write(addr, value);
            },

            // PPU Registers
            0x4000..=0x4017 => {
                // Does nothing.
                // TODO: Implement audio.
            }

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