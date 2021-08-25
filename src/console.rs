use std::{borrow::Borrow, fmt::Display};

use crate::{Bus, Cart, Cpu};

#[derive(Debug)]
pub struct Console<'a> {
    pub cpu: Cpu,
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
}

impl<'a> Display for Console<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cpu: {:#?}", self.cpu)?;
        write!(f, "Bus: {:#?}", self.bus)?;
        write!(f, "\n")?;

        self.bus.print_ram(f);

        Ok(())
    }
}