mod cannon;
mod canvas;
mod linalg;

use std::fs::File;

fn main() {
    let gravity = linalg::V4::make_vector(0.0, -0.1, 0.0);
    let wind = linalg::V4::make_vector(-0.01, 0.0, 0.0);
    let vel = linalg::V4::make_vector(1.0, 1.8, 0.0).normalize().mult(11.25);
    let mut canvas = canvas::Canvas::new(900, 500, canvas::Color::BLACK);

    println!("{} steps", cannon::fire(&vel, &gravity, &wind, &mut canvas));

    let mut file = File::create("cannon.ppm").expect("Could not open file");
    canvas.write_to_ppm(&mut file).expect("Could not write canvas");
}
