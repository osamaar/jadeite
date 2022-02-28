use jdasm_6502::{Operation, Mnemonic, AddrMode, Instruction};
use crate::Bus;

use super::{Cpu, InstructionTarget};

type OpFn<'a> = fn(&mut Cpu<'a>, &mut Bus, InstructionTarget)->();
type AddrFn<'a> = fn(&mut Cpu<'a>, &mut Bus, Instruction)->InstructionTarget;

pub(super) fn addr_handler<'a>(op: &Operation) -> AddrFn<'a> {
    match op.addr_mode {
        AddrMode::Accum     => Cpu::Accum,
        AddrMode::Imm       => Cpu::Imm,
        AddrMode::Absolute  => Cpu::Absolute,
        AddrMode::ZP        => Cpu::ZP,
        AddrMode::IdxZPX    => Cpu::IdxZPX,
        AddrMode::IdxZPY    => Cpu::IdxZPY,
        AddrMode::IdxAbsX   => Cpu::IdxAbsX,
        AddrMode::IdxAbsY   => Cpu::IdxAbsY,
        AddrMode::Implied   => Cpu::Implied,
        AddrMode::Relative  => Cpu::Relative,
        AddrMode::IdxIndX   => Cpu::IdxIndX,
        AddrMode::IndIdxY   => Cpu::IndIdxY,
        AddrMode::Indirect  => Cpu::Indirect,
    }
}

pub(super) fn op_handler<'a>(op: &Operation) -> OpFn<'a> {
    match op.mnemonic {
        Mnemonic::BRK => Cpu::BRK,
        Mnemonic::ORA => Cpu::ORA,
        Mnemonic::ASL => Cpu::ASL,
        Mnemonic::PHP => Cpu::PHP,
        Mnemonic::BPL => Cpu::BPL,
        Mnemonic::CLC => Cpu::CLC,
        Mnemonic::JSR => Cpu::JSR,
        Mnemonic::AND => Cpu::AND,
        Mnemonic::BIT => Cpu::BIT,
        Mnemonic::ROL => Cpu::ROL,
        Mnemonic::PLP => Cpu::PLP,
        Mnemonic::BMI => Cpu::BMI,
        Mnemonic::SEC => Cpu::SEC,
        Mnemonic::RTI => Cpu::RTI,
        Mnemonic::EOR => Cpu::EOR,
        Mnemonic::LSR => Cpu::LSR,
        Mnemonic::PHA => Cpu::PHA,
        Mnemonic::JMP => Cpu::JMP,
        Mnemonic::BVC => Cpu::BVC,
        Mnemonic::CLI => Cpu::CLI,
        Mnemonic::RTS => Cpu::RTS,
        Mnemonic::ADC => Cpu::ADC,
        Mnemonic::ROR => Cpu::ROR,
        Mnemonic::PLA => Cpu::PLA,
        Mnemonic::BVS => Cpu::BVS,
        Mnemonic::SEI => Cpu::SEI,
        Mnemonic::STA => Cpu::STA,
        Mnemonic::STY => Cpu::STY,
        Mnemonic::STX => Cpu::STX,
        Mnemonic::DEY => Cpu::DEY,
        Mnemonic::TXA => Cpu::TXA,
        Mnemonic::BCC => Cpu::BCC,
        Mnemonic::TYA => Cpu::TYA,
        Mnemonic::TXS => Cpu::TXS,
        Mnemonic::LDY => Cpu::LDY,
        Mnemonic::LDA => Cpu::LDA,
        Mnemonic::LDX => Cpu::LDX,
        Mnemonic::TAY => Cpu::TAY,
        Mnemonic::TAX => Cpu::TAX,
        Mnemonic::BCS => Cpu::BCS,
        Mnemonic::CLV => Cpu::CLV,
        Mnemonic::TSX => Cpu::TSX,
        Mnemonic::CPY => Cpu::CPY,
        Mnemonic::CMP => Cpu::CMP,
        Mnemonic::DEC => Cpu::DEC,
        Mnemonic::INY => Cpu::INY,
        Mnemonic::DEX => Cpu::DEX,
        Mnemonic::BNE => Cpu::BNE,
        Mnemonic::CLD => Cpu::CLD,
        Mnemonic::CPX => Cpu::CPX,
        Mnemonic::SBC => Cpu::SBC,
        Mnemonic::INC => Cpu::INC,
        Mnemonic::INX => Cpu::INX,
        Mnemonic::NOP => Cpu::NOP,
        Mnemonic::BEQ => Cpu::BEQ,
        Mnemonic::SED => Cpu::SED,
        Mnemonic::XXX => Cpu::XXX,
    }
}