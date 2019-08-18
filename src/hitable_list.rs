use crate::hitable::HitRecord;
use crate::hitable::Hitable;
use crate::ray::Ray;

pub struct HitableList {
    pub hitables: Vec<Box<dyn Hitable + Sync>>,
}

impl Hitable for HitableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut closest_hit_record: Option<HitRecord> = None;

        for hitable in &self.hitables {
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
