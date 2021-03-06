#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

mod camera;
mod ray;
mod vec3;

use camera::Camera;
use ray::Ray;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
use vec3::Vect3;

type Point = Vect3;
type Color = Vect3;

fn main() {
    let mut file = make_ppm();
    render(&mut file);
}

fn ray_color(ray: &Ray) -> Color {

    /*
    (LERP):= blended_value = (1-t)*start_vlaue + t*end_value
    */

    let unit_direction = ray.direction().unit_vector();
    /*
    Gradient direction determined by the unit vector component
    Gradient occurs at all because the direction vector is normalized
    .:. blending is totally controlled by `t`, not position on the plane
    */
    let t = 0.5 * (unit_direction.y + 1.0); 
    // LERP between white and blue in this case
    (1.0 - t) * Color::new(1., 1., 1.) + t * Color::new(0.5, 0.7, 1.)
}

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

    return file;
}

fn render(file: &mut File) {
    /* Image Dimensions */
    const ASPECT_RATIO: f64 = 16.0 / 9.0; // Widescreen
    const IMG_WIDTH: usize = 256;
    const IMG_HEIGHT: usize = 256;

    // First, write the file header
    let header = format!("P3\n{} {}\n255\n", IMG_WIDTH, IMG_HEIGHT);
    match file.write_all(header.as_bytes()) {
        Ok(_) => (),
        Err(why) => eprintln!("Could not write. {}", why),
    }

    let cam: Camera = Camera::default();

    /* Compute RGB triplets */
    
    // Top to bottom row
    for row in (0..IMG_HEIGHT).rev() {
        eprintln!("Scan lines left {}", row);
        // Leftmost column to rightmost
        for col in 0..IMG_WIDTH {

            // Coordinates where the ray intercepts the screen in the xy plane
            let u: f64 = col as f64 / (IMG_WIDTH - 1) as f64;
            let v: f64 = row as f64 / (IMG_WIDTH - 1) as f64;

            let ray_direction: Vect3 =
                cam.lower_left_corner + (u * cam.horizontal) + (v * cam.vertical) - cam.origin;
            let r: Ray = Ray::new(cam.origin, ray_direction);
            let pixel_color: Color = ray_color(&r);

            match file.write_all(pixel_color.write_color().as_bytes()) {
                Ok(_) => (),
                Err(why) => eprintln!("Could not write {}", why),
            }
        }
    }
    eprintln!("Done.");
}
