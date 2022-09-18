mod color;

pub use color::Color;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn colors_are_rgb_tuples() {
        let c: Color = (-0.5, 0.4, 1.7).into();

        assert_eq!(-0.5, c.r());
        assert_eq!(0.4, c.g());
        assert_eq!(1.7, c.b());
    }

    #[test]
    fn adding_colors() {
        let c1: Color = (0.9, 0.6, 0.75).into();
        let c2: Color = (0.7, 0.1, 0.25).into();

        assert_eq!(Color::from((1.6, 0.7, 1.0)), c1 + c2);
    }

    #[test]
    fn subtracting_colors() {
        let c1: Color = (0.9, 0.6, 0.75).into();
        let c2: Color = (0.7, 0.1, 0.25).into();

        assert_eq!(Color::from((0.2, 0.5, 0.5)), c1 - c2);
    }

    #[test]
    fn multiplying_color_by_scalar() {
        let c: Color = (0.2, 0.3, 0.4).into();

        assert_eq!(Color::from((0.4, 0.6, 0.8)), c * 2);
        assert_eq!(Color::from((0.4, 0.6, 0.8)), c * 2_f64);
    }

    #[test]
    fn multiplying_colors() {
        let c1: Color = (1, 0.2, 0.4).into();
        let c2: Color = (0.9, 1, 0.1).into();

        assert_eq!(Color::from((0.9, 0.2, 0.04)), c1 * c2);
    }
}
