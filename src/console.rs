use std::fmt::Display;

use crate::{Bus, Cart, Cpu};

#[derive(Debug)]
pub struct Console<'a> {
    pub cpu: Cpu<'a>,
    pub bus: Bus<'a>,
}

impl<'a> Console<'a> {
    pub fn new() -> Self {
        Self {
            cpu: Cpu::new(),
            bus: Bus::new(),
        }
    }

    pub fn insert_cart(&mut self, cart: &'a mut Cart) {
        self.bus.attach_cart(cart);
    }

    pub fn reset(&mut self) {
        self.cpu.reset(&mut self.bus);
    }

    pub fn reset_to(&mut self, offset: u16) {
        self.cpu.reset_to(&mut self.bus, offset);
    }

    pub fn step(&mut self) {
        self.cpu.step(&mut self.bus)
    }

    pub fn next(&mut self) {
        self.cpu.next(&mut self.bus)
    }
}

impl<'a> Display for Console<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // write!(f, "Bus: {:#?}", self.bus)?;
        // write!(f, ",\n")?;

        self.bus.print_page(f, 0x00000)?;
        println!();
        self.bus.print_page(f, 0x0c000)?;
        write!(f, "{}", self.cpu)?;

        Ok(())
    }
}