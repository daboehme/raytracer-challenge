use crate::color::Color;
use crate::linalg::{M4,V4};

use std::rc::Rc;

pub trait Pattern: std::fmt::Debug {
    fn color_at(&self, p: V4) -> Color;
}


#[derive(Copy,Clone,Debug,PartialEq)]
pub struct Solid {
    color: Color
}

impl Solid {
    pub fn new(c: Color) -> Solid {
        Solid { color: c }
    }
}

impl Pattern for Solid {
    fn color_at(&self, _: V4) -> Color {
        self.color
    }
}


#[derive(Clone,Debug)]
pub struct TransformedPattern {
    pattern: Rc<dyn Pattern>,
    transform_i: M4
}

impl TransformedPattern {
    pub fn new<T: Pattern + 'static>(p: T, m: &M4) -> TransformedPattern {
        TransformedPattern {
            pattern: Rc::new(p),
            transform_i: m.invert()
        }
    }

    pub fn new_from_rc(p: Rc<dyn Pattern>, m: &M4) -> TransformedPattern {
        TransformedPattern {
            pattern: p,
            transform_i: m.invert()
        }
    }
}

impl Pattern for TransformedPattern {
    fn color_at(&self, p: V4) -> Color {
        self.pattern.color_at(self.transform_i * p)
    }
}


#[derive(Copy,Clone,Debug,PartialEq)]
pub struct Stripes {
    a: Color,
    b: Color
}

impl Stripes {
    pub fn new(a: Color, b: Color) -> Stripes {
        Stripes { a: a, b: b }
    }
}

impl Pattern for Stripes {
    fn color_at(&self, p: V4) -> Color {
        if (p.x().floor() as i32) % 2 == 0 {
            self.a
        } else {
            self.b
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::transform::Transform;
    use super::*;

    use float_cmp::*;

    #[test]
    fn stripes() {
        let s = Stripes::new(Color::WHITE, Color::BLACK);

        assert_eq!(s.color_at(V4::new_point( 0.0, 0.0, 0.0)), Color::WHITE);
        assert_eq!(s.color_at(V4::new_point( 0.0, 1.0, 0.0)), Color::WHITE);
        assert_eq!(s.color_at(V4::new_point( 0.0, 2.0, 0.0)), Color::WHITE);

        assert_eq!(s.color_at(V4::new_point( 0.0, 0.0, 1.0)), Color::WHITE);
        assert_eq!(s.color_at(V4::new_point( 0.0, 0.0, 2.0)), Color::WHITE);

        assert_eq!(s.color_at(V4::new_point( 0.9, 0.0, 0.0)), Color::WHITE);
        assert_eq!(s.color_at(V4::new_point( 1.0, 0.0, 0.0)), Color::BLACK);
        assert_eq!(s.color_at(V4::new_point(-0.1, 0.0, 0.0)), Color::BLACK);
        assert_eq!(s.color_at(V4::new_point(-1.0, 0.0, 0.0)), Color::BLACK);
        assert_eq!(s.color_at(V4::new_point(-1.1, 0.0, 0.0)), Color::WHITE);
    }

    #[derive(Copy,Clone,Debug)]
    struct TestPattern ();

    impl Pattern for TestPattern {
        fn color_at(&self, p: V4) -> Color {
            Color::from(p)
        }
    }

    #[test]
    fn transformed_pattern() {
        let t = Transform::new().scale(2.0, 2.0, 2.0);
        let s = TransformedPattern::new(TestPattern(), &t.matrix);
        let r = V4::from(s.color_at(V4::new_point(2.0, 3.0, 4.0)));

        assert!(approx_eq!(V4, r, V4::new_vector(1.0, 1.5, 2.0), epsilon = 0.0001));
    }
}
