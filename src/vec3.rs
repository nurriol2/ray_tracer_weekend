/*
Minimal 3 component vector module.

I called this Vect3 so VS Code stops trying to import Vec3 from another crate.

TODO: Instead of type aliasing, use enum Vect3
*/

use float_cmp::approx_eq;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vect3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vect3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Vect3 {
            x: e0,
            y: e1,
            z: e2,
        }
    }

    /* Convenience functions for Points */
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn dot(&self, other: Vect3) -> f64 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }

    pub fn cross(&self, other: Vect3) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn unit_vector(&self) -> Self {
        *self / self.length()
    }

    /* Convenience functions for Colors */
    pub fn write_color(&self) -> String {
        // Write the translated [0,255] value of each color component
        let r = (255.999 * self.x) as isize;
        let g = (255.999 * self.y) as isize;
        let b = (255.999 * self.z) as isize;
        format!("{} {} {}\n", r, g, b)
    }
}

impl Default for Vect3 {
    fn default() -> Self {
        Vect3 {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }
}

impl std::ops::Neg for Vect3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vect3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl std::ops::Add for Vect3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::AddAssign for Vect3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::Sub<Vect3> for Vect3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::SubAssign<Vect3> for Vect3 {
    fn sub_assign(&mut self, rhs: Vect3) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::Mul<Vect3> for Vect3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl std::ops::Mul<f64> for Vect3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

// TODO:  It's easier to implement on numerics than write a generic
impl std::ops::MulAssign<f64> for Vect3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl std::ops::Div<f64> for Vect3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl std::ops::DivAssign<f64> for Vect3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

#[cfg(test)]
mod tests {

    use float_cmp::approx_eq;

    use super::*;

    fn two_vectors() -> (Vect3, Vect3) {
        let a = Vect3::new(1., 2., 3.);
        let b = Vect3::new(4., 5., 6.);
        (a, b)
    }

    #[test]
    fn compute_vector_len() {
        let (a, _) = two_vectors();
        let l = a.length();
        let l2 = a.length_squared();
        assert!(approx_eq!(f64, l, 14.0_f64.sqrt()));
        assert!(approx_eq!(f64, l2, 14.0));
    }

    #[test]
    fn dot_product_scenario() {
        let (a, b) = two_vectors();
        let dot_product = a.dot(b);
        assert!(approx_eq!(f64, dot_product, 32.0));
    }

    #[test]
    fn cross_product_scenario() {
        let (a, b) = two_vectors();
        let cross = a.cross(b);
        let ans = Vect3::new(-3., 6., -3.);

        assert_eq!(cross, ans);
    }

    #[test]
    fn unit_vectors_len_one() {
        let (a, _) = two_vectors();
        let one = a.unit_vector().length();
        assert!(approx_eq!(f64, one, 1.0));
    }

    #[test]
    fn negating_vector() {
        let (a, _) = two_vectors();
        let neg_a = -a;
        assert_eq!(neg_a, Vect3::new(-1., -2., -3.));
    }

    #[test]
    fn adding_vectors() {
        let (a, b) = two_vectors();
        let sumv = a + b;
        assert_eq!(sumv, Vect3::new(5., 7., 9.));
    }

    #[test]
    fn adding_assignment() {
        let mut a = Vect3::new(1., 2., 3.);
        let b = Vect3::new(4., 5., 6.);
        a += b;
        assert_eq!(a, Vect3::new(5., 7., 9.));
    }

    #[test]
    fn subtracting_vectors() {
        let (a, b) = two_vectors();
        let diff = b - a;
        assert_eq!(diff, Vect3::new(3., 3., 3.));
    }

    #[test]
    fn subtracting_assignment() {
        let mut a = Vect3::new(4., 5., 6.);
        let b = Vect3::new(1., 2., 3.);
        a -= b;
        assert_eq!(a, Vect3::new(3., 3., 3.,));
    }

    #[test]
    fn multiplying_vectors() {
        let (a, b) = two_vectors();
        let prod = a * b;
        assert_eq!(prod, Vect3::new(4., 10., 18.));
    }

    #[test]
    fn multiplying_assignment() {
        let mut a = Vect3::new(1., 2., 3.);
        a *= 2.0;
        assert_eq!(a, Vect3::new(2., 4., 6.));
    }

    #[test]
    fn vector_divide_by_scalar() {
        let (a, _) = two_vectors();
        let quot = a / 2.;
        assert_eq!(quot, Vect3::new(0.5, 1.0, 1.5));
    }

    #[test]
    fn division_assignmetn() {
        let mut a = Vect3::new(1., 2., 3.);
        a /= 2.;
        assert_eq!(a, Vect3::new(0.5, 1.0, 1.5));
    }
}
