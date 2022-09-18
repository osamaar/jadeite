use std::{fmt::{Debug, Write}, ptr};
use crate::{Cart, Ppu};

pub struct CpuBus<'a> {
    ram: *mut Box<[u8]>,
    cart: *mut Option<&'a mut Cart>,
    ppu: *mut Ppu<'a>,
}

impl<'a> CpuBus<'a> {
    pub fn new() -> Self {
        Self {
            ram: ptr::null_mut(),
            cart: ptr::null_mut(),
            ppu: ptr::null_mut(),
        }
    }

    pub fn init(&mut self, wram: &mut Box<[u8]>, cart: &mut Option<&'a mut Cart>, ppu: &mut Ppu<'a>) {
        self.ram = wram;
        self.cart = cart;
        self.ppu = ppu;
    }

    fn ram_ref(&self) -> &Box<[u8]> {
        unsafe { &*self.ram }
    }

    fn ram_mut(&self) -> &mut Box<[u8]> {
        unsafe { &mut *self.ram }
    }

    fn cart_ref(&self) -> &Cart {
        unsafe{ &*self.cart }.as_ref().unwrap()
    }

    fn cart_mut(&self) -> &mut Cart {
        unsafe{ &mut *self.cart }.as_mut().unwrap()
    }

    fn ppu_mut(&self) -> &mut Ppu<'a> {
        unsafe { &mut *self.ppu }
    }

    pub fn read(&self, addr: u16) -> u8 {
        let ram = self.ram_ref();
        let cart = self.cart_ref();
        let ppu = self.ppu_mut();

        let value = match addr {
            // RAM & Mirros
            0x0000..=0x01fff => {
                // ram[(addr&0x07ff) as usize]
                let ret = ram[(addr&0x07ff) as usize];
                ret
            },

            // PPU Registers & Mirrors
            0x2000..=0x3fff => {
                let addr_adj = 0x2000 | (addr&0x0007);
                let value = ppu.read(addr_adj);
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
        let ram = self.ram_mut();
        let cart = self.cart_mut();
        let ppu = self.ppu_mut();

        // println!("+ WRITE: @{:04X} = {:02X}", addr, value);

        match addr {
            // RAM & Mirros
            0x0000..=0x01fff => {
                ram[(addr&0x07ff) as usize] = value;
            },

            // PPU Registers & Mirrors
            0x2000..=0x3fff => {
                let addr_adj = 0x2000 | (addr&0x0007);
                ppu.write(addr_adj, value);
                // println!("= Write: @{:04X} (ADJ: {:04X}) = {:02X}", addr, addr_adj, value);
            },

            // Cartridge
            0x8000..=0xffff => {
                cart.cpu_write(addr, value);
            },

            // APU Registers
            0x4000..=0x4017 => {
                // Does nothing.
                // TODO: Implement audio.
                // todo!();
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

impl Debug for CpuBus<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ram = unsafe { &*self.ram };

        f.debug_struct("Bus")
        .field("ram", &format!("RAM: {} bytes of memory", ram.len()))
        .field("cart", &self.cart)
        .finish()
    }
}