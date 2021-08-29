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

        let p: u8 = (&registers.P).into();


        self.cycles = op.cycles;
        (op.address_mode_fn)(self, bus);

        println!(
            "{}{:6}{}  CYC:{:_>6}    {:08b}",
            self.this_op, "", registers, clock_count, p
        );

        (op.op_fn)(self, bus);

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
            AddrMode::Absolute(_) |
            AddrMode::ZP(_) |
            AddrMode::IdxZPX(_) |
            AddrMode::IdxZPY(_) |
            AddrMode::IdxAbsX(_) |
            AddrMode::IdxAbsY(_) |
            AddrMode::IdxIndX(_) |
            AddrMode::IndIdxY(_) |
            AddrMode::Indirect(_) => {
                bus.read(self.addr_target as u16)
            }

            AddrMode::Accum => todo!(),
            AddrMode::Imm(byte) => byte,
            AddrMode::Implied => todo!(),
            AddrMode::Relative(_, _) => todo!(),
        }
    }

    fn store(&mut self, value: u8, bus: &mut Bus) {
        match self.this_op.addr_mode {
            AddrMode::Absolute(_) |
            AddrMode::ZP(_) |
            AddrMode::IdxZPX(_) |
            AddrMode::IdxZPY(_) |
            AddrMode::IdxAbsX(_) |
            AddrMode::IdxAbsY(_) |
            AddrMode::IdxIndX(_) |
            AddrMode::IndIdxY(_) |
            AddrMode::Indirect(_) => {
                bus.write(self.addr_target as u16, value);
            }

            AddrMode::Accum => todo!(),
            AddrMode::Imm(_) => todo!(),
            AddrMode::Implied => todo!(),
            AddrMode::Relative(_, _) => todo!(),
        }
    }

    fn branch(&mut self, bus: &mut Bus) {
        // Jump happened
        self.cycles += 1;

        let page_pc = self.reg.PC & 0xff00;
        let page_target = self.addr_target & 0xff00;

        if page_pc != page_target {
            // Page borders crossed
            self.cycles += 1;
        }

        self.reg.PC = self.addr_target;
    }

    // Addressing Modes
    fn Accum(&mut self, bus: &mut Bus) { unimplemented!() }

    fn Imm(&mut self, bus: &mut Bus) {
        let fetched = self.pc_advance(bus);
        self.this_op.addr_mode = AddrMode::Imm(fetched);
    }

    fn Absolute(&mut self, bus: &mut Bus) {
        let lo = self.pc_advance(bus) as u16;
        let hi = self.pc_advance(bus) as u16;
        self.addr_target = (hi << 8) + lo;
        self.this_op.addr_mode = AddrMode::Absolute(self.addr_target);
    }

    fn ZP(&mut self, bus: &mut Bus) {
        self.addr_target = self.pc_advance(bus) as u16;
        self.this_op.addr_mode = AddrMode::ZP(self.addr_target as u8);
    }

    fn IdxZPX(&mut self, bus: &mut Bus) { unimplemented!() }
    fn IdxZPY(&mut self, bus: &mut Bus) { unimplemented!() }
    fn IdxAbsX(&mut self, bus: &mut Bus) { unimplemented!() }
    fn IdxAbsY(&mut self, bus: &mut Bus) { unimplemented!() }

    fn Implied(&mut self, _bus: &mut Bus) {
        self.this_op.addr_mode = AddrMode::Implied;
    }

    fn Relative(&mut self, bus: &mut Bus) {
        // let addr_rel = ((cpu.pc_advance(bus) as i8) as i16) as u16;

        // Casting from a smaller integer to a larger integer (e.g. u8 -> u32) will
        //     zero-extend if the source is unsigned
        //     sign-extend if the source is signed
        // See: https://doc.rust-lang.org/reference/expressions/operator-expr.html?highlight=cast#type-cast-expressions
        let operand = self.pc_advance(bus);
        let addr_rel = (operand as i8) as u16;
        let temp = Wrapping(self.reg.PC) + Wrapping(addr_rel);
        self.addr_target = temp.0;
        self.this_op.addr_mode = AddrMode::Relative(operand, self.addr_target);
    }

    fn IdxIndX(&mut self, bus: &mut Bus) { unimplemented!() }
    fn IndIdxY(&mut self, bus: &mut Bus) { unimplemented!() }
    fn Indirect(&mut self, bus: &mut Bus) { unimplemented!() }

    // Instructions
    fn XXX(&mut self, bus: &mut Bus) { unimplemented!(); }

    /// Add with Carry
    fn ADC(&mut self, bus: &mut Bus) {
        // V  A M S  A^S  M^S    &
        // 0  0 0 0    0    0    0
        // 1  0 0 1    1    1    1
        // 1  1 1 0    1    1    1
        // 0  1 1 1    0    0    0
        // 0  0 1 0    0    1    0
        // 0  0 1 1    1    0    0
        // 0  1 0 0    1    0    0
        // 0  1 0 1    0    1    0

        // C = carry from 7th bit - indicates unsigned overflow.
        // V = carry from 6th bit - indicates signed overflow.

        let m = self.fetch(bus);
        let a = self.reg.A;
        let sum = a as u16 + m  as u16 + self.reg.P.carry as u16;
        self.reg.P.carry = (sum & 0x100) != 0;
        let sum = sum as u8;
        self.reg.A = sum;
        self.reg.P.overflow = (((a^sum) & (m^sum)) & 0x80) != 0;
        self.reg.P.zero = self.reg.A == 0;
        self.reg.P.negative = (self.reg.A & 0x80) != 0;
    }

    /// Logical AND
    fn AND(&mut self, bus: &mut Bus) {
        let m = self.fetch(bus);
        self.reg.A = self.reg.A & m;
        self.reg.P.zero = self.reg.A == 0;
        self.reg.P.negative = (self.reg.A & 0x80) != 0;
    }

    fn ASL(&mut self, bus: &mut Bus) { unimplemented!(); }

    /// Branch if Carry Clear
    fn BCC(&mut self, bus: &mut Bus) {
        if !self.reg.P.carry {
            self.branch(bus);
        }
    }

    /// Branch if Carry Set
    fn BCS(&mut self, bus: &mut Bus) {
        if self.reg.P.carry {
            self.branch(bus);
        }
    }

    /// Branch if Equal
    fn BEQ(&mut self, bus: &mut Bus) {
        if self.reg.P.zero {
            self.branch(bus);
        }
    }

    /// Bit Test
    fn BIT(&mut self, bus: &mut Bus) {
        // N = M(7), V = M(6), Z = A & M
        let m = self.fetch(bus);
        self.reg.P.negative = (m & 0x80) != 0;
        self.reg.P.overflow = (m & 0x40) != 0;
        self.reg.P.zero = (self.reg.A & m) == 0;
        self.store(m, bus);
    }

    /// Branch if Minus
    fn BMI(&mut self, bus: &mut Bus) {
        if self.reg.P.negative {
            self.branch(bus);
        }
    }

    /// Branch if Not Equal
    fn BNE(&mut self, bus: &mut Bus) {
        if !self.reg.P.zero {
            self.branch(bus);
        }
    }

    /// Branch if Positive
    fn BPL(&mut self, bus: &mut Bus) {
        if !self.reg.P.negative {
            self.branch(bus);
        }
    }

    fn BRK(&mut self, bus: &mut Bus) { unimplemented!(); }

    /// Branch if Overflow Clear
    fn BVC(&mut self, bus: &mut Bus) {
        if !self.reg.P.overflow {
            self.branch(bus);
        }
    }

    /// Branch if Overflow Set
    fn BVS(&mut self, bus: &mut Bus) {
        if self.reg.P.overflow {
            self.branch(bus);
        }
    }

    /// Clear Carry Flag
    fn CLC(&mut self, bus: &mut Bus) {
        self.reg.P.carry = false;
    }

    /// Clear Decimal Flag
    fn CLD(&mut self, bus: &mut Bus) {
        self.reg.P.decimal = false;
    }

    fn CLI(&mut self, bus: &mut Bus) { unimplemented!(); }

    /// Clear Overflow Flag
    fn CLV(&mut self, bus: &mut Bus) {
        self.reg.P.overflow = false;
    }

    /// Compare
    fn CMP(&mut self, bus: &mut Bus) {
        let a = self.reg.A;
        let m = self.fetch(bus);
        let result = a.wrapping_sub(m);
        self.reg.P.carry = a >= m;
        self.reg.P.zero = a == m;
        self.reg.P.negative = (result & 0x80) != 0;
    }

    fn CPX(&mut self, bus: &mut Bus) { unimplemented!(); }
    fn CPY(&mut self, bus: &mut Bus) { unimplemented!(); }
    fn DEC(&mut self, bus: &mut Bus) { unimplemented!(); }
    fn DEX(&mut self, bus: &mut Bus) { unimplemented!(); }
    fn DEY(&mut self, bus: &mut Bus) { unimplemented!(); }

    /// Exclusive OR
    fn EOR(&mut self, bus: &mut Bus) {
        let m = self.fetch(bus);
        self.reg.A = self.reg.A ^ m;
        self.reg.P.zero = self.reg.A == 0;
        self.reg.P.negative = (self.reg.A & 0x80) != 0;
    }

    fn INC(&mut self, bus: &mut Bus) { unimplemented!(); }
    fn INX(&mut self, bus: &mut Bus) { unimplemented!(); }
    fn INY(&mut self, bus: &mut Bus) { unimplemented!(); }

    /// Jump
    fn JMP(&mut self, bus: &mut Bus) {
        self.reg.PC = self.addr_target;
    }

    /// Jump to Subroutine
    fn JSR(&mut self, bus: &mut Bus) {
        // Push PC (already advanced by Cpu::Absolute)
        let lo = (self.reg.PC & 0xff) as u8;
        let hi = ((self.reg.PC >> 8) & 0xff) as u8;
        self.push_stack(bus, lo);
        self.push_stack(bus, hi);

        // Jump
        self.reg.PC = self.addr_target;
    }

    /// Load Accumulator
    fn LDA(&mut self, bus: &mut Bus) {
        let fetched = self.fetch(bus);
        self.reg.A = fetched;
        self.reg.P.zero = fetched == 0;
        self.reg.P.negative = (fetched & 0x80) != 0;
    }

    /// Load X Register
    fn LDX(&mut self, bus: &mut Bus) {
        let fetched = self.fetch(bus);
        self.reg.X = fetched;
        self.reg.P.zero = fetched == 0;
        self.reg.P.negative = (fetched & 0x80) != 0;
    }

    fn LDY(&mut self, bus: &mut Bus) { unimplemented!(); }
    fn LSR(&mut self, bus: &mut Bus) { unimplemented!(); }

    /// No Operation
    fn NOP(&mut self, _bus: &mut Bus) {

    }

    fn OR(&mut self, bus: &mut Bus) { unimplemented!(); }

    /// Logical Inclusive OR
    fn ORA(&mut self, bus: &mut Bus) {
        let m = self.fetch(bus);
        self.reg.A = self.reg.A | m;
        self.reg.P.zero = self.reg.A == 0;
        self.reg.P.negative = (self.reg.A & 0x80) != 0;
    }

    /// Push Accumulator
    fn PHA(&mut self, bus: &mut Bus) {
        self.push_stack(bus, self.reg.A);
    }

    /// Push Processor Status
    fn PHP(&mut self, bus: &mut Bus) {
        let mut status = self.reg.P;
        status.brk = true;
        status.unused = true;
        self.push_stack(bus, (&status).into());
    }

    /// Pull Accumulator
    fn PLA(&mut self, bus: &mut Bus) {
        let byte = self.pop_stack(bus);
        self.reg.A = byte;
        self.reg.P.zero = byte == 0;
        self.reg.P.negative = (byte & 0x80) != 0;
    }

    /// Pull Processor Status
    fn PLP(&mut self, bus: &mut Bus) {
        self.reg.P = self.pop_stack(bus).into();
    }

    fn ROL(&mut self, bus: &mut Bus) { unimplemented!(); }
    fn ROR(&mut self, bus: &mut Bus) { unimplemented!(); }
    fn RTI(&mut self, bus: &mut Bus) { unimplemented!(); }

    /// Return from Subroutine
    fn RTS(&mut self, bus: &mut Bus) {
        let hi = self.pop_stack(bus) as u16;
        let lo = self.pop_stack(bus) as u16;
        self.reg.PC = (hi << 8) | lo;
    }

    fn SBC(&mut self, bus: &mut Bus) { unimplemented!(); }

    /// Set Carry Flag
    fn SEC(&mut self, bus: &mut Bus) {
        self.reg.P.carry = true;
    }

    /// Set Decimal Flag
    fn SED(&mut self, bus: &mut Bus) {
        self.reg.P.decimal = true;
    }

    /// Set Interrupt Disable
    fn SEI(&mut self, bus: &mut Bus) {
        self.reg.P.interrupt = true;
    }

    /// Store Accumulator
    fn STA(&mut self, bus: &mut Bus) {
        bus.write(self.addr_target, self.reg.A);
    }

    /// Store X Register
    fn STX(&mut self, bus: &mut Bus) {
        bus.write(self.addr_target, self.reg.X);
    }

    fn STY(&mut self, bus: &mut Bus) { unimplemented!(); }
    fn TAX(&mut self, bus: &mut Bus) { unimplemented!(); }
    fn TAY(&mut self, bus: &mut Bus) { unimplemented!(); }
    fn TSX(&mut self, bus: &mut Bus) { unimplemented!(); }
    fn TXA(&mut self, bus: &mut Bus) { unimplemented!(); }
    fn TXS(&mut self, bus: &mut Bus) { unimplemented!(); }
    fn TYA(&mut self, bus: &mut Bus) { unimplemented!(); }
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
