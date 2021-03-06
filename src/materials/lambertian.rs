use crate::hitable::HitRecord;
use crate::materials::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug)]
pub struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        let p = hit_record.position;
        let target = hit_record.position + hit_record.normal + Vec3::random_in_unit_sphere();
        let direction = target - p;
        Some((self.albedo, Ray::new(p, direction)))
    }
}
