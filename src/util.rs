extern crate rand;

use vec3::Vec3;

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::new(rand::random(), rand::random(), rand::random()) * 2
            - &Vec3::new(1.0, 1.0, 1.0);
        if p.squared_length() < 1.0 {
            return p;
        }
    }
}
