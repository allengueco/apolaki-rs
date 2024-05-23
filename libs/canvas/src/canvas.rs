use apolaki_color::Color;
use std::ops::Index;

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
            let mut index = 70; // max line length
            let mut chars = pixel_row.join(" ");

            // we try to find the character at an index of a multiple of 70 (the max line length)
            while let Some(char) = chars.chars().nth(index) {
                match char {
                    // if it's a blank space already, replace with newline
                    ' ' => chars.replace_range(index..index + 1, "\n"),
                    // if not, we find the last blank space from the right.
                    _ => {
                        if let Some(offset) = chars[index - 70..index].rfind(' ') {
                            let computed_index = index - 70 + offset;
                            chars.replace_range(computed_index..computed_index + 1, "\n")
                        }
                    }
                }
                index += 70;
            }

            chars.push('\n');
            acc.push_str(&chars);
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
