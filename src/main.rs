mod canvas;
mod experiment;
mod linalg;
mod objects;
mod ray;
mod render;
mod transform;

use std::fs::File;

fn main() {
    let canvas = experiment::draw_sphere_lighting();

    let mut file = File::create("render.ppm").expect("Could not open file");
    canvas.write_to_ppm(&mut file).expect("Could not write canvas");
}
