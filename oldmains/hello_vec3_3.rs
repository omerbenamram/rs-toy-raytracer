extern crate rs_raytracer;

use std::f64;

use rs_raytracer::hitable::Hitable;
use rs_raytracer::hitable_list::HitableList;
use rs_raytracer::ray::Ray;
use rs_raytracer::sphere::Sphere;
use rs_raytracer::vec3::Vec3;

fn calculate_color(r: &Ray, world: &Hitable) -> Vec3 {
    match world.hit(r, 0.0, f64::MAX) {
        Some(hit) => {
            return Vec3::new(hit.normal.x + 1.0, hit.normal.y + 1.0, hit.normal.z + 1.0) / 2
        }
        None => {
            let unit_vec = r.direction.make_unit_vec();
            let t = (unit_vec.y + 1.0) / 2.0;

            return Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t;
        }
    }
}

fn main() {
    let nx = 200;
    let ny = 100;
    println!("P3\n {} {}\n255", nx, ny);

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    let world = HitableList {
        hitables: vec![
            Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)),
            Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)),
        ],
    };

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f64 / nx as f64;
            let v = j as f64 / ny as f64;

            let color = Ray::new(
                origin,
                &lower_left_corner + &(horizontal * u) + (vertical * v),
            );

            let color = calculate_color(&color, &world);

            let ir = (255.99 * color.x) as i64;
            let ig = (255.99 * color.y) as i64;
            let ib = (255.99 * color.z) as i64;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
