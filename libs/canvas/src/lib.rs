mod canvas;

pub use canvas::Canvas;

#[cfg(test)]
mod canvas_tests {
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
        c.write(2, 3, red);

        assert_eq!(red, c[PixelIndex(2, 3)])
    }

    #[test]
    fn constructing_the_ppm_header() {
        let c = Canvas::with_size(5, 3);
        let ppm_header = c.ppm_header();

        let expected = "\
            P3\n\
            5 3\n\
            255\n";
        assert_eq!(expected, ppm_header)
    }

    #[test]
    fn constructing_the_ppm_pixel_data() {
        let mut c = Canvas::with_size(5, 3);
        let c1: Color = (1.5, 0, 0).into();
        let c2: Color = (0, 0.5, 0).into();
        let c3: Color = (-0.5, 0, 1).into();

        c.write(0, 0, c1);
        c.write(2, 1, c2);
        c.write(4, 2, c3);

        let expected = "\
            255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n\
            0 0 0 0 0 0 0 128 0 0 0 0 0 0 0\n\
            0 0 0 0 0 0 0 0 0 0 0 0 0 0 255\n";

        let actual = c.ppm_body();
        assert_eq!(expected, actual);
    }

    #[test]
    fn splitting_long_lines_in_ppm_files() {
        let c = Canvas::with_size(10, 2).with_default_color((1, 0.8, 0.6).into());

        let ppm_body = c.ppm_body();
        println!("{}", &c.to_ppm_string());
        let expected = "\
            255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n\
            153 255 204 153 255 204 153 255 204 153 255 204 153\n\
            255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n\
            153 255 204 153 255 204 153 255 204 153 255 204 153\n";

        assert_eq!(expected, ppm_body);
    }

    #[test]
    fn ppm_files_are_terminated_by_a_newline_character() {
        let c = Canvas::with_size(10, 2);

        let ppm = c.to_ppm_string();
        println!("{}", &ppm);
        assert!(ppm.ends_with('\n'));
    }
}
