#![allow(dead_code)]

mod camera;
mod canvas;
mod color;
mod config;
mod cube;
mod cylinder;
mod lighting;
mod linalg;
mod material;
mod pattern;
mod plane;
mod ray;
mod sceneparser;
mod shape;
mod sphere;
mod transform;
mod world;

use std::env;
use std::error;
use std::fs::File;
use std::io::prelude::*;
use std::process;
use std::time::{SystemTime};

use camera::Camera;
use config::{Config,ConfigError};
use world::World;

fn process(config: &Config, camera: &Camera, world: &World) {
    let t1 = SystemTime::now();

    let img = camera.render(&world);

    let t2 = SystemTime::now();

    img.save(&config.output_file_name).expect("Could not write file");

    let t3 = SystemTime::now();

    let render_t = t2.duration_since(t1).unwrap().as_millis();
    let write_t  = t3.duration_since(t2).unwrap().as_millis();

    println!("Done (render: {}ms, write: {}ms).", render_t, write_t);
}

fn setup(config: &Config) -> Result<(Camera, World), Box<dyn error::Error>> {
    let mut file = File::open(&config.input_file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let (camera, world) = sceneparser::read_yaml_scene_config(&contents)?;

    Ok((camera,world))
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = match Config::new(&args) {
        Ok(cfg) => cfg,
        Err(ConfigError::UsageOutputRequested) => {
            process::exit(0);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1)
        }
    };

    let (camera, world) = setup(&config).unwrap_or_else(|x| {
            eprintln!("Setup error: {}", x);
            process::exit(1)
        });

    process(&config, &camera, &world);
}
