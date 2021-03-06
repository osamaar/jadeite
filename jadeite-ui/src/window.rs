#![allow(dead_code, unused)]

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;

use crate::global_state::GlobalState;

pub struct JWindow {
    canvas: Canvas<Window>,
    screen_tex: Texture,
    screen_buf: PixelBuffer,
    bg_color: Color,
    done: bool,
}

impl JWindow {
    pub fn new(global: &GlobalState, w: u32, h: u32) -> Self {
        let video = &global.video;

        let win = video.window("Jadeite", w, h).build().unwrap();
        let canvas = win.into_canvas().accelerated().build().unwrap();
        let screen_buf = PixelBuffer::new(w, h);

        let mut screen_tex = canvas
            .create_texture(
                PixelFormatEnum::RGBA8888,
                sdl2::render::TextureAccess::Streaming,
                w,
                h,
            )
            .unwrap();

        screen_tex.set_blend_mode(sdl2::render::BlendMode::Blend);

        screen_tex.with_lock(Rect::new(0, 0, w, h), |buf, _| {
            buf.fill(0x00);
        }).unwrap();

        let bg_color = Color::RGBA(0x3B, 0x3B, 0x3A, 0xFF);

        Self {
            canvas,
            screen_tex,
            screen_buf,
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
            _ => false,
        }
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn update(&mut self) {}

    pub fn clear(&mut self) {
        self.canvas.set_draw_color(self.bg_color);
        self.canvas.clear();
    }

    pub fn draw(&mut self) {
        self.screen_buf.blit_to_texture(&mut self.screen_tex);

        let q = self.screen_tex.query();
        let bb = (0i32, 0i32, q.width, q.height);
        self.canvas.copy(&self.screen_tex, None, Some(bb.into()));

        self.canvas.present();
    }

    pub fn buffer(&mut self) -> &mut PixelBuffer {
        &mut self.screen_buf
    }
}

pub struct PixelBuffer {
    data: Box<[u8]>,
    w: u32,
    h: u32,
}

impl PixelBuffer {
    pub fn new(w: u32, h: u32) -> Self {
        let data = vec![0u8; (w*h*4) as usize];
        let data = data.into_boxed_slice();
        Self { data, w, h }
    }

    pub fn width(&self) -> u32 { self.w }
    pub fn height(&self) -> u32 { self.h }
    pub fn pixels(&self) -> &[u8] { &*self.data }
    pub fn pixels_mut(&mut self) -> &mut [u8] { &mut *self.data }

    fn blit_to_texture(&self, dst: &mut Texture) {
        dst.with_lock(Rect::new(0, 0, self.w, self.h), |buf, pitch| {
            buf.copy_from_slice(self.pixels());
        });
    }

    pub fn blit_to_buffer(&self, dst: &mut [u8]) {
        dst.copy_from_slice(self.pixels());
    }

    pub fn clear(&mut self) {
        self.data.fill(0x00);
    }
}
