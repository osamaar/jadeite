use std::fmt::Display;
use crate::{CpuBus, Cart, Cpu, Ppu, PpuBus};

/// Encapsulates the functinonality of the whole system.
/// 
/// Note: This is a self-referential struct; Do not move after creation.
/// 
/// TODO: Investigate `Pin`ing this struct.
#[derive(Debug)]
pub struct Console<'a> {
    pub cart: Option<&'a mut Cart>,
    pub wram: Box<[u8]>,
    pub vram: Box<[u8]>,
    pub cpu_bus: CpuBus<'a>,
    pub ppu_bus: PpuBus<'a>,
    pub cpu: Cpu<'a>,
    pub ppu: Ppu<'a>,
}

impl<'a> Console<'a> {
    pub fn new() -> Self {
        let cart = None;
        let wram = vec![0x0u8; 0x800].into_boxed_slice();
        let vram = vec![0x0u8; 0x800].into_boxed_slice();

        let ppu_bus = PpuBus::new();
        let ppu = Ppu::new();

        let mut cpu_bus = CpuBus::new();
        let cpu = Cpu::new(&mut cpu_bus);

        Self { cart, cpu, ppu, wram, vram, cpu_bus, ppu_bus }
    }

    pub fn init(self: &mut Self) {
        self.cpu_bus.init(&mut self.wram, &mut self.cart, &mut self.ppu);
        self.ppu_bus.init(&mut self.vram, &mut self.cart);
        self.ppu.init(&mut self.ppu_bus);
    }

    pub fn insert_cart(&mut self, cart: &'a mut Cart) {
        self.cart = Some(cart);
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
        self.ppu.step(&self.ppu_bus);

        if self.ppu.nmi_signal {
            self.cpu.nmi_triggered = true;
            self.ppu.nmi_signal = false;
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