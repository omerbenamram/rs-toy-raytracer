extern crate rand;
extern crate rs_raytracer;

use std::f64;

use rs_raytracer::camera::Camera;
use rs_raytracer::hitable::Hitable;
use rs_raytracer::hitable_list::HitableList;
use rs_raytracer::materials::dielectric::Dielectric;
use rs_raytracer::materials::lambertian::Lambertian;
use rs_raytracer::materials::metal::Metal;
use rs_raytracer::materials::Material;
use rs_raytracer::materials::Scatter;
use rs_raytracer::ray::Ray;
use rs_raytracer::sphere::Sphere;
use rs_raytracer::util::random_in_unit_sphere;
use rs_raytracer::vec3::Vec3;

fn calculate_color(r: &Ray, world: &Hitable, depth: i32) -> Vec3 {
    match world.hit(r, 0.001, f64::MAX) {
        Some(hit) => match hit.material.scatter(r, &hit) {
            Some((albedo, scattered)) => {
                if depth < 50 {
                    return albedo * calculate_color(&scattered, world, depth + 1);
                }
                return Vec3::origin();
            }
            None => return Vec3::origin(),
        },
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

    let cam = Camera::new(
        Vec3::new(-2.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        90.0,
        nx as f64 / ny as f64,
    );

    let world = HitableList {
        hitables: vec![
            Box::new(Sphere::new(
                Vec3::new(0.0, 0.0, -1.0),
                0.5,
                Material::Lambertian(Lambertian::new(Vec3::new(0.1, 0.2, 0.5))),
            )),
            Box::new(Sphere::new(
                Vec3::new(0.0, -100.5, -1.0),
                100.0,
                Material::Lambertian(Lambertian::new(Vec3::new(0.8, 0.8, 0.0))),
            )),
            Box::new(Sphere::new(
                Vec3::new(1.0, 0.0, -1.0),
                0.5,
                Material::Metal(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.3)),
            )),
            Box::new(Sphere::new(
                Vec3::new(-1.0, 0.0, -1.0),
                0.5,
                Material::Dielectric(Dielectric::new(1.5)),
            )),
        ],
    };

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for s in 0..ns {
                let u: f64 = (i as f64 + rand::random::<f64>()) as f64 / nx as f64;
                let v: f64 = (j as f64 + rand::random::<f64>()) as f64 / ny as f64;
                let r = cam.get_ray(u, v);
                col += &calculate_color(&r, &world, 0);
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
