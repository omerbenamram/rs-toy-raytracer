extern crate rand;

use crate::hitable::HitRecord;
use crate::materials::Scatter;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug)]
pub struct Dielectric {
    pub refraction_idx: f64,
}

impl Dielectric {
    pub fn new(refraction_idx: f64) -> Dielectric {
        Dielectric { refraction_idx }
    }
    fn reflect(v: &Vec3, normal: &Vec3) -> Vec3 {
        v - &(v.dot(normal) * 2.0 * normal)
    }

    fn refract(v: &Vec3, normal: &Vec3, ni_over_nt: f64) -> Option<Vec3> {
        let uv = v.make_unit_vec();
        let dt = uv.dot(normal);
        let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
        if discriminant > 0.0 {
            return Some((uv - &(dt * normal)) * ni_over_nt - &(discriminant.sqrt() * normal));
        }
        None
    }

    fn schlick(cosine: f64, refraction_index: f64) -> f64 {
        let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}

impl Scatter for Dielectric {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        let (outward_normal, ni_over_nt, cosine) = if r_in.direction.dot(&hit_record.normal) > 0.0 {
            let outward_normal = &hit_record.normal * -1.0;
            (
                outward_normal,
                self.refraction_idx,
                self.refraction_idx
                    * (&r_in.direction.dot(&hit_record.normal) / &r_in.direction.length()),
            )
        } else {
            // total refraction
            let outward_normal = hit_record.normal.clone();
            (
                outward_normal,
                1.0 / self.refraction_idx,
                -1.0 * (&r_in.direction.dot(&hit_record.normal) / &r_in.direction.length()),
            )
        };

        let reflected = Dielectric::reflect(&r_in.direction, &hit_record.normal);
        let attenuation = Vec3::new(1.0, 1.0, 1.0);

        match Dielectric::refract(&r_in.direction, &outward_normal, ni_over_nt) {
            Some(refracted) => {
                // Calculate chance for total internal refraction
                let reflect_prob = Dielectric::schlick(cosine, self.refraction_idx);
                match rand::random::<f64>() < reflect_prob {
                    true => {
                        return Some((
                            attenuation,
                            Ray::new(hit_record.position.clone(), reflected),
                        ))
                    }
                    false => {
                        return Some((
                            attenuation,
                            Ray::new(hit_record.position.clone(), refracted),
                        ))
                    }
                }
            }
            // Reflect
            None => {
                return Some((
                    attenuation,
                    Ray::new(hit_record.position.clone(), reflected),
                ))
            }
        };
    }
}
