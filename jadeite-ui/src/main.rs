mod debug;
mod window;
mod config;
mod global_state;
mod text;

use std::fs::File;
use std::io::Read;

use jadeite::{Console, Cart};
use debug::DebugOut;
use text::TextRenderer;
use window::{JWindow, PixelBuffer};
use global_state::GlobalState;

const WIDTH: u32 = 960;
const HEIGHT: u32 = 540;

fn main() -> Result<(), ()> {
    let mut cart = Cart::read_file("resources/nestest.nes").map_err(|_| ())?;
    // let mut cart = Cart::read_file("resources/ex.nes").map_err(|_| ())?;
    let mut nes = Console::new();
    nes.insert_cart(&mut cart);
    // nes.reset_to(0xc000);
    nes.reset();

    let mut f = File::open("resources/nestest.log").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).map_err(|_| panic!("hurr durr!"))?;

    // let mut out = DebugOut::new(&s);
    // let mut out = &mut std::io::stdout();
    // nes.cpu.debug_to(&mut out);

    // println!("{}", nes);

    let mut global_state = GlobalState::init();

    let mut overlay = PixelBuffer::new(WIDTH, HEIGHT);
    let mut win = JWindow::new(&global_state, WIDTH, HEIGHT);
    let text_renderer = TextRenderer::new("resources/OpenSans-Regular.ttf");


    let cpf = 29833;
    let cpf = cpf/100;

    loop {
        // Input
        for event in global_state.event_pump.poll_iter() {
            let _processed = win.process_event(&event);
        }

        // Update
        for _ in 0..cpf {
            nes.step();
            // let mut s = String::new();
            // nes.bus.print_page(&mut s, 0x0100).unwrap();
            // println!("{}", s);
            // print!("{}", nes.cpu);
        }

        // Draw
        win.clear();
        let ppuout = &nes.ppu.borrow().output;
        let buf = win.buffer();
        let w = buf.width() as usize;
        let win_pixels = win.buffer().pixels_mut();

        for y in 0..240 {
            for x in 0..256 {
                let p = ppuout[y * 256 + x];

                let i = y * w + x;
                win_pixels[i * 4] = p.r;
                win_pixels[i * 4 + 1] = p.g;
                win_pixels[i * 4 + 2] = p.b;
                win_pixels[i * 4 + 3] = p.a;
            }
        }

        update_overlay(&mut overlay, &text_renderer, &nes);
        overlay.blit_to_buffer_with_alpha(win_pixels);

        win.draw();

        if win.is_done() {
            break Ok(());
        }
    }

    // Ok(())
}

fn update_overlay(pb: &mut PixelBuffer, tr: &TextRenderer, nes: &Console) {
    pb.clear();

    // nes.bus.print_page(&mut s, 0x00).unwrap();

    // tr.render_text(
    //     "Jadeite NES Emulator",
    //     pb,
    //     100,
    //     100,
    // );
    // tr.render_text(&s, pb, 100, 150);

    let ss = format!("{}", nes.cpu.clock_count);
    tr.render_text(&ss, pb, 300, 50);

}
