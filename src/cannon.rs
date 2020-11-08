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

pub fn fire(velocity: &linalg::V4, gravity: &linalg::V4, wind: &linalg::V4) -> i32 {
    let mut p = Projectile {
        pos: linalg::V4::make_point(0.0, 1.0, 0.0),
        velocity: *velocity
    };

    let v = linalg::V4::add(gravity, &wind);

    let mut count = 0;

    while p.pos.y() > 0.0 {
        p = tick(&p, &v);
        count += 1;
    };

    count
}
