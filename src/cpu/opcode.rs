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
#[derive(Clone, Copy)]
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
        let mut out = String::new();
        let mut asm = String::new();

        write!(out, "{:04X}", self.pc)?;
        write!(out, "{:4}{:02X} ", "", self.opcode)?;

        match self.addr_mode {
            AddrMode::Accum => {
                write!(out, "{0:2} {0:2}", "")?;
                write!(asm, "{:2}", "A")?;
            },
            AddrMode::Imm(a) => {
                write!(out, "{:02X} {:2}", a, "")?;
                write!(asm, "#${:02X}", a)?;
            },
            AddrMode::Absolute(a) => {
                write!(out, "{:02X} {:02X}", a.lo(), a.hi())?;
                write!(asm, "${:04X}", a)?;
            },
            AddrMode::ZP(a) => {
                write!(out, "{:02X} {:2}", a, "")?;
                write!(asm, "#${:02X}", a)?;
            },
            AddrMode::IdxZPX(a) => {
                write!(out, "{:02X} {:2}", a, "")?;
                write!(asm, "${:02X}, X", a)?;
            },
            AddrMode::IdxZPY(a) => {
                write!(out, "{:02X} {:2}", a, "")?;
                write!(asm, "${:02X}, Y", a)?;
            },
            AddrMode::IdxAbsX(a) => {
                write!(out, "{:02X} {:02X}", a.lo(), a.hi())?;
                write!(asm, "${:02X},X", a)?;
            },
            AddrMode::IdxAbsY(a) => {
                write!(out, "{:02X} {:02X}", a.lo(), a.hi())?;
                write!(asm, "${:02X},Y", a)?;
            },
            AddrMode::Implied => {
                write!(out, "{0:2} {0:2}", " ")?;
            },
            AddrMode::Relative(a, b) => {
                write!(out, "{:02X} {:2}", a, "")?;
                write!(asm, "${:02X}", b)?;
            },
            AddrMode::IdxIndX(a) => {
                write!(out, "{:02X} {:2}", a, "")?;
                write!(asm, "(${:02X}, X)", a)?;
            },
            AddrMode::IndIdxY(a) => {
                write!(out, "{:02X} {:2}", a, "")?;
                write!(asm, "(${:02X}, Y)", a)?;
            },
            AddrMode::Indirect(a) => {
                write!(out, "{:02X} {:02X}", a&0xFF, (a>>8)&0xFF)?;
                write!(asm, "(${:04X})", a)?;
            },
        }

        write!(out, "{:4}{} {}", "", self.mnemonic, asm)?;
        out.fmt(f)
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
