#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

mod vec3;

use std::env;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
use vec3::Vect3;

type Point = Vect3;
type Color = Vect3;

fn make_ppm() -> File {
    /* Creating an output file */
    let args: Vec<String> = env::args().collect(); // Command line arguments
    let filename = &args[1]; // Name of the output file
    let path = Path::new(filename); // Struct for working with file paths

    // Define settings for the output file
    let file = OpenOptions::new()
        .read(true)
        //.append(true)
        .write(true)
        .create(true)
        .open(path)
        .unwrap();

    return file
}

fn hello_ppm(file: &mut File){
    /* Image Dimensions */
    const IMG_WIDTH: usize = 256;
    const IMG_HEIGHT: usize = 256;

    // First, write the file header
    let header = format!("P3\n{} {}\n255\n", IMG_WIDTH, IMG_HEIGHT);
    match file.write_all(header.as_bytes()) {
        Ok(_) => (),
        Err(why) => eprintln!("Could not write. {}", why),
    }

    // Calculate RGB tripletes
    for row in (0..IMG_HEIGHT).rev() {
        eprintln!("Scan lines left {}", row);
        // Top to bottom row
        for col in 0..IMG_WIDTH {
            // Leftmost column to rightmost
            let pixel_color: Color = Vect3::new(
                col as f64 / (IMG_WIDTH - 1) as f64,
                row as f64 / (IMG_HEIGHT - 1) as f64,
                0.25,
            );

            match file.write_all(pixel_color.write_color().as_bytes()) {
                Ok(_) => (),
                Err(why) => eprintln!("Could not write {}", why),
            }
        }
    }
    eprintln!("Done.");
}

fn main() {
    let mut file = make_ppm();
    hello_ppm(&mut file);
}
