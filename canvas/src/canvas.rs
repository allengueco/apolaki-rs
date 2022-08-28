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
            pixels: vec![vec![Color::default(); width]; height],
        }
    }

    pub fn with_default_color(&mut self, default_color: Color) -> Self {
        Self {
            width: self.width,
            height: self.height,
            pixels: vec![vec![default_color; self.width]; self.height],
        }
    }

    pub fn write(&mut self, x: usize, y: usize, pixel: Color) {
        self.pixels[y][x] = pixel
    }

    pub fn to_ppm_string(&self) -> String {
        self.ppm_header() + self.ppm_body().as_str()
    }

    pub fn ppm_header(&self) -> String {
        format!("P3\n{} {}\n255\n", self.width, self.height)
    }

    pub fn ppm_body(&self) -> String {
        let ppm_strings: Vec<Vec<String>> = self
            .pixels
            .iter()
            .map(|ps| ps.iter().map(|c| c.to_ppm_color()).collect())
            .collect();

        let mut acc = String::new();

        for pixel_row in ppm_strings {
            let mut res = String::new();
            let mut index = 70; // max line length
            let mut chars = pixel_row.join(" ");

            // we try to find the character which is at an index of a multiple of 70 (the max range)
            while let Some(char) = chars.chars().nth(index) {
                match char {
                    ' ' => chars.replace_range(index..index + 1, "\n"),
                    _ => {
                        if let Some(idx) = chars[index - 70..index].rfind(' ') {
                            chars.replace_range(idx..idx + 1, "\n")
                        }
                    }
                }
                index += 70;
            }

            res.push_str(chars.as_str());
            res += "\n";
            acc.push_str(&res);
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
