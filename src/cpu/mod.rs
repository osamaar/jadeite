// #![feature(trace_macros)]
// trace_macros!(true);
#![allow(unused_variables, dead_code)]
#![allow(non_snake_case)]

mod opcode;
mod opcode_values;

use std::fmt::Debug;

use crate::Bus;
use self::opcode::Opcode;

pub struct Cpu {
    pub reg: Reg,
    pub cycles: u8,

    clock_count: u32,
    fetched: u8,
    addr_abs: u16,
    addr_rel: u16,

    opcode_table: Box<[Opcode]>,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            reg: Default::default(),
            opcode_table: opcode::create_opcode_table(),
            cycles: 0,
            fetched: 0,
            addr_abs: 0,
            addr_rel: 0,
            clock_count: 0,
        }
    }

    pub fn pc_advance(&mut self, bus: &mut Bus) -> u8 {
        let byte = bus.read(self.reg.PC);
        self.reg.PC += 1;
        byte
    }

    pub fn step(&mut self, bus: &mut Bus) {
        self.clock_count += 1;
        self.cycles -= 1;
        if self.cycles > 0 { return; }

        let byte = self.pc_advance(bus);
        let op = self.opcode_table[byte as usize];
        (op.address_mode_fn)(self);
        (op.op_fn)(self);
    }
    
    pub fn next(&mut self, bus: &mut Bus) {
        loop {
            self.step(bus);
            if self.cycles == 0 { break; }
        }
    }
    
    pub fn reset(&mut self, bus: &mut Bus) {
        self.cycles = 8;
        self.reg.P.interrupt = true;
        let pc_lo= bus.read(0xfffc) as u16;
        let pc_hi = bus.read(0xfffd) as u16;
        self.reg.PC = (pc_hi << 8) & pc_lo;
        bus.write(0x00, 0xff);
        bus.write(0xff, 0xff);
    }
    
    pub fn reset_to(&mut self, bus: &mut Bus, offset: u16) {
        self.cycles = 8;
        self.reg.P.interrupt = true;
        self.reg.PC = offset;
    }

    // Addressing Modes
    fn Accum(cpu: &mut Self) -> u8 { unimplemented!() }
    fn Imm(cpu: &mut Self) -> u8 { unimplemented!() }
    fn Absolute(cpu: &mut Self) -> u8 { unimplemented!() }
    fn ZP(cpu: &mut Self) -> u8 { unimplemented!() }
    fn IdxZPX(cpu: &mut Self) -> u8 { unimplemented!() }
    fn IdxZPY(cpu: &mut Self) -> u8 { unimplemented!() }
    fn IdxAbsX(cpu: &mut Self) -> u8 { unimplemented!() }
    fn IdxAbsY(cpu: &mut Self) -> u8 { unimplemented!() }
    fn Implied(cpu: &mut Self) -> u8 { unimplemented!() }
    fn Relative(cpu: &mut Self) -> u8 { unimplemented!() }
    fn IdxIndX(cpu: &mut Self) -> u8 { unimplemented!() }
    fn IndIdxY(cpu: &mut Self) -> u8 { unimplemented!() }
    fn Indirect(cpu: &mut Self) -> u8 { unimplemented!() }

    // Instructions
    fn XXX(cpu: &mut Self) { unimplemented!(); }
    fn ADC(cpu: &mut Self) { unimplemented!(); }
    fn AND(cpu: &mut Self) { unimplemented!(); }
    fn ASL(cpu: &mut Self) { unimplemented!(); }
    fn BCC(cpu: &mut Self) { unimplemented!(); }
    fn BCS(cpu: &mut Self) { unimplemented!(); }
    fn BEQ(cpu: &mut Self) { unimplemented!(); }
    fn BIT(cpu: &mut Self) { unimplemented!(); }
    fn BMI(cpu: &mut Self) { unimplemented!(); }
    fn BNE(cpu: &mut Self) { unimplemented!(); }
    fn BPL(cpu: &mut Self) { unimplemented!(); }
    fn BRK(cpu: &mut Self) { unimplemented!(); }
    fn BVC(cpu: &mut Self) { unimplemented!(); }
    fn BVS(cpu: &mut Self) { unimplemented!(); }
    fn CLC(cpu: &mut Self) { unimplemented!(); }
    fn CLD(cpu: &mut Self) { unimplemented!(); }
    fn CLI(cpu: &mut Self) { unimplemented!(); }
    fn CLV(cpu: &mut Self) { unimplemented!(); }
    fn CMP(cpu: &mut Self) { unimplemented!(); }
    fn CPX(cpu: &mut Self) { unimplemented!(); }
    fn CPY(cpu: &mut Self) { unimplemented!(); }
    fn DEC(cpu: &mut Self) { unimplemented!(); }
    fn DEX(cpu: &mut Self) { unimplemented!(); }
    fn DEY(cpu: &mut Self) { unimplemented!(); }
    fn EOR(cpu: &mut Self) { unimplemented!(); }
    fn INC(cpu: &mut Self) { unimplemented!(); }
    fn INX(cpu: &mut Self) { unimplemented!(); }
    fn INY(cpu: &mut Self) { unimplemented!(); }
    fn JMP(cpu: &mut Self) { unimplemented!(); }
    fn JSR(cpu: &mut Self) { unimplemented!(); }
    fn LDA(cpu: &mut Self) { unimplemented!(); }
    fn LDX(cpu: &mut Self) { unimplemented!(); }
    fn LDY(cpu: &mut Self) { unimplemented!(); }
    fn LSR(cpu: &mut Self) { unimplemented!(); }
    fn NP(cpu: &mut Self) { unimplemented!(); }
    fn OR(cpu: &mut Self) { unimplemented!(); }
    fn ORA(cpu: &mut Self) { unimplemented!(); }
    fn PHA(cpu: &mut Self) { unimplemented!(); }
    fn PHP(cpu: &mut Self) { unimplemented!(); }
    fn PLA(cpu: &mut Self) { unimplemented!(); }
    fn PLP(cpu: &mut Self) { unimplemented!(); }
    fn ROL(cpu: &mut Self) { unimplemented!(); }
    fn ROR(cpu: &mut Self) { unimplemented!(); }
    fn RTI(cpu: &mut Self) { unimplemented!(); }
    fn RTS(cpu: &mut Self) { unimplemented!(); }
    fn SBC(cpu: &mut Self) { unimplemented!(); }
    fn SEC(cpu: &mut Self) { unimplemented!(); }
    fn SED(cpu: &mut Self) { unimplemented!(); }
    fn SEI(cpu: &mut Self) { unimplemented!(); }
    fn STA(cpu: &mut Self) { unimplemented!(); }
    fn STX(cpu: &mut Self) { unimplemented!(); }
    fn STY(cpu: &mut Self) { unimplemented!(); }
    fn TAX(cpu: &mut Self) { unimplemented!(); }
    fn TAY(cpu: &mut Self) { unimplemented!(); }
    fn TSX(cpu: &mut Self) { unimplemented!(); }
    fn TXA(cpu: &mut Self) { unimplemented!(); }
    fn TXS(cpu: &mut Self) { unimplemented!(); }
    fn TYA(cpu: &mut Self) { unimplemented!(); }
}


