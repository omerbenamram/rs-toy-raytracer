pub mod dielectric;
pub mod lambertian;
pub mod metal;

use hitable::HitRecord;
use materials::dielectric::Dielectric;
use materials::lambertian::Lambertian;
use materials::metal::Metal;
use ray::Ray;
use vec3::Vec3;

#[derive(Debug)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

pub trait Scatter {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)>;
}

impl Scatter for Material {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        match self {
            Material::Lambertian(ref lambertian) => lambertian.scatter(r_in, hit_record),
            Material::Metal(ref metal) => metal.scatter(r_in, hit_record),
            Material::Dielectric(ref dielectric) => dielectric.scatter(r_in, hit_record),
        }
    }
}
