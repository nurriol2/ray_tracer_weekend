/*
Scene entity is anything that can be placed in the scene. In the simplest case,
that is a sphere or many spheres.

Traits are implemented here for all variants of `SceneEntity`.
*/

/* Imports and Type Aliases */
use vec3::Vect3;

type Point = Vect3;

/* Traits */
trait RayInteraction {
    fn ray_hit(
        entity: &SceneEntity,
        r: &Ray,
        t_min: f64,
        t_max: f64,
        hit_record: HitRecord,
    ) -> bool;
}

/* Structs */
struct Sphere {
    center: Point,
    radius: f64,
}

impl RayInteraction for Sphere {
    fn ray_hit(&self, r: &Ray, t_min: f64, t_max: f64, hit_record: HitRecord) -> bool {
        // Set up for discriminant calculation
        let oc: Vect3 = r.origin - self.center;
        let a: f64 = r.direction.length_squared();
        let half_b: f64 = r.direction.dot(oc);
        let c: f64 = oc.length_squared() - (radius * radius);

        // Simplified form of the quadratic formula discriminant
        let discriminant = (half_b * half_b) - (a * c);

        // Imaginary discriminant so ray did not interact with sphere
        if discriminant < 0.0 {
            return false;
        }

        /* Check if the + or - quadratic root falls between `t_min` and `t_max` */
        let sqrt_disc = discriminant.sqrt(); // RHS of the - operator in numerator of root

        let mut root = (-half_b - sqrt_disc) / a; // Assume first the negative root is within range
        // Check if the negative root is out of bounds
        if (root < t_min || t_max < root) {
            root = (-half_b + sqrt_disc) / a; // Check the positive root
            if (root < t_min || t_max < root) {
                return false; // Both failed, so the ray did not interact
            }
        }

        hit_record.t = root;
        hit_record.point = r.at(hit_record.t);
        let outward_normal = (hit_record.point - self.center) / self.radius;
        hit_record.set_front_face_normal(r, outward_normal);

        return true;
    }
}

struct HitRecord {
    point: Point,
    normal: Vect3,
    t: f64,
    front_face: bool,
}

impl HitRecord {
    fn set_front_face_normal(&mut self, r: &Ray, outward_normal: Vect3) -> bool {
        // Assume normal always points out
        self.front = r.direction.dot(outward_normal) < 0.0;
        self.normal = match self.front {
            true => outward_normal,
            false => -outward_normal,
        }
    }
}

/* Enums */

enum SceneEntity {
    Sphere(Sphere),
}