#[allow(non_snake_case)]
#[derive(Debug, Default)]
pub struct Reg {
    /// Accumulator
    pub A: u8,
    /// Index
    pub Y: u8,
    /// Index
    pub X: u8,
    /// Program Counter
    pub PC: u16,
    /// Stack Pointer
    pub S: u8,
    /// Status Register
    pub P: RegStatus,
}

#[allow(non_snake_case)]
#[derive(Debug, Default)]
pub struct RegStatus {
    /// `C`: Bit 0
    pub carry: bool,
    /// `Z`: Bit 1
    pub zero: bool,
    /// `I`: Bit 2
    pub interrupt: bool,
    /// `D`: Bit 3
    pub decimal: bool,
    /// `B`: Bit 4
    pub brk: bool,
    /// `Unused`: Bit 5
    pub unused: bool,
    /// `V`: Bit 6
    pub overflow: bool,
    /// `N`: Bit 7
    pub negative: bool,
}

impl From<RegStatus> for u8 {
    fn from(p: RegStatus) -> Self {
        ((p.carry     as u8) << 0) &
        ((p.zero      as u8) << 1) &
        ((p.interrupt as u8) << 2) &
        ((p.decimal   as u8) << 3) &
        ((p.brk       as u8) << 4) &
        ((p.unused    as u8) << 5) &
        ((p.overflow  as u8) << 6) &
        ((p.negative  as u8) << 7)
    }
}

impl From<u8> for RegStatus {
    fn from(b: u8) -> Self {
        Self {
            carry    : ((b & 0b0000_0001) >> 0) == 1,
            zero     : ((b & 0b0000_0010) >> 1) == 1,
            interrupt: ((b & 0b0000_0100) >> 2) == 1,
            decimal  : ((b & 0b0000_1000) >> 3) == 1,
            brk      : ((b & 0b0001_0000) >> 4) == 1,
            unused   : ((b & 0b0010_0000) >> 5) == 1,
            overflow : ((b & 0b0100_0000) >> 6) == 1,
            negative : ((b & 0b1000_0000) >> 7) == 1,
        }
    }
}

impl Debug for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cpu")
            .field("reg", &self.reg)
            .field("cycles", &self.cycles)
            .field("opcode_table", &"opcode_table")
            .finish()
    }
}

/// u16 extension: Decompose a u16 to a `lo` and `hi` bytes.
trait U16AsLoHiExt {
    /// Low byte of u16 value
    fn lo(&self) -> u8;
    /// High byte of u16 value
    fn hi(&self) -> u8;
}

impl U16AsLoHiExt for u16 {
    fn lo(&self) -> u8 {
        (self & 0xff) as u8
    }

    fn hi(&self) -> u8 {
        ((self >> 8) & 0xff) as u8
    }
}
