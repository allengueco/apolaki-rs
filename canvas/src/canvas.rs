use apolaki_color::Color;
use std::borrow::Borrow;
use std::path::Iter;

pub struct Canvas {
    pub width: isize,
    pub height: isize,
    pixels: Vec<Vec<Color>>,
}

impl Canvas {
    pub fn with_size(width: isize, height: isize) -> Self {
        Self {
            width,
            height,
            pixels: Vec::default(),
        }
    }

    pub fn pixels(&self) -> impl Iterator<Item = &Color> + '_ {
        self.pixels.iter().flatten()
    }

    fn get_row(&self, index: isize) -> Vec<&Color> {
        todo!()
    }

    fn get_col(&self, index: isize) -> Vec<&Color> {
        todo!()
    }
}
