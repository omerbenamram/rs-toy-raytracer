pub mod dielectric;
pub mod lambertian;
pub mod metal;

use crate::hitable::HitRecord;
use crate::materials::dielectric::Dielectric;
use crate::materials::lambertian::Lambertian;
use crate::materials::metal::Metal;
use crate::ray::Ray;
use crate::vec3::Vec3;

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
