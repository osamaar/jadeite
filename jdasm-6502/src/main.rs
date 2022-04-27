use std::fs::File;
use std::io::{Read, stdout, Write};

use clap::Parser;
use jdasm_6502::disasm;

/// A 6502 binary disassembler.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// File containing binary code to be disassembled.
    input_file: String,

    /// Output file. Omit to print to stdout.
    #[clap(short, long)]
    output_file: Option<String>,

    #[clap(short='s', long, default_value="0")]
    /// Start position in input file.
    offset: u16,

    /// How many bytes to disassemble at most.
    #[clap(short, long)]
    length: Option<u16>,
}

fn main() {
    let args = Args::parse();

    let mut outfile: File;
    let mut stdout_ = stdout();

    let output: &mut dyn Write = match args.output_file {
        Some(s) => {
            outfile = File::create(s).expect("Error writing to output file.");
            &mut outfile
        },
        None => &mut stdout_,
    };

    if let Ok(data) = read_input(&args.input_file) {
        let slice = &data[..];

        for instr in disasm(&slice, args.offset) {
            if let Some(n) = args.length {
                if instr.offset - args.offset >= n { break; }
            }
            writeln!(output, "{}", instr).expect("Error writing to output.");
        }
    } else {
        eprintln!("Error reading input file.")
    }

    // println!("input file: {} @{}", args.input_file, args.offset);
}

fn read_input(fname: &str) -> Result<Vec<u8>, ()> {
    let mut f = File::open(fname).map_err(|_| ())?;
    let mut data = Vec::new();
    f.read_to_end(&mut data).map_err(|_| ())?;

    Ok(data)
}