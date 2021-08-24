use super::opcode_values::add_all_ops;
use super::Cpu;

const OPCODE_COUNT: usize = 256;

// #[derive(Clone, Copy, Debug)]
// pub enum AddrMode {
//     Accum,
//     Imm,
//     Absolute,
//     ZP,
//     IdxZPX, IdxZPY,
//     IdxAbsX, IdxAbsY,
//     Implied,
//     Relative,
//     IdxIndX,
//     IndIdxY,
//     Indirect
// }

#[derive(Clone, Copy)]
pub struct Opcode {
    pub value: u8,
    pub mnemonic: &'static str,
    // pub address_mode: AddrMode,
    pub address_mode_fn: fn(&mut Cpu)->(),
    pub op_fn: fn(&mut Cpu)->(),
    pub len: u8,
    pub cycles: u8,
    pub cycles_added: u8,
    pub legal: bool,
}

pub fn create_opcode_table() -> Box<[Opcode]> {
    let mut table = Vec::new();
    table.reserve(OPCODE_COUNT);
    add_all_ops(&mut table);
    table.into_boxed_slice()
}
