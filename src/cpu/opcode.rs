use std::fmt::Display;
use std::fmt::Write;

use crate::Bus;
use crate::cpu::U16AsLoHiExt;
use super::opcode_values::add_all_ops;
use super::Cpu;

const OPCODE_COUNT: usize = 256;

#[derive(Clone, Copy, Debug)]
pub enum AddrMode {
    Accum,
    Imm(u8),
    Absolute(u16),
    ZP(u8),
    IdxZPX(u8), IdxZPY(u8),
    IdxAbsX(u16), IdxAbsY(u16),
    Implied,
    Relative(u8, u16),
    IdxIndX(u8),
    IndIdxY(u8),
    Indirect(u16)
}

/// For logging
pub(super) struct OpData {
    pub pc: u16,
    pub opcode: u8,
    pub mnemonic: &'static str,
    pub addr_mode: AddrMode,
}

impl Default for OpData {
    fn default() -> Self {
        Self {
            pc: Default::default(),
            opcode: Default::default(),
            mnemonic: Default::default(),
            addr_mode: AddrMode::Implied,
        }
    }
}

impl Display for OpData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();

        write!(f, "{:04X}", self.pc)?;
        write!(f, "{:4}{:02X} ", "", self.opcode)?;

        match self.addr_mode {
            AddrMode::Accum => todo!(),
            AddrMode::Imm(a) => {
                write!(f, "{:02X} {:2}", a, "")?;
                write!(s, "#${:02X}", a)?;
            },
            AddrMode::Absolute(a) => {
                write!(f, "{:02X} {:02X}", a.lo(), a.hi())?;
                write!(s, "${:04X}", a)?;
            },
            AddrMode::ZP(a) => {
                write!(f, "{:02X} {:2}", a, "")?;
                write!(s, "#${:02X}", a)?;
            },
            AddrMode::IdxZPX(_) => todo!(),
            AddrMode::IdxZPY(_) => todo!(),
            AddrMode::IdxAbsX(_) => todo!(),
            AddrMode::IdxAbsY(_) => todo!(),
            AddrMode::Implied => {
                write!(f, "{0:2} {0:2}", "")?;
            },
            AddrMode::Relative(a, b) => {
                write!(f, "{:02X} {:2}", a, "")?;
                write!(s, "${:02X}", b)?;
            },
            AddrMode::IdxIndX(_) => todo!(),
            AddrMode::IndIdxY(_) => todo!(),
            AddrMode::Indirect(_) => todo!(),
        }

        write!(f, "{:4}{} {:<6}", "", self.mnemonic, s)?;


        Ok(())
    }
}

#[derive(Clone, Copy)]
pub struct Opcode {
    pub value: u8,
    pub mnemonic: &'static str,
    // pub address_mode: AddrMode,
    pub address_mode_fn: fn(&mut Cpu, &mut Bus)->(),
    pub op_fn: fn(&mut Cpu, &mut Bus)->(),
    pub len: u8,
    pub cycles: u8,
    pub cycles_added: u8,
    pub legal: bool,
}

pub fn create_opcode_table() -> Box<[Opcode]> {
    let mut table = Vec::new();
    table.reserve(OPCODE_COUNT);
    add_all_ops(&mut table);
    table.into_boxed_slice()
}
