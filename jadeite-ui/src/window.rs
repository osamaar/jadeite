#![allow(dead_code, unused)]

use std::borrow::BorrowMut;

use rusttype::{point, Font, Scale, VMetrics};

use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::{Color, PixelFormatEnum},
    rect::Rect,
    render::{Canvas, Texture},
    video::Window,
    EventPump,
};

const WIDTH: u32 = 960;
const HEIGHT: u32 = 540;

pub struct JWindow<'a> {
    ctx: sdl2::Sdl,
    // win: sdl2::video::Window,
    canvas: Canvas<Window>,
    text_renderer: TextRenderer<'a>,
    screen_tex: Texture,
    event_pump: EventPump,
    pixel_buffer: Box<[u8]>,
    bg_color: Color,
    done: bool,
}

impl JWindow<'_> {
    pub fn new() -> Self {
        let ctx = sdl2::init().unwrap();
        let video = ctx.video().unwrap();

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

        // overlay_tex.with_lock(
        //     Rect::new(0, 0, 100, 100),
        //         |buf, pitch| {
        //             buf.fill(0xFF);
        //     }
        // );

        let event_pump = ctx.event_pump().unwrap();

        let pixel_buffer = vec![0u8; (WIDTH*HEIGHT*4) as usize];
        let pixel_buffer = pixel_buffer.into_boxed_slice();

        let bg_color = Color::RGBA(0x3B, 0x3B, 0x3A, 0xFF);

        Self {
            ctx,
            canvas,
            text_renderer,
            screen_tex,
            event_pump,
            pixel_buffer,
            bg_color,
            done: false,
        }
    }

    fn process_input(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    self.done = true;
                    break;
                }
                _ => {}
            }
        }
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn update(&mut self, counter: usize) {
        self.process_input();
        self.draw();
    }

    fn draw(&mut self) {
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

        // self.text_renderer.render_text(
        //     "Jadeite NES Emulator",
        //     &mut self.pixel_buffer,
        //     WIDTH as _,
        //     110,
        //     110
        // );
    }
}

struct TextRenderer<'a> {
    font: Font<'a>,
}

impl TextRenderer<'_> {
    fn new(fname: &str) -> Self {
        let v = std::fs::read("resources/OpenSans-Regular.ttf").unwrap();
        let font = Font::try_from_vec(v).unwrap();

        Self { font }
    }

    fn render_text(&self, text: &str, buf: &mut [u8], pitch: usize, x: i32, y: i32) {
        let scale = Scale::uniform(64.0);
        let color = (200, 200, 200);
        let vmetrics = self.font.v_metrics(scale);
        let line_height = (vmetrics.ascent - vmetrics.descent).ceil() as u32;
        let start = point(x as f32, y as f32);
        let glyphs = self.font.layout(text, scale, start);

        for glyph in glyphs {
            // println!("{:?}", glyph);
            // println!("{:?}", glyph.pixel_bounding_box());
            if let Some(bb) = glyph.pixel_bounding_box() {
                glyph.draw(|x, y, v| {
                    let xx = x + bb.min.x as u32;
                    let yy = y + bb.min.y as u32;
                    let i = (yy * 4 * pitch as u32 + xx*4) as usize;
                    let front = (v*255.0) as u8;
                    let blend_back =  (1.0 - v) as u8;

                    if let &[a, b, g, r] = &buf[i..i+4] {
                        buf[i]   = u8::max(front,a);
                        buf[i+1] = 255; // front + b * blend_back;
                        buf[i+2] = 255; // front + g * blend_back;
                        buf[i+3] = 255; // front + r * blend_back;
                        // temp[i*4] = (v*255.0) as u8;
                        // temp[i] = 0xFF;
                    }
                });
            }
        }

    }
}

fn blit_pixel_buffer(dst: &mut Texture, src: &[u8]) {
    dst.with_lock(Rect::new(0, 0, WIDTH, HEIGHT), |buf, pitch| {
        buf.copy_from_slice(src.as_ref());
    });

}