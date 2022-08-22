use apolaki_color::Color;
use std::borrow::Borrow;
use std::ops::Index;
use std::path::Iter;

#[derive(Debug)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Vec<Color>>,
}

impl Canvas {
    pub fn with_size(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![vec![Color::default(); height]; width],
        }
    }

    pub fn write(&mut self, x: usize, y: usize, pixel: Color) {
        self.pixels[y][x] = pixel
    }
}

#[derive(Copy, Clone, Debug)]
pub struct PixelIndex(pub usize, pub usize);

impl Index<PixelIndex> for Canvas {
    type Output = Color;

    fn index(&self, index: PixelIndex) -> &Self::Output {
        dbg!(&self.pixels);
        &self.pixels[index.1][index.0]
    }
}

// Top right starts at 0,
pub enum SliceIndex {
    Row(usize),
    Column(usize)
}

impl Index<SliceIndex> for Canvas {
    type Output = [Color];

    fn index(&self, index: SliceIndex) -> &Self::Output {
        todo!();
        match index {
            SliceIndex::Row(r) => {
                &self.pixels.index(r)
            },

            SliceIndex::Column(c) => {
                unimplemented!()
            }
        }
    }
}
