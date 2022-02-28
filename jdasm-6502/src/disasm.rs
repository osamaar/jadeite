
use super::constant::OPTABLE;
use super::structs::*;

pub struct AsmIter<'a, T: ByteSource> {
    data: &'a T,
    offset: u16,
}

impl<'a, T: ByteSource> AsmIter<'a, T> {
    fn new(data: &'a T, offset: u16) -> Self {
        AsmIter { data, offset }
    }

    pub fn bytes_read(&self) -> u16 {
        self.offset
    }
}

impl<T: ByteSource> Iterator for AsmIter<'_, T> {
    type Item = Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        if let Ok(mut instr) = disasm_one(self.data, self.offset) {
            instr.offset = self.offset;
            self.offset += instr.op.size as u16;
            Some(instr)
        } else {
            None
        }
    }
}


pub trait ByteSource {
    fn read_byte(&self, offset: u16) -> Result<u8, ()>;
}

impl ByteSource for &[u8] {
    fn read_byte(&self, offset: u16) -> Result<u8, ()> {
        match offset {
            n if n as usize >= self.len() => Err(()),
            _ => Ok(self[offset as usize])
        }
    }
}

/// Disassemble to the end and return a `Vec<Instruction>`. Stops at the end or
/// when next bytes don't make sense. Therefore, it doesn't guarantee that all
/// of `data` is read. To count bytes read, add `.offset` + `.size` of last
/// instruction in returned `Vec`.
pub fn disasm_all<T: ByteSource>(data: &T, offset: u16) -> Vec<Instruction> {
    disasm(data, offset).collect()
}

/// Helper function to read one byte at a time.
// fn read_byte<R: Read>(bytes: &mut Bytes<R>) -> Result<u8, ()> {
//     match bytes.next() {
//         Some(r) => r.or_else(|_| Err(())),
//         None => Err(()),
//     }
// }

/// Return an itarator over disassembled instructinos from `data`.
pub fn disasm<T: ByteSource>(data: &T, offset: u16) -> AsmIter<T> {
    AsmIter::new(data, offset)
}

/// Disassemble one instructino from data slice.
pub fn disasm_one<T: ByteSource>(data: &T, offset: u16) -> Result<Instruction, ()> {
    // let mut bytes = data.bytes();
    let op = OPTABLE[data.read_byte(offset)? as usize];
    
    match op.size {
        // n if n as usize > data.len() => Err(()),
        1 => Ok(Instruction{ op, operand: Operand::Null, offset }),

        2 => {
            let operand = Operand::Byte(data.read_byte(offset+1)?);
            Ok(Instruction{ op, operand, offset })
        },

        3 => {
            let op0 = data.read_byte(offset+1)? as u16;
            let op1 = data.read_byte(offset+2)? as u16;
            let operand = Operand::Word(op0 | (op1 << 8));
            Ok(Instruction{ op, operand, offset })
        },

        _ => unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hex_to_bin(s: &str) -> Vec<u8> {
            s.split_ascii_whitespace()
            .map(|x| u8::from_str_radix(x, 16).unwrap())
            .collect()
    }

    #[test]
    fn disasm_one_ok() {
        let prog = hex_to_bin("a9 01 8d 00 02 a9 05 8d 01 02 a9 08 8d 02 02");
        let instr = disasm_one(&prog.as_slice(), 0);
        let s = format!("{}", instr.unwrap());

        assert_eq!(s, "0000: LDA #$01");
    }

    #[test]
    fn disasm_ok() {
        let prog = hex_to_bin("a9 01 8d 00 02 a9 05 8d 01 02 a9 08 8d 02 02");
        let s = prog.as_slice();
        let mut iter = disasm(&s, 0);
        let f = |i| format!("{}", i);

        assert_eq!(iter.next().map(f), Some("0000: LDA #$01".to_owned()));
        assert_eq!(iter.next().map(f), Some("0002: STA $0200".to_owned()));
        assert_eq!(iter.next().map(f), Some("0005: LDA #$05".to_owned()));
        assert_eq!(iter.next().map(f), Some("0007: STA $0201".to_owned()));
        assert_eq!(iter.next().map(f), Some("000A: LDA #$08".to_owned()));
        assert_eq!(iter.next().map(f), Some("000C: STA $0202".to_owned()));
        assert_eq!(iter.next().map(f), None);
    }

    #[test]
    fn disasm_all_ok() {
        let prog = hex_to_bin("a9 01 8d 00 02 a9 05 8d 01 02 a9 08 8d 02 02");
        let instr  = disasm_all(&prog.as_slice(), 0);
        let lines: Vec<_> = instr.iter().map(|ln| format!("{}", ln)).collect();

        assert_eq!(lines[0], "0000: LDA #$01");
        assert_eq!(lines[1], "0002: STA $0200");
        assert_eq!(lines[2], "0005: LDA #$05");
        assert_eq!(lines[3], "0007: STA $0201");
        assert_eq!(lines[4], "000A: LDA #$08");
        assert_eq!(lines[5], "000C: STA $0202");
    }
}