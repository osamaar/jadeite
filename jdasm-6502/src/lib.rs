use std::{fmt::Display, io::{Read, Bytes}};

use crate::constant::OPTABLE;

mod constant;

#[derive(Clone, Copy)]
pub struct Operation {
    pub opcode: u8,
    pub mnemonic: Mnemonic,
    pub addr_mode: AddrMode,
    pub size: u8,
    pub cycles: u8,
    pub extra_cycles: u8,
}

pub struct Instruction {
    pub op: Operation,
    pub operand: Operand,
    pub offset: usize,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let operands = match self.operand {
            Operand::Null => "".to_owned(),
            Operand::Byte(b) => format!("{:02x}", b),
            Operand::Word(w) => format!("{:04x}", w),
        };

        let decorator = match self.op.addr_mode {
            AddrMode::Accum => "A",
            AddrMode::Imm | AddrMode::ZP => "#$",
            _ => "$",
        };

        write!(
            f,
            "{:04x}: {:?} {}{}",
            self.offset, self.op.mnemonic, decorator, operands
        )?;

        Ok(())
    }
}

pub enum Operand {
    Null,
    Byte(u8),
    Word(u16)
}

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum AddrMode {
    Accum,          // 1
    Imm,            // 2
    Absolute,       // 3
    ZP,             // 2

    IdxZPX,         // 2
    IdxZPY,         // 2

    IdxAbsX,        // 3
    IdxAbsY,        // 3

    Implied,        // 1
    Relative,       // 2
    IdxIndX,        // 2
    IndIdxY,        // 2
    Indirect,       // 3
}

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum Mnemonic {
    BRK, ORA, ASL, PHP, BPL, CLC, JSR, AND,
    BIT, ROL, PLP, BMI, SEC, RTI, EOR, LSR,
    PHA, JMP, BVC, CLI, RTS, ADC, ROR, PLA,
    BVS, SEI, STA, STY, STX, DEY, TXA, BCC,
    TYA, TXS, LDY, LDA, LDX, TAY, TAX, BCS,
    CLV, TSX, CPY, CMP, DEC, INY, DEX, BNE,
    CLD, CPX, SBC, INC, INX, NOP, BEQ, SED,
    XXX,    // Illigal Opcode
}

/// Disassemble whole data slice and return a `Vec<Instruction>`. Stops at the
/// end or when next bytes don't make sense. Therefore, it doesn't guarantee
/// that all of `data` is read. To count bytes read, add `.offset` + `.size` of
/// last instruction in returned `Vec`.
pub fn disasm_all<R: Read>(data: &mut R) -> Vec<Instruction> {
    disasm(data).collect()
}

pub struct AsmIter<'a, R: Read> {
    data: &'a mut R,
    offset: usize,
}

impl<'a, R: Read> AsmIter<'a, R> {
    fn new(data: &'a mut R) -> Self {
        AsmIter { data, offset: 0 }
    }

    pub fn bytes_read(&self) -> usize {
        self.offset
    }
}

impl<R: Read> Iterator for AsmIter<'_, R> {
    type Item = Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        if let Ok(mut instr) = disasm_one(&mut self.data) {
            instr.offset = self.offset;
            self.offset += instr.op.size as usize;
            Some(instr)
        } else {
            None
        }
    }
}

/// Helper function to read one byte at a time.
fn read_byte<R: Read>(bytes: &mut Bytes<R>) -> Result<u8, ()> {
    match bytes.next() {
        Some(r) => r.or_else(|_| Err(())),
        None => Err(()),
    }
}

/// Return an itarator over disassembled instructinos from `data`.
pub fn disasm<R: Read>(data: &mut R) -> AsmIter<R> {
    AsmIter::new(data)
}

/// Disassemble one instructino from data slice.
pub fn disasm_one<R: Read>(data: &mut R) -> Result<Instruction, ()> {
    let mut bytes = data.bytes();
    let op = OPTABLE[read_byte(&mut bytes)? as usize];
    
    match op.size {
        // n if n as usize > data.len() => Err(()),
        1 => Ok(Instruction{ op, operand: Operand::Null, offset: 0 }),

        2 => {
            let operand = Operand::Byte(read_byte(&mut bytes)?);
            Ok(Instruction{ op, operand, offset: 0 })
        },

        3 => {
            let op0 = read_byte(&mut bytes)? as u16;
            let op1 = read_byte(&mut bytes)? as u16;
            let operand = Operand::Word(op0 | (op1 << 8));
            Ok(Instruction{ op, operand, offset: 0})
        },

        _ => unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    fn hex_to_bin(s: &str) -> Vec<u8> {
            s.split_ascii_whitespace()
            .map(|x| u8::from_str_radix(x, 16).unwrap())
            .collect()
    }

    #[test]
    fn disasm_one_ok() {
        let prog = hex_to_bin("a9 01 8d 00 02 a9 05 8d 01 02 a9 08 8d 02 02");
        let instr = disasm_one(&mut Cursor::new(prog));
        let s = format!("{}", instr.unwrap());

        assert_eq!(s, "0000: LDA #$01");
    }

    #[test]
    fn disasm_ok() {
        let prog = hex_to_bin("a9 01 8d 00 02 a9 05 8d 01 02 a9 08 8d 02 02");
        let mut cursor = Cursor::new(prog);
        let mut iter = disasm(&mut cursor);
        let f = |i| format!("{}", i);

        assert_eq!(iter.next().map(f), Some("0000: LDA #$01".to_owned()));
        assert_eq!(iter.next().map(f), Some("0002: STA $0200".to_owned()));
        assert_eq!(iter.next().map(f), Some("0005: LDA #$05".to_owned()));
        assert_eq!(iter.next().map(f), Some("0007: STA $0201".to_owned()));
        assert_eq!(iter.next().map(f), Some("000a: LDA #$08".to_owned()));
        assert_eq!(iter.next().map(f), Some("000c: STA $0202".to_owned()));
        assert_eq!(iter.next().map(f), None);
    }

    #[test]
    fn disasm_all_ok() {
        let prog = hex_to_bin("a9 01 8d 00 02 a9 05 8d 01 02 a9 08 8d 02 02");
        let instr  = disasm_all(&mut Cursor::new(prog));
        let lines: Vec<_> = instr.iter().map(|ln| format!("{}", ln)).collect();

        assert_eq!(lines[0], "0000: LDA #$01");
        assert_eq!(lines[1], "0002: STA $0200");
        assert_eq!(lines[2], "0005: LDA #$05");
        assert_eq!(lines[3], "0007: STA $0201");
        assert_eq!(lines[4], "000a: LDA #$08");
        assert_eq!(lines[5], "000c: STA $0202");
    }
}
