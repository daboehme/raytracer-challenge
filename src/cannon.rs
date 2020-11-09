use crate::canvas;
use crate::linalg;

struct Projectile {
    pos: linalg::V4,
    velocity: linalg::V4
}

fn tick(p: &Projectile, v: &linalg::V4) -> Projectile {
    Projectile {
        pos: linalg::V4::add(&p.pos, &p.velocity),
        velocity: linalg::V4::add(&p.velocity, &v)
    }
}

pub fn fire(velocity: &linalg::V4, gravity: &linalg::V4, wind: &linalg::V4, canvas: &mut canvas::Canvas) -> i32 {
    let mut p = Projectile {
        pos: linalg::V4::make_point(0.0, 1.0, 0.0),
        velocity: *velocity
    };

    fn fits(p: &Projectile, canvas: &canvas::Canvas) -> bool {
        p.pos.x() > 0.0 && p.pos.x() < canvas.width  as f32 &&
        p.pos.y() > 0.0 && p.pos.y() < canvas.height as f32
    }

    let v = linalg::V4::add(gravity, &wind);
    let c = canvas::Color::new(0.8, 0.2, 0.2);

    let mut count = 0;

    while p.pos.y() > 0.0 {
        p = tick(&p, &v);
        count += 1;

        if fits(&p, canvas) {
            canvas.set(p.pos.x() as usize, p.pos.y() as usize, c)
        }
    };

    count
}
