use hitable::HitRecord;
use materials::Scatter;
use ray::Ray;
use util::random_in_unit_sphere;
use vec3::Vec3;

#[derive(Debug)]
pub struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Scatter for Lambertian {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        let p = hit_record.position.clone();
        let target = &hit_record.position + &hit_record.normal + random_in_unit_sphere();
        Some((self.albedo.clone(), Ray::new(p, target - &p)))
    }
}
