use std::cell::RefCell;
use std::rc::Rc;
use crate::Cart;

#[derive(Debug)]
pub struct PpuBus<'a> {
    vram: Rc<RefCell<Box<[u8]>>>,
    cart: Option<Rc<RefCell<&'a mut Cart>>>,
}

impl<'a> PpuBus<'a> {
    pub fn new(vram: Rc<RefCell<Box<[u8]>>>) -> Self {
        Self {
            vram,
            cart: None,
        }
    }

    pub fn attach_cart(&mut self, cart: Rc<RefCell<&'a mut Cart>>) {
        self.cart = Some(cart);
    }

    pub fn read(&self, addr: u16) -> u8 {
        let vram = self.vram.as_ref().borrow();
        let cart = self.cart.as_ref().unwrap().borrow();

        let value = match addr {
            // Pattern tables
            0x0000..=0x1fff => {
                cart.ppu_read(addr)
            }

            // Namtables & mirrors
            0x2000..=0x3eff => {
                vram[(addr & 0x7ff) as usize]
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
}