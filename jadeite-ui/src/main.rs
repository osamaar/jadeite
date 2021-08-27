use jadeite::{Console, Cart};

fn main() -> Result<(), ()> {
    let mut cart = Cart::read_file("resources/nestest.nes").map_err(|_| ())?;
    let mut nes = Console::new();
    nes.insert_cart(&mut cart);
    nes.reset_to(0xc000);
    // nes.reset();

    println!("{}", nes);
    println!();

    loop {
        nes.next();
        print!("{}", nes.cpu);
    }

    // Ok(())
}
