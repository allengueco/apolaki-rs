mod canvas;

use apolaki_color;

pub use canvas::Canvas;

#[cfg(test)]
mod tests {
    use crate::canvas::PixelIndex;
    use crate::Canvas;
    use apolaki_color::Color;

    #[test]
    fn creating_a_canvas() {
        let c = Canvas::with_size(10, 20);

        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);

        let is_all_white = c.pixels.iter().flatten().all(|&p| p == (0, 0, 0).into());

        assert!(is_all_white);
    }

    #[test]
    fn writing_a_pixel_to_canvas() {
        let red: Color = (1, 0, 0).into();
        let mut c = Canvas::with_size(10, 20);
        dbg!(&c);
        c.write(2, 3, red);

        assert_eq!(red, c[PixelIndex(2, 3)])
    }

    #[test]
    fn constructing_the_ppm_header() {
        let c = Canvas::with_size(5, 3);
        let ppm_header = c.ppm_header();

        assert_eq!(
            r"
        P3
        5 3
        255
        ",
            ppm_header
        )
    }
}
