mod debug;
mod window;
mod config;
mod global_state;
mod text;

use std::{fs::File, io::{Read, stdout}};

use jadeite::{Console, Cart};
use debug::DebugOut;
use window::JWindow;
use global_state::GlobalState;

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

    let mut global_state = GlobalState::init();
    let mut win = JWindow::new(&global_state);

    let mut counter = 0;

    loop {
        // nes.next();

        win.update(counter);

        for event in global_state.event_pump.poll_iter() {
            let _processed = win.process_event(&event);
        }

        if win.is_done() {
            break Ok(())
        }

        counter += 1;

        // let mut s = String::new();
        // nes.bus.print_page(&mut s, 0x0100).unwrap();
        // println!("{}", s);

        // print!("{}", nes.cpu);
    }

    // Ok(())
}
