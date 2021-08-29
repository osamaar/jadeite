use jadeite::{Console, Cart};

fn main() -> Result<(), ()> {
    let mut cart = Cart::read_file("resources/nestest.nes").map_err(|_| ())?;
    let mut nes = Console::new();
    nes.insert_cart(&mut cart);
    nes.reset_to(0xc000);
    // nes.reset();

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
