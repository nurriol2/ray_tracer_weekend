/*
Simple camera class.
This file also controls how the viewport dimensions.
*/

use crate::vec3::Vect3;

type Point = Vect3;

pub struct Camera {
    pub viewport_height: f64,
    pub viewport_width: f64,
    pub focal_length: f64,

    pub origin: Point,
    pub horizontal: Vect3,
    pub vertical: Vect3,

    pub lower_left_corner: Point,
}

impl Camera {
    pub fn new(
        self,
        viewport_height: f64,
        viewport_width: f64,
        focal_length: f64,
        origin: Point,
        horizontal: Vect3,
        vertical: Vect3,
        lower_left_corner: Point,
    ) -> Self {
        Self {
            viewport_height,
            viewport_width,
            focal_length,
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }
}

impl Default for Camera {
    fn default() -> Self {
        const ASPECT_RATIO: f64 = 16.0 / 9.0; // Widescreen
        let viewport_height: f64 = 2.0;
        let viewport_width: f64 = ASPECT_RATIO * viewport_height;
        let focal_length: f64 = 1.0;

        let origin: Point = Vect3::default();
        let horizontal: Vect3 = Vect3::new(viewport_width, 0., 0.);
        let vertical: Vect3 = Vect3::new(0., viewport_height, 0.);
        let lower_left_corner: Point =
            origin - (horizontal / 2.0) - (vertical / 2.0) - Vect3::new(0., 0., focal_length);

        Self {
            viewport_height,
            viewport_width,
            focal_length,
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }
}
