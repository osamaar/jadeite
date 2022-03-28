use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::{Bus, Cart, Cpu, Ppu};

#[derive(Debug)]
pub struct Console<'a> {
    pub cpu: Cpu<'a>,
    pub bus: Bus<'a>,
    pub ppu: Rc<RefCell<Ppu>>,
}

impl<'a> Console<'a> {
    pub fn new() -> Self {
        let cpu = Cpu::new();
        let ppu = RefCell::new(Ppu::new());
        let ppu = Rc::new(ppu);
        let bus = Bus::new(ppu.clone());

        Self { cpu, ppu, bus }
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
        {
            self.ppu_step();
            self.ppu_step();
            self.ppu_step();
        }
        self.cpu.step(&mut self.bus);
    }

    fn ppu_step(&mut self) {
            let mut ppu = (*self.ppu).borrow_mut();
            ppu.step(&mut self.bus);
            if ppu.nmi_signal {
                self.cpu.nmi_triggered = true;
                ppu.nmi_signal = false;
            }
    }

    pub fn next(&mut self) {
        {
            let mut ppu = (*self.ppu).borrow_mut();
        
            for _ in 0..self.cpu.cycles {
                ppu.step(&mut self.bus);
                ppu.step(&mut self.bus);
                ppu.step(&mut self.bus);
            }
        }

        self.cpu.next(&mut self.bus);
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