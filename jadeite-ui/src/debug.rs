use std::io::{ErrorKind, Read, Write};
use std::str::Lines;

pub(crate) struct DebugOut<'a> {
    log: Lines<'a>,
    accum: Vec<u8>,
    last: usize,
    line: String,
    error_state: bool,
}

impl<'a> DebugOut<'a> {
    pub(crate) fn new(s: &'a str) -> Self {
        Self {
            log: s.lines(),
            accum: Vec::new(),
            last: 0,
            line: String::new(),
            error_state: false,
        }
    }

    fn match_next(&mut self) -> Result<(), ()> {
        let good = match self.log.next() {
            Some(s) => Ok(s),
            None => Err(()),
        }?;

        let line = &mut self.line;

        let pc = &line[..4];
        let opcode = &line[6..8];
        let mnemonic = &line[9..12];
        let registers = &line[32..57];
        let cycles = &line[63..69];
        let cycles = cycles.trim_start_matches("_");

        let pc_g = &good[..4];
        let opcode_g = &good[6..8];
        let mnemonic_g = &good[16..19];
        let registers_g = &good[48..73];
        let cycles_g = &good[90..];

        let mut success = true;
        success &= pc == pc_g;
        success &= opcode == opcode_g;
        success &= mnemonic == mnemonic_g;
        success &= registers == registers_g;
        success &= cycles == cycles_g;

        match success {
            true => Ok(()),
            false => Err(())
        }
    }

    fn process(&mut self) -> Result<usize, ()> {
        let len = self.accum.len();
        let old = self.last;

        for i in self.last..len {
            let c = self.accum[i];
            if c as char == '\n' {
                let mut line = &self.accum[self.last..i];
                self.line.clear();
                line.read_to_string(&mut self.line).or(Err(()))?;

                let next = self.match_next();

                match next {
                    Ok(_) => {
                        println!("{}  {}", self.line, "[  OK   ]");
                    },
                    Err(_) => {
                        print!("{}  {}", self.line, "[ ERROR ]");
                        if !self.error_state {
                            print!(" <------ FIRST ERROR HERE");
                        }
                        println!();
                        self.error_state = true;
                    },
                }

                self.last = i + 1;
            }
        }

        // println!("OK");
        Ok(self.last - old)
    }
}

impl<'a> Write for DebugOut<'a> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        // let s = from_utf8(buf).unwrap();
        // self.match_next(s);
        let written = self.accum.write(buf);

        // let written = self.process();
        let _processed = self.process().or(
            Err(
                std::io::Error::new(
                    ErrorKind::Other,
                    "Out of reference log to compare."
                )
            )
        )?;

        written
    }

    fn flush(&mut self) -> std::io::Result<()> {
        // Passthrough
        // stdout().flush()
        Ok(())
    }
}