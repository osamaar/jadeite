#![allow(dead_code, unused)]

use rusttype::{Font, Scale, point};

use crate::window::PixelBuffer;

pub struct TextRenderer<'a> {
    font: Font<'a>,
}

impl TextRenderer<'_> {
    pub fn new(fname: &str) -> Self {
        let v = std::fs::read(fname).unwrap();
        let font = Font::try_from_vec(v).unwrap();

        Self { font }
    }

    pub fn render_text(&self, text: &str, pb: &mut PixelBuffer, x: i32, y: i32) {
        let scale = Scale::uniform(64.0);
        let color = (200, 200, 200);
        let vmetrics = self.font.v_metrics(scale);
        let line_height = (vmetrics.ascent - vmetrics.descent).ceil() as u32;
        let start = point(x as f32, y as f32);
        let glyphs = self.font.layout(text, scale, start);

        for glyph in glyphs {
            if let Some(bb) = glyph.pixel_bounding_box() {
                glyph.draw(|x, y, v| {
                    let xx = x + bb.min.x as u32;
                    let yy = y + bb.min.y as u32;
                    let i = (yy * 4 * pb.width() as u32 + xx*4) as usize;
                    let front = (v*255.0) as u8;
                    let blend_back =  (1.0 - v) as u8;

                    let pixels = pb.pixels_mut();

                    if let &[a, b, g, r] = &pixels[i..i+4] {
                        pixels[i]   = u8::max(front,a);
                        pixels[i+1] = 255;
                        pixels[i+2] = 255;
                        pixels[i+3] = 255;
                    }
                });
            }
        }

    }
}
