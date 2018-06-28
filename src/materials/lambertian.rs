use hitable::HitRecord;
use material::Material;
use ray::Ray;
use util::random_in_unit_sphere;
use vec3::Vec3;

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter<T>(&self, r_in: &Ray, hit_record: &HitRecord<T>) -> (Vec3, Ray)
    where
        T: Material,
    {
        let p = hit_record.position.clone();
        let target = &hit_record.position + &hit_record.normal + random_in_unit_sphere();
        (self.albedo.clone(), Ray::new(p, target - &p))
    }
}