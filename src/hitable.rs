use crate::materials::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct HitRecord<'a> {
    pub t: f64,
    pub position: Vec3,
    pub normal: Vec3,
    pub material: &'a (dyn Material),
}

impl<'a> HitRecord<'a> {
    pub fn new(t: f64, position: Vec3, normal: Vec3, material: &'a (dyn Material)) -> HitRecord {
        HitRecord {
            t,
            position,
            normal,
            material,
        }
    }
}

pub type HitableList = Vec<Box<dyn Hitable>>;

impl Hitable for HitableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut closest_hit_record: Option<HitRecord> = None;

        for hitable in self.iter() {
            match hitable.hit(ray, t_min, closest_so_far) {
                None => continue,
                Some(hit) => {
                    closest_so_far = hit.t;
                    closest_hit_record = Some(hit);
                }
            }
        }

        closest_hit_record
    }
}

pub trait Hitable: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
