mod debug;

use std::{fs::File, io::{Read, stdout}};

use jadeite::{Console, Cart};
use debug::DebugOut;

fn main() -> Result<(), ()> {
    let mut cart = Cart::read_file("resources/nestest.nes").map_err(|_| ())?;
    let mut nes = Console::new();
    nes.insert_cart(&mut cart);
    nes.reset_to(0xc000);
    // nes.reset();

    let mut f = File::open("resources/nestest.log").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).map_err(|_| panic!("hurr durr!"))?;

    let mut out = DebugOut::new(&s);
    // let mut out = &mut stdout();
    nes.cpu.debug_to(&mut out);

    // println!("{}", nes);

    loop {
        nes.next();

        // let mut s = String::new();
        // nes.bus.print_page(&mut s, 0x0100).unwrap();
        // println!("{}", s);

        // print!("{}", nes.cpu);
    }

    // Ok(())
}
