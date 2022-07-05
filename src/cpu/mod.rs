#![allow(unused_variables, dead_code)]
#![allow(non_snake_case)]

mod fn_table;

use std::fmt::{Debug, Display};
use std::io::Write;
use std::num::Wrapping;

use jdasm_6502::{disasm_one, ByteSource, Instruction, Operand};

use crate::CpuBus;
use self::fn_table::{ addr_handler, op_handler };

impl ByteSource for CpuBus<'_> {
    fn read_byte(&self, offset: u16) -> Result<u8, ()> {
        Ok(self.read(offset))
    }
}

#[derive(Clone, Copy, Debug)]
enum InstructionTarget {
    /// No explicit target. Target is implied.
    Null,

    /// Cpu `A` register.
    Accumulator,

    /// Value is contained in `Instruction` operators.
    Immediate(u8),

    /// Resolved memory address.
    MemoryAddress(u16),
}

pub struct Cpu<'a> {
    pub reg: Reg,
    pub cycles: u8,
    pub ops: usize,
    pub extra_cycles_branch: u8,
    pub extra_cycles_page_bounds: u8,
    pub nmi_triggered: bool,

    pub clock_count: usize,
    debug_out: Option<Box<&'a mut dyn Write>>,
}

impl<'a> Cpu<'a> {
    pub fn new() -> Self {
        Self {
            reg: Default::default(),
            // opcode_table: opcode::create_opcode_table(),
            cycles: 0,
            ops: 0,
            // addr_target: 0,
            clock_count: 0,
            // this_op: Default::default(),
            debug_out: None,
            extra_cycles_branch: 0,
            extra_cycles_page_bounds: 0,
            nmi_triggered: false,
        }
    }

    pub fn debug_to(&mut self, d: &'a mut dyn Write) {
        self.debug_out = Some(Box::new(d));
    }

    pub fn pc_advance(&mut self, bus: &mut CpuBus) -> u8 {
        let byte = bus.read(self.reg.PC);
        self.reg.PC += 1;
        byte
    }

    pub fn step(&mut self, bus: &mut CpuBus) {
        if self.nmi_triggered {
            self.nmi(bus);
            self.nmi_triggered = false;
        } else {
            self.cycles -= 1;

            if self.cycles == 0 {
                self.process_instruction(bus);
            }
        }

        self.clock_count += self.cycles as usize;
    }

    fn process_instruction(&mut self, bus: &mut CpuBus) {
        // print!("{:06}| {:#06x}: ", self.ops, self.reg.PC);
        self.reg.P.unused = true;
        self.extra_cycles_branch = 0;
        self.extra_cycles_page_bounds = 0;

        let instr = disasm_one(bus, self.reg.PC).unwrap();
        // println!("{}", instr);
        self.reg.PC += instr.op.size as u16;

        // Debug output.
        let registers = self.reg;
        let p: u8 = (&registers.P).into();
        let clock = self.clock_count;
        let err0 = bus.read(0x02);
        let err1 = bus.read(0x03);

        if let Some(out) = &mut self.debug_out {
            writeln!(
                out,
                // "{:04X}{:<32}{}  CYC:{:_>6}  {:08b}  [{:02X} {:02X}]",
                // instr.offset, instr, registers, clock_count, p, err0, err1
                "{:<32}{}  CYC:{:_>6}  {:08b}  [{:02X} {:02X}]",
                instr, registers, clock, p, err0, err1
            ).unwrap();
        }

        // (op.address_mode_fn)(self, bus);
        let target = addr_handler(&instr.op)(self, bus, instr);
        op_handler(&instr.op)(self, bus, target);

        self.cycles += instr.op.cycles;
        self.cycles += self.extra_cycles_page_bounds * instr.op.extra_cycles;
        self.cycles += self.extra_cycles_branch;
        self.ops += 1;
    }
    
    pub fn next(&mut self, bus: &mut CpuBus) {
        while self.cycles > 0 {
            self.step(bus);
        }
    }
    
    pub fn reset(&mut self, bus: &mut CpuBus) {
        let pc_lo= bus.read(0xfffc) as u16;
        let pc_hi = bus.read(0xfffd) as u16;
        let pc = (pc_hi << 8) | pc_lo;
        self.reset_to(bus, pc);
    }
    
