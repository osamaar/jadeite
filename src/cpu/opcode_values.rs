use super::opcode::Opcode;
use super::Cpu;

macro_rules! add_op {
    ($t:ident, $c:expr, $m:ident, $a:ident, $l:expr, $cy:expr, $ca:expr) => {
        add_op!(@expand $t, $c, $m, $a, $l, $cy, $ca, stringify!($m));
    };

    (@expand $t:ident, $c:expr, $m:ident, $a:ident, $l:expr, $cy:expr, $ca:expr, $mstr:expr) => {
        $t.push(Opcode{
            value: $c,
            mnemonic: $mstr,
            address_mode_fn: Cpu::$a,
            op_fn: Cpu::$m,
            len: $l,
            cycles: $cy,
            cycles_added: $ca,
            legal: match $mstr {
                "XXX" => false,
                _ => true,
            }
        })
    };
}

pub fn add_all_ops(table: &mut Vec<Opcode>) {
    add_op!(table, 0x00,  BRK,  Implied , 1, 7, 0);
    add_op!(table, 0x01,  ORA,  IdxIndX , 2, 6, 0);
    add_op!(table, 0x02,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0x03,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0x04,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0x05,  ORA,  ZP      , 2, 3, 0);
    add_op!(table, 0x06,  ASL,  ZP      , 2, 5, 0);
    add_op!(table, 0x07,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0x08,  PHP,  Implied , 1, 3, 0);
    add_op!(table, 0x09,  ORA,  Imm     , 2, 2, 0);
    add_op!(table, 0x0a,  ASL,  Accum   , 1, 2, 0);
    add_op!(table, 0x0b,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0x0c,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0x0d,  ORA,  Absolute, 3, 4, 0);
    add_op!(table, 0x0e,  ASL,  Absolute, 3, 6, 0);
    add_op!(table, 0x0f,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0x10,  BPL,  Relative, 2, 2, 1);
    add_op!(table, 0x11,  ORA,  IndIdxY , 2, 5, 2);
    add_op!(table, 0x12,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0x13,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0x14,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0x15,  ORA,  IdxZPX  , 2, 4, 0);
    add_op!(table, 0x16,  ASL,  IdxZPX  , 2, 6, 0);
    add_op!(table, 0x17,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0x18,  CLC,  Implied , 1, 2, 0);
    add_op!(table, 0x19,  ORA,  IdxAbsY , 3, 4, 2);
    add_op!(table, 0x1a,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0x1b,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0x1c,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0x1d,  ORA,  IdxAbsX , 3, 4, 2);
    add_op!(table, 0x1e,  ASL,  IdxAbsX , 3, 7, 0);
    add_op!(table, 0x1f,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0x20,  JSR,  Absolute, 3, 6, 0);
    add_op!(table, 0x21,  AND,  IdxIndX , 2, 6, 0);
    add_op!(table, 0x22,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0x23,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0x24,  BIT,  ZP      , 2, 3, 0);
    add_op!(table, 0x25,  AND,  ZP      , 2, 3, 0);
    add_op!(table, 0x26,  ROL,  ZP      , 2, 5, 0);
    add_op!(table, 0x27,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0x28,  PLP,  Implied , 1, 4, 0);
    add_op!(table, 0x29,  AND,  Imm     , 2, 2, 0);
    add_op!(table, 0x2a,  ROL,  Accum   , 1, 2, 0);
    add_op!(table, 0x2b,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0x2c,  BIT,  Absolute, 3, 4, 0);
    add_op!(table, 0x2d,  AND,  Absolute, 3, 4, 0);
    add_op!(table, 0x2e,  ROL,  Absolute, 3, 6, 0);
    add_op!(table, 0x2f,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0x30,  BMI,  Relative, 2, 2, 1);
    add_op!(table, 0x31,  AND,  IndIdxY , 2, 5, 2);
    add_op!(table, 0x32,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0x33,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0x34,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0x35,  AND,  IdxZPX  , 2, 4, 0);
    add_op!(table, 0x36,  ROL,  IdxZPX  , 2, 6, 0);
    add_op!(table, 0x37,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0x38,  SEC,  Implied , 1, 2, 0);
    add_op!(table, 0x39,  AND,  IdxAbsY , 3, 4, 2);
    add_op!(table, 0x3a,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0x3b,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0x3c,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0x3d,  AND,  IdxAbsX , 3, 4, 2);
    add_op!(table, 0x3e,  ROL,  IdxAbsX , 3, 7, 0);
    add_op!(table, 0x3f,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0x40,  RTI,  Implied , 1, 6, 0);
    add_op!(table, 0x41,  EOR,  IdxIndX , 2, 6, 0);
    add_op!(table, 0x42,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0x43,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0x44,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0x45,  EOR,  ZP      , 2, 3, 0);
    add_op!(table, 0x46,  LSR,  ZP      , 2, 5, 0);
    add_op!(table, 0x47,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0x48,  PHA,  Implied , 1, 3, 0);
    add_op!(table, 0x49,  EOR,  Imm     , 2, 2, 0);
    add_op!(table, 0x4a,  LSR,  Accum   , 1, 2, 0);
    add_op!(table, 0x4b,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0x4c,  JMP,  Absolute, 3, 3, 0);
    add_op!(table, 0x4d,  EOR,  Absolute, 3, 4, 0);
    add_op!(table, 0x4e,  LSR,  Absolute, 3, 6, 0);
    add_op!(table, 0x4f,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0x50,  BVC,  Relative, 2, 2, 1);
    add_op!(table, 0x51,  OR ,  IndIdxY , 2, 5, 2);
    add_op!(table, 0x52,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0x53,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0x54,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0x55,  EOR,  IdxZPX  , 2, 4, 0);
    add_op!(table, 0x56,  LSR,  IdxZPX  , 2, 6, 0);
    add_op!(table, 0x57,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0x58,  CLI,  Implied , 1, 2, 0);
    add_op!(table, 0x59,  EOR,  IdxAbsY , 3, 4, 2);
    add_op!(table, 0x5a,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0x5b,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0x5c,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0x5d,  EOR,  IdxAbsX , 3, 4, 2);
    add_op!(table, 0x5e,  LSR,  IdxAbsX , 3, 7, 0);
    add_op!(table, 0x5f,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0x60,  RTS,  Implied , 1, 6, 0);
    add_op!(table, 0x61,  ADC,  IdxIndX , 2, 6, 0);
    add_op!(table, 0x62,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0x63,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0x64,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0x65,  ADC,  ZP      , 2, 3, 0);
    add_op!(table, 0x66,  ROR,  ZP      , 2, 5, 0);
    add_op!(table, 0x67,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0x68,  PLA,  Implied , 1, 4, 0);
    add_op!(table, 0x69,  ADC,  Imm     , 2, 2, 0);
    add_op!(table, 0x6a,  ROR,  Accum   , 1, 2, 0);
    add_op!(table, 0x6b,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0x6c,  JMP,  Indirect, 3, 5, 0);
    add_op!(table, 0x6d,  ADC,  Absolute, 3, 4, 0);
    add_op!(table, 0x6e,  ROR,  Absolute, 3, 6, 0);
    add_op!(table, 0x6f,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0x70,  BVS,  Relative, 2, 2, 1);
    add_op!(table, 0x71,  ADC,  IndIdxY , 2, 5, 2);
    add_op!(table, 0x72,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0x73,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0x74,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0x75,  ADC,  IdxZPX  , 2, 4, 0);
    add_op!(table, 0x76,  ROR,  IdxZPX  , 2, 6, 0);
    add_op!(table, 0x77,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0x78,  SEI,  Implied , 1, 2, 0);
    add_op!(table, 0x79,  ADC,  IdxAbsY , 3, 4, 2);
    add_op!(table, 0x7a,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0x7b,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0x7c,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0x7d,  ADC,  IdxAbsX , 3, 4, 2);
    add_op!(table, 0x7e,  ROR,  IdxAbsX , 3, 7, 0);
    add_op!(table, 0x7f,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0x80,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0x81,  STA,  IdxIndX , 2, 6, 0);
    add_op!(table, 0x82,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0x83,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0x84,  STY,  ZP      , 2, 3, 0);
    add_op!(table, 0x85,  STA,  ZP      , 2, 3, 0);
    add_op!(table, 0x86,  STX,  ZP      , 2, 3, 0);
    add_op!(table, 0x87,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0x88,  DEY,  Implied , 1, 2, 0);
    add_op!(table, 0x89,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0x8a,  TXA,  Implied , 1, 2, 0);
    add_op!(table, 0x8b,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0x8c,  STY,  Absolute, 3, 4, 0);
    add_op!(table, 0x8d,  STA,  Absolute, 3, 4, 0);
    add_op!(table, 0x8e,  STX,  Absolute, 3, 4, 0);
    add_op!(table, 0x8f,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0x90,  BCC,  Relative, 2, 2, 1);
    add_op!(table, 0x91,  STA,  IndIdxY , 2, 6, 0);
    add_op!(table, 0x92,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0x93,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0x94,  STY,  IdxZPX  , 2, 4, 0);
    add_op!(table, 0x95,  STA,  IdxZPX  , 2, 4, 0);
    add_op!(table, 0x96,  STX,  IdxZPY  , 2, 4, 0);
    add_op!(table, 0x97,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0x98,  TYA,  Implied , 1, 2, 0);
    add_op!(table, 0x99,  STA,  IdxAbsY , 3, 5, 0);
    add_op!(table, 0x9a,  TXS,  Implied , 1, 2, 0);
    add_op!(table, 0x9b,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0x9c,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0x9d,  STA,  IdxAbsX , 3, 5, 0);
    add_op!(table, 0x9e,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0x9f,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0xa0,  LDY,  Imm     , 2, 2, 0);
    add_op!(table, 0xa1,  LDA,  IdxIndX , 2, 6, 0);
    add_op!(table, 0xa2,  LDX,  Imm     , 2, 2, 0);
    add_op!(table, 0xa3,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0xa4,  LDY,  ZP      , 2, 3, 0);
    add_op!(table, 0xa5,  LDA,  ZP      , 2, 3, 0);
    add_op!(table, 0xa6,  LDX,  ZP      , 2, 3, 0);
    add_op!(table, 0xa7,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0xa8,  TAY,  Implied , 1, 2, 0);
    add_op!(table, 0xa9,  LDA,  Imm     , 2, 2, 0);
    add_op!(table, 0xaa,  TAX,  Implied , 1, 2, 0);
    add_op!(table, 0xab,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0xac,  LDY,  Absolute, 3, 4, 0);
    add_op!(table, 0xad,  LDA,  Absolute, 3, 4, 0);
    add_op!(table, 0xae,  LDX,  Absolute, 3, 4, 0);
    add_op!(table, 0xaf,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0xb0,  BCS,  Relative, 2, 2, 1);
    add_op!(table, 0xb1,  LDA,  IndIdxY , 2, 5, 2);
    add_op!(table, 0xb2,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0xb3,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0xb4,  LDY,  IdxZPX  , 2, 4, 0);
    add_op!(table, 0xb5,  LDA,  IdxZPX  , 2, 4, 0);
    add_op!(table, 0xb6,  LDX,  IdxZPY  , 2, 4, 0);
    add_op!(table, 0xb7,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0xb8,  CLV,  Implied , 1, 2, 0);
    add_op!(table, 0xb9,  LDA,  IdxAbsY , 3, 4, 2);
    add_op!(table, 0xba,  TSX,  Implied , 1, 2, 0);
    add_op!(table, 0xbb,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0xbc,  LDY,  IdxAbsX , 3, 4, 1);
    add_op!(table, 0xbd,  LDA,  IdxAbsX , 3, 4, 2);
    add_op!(table, 0xbe,  LDX,  IdxAbsY , 3, 4, 2);
    add_op!(table, 0xbf,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0xc0,  CPY,  Imm     , 2, 2, 0);
    add_op!(table, 0xc1,  CMP,  IdxIndX , 2, 6, 0);
    add_op!(table, 0xc2,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0xc3,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0xc4,  CPY,  ZP      , 2, 3, 0);
    add_op!(table, 0xc5,  CMP,  ZP      , 2, 3, 0);
    add_op!(table, 0xc6,  DEC,  ZP      , 2, 5, 0);
    add_op!(table, 0xc7,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0xc8,  INY,  Implied , 1, 2, 0);
    add_op!(table, 0xc9,  CMP,  Imm     , 2, 2, 0);
    add_op!(table, 0xca,  DEX,  Implied , 1, 2, 0);
    add_op!(table, 0xcb,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0xcc,  CPY,  Absolute, 3, 4, 0);
    add_op!(table, 0xcd,  CMP,  Absolute, 3, 4, 0);
    add_op!(table, 0xce,  DEC,  Absolute, 3, 6, 0);
    add_op!(table, 0xcf,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0xd0,  BNE,  Relative, 2, 2, 1);
    add_op!(table, 0xd1,  CMP,  IndIdxY , 2, 5, 2);
    add_op!(table, 0xd2,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0xd3,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0xd4,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0xd5,  CMP,  IdxZPX  , 2, 4, 0);
    add_op!(table, 0xd6,  DEC,  IdxZPX  , 2, 6, 0);
    add_op!(table, 0xd7,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0xd8,  CLD,  Implied , 1, 2, 0);
    add_op!(table, 0xd9,  CMP,  IdxAbsY , 3, 4, 2);
    add_op!(table, 0xda,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0xdb,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0xdc,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0xdd,  CMP,  IdxAbsX , 3, 4, 0);
    add_op!(table, 0xde,  DEC,  IdxAbsX , 3, 7, 0);
    add_op!(table, 0xdf,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0xe0,  CPX,  Imm     , 2, 2, 0);
    add_op!(table, 0xe1,  SBC,  IdxIndX , 2, 6, 0);
    add_op!(table, 0xe2,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0xe3,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0xe4,  CPX,  ZP      , 2, 3, 0);
    add_op!(table, 0xe5,  SBC,  ZP      , 2, 3, 0);
    add_op!(table, 0xe6,  INC,  ZP      , 2, 5, 0);
    add_op!(table, 0xe7,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0xe8,  INX,  Implied , 1, 2, 0);
    add_op!(table, 0xe9,  SBC,  Imm     , 2, 2, 0);
    add_op!(table, 0xea,  NP ,  Implied , 1, 2, 0);
    add_op!(table, 0xeb,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0xec,  CPX,  Absolute, 3, 4, 0);
    add_op!(table, 0xed,  SBC,  Absolute, 3, 4, 0);
    add_op!(table, 0xee,  INC,  Absolute, 3, 6, 0);
    add_op!(table, 0xef,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0xf0,  BEQ,  Relative, 2, 2, 1);
    add_op!(table, 0xf1,  SBC,  IndIdxY , 2, 5, 2);
    add_op!(table, 0xf2,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0xf3,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0xf4,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0xf5,  SBC,  IdxZPX  , 2, 4, 0);
    add_op!(table, 0xf6,  INC,  IdxZPX  , 2, 6, 0);
    add_op!(table, 0xf7,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0xf8,  SED,  Implied , 1, 2, 0);
    add_op!(table, 0xf9,  SBC,  IdxAbsY , 3, 4, 2);
    add_op!(table, 0xfa,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0xfb,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0xfc,  XXX,  Implied , 1, 2, 0);
    add_op!(table, 0xfd,  SBC,  IdxAbsX , 3, 4, 2);
    add_op!(table, 0xfe,  INC,  IdxAbsX , 3, 7, 0);
    add_op!(table, 0xff,  XXX,  Implied , 1, 2, 0);
}
