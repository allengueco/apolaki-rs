mod canvas;

use apolaki_color;

pub use canvas::Canvas;

#[cfg(test)]
mod tests {
    use crate::Canvas;

    #[test]
    fn creating_a_canvas() {
        let c = Canvas::with_size(10, 20);

        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);
    }
}
