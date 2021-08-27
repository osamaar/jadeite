#![allow(unused_variables, dead_code)]
#![allow(non_snake_case)]

mod opcode;
mod opcode_values;

use std::fmt::{Debug, Display};

use crate::Bus;
use self::opcode::Opcode;

pub struct Cpu {
    pub reg: Reg,
    pub cycles: u8,
    pub ops: usize,

    clock_count: usize,
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
            ops: 0,
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
        self.process_instruction(bus);

    }

    fn process_instruction(&mut self, bus: &mut Bus) {
        print!("{:06}| {:#06x}: ", self.ops, self.reg.PC);

        let byte = self.pc_advance(bus);
        let op = self.opcode_table[byte as usize];

        println!("[{}] [{:#04x}] [{}]", op.len, op.value, op.mnemonic);

        (op.address_mode_fn)(self, bus);
        (op.op_fn)(self, bus);

        self.cycles = op.cycles;
        self.ops += 1;
    }
    
    pub fn next(&mut self, bus: &mut Bus) {
        let mut remaining = self.cycles;

        while remaining > 0 {
            self.step(bus);
            remaining -= 1;
        }
    }
    
    pub fn reset(&mut self, bus: &mut Bus) {
        let pc_lo= bus.read(0xfffc) as u16;
        let pc_hi = bus.read(0xfffd) as u16;
        let pc = (pc_hi << 8) | pc_lo;
        self.reset_to(bus, pc);
    }
    
    pub fn reset_to(&mut self, bus: &mut Bus, offset: u16) {
        self.cycles = 8;
        self.reg.P.interrupt = true;
        self.reg.PC = offset;

        // Cpu starts up in 8 cycles,
        // sets SP to 00,
        // then accesses and decreases SP 3 times
        // 00 => FF => FE => FD
        // leaving SP = 0xFD
        // See: https://www.pagetable.com/?p=410
        self.reg.S = 0xFD;
    }

    fn push_stack(&mut self, bus: &mut Bus, value: u8) {
        self.reg.S -= 1;
        let addr = 0x0100 | self.reg.S as u16;
        bus.write(addr, value);
    }

    fn pop_stack(&mut self, bus: &mut Bus) -> u8 {
        let addr = 0x0100 | self.reg.S as u16;
        let byte = bus.read(addr);
        self.reg.S += 1;
        byte
    }

    // Addressing Modes
    fn Accum(cpu: &mut Self, bus: &mut Bus) { unimplemented!() }

    fn Imm(cpu: &mut Self, bus: &mut Bus) {
        cpu.fetched = cpu.pc_advance(bus);
    }

    fn Absolute(cpu: &mut Self, bus: &mut Bus) {
        let lo = cpu.pc_advance(bus) as u16;
        let hi = cpu.pc_advance(bus) as u16;
        cpu.addr_abs = (hi << 8) + lo;
    }

    fn ZP(cpu: &mut Self, bus: &mut Bus) {
        cpu.addr_abs = cpu.pc_advance(bus) as u16;
    }

    fn IdxZPX(cpu: &mut Self, bus: &mut Bus) { unimplemented!() }
    fn IdxZPY(cpu: &mut Self, bus: &mut Bus) { unimplemented!() }
    fn IdxAbsX(cpu: &mut Self, bus: &mut Bus) { unimplemented!() }
    fn IdxAbsY(cpu: &mut Self, bus: &mut Bus) { unimplemented!() }

    fn Implied(_cpu: &mut Self, _bus: &mut Bus) {

    }

    fn Relative(cpu: &mut Self, bus: &mut Bus) { unimplemented!() }
    fn IdxIndX(cpu: &mut Self, bus: &mut Bus) { unimplemented!() }
    fn IndIdxY(cpu: &mut Self, bus: &mut Bus) { unimplemented!() }
    fn Indirect(cpu: &mut Self, bus: &mut Bus) { unimplemented!() }

    // Instructions
    fn XXX(cpu: &mut Self, bus: &mut Bus) { unimplemented!(); }
    fn ADC(cpu: &mut Self, bus: &mut Bus) { unimplemented!(); }
    fn AND(cpu: &mut Self, bus: &mut Bus) { unimplemented!(); }
    fn ASL(cpu: &mut Self, bus: &mut Bus) { unimplemented!(); }
    fn BCC(cpu: &mut Self, bus: &mut Bus) { unimplemented!(); }
    fn BCS(cpu: &mut Self, bus: &mut Bus) { unimplemented!(); }
    fn BEQ(cpu: &mut Self, bus: &mut Bus) { unimplemented!(); }
    fn BIT(cpu: &mut Self, bus: &mut Bus) { unimplemented!(); }
    fn BMI(cpu: &mut Self, bus: &mut Bus) { unimplemented!(); }
    fn BNE(cpu: &mut Self, bus: &mut Bus) { unimplemented!(); }
    fn BPL(cpu: &mut Self, bus: &mut Bus) { unimplemented!(); }
    fn BRK(cpu: &mut Self, bus: &mut Bus) { unimplemented!(); }
    fn BVC(cpu: &mut Self, bus: &mut Bus) { unimplemented!(); }
    fn BVS(cpu: &mut Self, bus: &mut Bus) { unimplemented!(); }
    fn CLC(cpu: &mut Self, bus: &mut Bus) { unimplemented!(); }
    fn CLD(cpu: &mut Self, bus: &mut Bus) { unimplemented!(); }
    fn CLI(cpu: &mut Self, bus: &mut Bus) { unimplemented!(); }
    fn CLV(cpu: &mut Self, bus: &mut Bus) { unimplemented!(); }
    fn CMP(cpu: &mut Self, bus: &mut Bus) { unimplemented!(); }
    fn CPX(cpu: &mut Self, bus: &mut Bus) { unimplemented!(); }
    fn CPY(cpu: &mut Self, bus: &mut Bus) { unimplemented!(); }
    fn DEC(cpu: &mut Self, bus: &mut Bus) { unimplemented!(); }
    fn DEX(cpu: &mut Self, bus: &mut Bus) { unimplemented!(); }
    fn DEY(cpu: &mut Self, bus: &mut Bus) { unimplemented!(); }
    fn EOR(cpu: &mut Self, bus: &mut Bus) { unimplemented!(); }
    fn INC(cpu: &mut Self, bus: &mut Bus) { unimplemented!(); }
    fn INX(cpu: &mut Self, bus: &mut Bus) { unimplemented!(); }
    fn INY(cpu: &mut Self, bus: &mut Bus) { unimplemented!(); }

    fn JMP(cpu: &mut Self, bus: &mut Bus) {
        cpu.reg.PC = cpu.addr_abs;
    }

    fn JSR(cpu: &mut Self, bus: &mut Bus) {
        // Push PC (already advanced by Cpu::Absolute)
        let lo = (cpu.reg.PC & 0xff) as u8;
        let hi = ((cpu.reg.PC >> 8) & 0xff) as u8;
        cpu.push_stack(bus, lo);
        cpu.push_stack(bus, hi);

        // Jump
        cpu.reg.PC = cpu.addr_abs;
    }

    fn LDA(cpu: &mut Self, bus: &mut Bus) { unimplemented!(); }

    fn LDX(cpu: &mut Self, bus: &mut Bus) {
        cpu.reg.X = cpu.fetched;
    }

    fn LDY(cpu: &mut Self, bus: &mut Bus) { unimplemented!(); }
    fn LSR(cpu: &mut Self, bus: &mut Bus) { unimplemented!(); }

    fn NOP(_cpu: &mut Self, _bus: &mut Bus) {

    }

    fn OR(cpu: &mut Self, bus: &mut Bus) { unimplemented!(); }
    fn ORA(cpu: &mut Self, bus: &mut Bus) { unimplemented!(); }
    fn PHA(cpu: &mut Self, bus: &mut Bus) { unimplemented!(); }
    fn PHP(cpu: &mut Self, bus: &mut Bus) { unimplemented!(); }
    fn PLA(cpu: &mut Self, bus: &mut Bus) { unimplemented!(); }
    fn PLP(cpu: &mut Self, bus: &mut Bus) { unimplemented!(); }
    fn ROL(cpu: &mut Self, bus: &mut Bus) { unimplemented!(); }
    fn ROR(cpu: &mut Self, bus: &mut Bus) { unimplemented!(); }
    fn RTI(cpu: &mut Self, bus: &mut Bus) { unimplemented!(); }
    fn RTS(cpu: &mut Self, bus: &mut Bus) { unimplemented!(); }
    fn SBC(cpu: &mut Self, bus: &mut Bus) { unimplemented!(); }
    fn SEC(cpu: &mut Self, bus: &mut Bus) { unimplemented!(); }
    fn SED(cpu: &mut Self, bus: &mut Bus) { unimplemented!(); }
    fn SEI(cpu: &mut Self, bus: &mut Bus) { unimplemented!(); }
    fn STA(cpu: &mut Self, bus: &mut Bus) { unimplemented!(); }

    fn STX(cpu: &mut Self, bus: &mut Bus) {
        bus.write(cpu.addr_abs, cpu.reg.X);
    }

    fn STY(cpu: &mut Self, bus: &mut Bus) { unimplemented!(); }
    fn TAX(cpu: &mut Self, bus: &mut Bus) { unimplemented!(); }
    fn TAY(cpu: &mut Self, bus: &mut Bus) { unimplemented!(); }
    fn TSX(cpu: &mut Self, bus: &mut Bus) { unimplemented!(); }
    fn TXA(cpu: &mut Self, bus: &mut Bus) { unimplemented!(); }
    fn TXS(cpu: &mut Self, bus: &mut Bus) { unimplemented!(); }
    fn TYA(cpu: &mut Self, bus: &mut Bus) { unimplemented!(); }
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

