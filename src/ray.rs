use vec3::Vect3;

struct Ray{
    pub origin: Vect3,
    pub direction: Vect3,
}

impl Ray{

    pub fn new(origin: Point, direction: Vect3) -> Self{
        Self{
            origin,
            direction,
        }
    }
    

    pub fn origin(&self) -> Point {
        self.origin
    }

    pub fn direction() -> Vect3 {
        self.direction
    }
}

impl Default for Ray{
    fn default() -> Self {
        Self{
            origin: Vect3::new(0., 0., 0.,),
            direction: Vect3::new(0., 0., 0.),
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;



}