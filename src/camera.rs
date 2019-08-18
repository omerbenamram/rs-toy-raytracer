use crate::ray::Ray;
use crate::vec3::Vec3;
use std::f64;

pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lens_radius: f64,
}

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f64,
        aspect: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        let theta = vfov * f64::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let w = (lookfrom - lookat).make_unit_vec();
        let u = vup.cross(w).make_unit_vec();
        let v = w.cross(u);

        Camera {
            lower_left_corner: lookfrom
                - (u * half_width * focus_dist)
                - (v * half_height * focus_dist)
                - (w * focus_dist),
            horizontal: u * (half_width * 2.0 * focus_dist),
            vertical: v * (half_height * 2.0 * focus_dist),
            origin: lookfrom,
            lens_radius: aperture / 2.0,
        }
    }
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let orig = self.origin.clone();
        let result = self.lower_left_corner + (self.horizontal * u) + (self.vertical * v) - orig;
        Ray::new(orig, result)
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            origin: Vec3::new(0.0, 0.0, 0.0),
            lower_left_corner: Vec3::new(-2.0, -1.0, -1.0),
            horizontal: Vec3::new(4.0, 0.0, 0.0),
            vertical: Vec3::new(0.0, 2.0, 0.0),
            lens_radius: 1.0,
        }
    }
}
