use sdl2::{EventPump, event::Event, keyboard::Keycode, pixels::Color, render::Canvas, video::Window};

pub struct JWindow {
    ctx: sdl2::Sdl,
    // win: sdl2::video::Window,
    canvas: Canvas<Window>,
    event_pump: EventPump,
    bg_color: Color,
}

impl JWindow {
    pub fn create() -> Self {
        let ctx = sdl2::init().unwrap();
        let video = ctx.video().unwrap();

        let win = video.window("Jadeite", 960, 540)
            .build()
            .unwrap();

        let canvas = win.into_canvas()
            .accelerated()
            .build()
            .unwrap();

        let event_pump = ctx.event_pump().unwrap();
        let bg_color = Color::RGBA(0x22, 0x22, 0x22, 0xFF);


        Self { ctx, canvas, event_pump, bg_color }
    }

    fn process_input(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    break
                },
                Event::KeyDown { keycode, ..} => {
                    if keycode == Some(Keycode::Escape) {
                        break
                    }
                },
                _ => {}
            }
        }
    }

    pub fn update(&mut self, counter: usize) {
        self.process_input();
        self.draw();
    }

    fn draw(&mut self) {
        self.canvas.set_draw_color(self.bg_color);
        self.canvas.clear();
        self.canvas.present();
    }

}