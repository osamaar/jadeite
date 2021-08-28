#![allow(unused_variables, dead_code)]
#![allow(non_snake_case)]

mod opcode;
mod opcode_values;

use std::{fmt::{Debug, Display}, num::Wrapping};

use crate::Bus;
use self::opcode::{AddrMode, OpData, Opcode};

pub struct Cpu {
    pub reg: Reg,
    pub cycles: u8,
    pub ops: usize,

    clock_count: usize,
    addr_target: u16,
    this_op: OpData,

    opcode_table: Box<[Opcode]>,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            reg: Default::default(),
            opcode_table: opcode::create_opcode_table(),
            cycles: 0,
            ops: 0,
            addr_target: 0,
            clock_count: 0,
            this_op: Default::default(),
        }
    }

    pub fn pc_advance(&mut self, bus: &mut Bus) -> u8 {
        let byte = bus.read(self.reg.PC);
        self.reg.PC += 1;
        byte
    }

    pub fn step(&mut self, bus: &mut Bus) {
        self.cycles -= 1;

        if self.cycles == 0 {
            self.process_instruction(bus);
        }

        self.clock_count += 1;

    }

    fn process_instruction(&mut self, bus: &mut Bus) {
        // print!("{:06}| {:#06x}: ", self.ops, self.reg.PC);
        self.reg.P.unused = true;

        self.this_op.pc = self.reg.PC;

        let byte = self.pc_advance(bus);

        let op = self.opcode_table[byte as usize];
        self.this_op.opcode = op.value;
        self.this_op.mnemonic = op.mnemonic;
        let clock_count = self.clock_count;
        let registers = self.reg;

        self.cycles = op.cycles;
        (op.address_mode_fn)(self, bus);
        (op.op_fn)(self, bus);

        println!(
            "{}{:6}{}  CYC:{:_>6}",
            self.this_op, "", registers, clock_count
        );

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
        // println!("Stack push: {:#04X} =>Addr:{:04X}", value, addr);
    }

    fn pop_stack(&mut self, bus: &mut Bus) -> u8 {
        let addr = 0x0100 | self.reg.S as u16;
        let byte = bus.read(addr);
        self.reg.S += 1;
        // println!("Stack Pop: {:#04X} <=Addr:{:04X}", byte, addr);
        byte
    }

    /// Centralized op target access. All ops can use this to avoid switching
    /// addressing logic based on current instruction's addressing mode.
    fn fetch(&mut self, bus: &mut Bus) -> u8 {
        match self.this_op.addr_mode {
            AddrMode::Accum => todo!(),
            AddrMode::Imm(byte) => byte,
            AddrMode::Absolute(_) => todo!(),
            AddrMode::ZP(addr) => bus.read(addr as u16),
            AddrMode::IdxZPX(_) => todo!(),
            AddrMode::IdxZPY(_) => todo!(),
            AddrMode::IdxAbsX(_) => todo!(),
            AddrMode::IdxAbsY(_) => todo!(),
            AddrMode::Implied => todo!(),
            AddrMode::Relative(_, _) => todo!(),
            AddrMode::IdxIndX(_) => todo!(),
            AddrMode::IndIdxY(_) => todo!(),
            AddrMode::Indirect(_) => todo!(),
        }
    }

    fn store(&mut self, value: u8, bus: &mut Bus) {
        match self.this_op.addr_mode {
            AddrMode::Accum => todo!(),
            AddrMode::Imm(_) => todo!(),
            AddrMode::Absolute(_) => todo!(),
            AddrMode::ZP(addr) => bus.write(addr as u16, value),
            AddrMode::IdxZPX(_) => todo!(),
            AddrMode::IdxZPY(_) => todo!(),
            AddrMode::IdxAbsX(_) => todo!(),
            AddrMode::IdxAbsY(_) => todo!(),
            AddrMode::Implied => todo!(),
            AddrMode::Relative(_, _) => todo!(),
            AddrMode::IdxIndX(_) => todo!(),
            AddrMode::IndIdxY(_) => todo!(),
            AddrMode::Indirect(_) => todo!(),
        }
    }

    fn branch(cpu: &mut Self, bus: &mut Bus) {
        // Jump happened
        cpu.cycles += 1;

        let page_pc = cpu.reg.PC & 0xff00;
        let page_target = cpu.addr_target & 0xff00;

        if page_pc != page_target {
            // Page borders crossed
            cpu.cycles += 1;
        }

        cpu.reg.PC = cpu.addr_target;
    }

    // Addressing Modes
    fn Accum(cpu: &mut Self, bus: &mut Bus) { unimplemented!() }

    fn Imm(cpu: &mut Self, bus: &mut Bus) {
        let fetched = cpu.pc_advance(bus);
        cpu.this_op.addr_mode = AddrMode::Imm(fetched);
    }

    fn Absolute(cpu: &mut Self, bus: &mut Bus) {
        let lo = cpu.pc_advance(bus) as u16;
        let hi = cpu.pc_advance(bus) as u16;
        cpu.addr_target = (hi << 8) + lo;
        cpu.this_op.addr_mode = AddrMode::Absolute(cpu.addr_target);
    }

    fn ZP(cpu: &mut Self, bus: &mut Bus) {
        cpu.addr_target = cpu.pc_advance(bus) as u16;
        cpu.this_op.addr_mode = AddrMode::ZP(cpu.addr_target as u8);
    }

    fn IdxZPX(cpu: &mut Self, bus: &mut Bus) { unimplemented!() }
    fn IdxZPY(cpu: &mut Self, bus: &mut Bus) { unimplemented!() }
    fn IdxAbsX(cpu: &mut Self, bus: &mut Bus) { unimplemented!() }
    fn IdxAbsY(cpu: &mut Self, bus: &mut Bus) { unimplemented!() }

    fn Implied(cpu: &mut Self, _bus: &mut Bus) {
        cpu.this_op.addr_mode = AddrMode::Implied;
    }

    fn Relative(cpu: &mut Self, bus: &mut Bus) {
        // let addr_rel = ((cpu.pc_advance(bus) as i8) as i16) as u16;

        // Casting from a smaller integer to a larger integer (e.g. u8 -> u32) will
        //     zero-extend if the source is unsigned
        //     sign-extend if the source is signed
        // See: https://doc.rust-lang.org/reference/expressions/operator-expr.html?highlight=cast#type-cast-expressions
        let operand = cpu.pc_advance(bus);
        let addr_rel = (operand as i8) as u16;
        let temp = Wrapping(cpu.reg.PC) + Wrapping(addr_rel);
        cpu.addr_target = temp.0;
        cpu.this_op.addr_mode = AddrMode::Relative(operand, cpu.addr_target);
    }

    fn IdxIndX(cpu: &mut Self, bus: &mut Bus) { unimplemented!() }
    fn IndIdxY(cpu: &mut Self, bus: &mut Bus) { unimplemented!() }
    fn Indirect(cpu: &mut Self, bus: &mut Bus) { unimplemented!() }

    // Instructions
    fn XXX(cpu: &mut Self, bus: &mut Bus) { unimplemented!(); }
    fn ADC(cpu: &mut Self, bus: &mut Bus) { unimplemented!(); }
    fn AND(cpu: &mut Self, bus: &mut Bus) { unimplemented!(); }
    fn ASL(cpu: &mut Self, bus: &mut Bus) { unimplemented!(); }

    fn BCC(cpu: &mut Self, bus: &mut Bus) {
        if !cpu.reg.P.carry {
            Self::branch(cpu, bus);
        }
    }

    fn BCS(cpu: &mut Self, bus: &mut Bus) {
        if cpu.reg.P.carry {
            Self::branch(cpu, bus);
        }
    }

    fn BEQ(cpu: &mut Self, bus: &mut Bus) {
        if cpu.reg.P.zero {
            Self::branch(cpu, bus);
        }
    }

    fn BIT(cpu: &mut Self, bus: &mut Bus) {
        // N = M(7), V = M(6), Z = A & M
        // let m = bus.read(cpu.addr_target);
        let m = cpu.fetch(bus);
        cpu.reg.P.negative = (m & 0x80) != 0;
        cpu.reg.P.overflow = (m & 0x40) != 0;
        cpu.reg.P.zero = (cpu.reg.A & m) == 0;
        cpu.store(m, bus);
    }

    fn BMI(cpu: &mut Self, bus: &mut Bus) { unimplemented!(); }

    fn BNE(cpu: &mut Self, bus: &mut Bus) {
        if !cpu.reg.P.zero {
            Self::branch(cpu, bus);
        }
    }

    fn BPL(cpu: &mut Self, bus: &mut Bus) {
        if !cpu.reg.P.negative {
            Self::branch(cpu, bus);
        }
    }

    fn BRK(cpu: &mut Self, bus: &mut Bus) { unimplemented!(); }

    fn BVC(cpu: &mut Self, bus: &mut Bus) {
        if !cpu.reg.P.overflow {
            Self::branch(cpu, bus);
        }
    }

    fn BVS(cpu: &mut Self, bus: &mut Bus) {
        if cpu.reg.P.overflow {
            Self::branch(cpu, bus);
        }
    }

    fn CLC(cpu: &mut Self, bus: &mut Bus) {
        cpu.reg.P.carry = false;
    }

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
        cpu.reg.PC = cpu.addr_target;
    }

    fn JSR(cpu: &mut Self, bus: &mut Bus) {
        // Push PC (already advanced by Cpu::Absolute)
        let lo = (cpu.reg.PC & 0xff) as u8;
        let hi = ((cpu.reg.PC >> 8) & 0xff) as u8;
        cpu.push_stack(bus, lo);
        cpu.push_stack(bus, hi);

        // Jump
        cpu.reg.PC = cpu.addr_target;
    }

    fn LDA(cpu: &mut Self, bus: &mut Bus) {
        let fetched = cpu.fetch(bus);
        cpu.reg.A = fetched;
        cpu.reg.P.zero = fetched == 0;
        cpu.reg.P.negative = (fetched & 0x80) != 0;
    }

    fn LDX(cpu: &mut Self, bus: &mut Bus) {
        let fetched = cpu.fetch(bus);
        cpu.reg.X = fetched;
        cpu.reg.P.zero = fetched == 0;
        cpu.reg.P.negative = (fetched & 0x80) != 0;
    }

    fn LDY(cpu: &mut Self, bus: &mut Bus) { unimplemented!(); }
    fn LSR(cpu: &mut Self, bus: &mut Bus) { unimplemented!(); }

    fn NOP(_cpu: &mut Self, _bus: &mut Bus) {

    }

    fn OR(cpu: &mut Self, bus: &mut Bus) { unimplemented!(); }
    fn ORA(cpu: &mut Self, bus: &mut Bus) { unimplemented!(); }
    fn PHA(cpu: &mut Self, bus: &mut Bus) { unimplemented!(); }

    fn PHP(cpu: &mut Self, bus: &mut Bus) {
        let mut status = cpu.reg.P;
        status.brk = true;
        status.unused = true;
        cpu.push_stack(bus, (&status).into());
    }

    fn PLA(cpu: &mut Self, bus: &mut Bus) {
        let byte = cpu.pop_stack(bus);
        cpu.reg.A = byte;
        if byte == 0 { cpu.reg.P.zero = true; }
        if (byte & 0x80) == 1 { cpu.reg.P.negative = true; }
    }

    fn PLP(cpu: &mut Self, bus: &mut Bus) { unimplemented!(); }
    fn ROL(cpu: &mut Self, bus: &mut Bus) { unimplemented!(); }
    fn ROR(cpu: &mut Self, bus: &mut Bus) { unimplemented!(); }
    fn RTI(cpu: &mut Self, bus: &mut Bus) { unimplemented!(); }

    fn RTS(cpu: &mut Self, bus: &mut Bus) {
        let hi = cpu.pop_stack(bus) as u16;
        let lo = cpu.pop_stack(bus) as u16;
        cpu.reg.PC = (hi << 8) | lo;
    }

    fn SBC(cpu: &mut Self, bus: &mut Bus) { unimplemented!(); }

    fn SEC(cpu: &mut Self, bus: &mut Bus) {
        cpu.reg.P.carry = true;
    }

    fn SED(cpu: &mut Self, bus: &mut Bus) {
        cpu.reg.P.decimal = true;
    }

    fn SEI(cpu: &mut Self, bus: &mut Bus) {
        cpu.reg.P.interrupt = true;
    }

    fn STA(cpu: &mut Self, bus: &mut Bus) {
        bus.write(cpu.addr_target, cpu.reg.A);
    }

    fn STX(cpu: &mut Self, bus: &mut Bus) {
        bus.write(cpu.addr_target, cpu.reg.X);
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
#[derive(Debug, Default, Clone, Copy)]
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

impl Display for Reg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "A:{:02X}", self.A)?;
        write!(f, " X:{:02X}", self.X)?;
        write!(f, " Y:{:02X}", self.Y)?;
        write!(f, " P:{:02X}", u8::from(&self.P))?;
        write!(f, " SP:{:02X}", self.S)?;
        Ok(())
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Default, Clone, Copy)]
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
            f, "{} {} {} {} {} {} {} {}  ({:2x})\n",
            self.reg.P.negative as u8,
            self.reg.P.overflow as u8,
            self.reg.P.unused as u8,
            self.reg.P.brk as u8,
            self.reg.P.decimal as u8,
            self.reg.P.interrupt as u8,
            self.reg.P.zero as u8,
            self.reg.P.carry as u8,
            u8::from(&self.reg.P),
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