impl From<&RegStatus> for u8 {
    fn from(p: &RegStatus) -> Self {
        ((p.carry     as u8) << 0) |
        ((p.zero      as u8) << 1) |
        ((p.interrupt as u8) << 2) |
        ((p.decimal   as u8) << 3) |
        ((p.brk       as u8) << 4) |
        ((p.unused    as u8) << 5) |
        ((p.overflow  as u8) << 6) |
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

impl Display for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const W: usize = 4;
        const WL: usize = W + 2;
        const WS: usize = W - 2;

        write!(f, "{:_<40}\n", "")?;
        write!(f, "clock: {}\n", self.clock_count)?;
        write!(f, "Registers:\n")?;
        write!(
            f, "{:>2}{:>WL$}{:>WL$}{:>WL$}{:>WL$}{:>WL$}\n",
            "A", "Y", "X", "PC", "S", "P", WL=WL
        )?;

        write!(f, "{}{:>02X}", "", self.reg.A)?;
        write!(f, "{:W$}{:>02X}", "", self.reg.Y, W=W)?;
        write!(f, "{:W$}{:>02X}", "", self.reg.X, W=W)?;
        write!(f, "{:WS$}{:>04X}", "", self.reg.PC, WS=WS)?;
        write!(f, "{:W$}{:>02X}", "", self.reg.S, W=W)?;
        write!(f, "{:W$}{:>02X}", "", u8::from(&self.reg.P), W=W)?;

        write!(f, "\n")?;

        write!(f, "Status Flags:\n")?;
        write!(f, "N V _ B D I Z C\n")?;

        write!(
            f, "{} {} {} {} {} {} {} {}\n",
            self.reg.P.negative as u8,
            self.reg.P.overflow as u8,
            self.reg.P.unused as u8,
            self.reg.P.brk as u8,
            self.reg.P.decimal as u8,
            self.reg.P.interrupt as u8,
            self.reg.P.zero as u8,
            self.reg.P.negative as u8,
        )?;
        write!(f, "{:_<40}\n", "")?;

        Ok(())
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
