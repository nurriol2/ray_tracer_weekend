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

fn ray_hit_sphere(radius: f64, center: Point, r: &Ray) -> f64 {
    let oc = r.origin - center;
    let a = r.direction.length_squared();
    let half_b = r.direction.dot(oc);
    let c = oc.length_squared() - (radius * radius);
    let discriminant = (half_b * half_b) - (a * c);
    if discriminant < 0.0 {
        return -1.0
    } else {
        return (-half_b - discriminant.sqrt() ) / a
    }
}

fn ray_color(ray: &Ray) -> Color {

    /*
    Determine the color seen by this ray.
    
    (LERP):= blended_value = (1-t)*start_vlaue + t*end_value
    */

    // Add a (green) sphere to the scene
    let sphere_center: Point = Point::new(0., 0., -1.);
    let sphere_radius = 0.5;

    let t = ray_hit_sphere(sphere_radius, sphere_center, ray);
    if t > 0.0 {
        let normal: Vect3 = (ray.at(t) - Vect3::new(0., 0., -1.)).unit_vector();
        return 0.5 * Color::new(normal.x + 1.0, normal.y + 1.0, normal.z + 1.0)
    }

    // Render the background as a gradient
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
    const IMG_WIDTH: usize = 400;
    const IMG_HEIGHT: usize = (IMG_WIDTH as f64 / ASPECT_RATIO) as usize; 

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
            let v: f64 = row as f64 / (IMG_HEIGHT - 1) as f64;

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
