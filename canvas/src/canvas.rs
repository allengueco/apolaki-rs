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

    pub fn to_ppm_string(&self) -> String {
        todo!()
    }

    pub fn ppm_header(&self) -> String {
        format!("P3\n{} {}\n255\n", self.width, self.height)
    }

    pub fn ppm_body(&self) -> String {
        const MAX_LENGTH: usize = 70;
        let ppm_strings: Vec<&str> = self
            .pixels
            .iter()
            .flatten()
            .flat_map(|c| c.to_ppm_color().split(" "))
            .collect();

        let mut acc = String::new();
        let mut current_line_length: usize = 0;

        for ppm_str in ppm_strings {
            acc.push_str(ppm_str.clone());

            match current_line_length {
                0..=68 => {
                    acc.push(' ');
                    current_line_length = acc.len()
                }
                _ => {
                    acc.push('\n');
                    current_line_length = 0
                }
            }
        }

        acc
    }
}

#[derive(Copy, Clone, Debug)]
pub struct PixelIndex(pub usize, pub usize);

impl Index<PixelIndex> for Canvas {
    type Output = Color;

    fn index(&self, index: PixelIndex) -> &Self::Output {
        &self.pixels[index.1][index.0]
    }
}

// Top right starts at 0,
pub enum SliceIndex {
    Row(usize),
    Column(usize),
}

impl Index<SliceIndex> for Canvas {
    type Output = [Color];

    fn index(&self, index: SliceIndex) -> &Self::Output {
        todo!();
        match index {
            SliceIndex::Row(r) => &self.pixels.index(r),

            SliceIndex::Column(c) => {
                unimplemented!()
            }
        }
    }
}
