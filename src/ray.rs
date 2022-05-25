/*
Ray that can be sent into a scene.
Rays can be created, placed, and directed into a scene.
*/

use crate::vec3::Vect3;

type Point = Vect3;
type Color = Vect3;

pub struct Ray {
    pub origin: Vect3,
    pub direction: Vect3,
}

impl Ray {
    pub fn new(origin: Point, direction: Vect3) -> Self {
        Self { origin, direction }
    }

    pub fn origin(&self) -> Point {
        self.origin
    }

    pub fn direction(&self) -> Vect3 {
        self.direction
    }

    pub fn at(&self, t: f64) -> Vect3 {
        self.origin + (t * self.direction)
    }
}

impl Default for Ray {
    fn default() -> Self {
        Self {
            origin: Vect3::new(0., 0., 0.),
            direction: Vect3::new(0., 0., 0.),
        }
    }
}