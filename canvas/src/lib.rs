mod canvas;

use apolaki_color;

pub use canvas::Canvas;

#[cfg(test)]
mod tests {
    use apolaki_color::Color;
    use crate::Canvas;
    use crate::canvas::PixelIndex;

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
}
