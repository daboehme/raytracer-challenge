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
use std::time::{SystemTime};

fn main() {
    let t1 = SystemTime::now();

    let canvas = experiment::draw_world();

    let t2 = SystemTime::now();

    let mut file = File::create("render.ppm").expect("Could not open file");
    canvas.write_to_ppm(&mut file).expect("Could not write canvas");

    let t3 = SystemTime::now();

    let render_t = t2.duration_since(t1).unwrap().as_millis();
    let write_t  = t3.duration_since(t2).unwrap().as_millis();

    println!("Done (render: {}ms, write: {}ms).", render_t, write_t);
}
