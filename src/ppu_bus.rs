use std::ptr;

use crate::Cart;

#[derive(Debug)]
pub struct PpuBus<'a> {
    vram: *mut Box<[u8]>,
    cart: *mut Option<&'a mut Cart>,
}

impl<'a> PpuBus<'a> {
    pub fn new() -> Self {
        Self {
            vram: ptr::null_mut(),
            cart: ptr::null_mut(),
        }
    }

    pub fn init(&mut self, vram: &mut Box<[u8]>, cart: &mut Option<&'a mut Cart>) {
        self.vram = vram;
        self.cart = cart;
    }

    fn vram_ref(&self) -> &Box<[u8]> {
        unsafe { &*self.vram }
    }

    fn vram_mut(&self) -> &mut Box<[u8]> {
        unsafe { &mut *self.vram }
    }

    fn cart_ref(&self) -> &Cart {
        unsafe { &*self.cart }.as_ref().unwrap()
    }

    pub fn read(&self, addr: u16) -> u8 {
        let vram = self.vram_ref();
        let cart = self.cart_ref();

        let value = match addr {
            // Pattern tables
            0x0000..=0x1fff => {
                cart.ppu_read(addr)
            }

            // Namtables & mirrors
            0x2000..=0x3eff => {
                let v = vram[(addr & 0x7ff) as usize];
                let adj = addr&0x7ff;
                v
            }

            // Palette RAM indexes & mirrors
            0x3f00..=0x3f1f => {
                // Fixed/non-programmable. Separate from VRAM
                todo!()
            }

            a => panic!("Bus: Adressing nowhere: {:#06X}", a),
        };

        value
    }

    pub fn write(&self, addr: u16, value: u8) {
        match addr {
            // 2kb of VRAM
            0x2000..=0x3eff => {
                let vram = self.vram_mut();
                let addr = addr & 0x7ff;
                vram[addr as usize] = value;

            },
            
            // Palette RAM
            0x3f00..=0x3fff => {
                // todo!()
            },

            _ => unreachable!()
        };
    }
}