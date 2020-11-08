mod cannon;
mod linalg;

fn main() {
    let gravity = linalg::V4::make_vector(0.0, -0.1, 0.0);
    let wind = linalg::V4::make_vector(-0.01, 0.0, 0.0);
    let vel = linalg::V4::make_vector(1.0, 1.0, 0.0).normalize();

    println!("{} steps", cannon::fire(&vel, &gravity, &wind))
}
