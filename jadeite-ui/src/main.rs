use jadeite::{Console, Rom};

fn main() -> Result<(), ()> {
    let _rom = Rom::read_file("resources/nestest.nes").map_err(|_| ())?;
    let nes = Console::new();

    // let mut count = 0;

    // for op in cpu.opcodes.into_iter() {
    //     if op.legal {
    //         println!("{:#02x}: {}", op.code, op.mnemonic);
    //         count += 1;
    //     }
    // }

    // println!("count: {}", count);

    println!("{:#?}", nes);
    Ok(())
}
