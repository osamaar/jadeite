use crate::{Bus, Cpu};

#[derive(Debug)]
pub struct Console {
    pub cpu: Cpu,
    pub bus: Bus,
}

impl Console {
    pub fn new() -> Self {
        Self {
            cpu: Cpu::new(),
            bus: Bus::new(),
        }
    }
}