extern crate rand;
extern crate rs_raytracer;

use std::f64;

use rs_raytracer::camera::Camera;
use rs_raytracer::hitable::Hitable;
use rs_raytracer::hitable_list::HitableList;
use rs_raytracer::ray::Ray;
use rs_raytracer::sphere::Sphere;
use rs_raytracer::vec3::Vec3;

fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::new(rand::random(), rand::random(), rand::random()) * 2
            - &Vec3::new(1.0, 1.0, 1.0);
        if p.squared_length() < 1.0 {
            return p;
        }
    }
}

fn calculate_color(r: &Ray, world: &Hitable) -> Vec3 {
    match world.hit(r, 0.001, f64::MAX) {
        Some(hit) => {
            let target = &hit.position + &hit.normal + random_in_unit_sphere();
            return calculate_color(&Ray::new(hit.position, target - &hit.position), world) / 2;
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
    let ns = 100;
    println!("P3\n {} {}\n255", nx, ny);

    let cam = Camera::default();

    let world = HitableList {
        hitables: vec![
            Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)),
            Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)),
        ],
    };

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for s in 0..ns {
                let u: f64 = (i as f64 + rand::random::<f64>()) as f64 / nx as f64;
                let v: f64 = (j as f64 + rand::random::<f64>()) as f64 / ny as f64;
                let r = cam.get_ray(u, v);
                col += &calculate_color(&r, &world);
            }
            col /= ns as f64;
            col = Vec3::new(col.x.sqrt(), col.y.sqrt(), col.z.sqrt());

            let ir = (255.99 * col.x) as i64;
            let ig = (255.99 * col.y) as i64;
            let ib = (255.99 * col.z) as i64;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
