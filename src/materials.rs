use crate::math::vec3::*;
use crate::math::*;
use crate::types::*;

pub type MaterialPtr = std::sync::Arc<dyn Material + Send + Sync>;

pub trait Material {
    fn scatter(&self, ray_in: Ray, record: HitRecord) -> Option<(Ray, Color)>;
}

pub struct Dielectric {
    pub refraction_index: Num,
}

impl Material for Dielectric {
    fn scatter(&self, r_in: Ray, record: HitRecord) -> Option<(Ray, Color)> {
        let etai_over_etat = if record.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = r_in.direction.unit_vector();

        let cos_theta = Num::min(-unit_direction.dot(record.normal), 1.0);
        let sin_theta = Num::sqrt(1.0 - cos_theta * cos_theta);

        let next_direction = if etai_over_etat * sin_theta > 1.0 {
            reflect(unit_direction, record.normal)
        } else {
            let reflect_prob = schlick(cos_theta, etai_over_etat);
            if random_num() < reflect_prob {
                reflect(unit_direction, record.normal)
            } else {
                refract(unit_direction, record.normal, etai_over_etat)
            }
        };
        Some((
            Ray {
                origin: record.position,
                direction: next_direction,
            },
            Color::one(),
        ))
    }
}
fn schlick(cosine: Num, ref_idx: Num) -> Num {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    return r0 + (1.0 - r0) * Num::powi(1.0 - cosine, 5);
}

fn refract(uv: Vec3, normal: Vec3, etai_over_etat: Num) -> Vec3 {
    let cos_theta = -uv.dot(normal);
    let r_out_parallel = (uv + normal * cos_theta) * etai_over_etat;
    let r_out_perp = normal * -Num::sqrt(1.0 - r_out_parallel.magnitude_squared());
    r_out_parallel + r_out_perp
}

pub struct Metal {
    pub albedo: Color,
    fuzz: Num,
}

impl Metal {
    pub fn fuzz(&self) -> Num {
        self.fuzz
    }
    pub fn new(albedo: Color, fuzz: Num) -> Metal {
        Metal {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: Ray, record: HitRecord) -> Option<(Ray, Color)> {
        let reflected = reflect(r_in.direction.unit_vector(), record.normal);
        if reflected.dot(record.normal) > 0.0 {
            Some((
                Ray {
                    origin: record.position,
                    direction: reflected + (random_in_unit_sphere() * self.fuzz),
                },
                self.albedo,
            ))
        } else {
            None
        }
    }
}
fn reflect(vec: Vec3, normal: Vec3) -> Vec3 {
    let b = normal * vec.dot(normal);
    vec - (b * 2.0)
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, _: Ray, record: HitRecord) -> Option<(Ray, Color)> {
        let scatter_direction = record.normal + random_unit_vector();
        Some((
            Ray {
                origin: record.position,
                direction: scatter_direction,
            },
            self.albedo,
        ))
    }
}

// for lambertian diffuse
fn random_unit_vector() -> Vec3 {
    let mut rng = random_num_generator_rng();
    let a = rng(0.0, 2.0 * crate::math::PI);
    let z = rng(-1.0, 1.0);
    let r = Num::sqrt(1.0 - z * z);
    return Vec3::new(r * Num::cos(a), r * Num::sin(a), z);
}

fn random_in_unit_sphere() -> Vec3 {
    let mut rng = random_vec3_generator_range();
    loop {
        let vec = rng(-1.0, 1.0);
        if vec.magnitude_squared() < 1.0 {
            return vec;
        }
    }
}

fn random_in_hemisphere(normal: Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if in_unit_sphere.dot(normal) > 0.0 {
        // In the same hemisphere as the normal
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}