    pub fn reset_to(&mut self, bus: &mut CpuBus, offset: u16) {
        self.cycles = 7;
        self.clock_count = self.cycles as usize;
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

    fn push_stack(&mut self, bus: &mut CpuBus, value: u8) {
        let addr = 0x0100 | self.reg.S as u16;
        bus.write(addr, value);
        self.reg.S -= 1;
    }

    fn pop_stack(&mut self, bus: &mut CpuBus) -> u8 {
        self.reg.S += 1;
        let addr = 0x0100 | self.reg.S as u16;
        bus.read(addr)
    }

    fn nmi(&mut self, bus: &mut CpuBus) {
        // $FFFA $FFFB
        let pc: u16 = self.reg.PC as u16;
        self.push_stack(bus, pc.hi());
        self.push_stack(bus, pc.lo());

        let mut p = self.reg.P;
        p.brk = false;
        self.push_stack(bus, (&p).into());

        let pcl = bus.read(0xFFFA) as u16;
        let pch = bus.read(0xFFFB) as u16;
        self.reg.PC = (pch << 8) | pcl;
    }

    /// Centralized op target access. All ops can use this to avoid switching
    /// addressing logic based on current instruction's addressing mode.
    fn fetch(&mut self, bus: &mut CpuBus, target: InstructionTarget) -> u8 {
        match target {
            InstructionTarget::Null => unreachable!(),
            InstructionTarget::Accumulator => self.reg.A,
            InstructionTarget::Immediate(b) => b,
            InstructionTarget::MemoryAddress(addr) => bus.read(addr),
        }
    }

    fn store(&mut self, value: u8, bus: &mut CpuBus, target: InstructionTarget) {
        match target {
            InstructionTarget::Null => unreachable!(),
            InstructionTarget::Accumulator => self.reg.A = value,
            InstructionTarget::Immediate(_) => unreachable!(),
            InstructionTarget::MemoryAddress(addr) => bus.write(addr, value),
            
        }
    }

    fn branch(&mut self, bus: &mut CpuBus, target: InstructionTarget) {
        let addr_target = match target {
            InstructionTarget::MemoryAddress(w) => w,
            _ => unreachable!()
        };

        // Jump happened
        self.extra_cycles_branch = 1;

        let page_pc = (self.reg.PC - 0) & 0xff00;
        let page_target = addr_target & 0xff00;

        if page_pc != page_target {
            // Page borders crossed
            self.extra_cycles_branch += 1;
        }

        self.reg.PC = addr_target;
    }

    // Addressing Modes
    fn Accum(&mut self, bus: &mut CpuBus, instr: Instruction) -> InstructionTarget {
        InstructionTarget::Accumulator
    }

    fn Imm(&mut self, bus: &mut CpuBus, instr: Instruction) -> InstructionTarget {
        let value = match instr.operand {
            Operand::Byte(b) => b,
            _ => unreachable!(),
        };
          
        InstructionTarget::Immediate(value)
    }

    fn Absolute(&mut self, bus: &mut CpuBus, instr: Instruction) -> InstructionTarget {
        match instr.operand {
            Operand::Word(w) => InstructionTarget::MemoryAddress(w),
            _ => unreachable!()
        }
    }

    fn ZP(&mut self, bus: &mut CpuBus, instr: Instruction) -> InstructionTarget {
        match instr.operand {
            Operand::Byte(b) => InstructionTarget::MemoryAddress(b as u16),
            _ => unreachable!()
        }
    }

    /// Indexed Zero Page [ZP, X]
    fn IdxZPX(&mut self, bus: &mut CpuBus, instr: Instruction) -> InstructionTarget {
        let base = match instr.operand {
            Operand::Byte(b) => b as u16,
            _ => unreachable!()
        };

        let offset = self.reg.X as u16;
        InstructionTarget::MemoryAddress((base + offset) & 0xFF)

    }

    /// Indexed Zero Page [ZP, Y]
    fn IdxZPY(&mut self, bus: &mut CpuBus, instr: Instruction) -> InstructionTarget {
        let base = match instr.operand {
            Operand::Byte(b) => b as u16,
            _ => unreachable!()
        };

        let offset = self.reg.Y as u16;
        InstructionTarget::MemoryAddress((base + offset) & 0xFF)
    }

    /// Indexed Absolute [ABS, X]
    fn IdxAbsX(&mut self, bus: &mut CpuBus, instr: Instruction) -> InstructionTarget {
        let base = match instr.operand {
            Operand::Word(w) => w,
            _ => unreachable!()
        };

        let offset = self.reg.X as u16;
        let addr_target = base + offset;
        let extra = (base & 0xFF00) != (addr_target & 0xFF00);
        self.extra_cycles_page_bounds = extra.into();
        InstructionTarget::MemoryAddress(addr_target)
    }

    /// Indexed Absolute [ABS, Y]
    fn IdxAbsY(&mut self, bus: &mut CpuBus, instr: Instruction) -> InstructionTarget {
        let base = match instr.operand {
            Operand::Word(w) => w,
            _ => unreachable!()
        };

        let offset = self.reg.Y as u16;
        let addr_target = base.wrapping_add(offset);
        let extra = (base & 0xFF00) != (addr_target & 0xFF00);
        self.extra_cycles_page_bounds = extra.into();
        InstructionTarget::MemoryAddress(addr_target)
    }

    fn Implied(&mut self, _bus: &mut CpuBus, instr: Instruction) -> InstructionTarget {
        InstructionTarget::Null
    }

    fn Relative(&mut self, bus: &mut CpuBus, instr: Instruction) -> InstructionTarget {
        // Casting from a smaller integer to a larger integer (e.g. u8 -> u32) will
        //     zero-extend if the source is unsigned
        //     sign-extend if the source is signed
        // See: https://doc.rust-lang.org/reference/expressions/operator-expr.html?highlight=cast#type-cast-expressions

        let operand = match instr.operand {
            Operand::Byte(b) => b as u16,
            _ => unreachable!()
        };

        let addr_rel = (operand as i8) as u16;
        let temp = Wrapping(self.reg.PC) + Wrapping(addr_rel);
        InstructionTarget::MemoryAddress(temp.0)
    }

    /// Indexed Indirect [(IND, X)]
    fn IdxIndX(&mut self, bus: &mut CpuBus, instr: Instruction) -> InstructionTarget {
        let base = match instr.operand {
            Operand::Byte(b) => b as u16,
            _ => unreachable!()
        };

        let offset = self.reg.X as u16;
        let loc_zp = (base + offset) & 0xFF;
        let lo = bus.read(loc_zp) as u16;
        let hi = bus.read((loc_zp + 1) & 0xFF) as u16;
        InstructionTarget::MemoryAddress((hi << 8) | lo)
    }

    /// Indirect Indexed [(IND), Y]
    fn IndIdxY(&mut self, bus: &mut CpuBus, instr: Instruction) -> InstructionTarget {
        let loc_zp = match instr.operand {
            Operand::Byte(b) => b as u16,
            _ => unreachable!()
        };

        let lo = bus.read(loc_zp) as u16;
        let loc_1 = (loc_zp + 1) & 0xFF;
        let hi = bus.read((loc_zp + 1) & 0xFF) as u16;
        let base = (hi << 8) | lo;
        let offset = self.reg.Y as u16;
        let addr_target = base.wrapping_add(offset);
        let extra = (base & 0xFF00) != (addr_target & 0xFF00);
        self.extra_cycles_page_bounds = extra.into();
        InstructionTarget::MemoryAddress(addr_target)
    }

    /// Absolute Indirect [(IND, X)] [JMP (IND) Only]
    fn Indirect(&mut self, bus: &mut CpuBus, instr: Instruction) -> InstructionTarget {
        let loc = match instr.operand {
            Operand::Word(w) => w,
            _ => unreachable!()
        };

        // Indirect addressing in original 6502 has a bug on page boundaries.
        // It wraps like Zero Page instructions do.
        let hi_addr = (loc & 0xFF00) | ((loc+1) & 0x00FF);

        let lo = bus.read(loc) as u16;
        let hi = bus.read(hi_addr) as u16;
        InstructionTarget::MemoryAddress((hi << 8) | lo)
    }

    // Instructions

    // Illegal instruction
    fn XXX(&mut self, bus: &mut CpuBus, target: InstructionTarget) {
        unimplemented!();
        // self.NOP(bus);
    }

    /// Add with Carry
    fn ADC(&mut self, bus: &mut CpuBus, target: InstructionTarget) {
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

        let m = self.fetch(bus, target);
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
    fn AND(&mut self, bus: &mut CpuBus, target: InstructionTarget) {
        let m = self.fetch(bus, target);
        self.reg.A = self.reg.A & m;
        self.reg.P.zero = self.reg.A == 0;
        self.reg.P.negative = (self.reg.A & 0x80) != 0;
    }

    /// Shift Left one bit
    fn ASL(&mut self, bus: &mut CpuBus, target: InstructionTarget) {
        let operand = self.fetch(bus, target);
        self.reg.P.carry = (operand & 0x80) != 0;
        let operand = operand << 1;
        self.store(operand, bus, target);
        self.reg.P.zero = operand == 0;
        self.reg.P.negative = (operand & 0x80) != 0;
    }

    /// Branch if Carry Clear
    fn BCC(&mut self, bus: &mut CpuBus, target: InstructionTarget) {
        if !self.reg.P.carry {
            self.branch(bus, target);
        }
    }

    /// Branch if Carry Set
    fn BCS(&mut self, bus: &mut CpuBus, target: InstructionTarget) {
        if self.reg.P.carry {
            self.branch(bus, target);
        }
    }

    /// Branch if Equal
    fn BEQ(&mut self, bus: &mut CpuBus, target: InstructionTarget) {
        if self.reg.P.zero {
            self.branch(bus, target);
        }
    }

    /// Bit Test
    fn BIT(&mut self, bus: &mut CpuBus, target: InstructionTarget) {
        // N = M(7), V = M(6), Z = A & M
        let m = self.fetch(bus, target);
        self.reg.P.negative = (m & 0x80) != 0;
        self.reg.P.overflow = (m & 0x40) != 0;
        self.reg.P.zero = (self.reg.A & m) == 0;
        self.store(m, bus, target);
    }

    /// Branch if Minus
    fn BMI(&mut self, bus: &mut CpuBus, target: InstructionTarget) {
        if self.reg.P.negative {
            self.branch(bus, target);
        }
    }

    /// Branch if Not Equal
    fn BNE(&mut self, bus: &mut CpuBus, target: InstructionTarget) {
        if !self.reg.P.zero {
            self.branch(bus, target);
        }
    }

    /// Branch if Positive
    fn BPL(&mut self, bus: &mut CpuBus, target: InstructionTarget) {
        if !self.reg.P.negative {
            self.branch(bus, target);
        }
    }

    /// Force Interrupt
    fn BRK(&mut self, bus: &mut CpuBus, target: InstructionTarget) {
        self.reg.P.brk = true;
        let lo = (self.reg.PC & 0xFF) as u8;
        let hi = ((self.reg.PC >> 8) & 0xFF) as u8;
        self.push_stack(bus, hi);
        self.push_stack(bus, lo);
        self.push_stack(bus, (&self.reg.P).into());
        let hi = bus.read(0xFFFE) as u16;
        let lo = bus.read(0xFFFF) as u16;
        self.reg.PC = (hi << 8) | lo;
    }

    /// Branch if Overflow Clear
    fn BVC(&mut self, bus: &mut CpuBus, target: InstructionTarget) {
        if !self.reg.P.overflow {
            self.branch(bus, target);
        }
    }

    /// Branch if Overflow Set
    fn BVS(&mut self, bus: &mut CpuBus, target: InstructionTarget) {
        if self.reg.P.overflow {
            self.branch(bus, target);
        }
    }

    /// Clear Carry Flag
    fn CLC(&mut self, bus: &mut CpuBus, target: InstructionTarget) {
        self.reg.P.carry = false;
    }

    /// Clear Decimal Flag
    fn CLD(&mut self, bus: &mut CpuBus, target: InstructionTarget) {
        self.reg.P.decimal = false;
    }

    /// Clear Interrupt Disable
    fn CLI(&mut self, bus: &mut CpuBus, target: InstructionTarget) {
        self.reg.P.interrupt = false;
    }

    /// Clear Overflow Flag
    fn CLV(&mut self, bus: &mut CpuBus, target: InstructionTarget) {
        self.reg.P.overflow = false;
    }

    /// Compare
    fn CMP(&mut self, bus: &mut CpuBus, target: InstructionTarget) {
        let a = self.reg.A;
        let m = self.fetch(bus, target);
        let result = a.wrapping_sub(m);
        self.reg.P.carry = a >= m;
        self.reg.P.zero = a == m;
        self.reg.P.negative = (result & 0x80) != 0;
    }

    fn CPX(&mut self, bus: &mut CpuBus, target: InstructionTarget) {
        let x = self.reg.X;
        let m = self.fetch(bus, target);
        let result = x.wrapping_sub(m);
        self.reg.P.carry = x >= m;
        self.reg.P.zero = x == m;
        self.reg.P.negative = (result & 0x80) != 0;
    }

    fn CPY(&mut self, bus: &mut CpuBus, target: InstructionTarget) {
        let y = self.reg.Y;
        let m = self.fetch(bus, target);
        let result = y.wrapping_sub(m);
        self.reg.P.carry = y >= m;
        self.reg.P.zero = y == m;
        self.reg.P.negative = (result & 0x80) != 0;
    }

    /// Decrement Memory
    fn DEC(&mut self, bus: &mut CpuBus, target: InstructionTarget) {
        let m = self.fetch(bus, target).wrapping_sub(1);
        self.reg.P.zero = m == 0;
        self.reg.P.negative = (m & 0x80) != 0;
        self.store(m, bus, target);
    }

    /// Decrement X Register
    fn DEX(&mut self, bus: &mut CpuBus, target: InstructionTarget) {
        let x = self.reg.X.wrapping_sub(1);
        self.reg.X = x;
        self.reg.P.zero = x == 0;
        self.reg.P.negative = (x & 0x80) != 0;
    }

    /// Decrement Y Register
    fn DEY(&mut self, bus: &mut CpuBus, target: InstructionTarget) {
        let y = self.reg.Y.wrapping_sub(1);
        self.reg.Y = y;
        self.reg.P.zero = y == 0;
        self.reg.P.negative = (y & 0x80) != 0;
    }

    /// Exclusive OR
    fn EOR(&mut self, bus: &mut CpuBus, target: InstructionTarget) {
        let m = self.fetch(bus, target);
        self.reg.A = self.reg.A ^ m;
        self.reg.P.zero = self.reg.A == 0;
        self.reg.P.negative = (self.reg.A & 0x80) != 0;
    }

    /// Increment Memory
    fn INC(&mut self, bus: &mut CpuBus, target: InstructionTarget) {
        let m = self.fetch(bus, target).wrapping_add(1);
        self.reg.P.zero = m == 0;
        self.reg.P.negative = (m & 0x80) != 0;
        self.store(m, bus, target);
    }

    /// Increment X Register
    fn INX(&mut self, bus: &mut CpuBus, target: InstructionTarget) {
        let x = self.reg.X.wrapping_add(1);
        self.reg.X = x;
        self.reg.P.zero = x == 0;
        self.reg.P.negative = (x & 0x80) != 0;
    }

    /// Increment Y Register
    fn INY(&mut self, bus: &mut CpuBus, target: InstructionTarget) {
        let y = self.reg.Y.wrapping_add(1);
        self.reg.Y = y;
        self.reg.P.zero = y == 0;
        self.reg.P.negative = (y & 0x80) != 0;
    }

    /// Jump
    fn JMP(&mut self, bus: &mut CpuBus, target: InstructionTarget) {
        self.reg.PC = match target {
            InstructionTarget::MemoryAddress(w) => w,
            _ => unreachable!()
        };
    }

    /// Jump to Subroutine
    fn JSR(&mut self, bus: &mut CpuBus, target: InstructionTarget) {
        // Push PC (already advanced by Cpu::Absolute)
        let loc = self.reg.PC - 1;
        let lo = (loc & 0xff) as u8;
        let hi = ((loc >> 8) & 0xff) as u8;
        self.push_stack(bus, hi);
        self.push_stack(bus, lo);

        // Jump
        self.reg.PC = match target {
            InstructionTarget::MemoryAddress(w) => w,
            _ => unreachable!()
        };
    }

    /// Load Accumulator
    fn LDA(&mut self, bus: &mut CpuBus, target: InstructionTarget) {
        let fetched = self.fetch(bus, target);
        self.reg.A = fetched;
        self.reg.P.zero = fetched == 0;
        self.reg.P.negative = (fetched & 0x80) != 0;
    }

    /// Load X Register
    fn LDX(&mut self, bus: &mut CpuBus, target: InstructionTarget) {
        let fetched = self.fetch(bus, target);
        self.reg.X = fetched;
        self.reg.P.zero = fetched == 0;
        self.reg.P.negative = (fetched & 0x80) != 0;
    }

    /// Load Y Register
    fn LDY(&mut self, bus: &mut CpuBus, target: InstructionTarget) {
        let fetched = self.fetch(bus, target);
        self.reg.Y = fetched;
        self.reg.P.zero = fetched == 0;
        self.reg.P.negative = (fetched & 0x80) != 0;
    }

    /// Logical Shift Right
    fn LSR(&mut self, bus: &mut CpuBus, target: InstructionTarget) {
        let operand = self.fetch(bus, target);
        self.reg.P.carry = (operand & 1) == 1;
        let operand = (operand >> 1) & 0x7F;
        self.reg.P.zero = operand == 0;
        self.reg.P.negative = (operand & 0x80) != 0;
        self.store(operand, bus, target);
    }

    /// No Operation
    fn NOP(&mut self, _bus: &mut CpuBus, target: InstructionTarget) {

    }

    /// Logical Inclusive OR
    fn ORA(&mut self, bus: &mut CpuBus, target: InstructionTarget) {
        let m = self.fetch(bus, target);
        self.reg.A = self.reg.A | m;
        self.reg.P.zero = self.reg.A == 0;
        self.reg.P.negative = (self.reg.A & 0x80) != 0;
    }

    /// Push Accumulator
    fn PHA(&mut self, bus: &mut CpuBus, target: InstructionTarget) {
        self.push_stack(bus, self.reg.A);
    }

    /// Push Processor Status
    fn PHP(&mut self, bus: &mut CpuBus, target: InstructionTarget) {
        let mut status = self.reg.P;
        status.brk = true;
        status.unused = true;
        self.push_stack(bus, (&status).into());
    }

    /// Pull Accumulator
    fn PLA(&mut self, bus: &mut CpuBus, target: InstructionTarget) {
        let byte = self.pop_stack(bus);
        self.reg.A = byte;
        self.reg.P.zero = byte == 0;
        self.reg.P.negative = (byte & 0x80) != 0;
    }

    /// Pull Processor Status
    fn PLP(&mut self, bus: &mut CpuBus, target: InstructionTarget) {
        // Ignore bit 4 and 5 of pulled value
        // https://wiki.nesdev.com/w/index.php?title=Status_flags#The_B_flag
        let old: u8 = u8::from(&self.reg.P) & 0b0011_0000;
        let new: u8  = self.pop_stack(bus) & 0b1100_1111;
        self.reg.P = (new | old).into();
    }

    /// Rotate Left
    fn ROL(&mut self, bus: &mut CpuBus, target: InstructionTarget) {
        let operand = self.fetch(bus, target);
        let old_carry = self.reg.P.carry as u8;
        self.reg.P.carry = (operand & 0x80) != 0;
        let operand = (operand << 1) | old_carry;
        self.reg.P.zero = operand == 0;
        self.reg.P.negative = (operand & 0x80) != 0;
        self.store(operand, bus, target);
    }

    /// Rotate Right
    fn ROR(&mut self, bus: &mut CpuBus, target: InstructionTarget) {
        let operand = self.fetch(bus, target);
        let old_carry = self.reg.P.carry as u8;
        self.reg.P.carry = (operand & 1) == 1;
        let operand = (operand >> 1) | (old_carry << 7);
        self.reg.P.zero = operand == 0;
        self.reg.P.negative = (operand & 0x80) != 0;
        self.store(operand, bus, target);

    }

    /// Return from Interrupt
    fn RTI(&mut self, bus: &mut CpuBus, target: InstructionTarget) {
        self.reg.P = self.pop_stack(bus).into();
        let lo = self.pop_stack(bus) as u16;
        let hi = self.pop_stack(bus) as u16;
        self.reg.PC = (hi << 8) | lo;
    }

    /// Return from Subroutine
    fn RTS(&mut self, bus: &mut CpuBus, target: InstructionTarget) {
        let lo = self.pop_stack(bus) as u16;
        let hi = self.pop_stack(bus) as u16;
        self.reg.PC = ((hi << 8) | lo) + 1;
    }

    /// Subtract with Carry
    fn SBC(&mut self, bus: &mut CpuBus, target: InstructionTarget) {
        let m = !self.fetch(bus, target);
        let a = self.reg.A;
        let carry_in = self.reg.P.carry as u16;
        let sum = a as u16 + m as u16 + carry_in;
        self.reg.P.carry = (sum & 0xff00) != 0;
        let sum = sum as u8;
        self.reg.A = sum;
        self.reg.P.overflow = (((a^sum) & (m^sum)) & 0x80) != 0;
        self.reg.P.zero = self.reg.A == 0;
        self.reg.P.negative = (self.reg.A & 0x80) != 0;
    }

    /// Set Carry Flag
    fn SEC(&mut self, bus: &mut CpuBus, target: InstructionTarget) {
        self.reg.P.carry = true;
    }

    /// Set Decimal Flag
    fn SED(&mut self, bus: &mut CpuBus, target: InstructionTarget) {
        self.reg.P.decimal = true;
    }

    /// Set Interrupt Disable
    fn SEI(&mut self, bus: &mut CpuBus, target: InstructionTarget) {
        self.reg.P.interrupt = true;
    }

    /// Store Accumulator
    fn STA(&mut self, bus: &mut CpuBus, target: InstructionTarget) {
        self.store(self.reg.A, bus, target);
    }

    /// Store X Register
    fn STX(&mut self, bus: &mut CpuBus, target: InstructionTarget) {
        self.store(self.reg.X, bus, target);
    }

    /// Store Y Register
    fn STY(&mut self, bus: &mut CpuBus, target: InstructionTarget) {
        self.store(self.reg.Y, bus, target);
    }

    /// Transfer Accumulator to X
    fn TAX(&mut self, bus: &mut CpuBus, target: InstructionTarget) {
        self.reg.X = self.reg.A;
        self.reg.P.zero = self.reg.X == 0;
        self.reg.P.negative = (self.reg.X & 0x80) != 0;
    }

    /// Transfer Accumulator to Y
    fn TAY(&mut self, bus: &mut CpuBus, target: InstructionTarget) {
        self.reg.Y = self.reg.A;
        self.reg.P.zero = self.reg.Y == 0;
        self.reg.P.negative = (self.reg.Y & 0x80) != 0;
    }

    /// Transfer Stack Pointer to X
    fn TSX(&mut self, bus: &mut CpuBus, target: InstructionTarget) {
        self.reg.X = self.reg.S;
        self.reg.P.zero = self.reg.X == 0;
        self.reg.P.negative = (self.reg.X & 0x80) != 0;
    }

    /// Transfer X to Accumulator
    fn TXA(&mut self, bus: &mut CpuBus, target: InstructionTarget) {
        self.reg.A = self.reg.X;
        self.reg.P.zero = self.reg.A == 0;
        self.reg.P.negative = (self.reg.A & 0x80) != 0;
    }

    /// Transfer X to Stack Pointer
    fn TXS(&mut self, bus: &mut CpuBus, target: InstructionTarget) {
        self.reg.S = self.reg.X;
    }

    /// Transfer Y to Accumulator
    fn TYA(&mut self, bus: &mut CpuBus, target: InstructionTarget) {
        self.reg.A = self.reg.Y;
        self.reg.P.zero = self.reg.A == 0;
        self.reg.P.negative = (self.reg.A & 0x80) != 0;
    }
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

impl Debug for Cpu<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cpu")
            .field("reg", &self.reg)
            .field("cycles", &self.cycles)
            .field("opcode_table", &"opcode_table")
            .finish()
    }
}

impl Display for Cpu<'_> {
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
