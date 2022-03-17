use std::fmt::Display;


#[derive(Clone, Copy)]
pub struct Operation {
    pub opcode: u8,
    pub mnemonic: Mnemonic,
    pub addr_mode: AddrMode,
    pub size: u8,
    pub cycles: u8,
    pub extra_cycles: u8,
}

#[derive(Clone, Copy)]
pub struct Instruction {
    pub op: Operation,
    pub operand: Operand,
    pub offset: u16,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let operands = match self.operand {
            Operand::Null => "".to_owned(),
            Operand::Byte(b) => match self.op.addr_mode {
                AddrMode::Relative => format!("{:04X}", self.offset + self.op.size as u16 + b as u16),
                _ => format!("{:02X}", b)
            },
            Operand::Word(w) => format!("{:04X}", w),
        };

        let decorator = match self.op.addr_mode {
            AddrMode::Accum => "A",
            AddrMode::Imm | AddrMode::ZP => "#$",
            _ => "$",
        };

        let decorator = match self.operand {
            Operand::Null => "",
            _ => decorator,
        };

        let s = format!(
            "{:04X}: {:02X} {:?} {}{}",
            self.offset, self.op.opcode, self.op.mnemonic, decorator, operands
        );

        s.fmt(f)
    }
}

#[derive(Clone, Copy)]
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
