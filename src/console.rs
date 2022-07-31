use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::{CpuBus, Cart, Cpu, Ppu, PpuBus};

#[derive(Debug)]
pub struct Console<'a> {
    pub cart: Option<Rc<RefCell<&'a mut Cart>>>,
    pub cpu: Cpu<'a>,
    pub ppu: Rc<RefCell<Ppu>>,
    pub wram: Rc<RefCell<Box<[u8]>>>,
    pub vram: Rc<RefCell<Box<[u8]>>>,
    pub cpu_bus: CpuBus<'a>,
    pub ppu_bus: PpuBus<'a>,
}

impl<'a> Console<'a> {
    pub fn new() -> Self {
        let cart = None;
        let cpu = Cpu::new();
        let ppu = RefCell::new(Ppu::new());
        let ppu = Rc::new(ppu);

        let wram = vec![0x0u8; 0x800].into_boxed_slice();
        let wram = RefCell::new(wram);
        let wram = Rc::new(wram);

        let vram = vec![0x0u8; 0x800].into_boxed_slice();
        let vram = RefCell::new(vram);
        let vram = Rc::new(vram);

        let cpu_bus = CpuBus::new(wram.clone(), ppu.clone());
        let ppu_bus = PpuBus::new(vram.clone());

        Self { cart, cpu, ppu, wram, vram, cpu_bus, ppu_bus }
    }

    pub fn insert_cart(&mut self, cart: &'a mut Cart) {
        let cart = RefCell::new(cart);
        let cart = Rc::new(cart);
        self.cart = Some(cart);
        let cart = self.cart.as_ref().unwrap();
        self.cpu_bus.attach_cart(cart.clone());
        self.ppu_bus.attach_cart(cart.clone());
    }

    pub fn reset(&mut self) {
        self.cpu.reset(&mut self.cpu_bus);
    }

    pub fn reset_to(&mut self, offset: u16) {
        self.cpu.reset_to(&mut self.cpu_bus, offset);
    }

    pub fn step(&mut self) {
        {
            self.ppu_step();
            self.ppu_step();
            self.ppu_step();
        }
        self.cpu.step(&mut self.cpu_bus);
    }

    fn ppu_step(&mut self) {
        let mut ppu = (*self.ppu).borrow_mut();
        ppu.step(&self.ppu_bus);

        if ppu.nmi_signal {
            self.cpu.nmi_triggered = true;
            ppu.nmi_signal = false;
        }
    }

    pub fn next(&mut self) {
        for _ in 0..self.cpu.cycles {
            self.ppu_step();
            self.ppu_step();
            self.ppu_step();
        }

        self.cpu.next(&mut self.cpu_bus);
    }
}

impl<'a> Display for Console<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // write!(f, "Bus: {:#?}", self.bus)?;
        // write!(f, ",\n")?;

        self.cpu_bus.print_page(f, 0x00000)?;
        println!();
        self.cpu_bus.print_page(f, 0x0c000)?;
        write!(f, "{}", self.cpu)?;

        Ok(())
    }
}