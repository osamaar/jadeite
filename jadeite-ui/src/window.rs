#![allow(dead_code, unused)]

use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;
use sdl2::video::Window;
use sdl2::render::{Canvas, Texture};
use sdl2::pixels::{Color, PixelFormatEnum};

use crate::global_state::GlobalState;
use crate::text::TextRenderer;

const WIDTH: u32 = 960;
const HEIGHT: u32 = 540;

pub struct JWindow<'a> {
    canvas: Canvas<Window>,
    text_renderer: TextRenderer<'a>,
    screen_tex: Texture,
    pixel_buffer: Box<[u8]>,
    bg_color: Color,
    done: bool,
}

impl JWindow<'_> {
    pub fn new(global: &GlobalState) -> Self {
        let context = &global.context;
        let video = &global.video;

        let win = video.window("Jadeite", WIDTH, HEIGHT).build().unwrap();
        let canvas = win.into_canvas().accelerated().build().unwrap();
        let text_renderer = TextRenderer::new("resources/Tajawal-Regular.ttf");

        let mut screen_tex = canvas
            .create_texture(
                PixelFormatEnum::RGBA8888,
                sdl2::render::TextureAccess::Streaming,
                WIDTH,
                HEIGHT,
            )
            .unwrap();

        screen_tex.set_blend_mode(sdl2::render::BlendMode::Blend);

        screen_tex.with_lock(Rect::new(0, 0, WIDTH, HEIGHT), |buf, pitch| {
            buf.fill(0x00);
        });

        let pixel_buffer = vec![0u8; (WIDTH*HEIGHT*4) as usize];
        let pixel_buffer = pixel_buffer.into_boxed_slice();

        let bg_color = Color::RGBA(0x3B, 0x3B, 0x3A, 0xFF);

        Self {
            canvas,
            text_renderer,
            screen_tex,
            pixel_buffer,
            bg_color,
            done: false,
        }
    }

    pub fn id(&self) -> u32 {
        self.canvas.window().id()
    }

    pub fn process_event(&mut self, event: &Event) -> bool {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => {
                self.done = true;
                true
            }
            _ => false
        }
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn update(&mut self, counter: usize) {
    }

    pub fn draw(&mut self) {
        self.canvas.set_draw_color(self.bg_color);
        self.canvas.clear();
        self.pixel_buffer.fill(0);

        self.overlay();

        blit_pixel_buffer(&mut self.screen_tex, &self.pixel_buffer);

        let q = self.screen_tex.query();
        let bb = (0i32, 0i32, q.width, q.height);
        self.canvas.copy(&self.screen_tex, None, Some(bb.into()));

        self.canvas.present();
    }

    fn overlay(&mut self) {
        self.text_renderer.render_text(
            "Jadeite NES Emulator",
            &mut self.pixel_buffer,
            WIDTH as _,
            100,
            100
        );
    }
}

fn blit_pixel_buffer(dst: &mut Texture, src: &[u8]) {
    dst.with_lock(Rect::new(0, 0, WIDTH, HEIGHT), |buf, pitch| {
        buf.copy_from_slice(src.as_ref());
    });

}