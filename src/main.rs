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
mod wavyplane;
mod world;

use std::time::{SystemTime};

fn main() {
    let t1 = SystemTime::now();

    let img = experiment::draw_world();

    let t2 = SystemTime::now();

    img.save("render.png").expect("Could not write file");

    let t3 = SystemTime::now();

    let render_t = t2.duration_since(t1).unwrap().as_millis();
    let write_t  = t3.duration_since(t2).unwrap().as_millis();

    println!("Done (render: {}ms, write: {}ms).", render_t, write_t);
}
