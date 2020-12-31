use crate::lighting::Material;
use crate::linalg;
use crate::linalg::{M4,V4};
use crate::ray::Ray;

pub trait BaseShape {
    fn intersect(&self, r: &Ray) -> Vec<f32>;
    fn normal_at(&self, p: V4) -> V4;
}

pub struct Shape {
    base: Box<dyn BaseShape>,
    transform_i: M4,
    mat: Material
}

impl Shape {
    pub fn new(shape: Box<dyn BaseShape>, mat: &Material, trans: &M4) -> Shape {
        Shape {
            base: shape,
            transform_i: trans.invert(),
            mat: *mat
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<f32> {
        self.base.intersect(&ray.apply(&self.transform_i))
    }

    pub fn normal_at(&self, p: V4) -> V4 {
        let p = linalg::mvmul(&self.transform_i, &p);
        let n = self.base.normal_at(p);
        let n = linalg::mvmul(&self.transform_i.transpose(), &n);

        V4::make_vector(n.x(), n.y(), n.z()).normalize()
    }

    pub fn material(&self) -> Material {
        self.mat
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::color::Color;
    use crate::transform::Transform;

    use float_cmp::*;

    use std::rc::Rc;
    use std::cell::RefCell;

    const DEFAULT_MAT: Material = Material {
        color: Color { r: 1.0, g: 0.2, b: 1.0 },
        ambient: 0.1,
        diffuse: 0.9,
        specular: 0.9,
        shininess: 200.0
    };

    struct TestShape {
        ray: Rc<RefCell<Ray>>
    }

    impl BaseShape for TestShape {
        fn intersect(&self, r: &Ray) -> Vec<f32> {
            *self.ray.borrow_mut() = *r;
            vec![]
        }

        fn normal_at(&self, p: V4) -> V4 {
            p
        }
    }

    #[test]
    fn transform_intersect() {
        let res = Ray {
            origin: V4::make_point(0.0, 0.0, 0.0),
            direction: V4::make_vector(0.0, 0.0, 0.0)
        };
        let res = Rc::new(RefCell::new(res));

        {
            let t = Transform::new().scale(2.0, 2.0, 2.0);
            let s = Shape::new(Box::new(TestShape { ray: Rc::clone(&res) }), &DEFAULT_MAT, &t.matrix);

            let r = Ray {
                origin: V4::make_point(0.0, 0.0, -5.0),
                direction: V4::make_vector(0.0, 0.0, 1.0)
            };

            s.intersect(&r);
        }

        assert!(approx_eq!(V4, res.borrow().origin, V4::make_point(0.0, 0.0, -2.5)));
        assert!(approx_eq!(V4, res.borrow().direction, V4::make_vector(0.0, 0.0, 0.5)));
    }

    #[test]
    fn transform_normal() {
        let ray = Ray {
            origin: V4::make_point(0.0, 0.0, 0.0),
            direction: V4::make_vector(0.0, 0.0, 0.0)
        };

        let t = Transform::new().translate(0.0, 1.0, 0.0);
        let s = Shape::new(Box::new(TestShape { ray: Rc::new(RefCell::new(ray)) }), &DEFAULT_MAT, &t.matrix);

        let n = s.normal_at(V4::make_point(0.0, 1.70711, -0.70711));
        assert!(approx_eq!(V4, n, V4::make_vector(0.0, 0.70711, -0.70711), epsilon = 0.0001));
    }
}
