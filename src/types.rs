use crate::materials::*;
use crate::math::vec3::*;
use crate::math::*;
// #[derive(Clone, Copy)]
pub struct Camera {
    pub origin: Point,
    pub lower_left_corner: Point,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: Num,
    // use_ctor_please: (),
}

impl Camera {
    pub fn new(
        lookfrom: Point,
        lookat: Point,
        vup: Vec3,
        aspect_ratio: Num,
        vertical_fov: Num,
        aperture: Num,
        focus_dist: Num,
    ) -> Camera {
        let theta = degrees_to_radians(vertical_fov);
        let half_height = Num::tan(theta / 2.0);
        let half_width = aspect_ratio * half_height;

        let w = (lookfrom - lookat).unit_vector();
        let u = vup.cross(w).unit_vector();
        let v = w.cross(u);

        let lower_left_corner = lookfrom
            - (u * (focus_dist * half_width))
            - (v * (focus_dist * half_height))
            - (w * focus_dist);

        let horizontal = u * focus_dist * half_width * 2.0;
        let vertical = v * focus_dist * half_height * 2.0;

        let lens_radius = aperture / 2.0;
        Camera {
            origin: lookfrom,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            w,
            lens_radius,
        }
    }
    pub fn get_ray(&self, u: Num, v: Num) -> Ray {
        let rd = random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;
        Ray {
            origin: self.origin + offset,
            direction: (self.lower_left_corner + (self.horizontal * u) + (self.vertical * v))
                - self.origin
                - offset,
        }
    }
}

fn random_in_unit_disk() -> Vec3 {
    let mut rng = random_num_generator_rng();
    loop {
        let p = Vec3::new(rng(-1.0, 1.0), rng(-1.0, 1.0), 0);
        if p.magnitude_squared() < 1.0 {
            return p;
        }
    }
}

pub struct Triangle {
    pub p1: Point,
    pub p3: Point,
    pub p2: Point,
}
/* impl Hit for Cube {
    fn hit(&self, ray: &Ray, t_min: Num, t_max: Num) -> Option<HitRecord> {

    }
}
 */

pub struct Plane {
    pub p1: Point,
    pub normal: Vec3,
}

impl Hit for Plane {
    fn hit(&self, ray: &Ray, t_min: Num, t_max: Num) -> Option<HitRecord> {
        None
    }
}
pub struct Cube {
    pub center: Point,
    pub width: Num,
    pub material: MaterialPtr,
}

/* impl Hit for Cube {
    fn hit(&self, ray: &Ray, t_min: Num, t_max: Num) -> Option<HitRecord> {
        // let p be a point on the cube
        // (Cx +- (w /2), Cy +- (h/2), Cx +- (l / 2) = P
        // Vec3(Px-Cx, Py-Cy, Pz-Cz) = Vec3(w, h, l) /2
        // P +-- C = (w, h, l) /2
        // let d = Vec3(w, h, l)
        // P +- C = d / 2
        // P(t) = A + tB (Ray formula): A = origin, B = direction
        // (A + tB) +- C = d
        // t = ((A + d) + C) / B
        // or
        // t = ((A + d) - C) / B
        let d = Vec3::new(self.width, self.width, self.width) / 2.0;
        let t = d + self.center - ray.origin;
        let t = Vec3 {
            x: t.x / ray.direction.x,
            y: t.y / ray.direction.y,
            z: t.z / ray.direction.z,
        };
        if t < t_max && t > t_min {
            let position = ray.origin + (ray.direction * t);

            Some(HitRecord::new(
                position,
                t,
                ray,
                outward_normal,
                self.material.clone,
            ))
        } else {
            None
        }
    }
} */

pub struct Sphere {
    pub center: Point,
    pub radius: Num,
    pub material: MaterialPtr,
}

impl Hit for Sphere {
    fn hit(&self, ray: &Ray, t_min: Num, t_max: Num) -> Option<HitRecord> {
        // (t^2 * b^2) + (2tb * (A−C)) + ((A−C) * (A−C)) − r^2 = 0
        // A = origin
        // b = direction
        // t = step
        // C = sphere center

        // use quadratic equation to solve
        // +- b * sqrt(b^2 * 4*a*c) / 2 * a

        // a = b^2  --  b dot b = |b|^2
        let a = ray.direction.magnitude_squared();

        let o_to_c = ray.origin - self.center; // (A - C)

        // b = 2b * (A - C) -- remove the 2
        let half_b = ray.direction.dot(o_to_c);

        // c = (A-C)^2 - r^2 -- again v dot b = |v|^2
        let c = o_to_c.magnitude_squared() - self.radius * self.radius;

        // b^2 * 4*a*c = (2*half_b)^2 - 4ac = 4halfb^2 - 4ac
        // = halfb^2 -ac (take common 4 out of root)
        let discriminant = (half_b * half_b) - (a * c);
        if discriminant > 0.0 {
            // hit sphere
            let root = Num::sqrt(discriminant);
            let mut solution = (-half_b - root) / a;
            let mut valid: bool = solution < t_max && solution > t_min;
            if !valid {
                solution = (-half_b - root) / a;
                valid = solution < t_max && solution > t_min;
            }
            if valid {
                let position = ray.at(solution);
                let outward_normal = (position - self.center) / self.radius;

                let record = HitRecord::new(
                    position,
                    solution,
                    ray,
                    outward_normal,
                    self.material.clone(),
                );
                return Some(record);
            }
        }
        // didn't hit sphere
        None
    }
}

pub type HittablesList = Vec<HittablePtr>;

impl Hit for HittablesList {
    fn hit(&self, ray: &Ray, t_min: Num, t_max: Num) -> Option<HitRecord> {
        let mut record = None;
        let mut closest_so_far = t_max;

        for object in self {
            let temp = object.hit(&ray, t_min, closest_so_far);
            if let Some(r) = temp {
                closest_so_far = r.t;
                record = Some(r);
            }
        }
        record
    }
}

pub type HittablePtr = std::sync::Arc<dyn Hit + Send + Sync>;

pub trait Hit {
    fn hit(&self, ray: &Ray, t_min: Num, t_max: Num) -> Option<HitRecord>;
}

pub struct HitRecord {
    pub position: Point,
    pub normal: Vec3,
    pub t: Num,
    pub front_face: bool,
    pub material: MaterialPtr,
}

impl HitRecord {
    pub fn new(
        position: Point,
        t: Num,
        ray: &Ray,
        outward_normal: Vec3,
        material: MaterialPtr,
    ) -> HitRecord {
        let front_face = ray.direction.dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        HitRecord {
            position,
            t,
            front_face,
            normal,
            material,
        }
    }

    pub fn set_normal(self, ray: &Ray, outward_normal: Vec3) -> HitRecord {
        let front_face = ray.direction.dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        HitRecord {
            position: self.position,
            t: self.t,
            front_face,
            normal,
            material: self.material,
        }
    }
}

#[derive(Debug)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(&self, t: Num) -> Point {
        self.origin + (self.direction * t)
    }
}

pub type Point = Vec3;

pub type Color = Vec3;

impl Color {
    pub fn ppm_fmt(self) -> String {
        format!(
            "{} {} {}\n",
            (255.999 * self.x) as i32,
            (255.999 * self.y) as i32,
            (255.999 * self.z) as i32
        )
    }
}

#[cfg(test)]
mod test_ray {
    use super::*;

    #[test]
    fn test_at() {
        let ray = Ray {
            origin: Vec3::zero(),
            direction: Vec3::one(),
        };
        assert_eq!(ray.at(5.0), Vec3::new(5, 5, 5));
    }
}
