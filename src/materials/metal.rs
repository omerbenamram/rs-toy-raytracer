use hitable::HitRecord;
use materials::Scatter;
use ray::Ray;
use util::random_in_unit_sphere;
use vec3::Vec3;

#[derive(Debug)]
pub struct Metal {
    albedo: Vec3,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Metal {
        let fuzz = if fuzz < 1.0 { fuzz } else { 1.0 };
        Metal { albedo, fuzz }
    }
    fn reflect(v: &Vec3, normal: &Vec3) -> Vec3 {
        v - &(v.dot(normal) * 2.0 * normal)
    }
}

impl Scatter for Metal {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflection = Metal::reflect(&r_in.direction.make_unit_vec(), &hit_record.normal);
        let scattered = Ray::new(
            hit_record.position.clone(),
            reflection + random_in_unit_sphere() * self.fuzz,
        );

        if scattered.direction.dot(&hit_record.normal) > 0.0 {
            return Some((self.albedo.clone(), scattered));
        } else {
            return None;
        }
    }
}
