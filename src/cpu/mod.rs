// #![feature(trace_macros)]
// trace_macros!(true);
#![allow(unused_variables)]
#![allow(non_snake_case)]

mod opcode;
mod opcode_values;

use std::fmt::Debug;

use crate::Bus;
use self::opcode::Opcode;

pub struct Cpu {
    pub reg: Reg,
    pub cycles: u8,

    // operand: u8,


    opcode_table: Box<[Opcode]>,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            reg: Default::default(),
            opcode_table: opcode::create_opcode_table(),
            cycles: 0,
        }
    }

    // Addressing Modes
    fn Accum(cpu: &mut Self) { }
    fn Imm(cpu: &mut Self) { }
    fn Absolute(cpu: &mut Self) { }
    fn ZP(cpu: &mut Self) { }
    fn IdxZPX(cpu: &mut Self) { }
    fn IdxZPY(cpu: &mut Self) { }
    fn IdxAbsX(cpu: &mut Self) { }
    fn IdxAbsY(cpu: &mut Self) { }
    fn Implied(cpu: &mut Self) { }
    fn Relative(cpu: &mut Self) { }
    fn IdxIndX(cpu: &mut Self) { }
    fn IndIdxY(cpu: &mut Self) { }
    fn Indirect(cpu: &mut Self) { }

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

    pub fn step(&mut self, bus: Bus) {
        self.cycles -= 1;
        if self.cycles > 0 { return; }
        let byte = bus.read(self.reg.PC);
        let op = self.opcode_table[byte as usize];
        (op.address_mode_fn)(self);
        (op.op_fn)(self);
    }
    
    pub fn next(self) {

    }
    
    pub fn reset(&mut self, bus: &Bus) {
        self.cycles = 8;
        self.reg.P.I = true;
        let pc_lo= bus.read(0xfffc) as u16;
        let pc_hi = bus.read(0xfffd) as u16;
        self.reg.PC = (pc_hi << 8) & pc_lo;
    }
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
    pub N: bool,
    pub V: bool,

    pub B: bool,
    pub D: bool,
    pub I: bool,
    pub Z: bool,
    pub C: bool,
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