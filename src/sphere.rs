use hitable::HitRecord;
use hitable::Hitable;
use materials::Material;
use ray::Ray;
use vec3::Vec3;

pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Material) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let origin_center = ray.origin - &self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = origin_center.dot(&ray.direction);
        let c = origin_center.dot(&origin_center) - self.radius * self.radius;

        let discriminant = b * b - a * b;

        // If any solutions
        if discriminant > 0.0 {
            // Check negative solution
            let temp = (-b - (b * b - a * c).sqrt()) / a;
            if (temp < t_max) && (temp > t_min) {
                let p = ray.point_at_parameter(temp);
                return Some(HitRecord::new(
                    temp,
                    p,
                    (p - &self.center) / self.radius,
                    &self.material,
                ));
            }
            // Check positive solution
            let temp = (-b + (b * b - a * c).sqrt()) / a;
            if (temp < t_max) && (temp > t_min) {
                let p = ray.point_at_parameter(temp);
                return Some(HitRecord::new(
                    temp,
                    p,
                    (p - &self.center) / self.radius,
                    &self.material,
                ));
            }
        }
        None
    }
}
