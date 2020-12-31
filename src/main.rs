mod camera;
mod canvas;
mod color;
mod experiment;
mod lighting;
mod linalg;
mod plane;
mod ray;
mod shape;
mod sphere;
mod transform;
mod world;

use std::fs::File;

fn main() {
    let canvas = experiment::draw_world();

    let mut file = File::create("render.ppm").expect("Could not open file");
    canvas.write_to_ppm(&mut file).expect("Could not write canvas");
}
