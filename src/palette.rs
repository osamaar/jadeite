use std::io::Read;
use std::fs::File;
use std::ops::{Index, IndexMut};


#[derive(Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

/// Represents a palette in `.pal` format. 64 entries stored as: R G B. 192 bytes total.
pub struct Palette([Color; 64]);

impl Index<usize> for Palette {
    type Output = Color;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Palette {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Palette {
    pub fn new() -> Self {
        Self([Color { r: 0, g: 0, b: 0 } ; 64])
    }

    pub fn read_data<R: Read>(src: &mut R) -> Result<Self, ()> {
        let mut data = [0u8; 192];
        src.read_exact(&mut data).or_else(|_| Err(()))?;

        let mut p = Palette::new();
        for i in 0..64 {
            let ii = i*3;
            p[i].r = data[ii];
            p[i].g = data[ii+1];
            p[i].b = data[ii+2];
        } 

        Ok(p)
    }

    pub fn from_file(filename: &str) -> Result<Self, ()> {
        let mut f = File::open(filename).or_else(|_| Err(()))?;
        Self::read_data(&mut f)
    }
}