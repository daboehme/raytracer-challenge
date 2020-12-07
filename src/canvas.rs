use std::error::Error;

use std::io::Write;

use crate::render::Color;

pub struct Canvas {
    pub width:  usize,
    pub height: usize,
    data:   Vec<Color>
}

impl Canvas {
    pub fn new(w: usize, h: usize, background: Color) -> Canvas {
        Canvas {
            width:  w,
            height: h,
            data:   vec![background; w*h]
        }
    }

    pub fn set(&mut self, x: usize, y: usize, color: Color) {
        self.data[y*self.width + x] = color
    }

    pub fn write_to_ppm(&self, output: &mut dyn Write) -> Result<(), Box<dyn Error>> {
        // write the header
        write!(output, "P3\n{} {}\n255\n", self.width, self.height)?;

        fn colorval(v: f32) -> u8 {
            unsafe {
                (v.max(0.0).min(1.0) * 255.0).to_int_unchecked::<u8>()
            }
        };

        for c in &self.data {
            write!(output, "{} {} {}\n", colorval(c.r), colorval(c.g), colorval(c.b))?;
        }

        Ok(())
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn write_to_ppm() {
        let mut canvas = Canvas::new(5, 3, Color::BLACK);

        canvas.set(0, 0, Color::new( 1.5, 0.0, 0.0));
        canvas.set(2, 1, Color::new( 0.0, 0.5, 0.0));
        canvas.set(4, 2, Color::new(-0.5, 0.0, 1.0));

        let mut output = Vec::<u8>::new();

        canvas.write_to_ppm(&mut output).expect("Failed to write");

        let expected = "P3
5 3
255
255 0 0
0 0 0
0 0 0
0 0 0
0 0 0
0 0 0
0 0 0
0 127 0
0 0 0
0 0 0
0 0 0
0 0 0
0 0 0
0 0 0
0 0 255
";

        assert_eq!(expected.as_bytes(), output);
    }
}