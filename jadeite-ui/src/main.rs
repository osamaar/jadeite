use jadeite::{Console, Cart};

fn main() -> Result<(), ()> {
    let mut cart = Cart::read_file("resources/nestest.nes").map_err(|_| ())?;
    let mut nes = Console::new();
    nes.insert_cart(&mut cart);
    nes.cpu.reset(&mut nes.bus);

    // let mut count = 0;

    // for op in cpu.opcodes.into_iter() {
    //     if op.legal {
    //         println!("{:#02x}: {}", op.code, op.mnemonic);
    //         count += 1;
    //     }
    // }

    // println!("count: {}", count);

    println!("{}", nes);
    Ok(())
}
