extern crate clap;
extern crate image;
extern crate raytracer;
extern crate serde;
extern crate serde_json;

use clap::{App, Arg};
use image::ImageFormat;
use raytracer::scene::*;
use std::fs::{File, OpenOptions};
use std::time::Instant;

fn main() {
    let begin = Instant::now();
    let app = App::new("raytracer")
        .version("1.0")
        .author("Merzouk")
        .arg(
            Arg::with_name("scene")
                .help("Sets the scene file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("image")
                .help("Sets the output image file")
                .required(true)
                .index(2),
        );
    let matches = app.get_matches();

    let scene_path = matches.value_of("scene").unwrap();
    let scene_file = File::open(scene_path).expect("File not found");

    let image_path = matches.value_of("image").unwrap();
    let mut image_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(image_path)
        .unwrap();

    let scene: Scene = serde_json::from_reader(scene_file).unwrap();

    let image = raytracer::render(&scene);

    image.save(&mut image_file, ImageFormat::PPM).unwrap();
    println!("Image generated successfully in : {:.2?}", begin.elapsed());
}
