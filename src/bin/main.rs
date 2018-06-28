extern crate indicatif;
extern crate rand;
extern crate rayon;
extern crate rs_raytracer;

use indicatif::ProgressBar;
use rayon::prelude::*;
use std::f64;

use indicatif::ProgressStyle;
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

fn generate_scene() -> HitableList {
    let mut world = HitableList { hitables: vec![] };

    world.hitables.push(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Material::Lambertian(Lambertian::new(Vec3::new(0.5, 0.5, 0.5))),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let center = Vec3::new(
                a as f64 + 0.9 * rand::random::<f64>(),
                0.2,
                b as f64 + 0.9 * rand::random::<f64>(),
            );
            let choose_mat = rand::random::<f64>();
            if (&center - &Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    world.hitables.push(Box::new(Sphere::new(
                        center.clone(),
                        0.2,
                        Material::Lambertian(Lambertian::new(Vec3::new(
                            rand::random(),
                            rand::random(),
                            rand::random(),
                        ))),
                    )))
                //metal
                } else if choose_mat < 0.95 {
                    world.hitables.push(Box::new(Sphere::new(
                        center.clone(),
                        0.2,
                        Material::Metal(Metal::new(
                            Vec3::new(
                                0.5 * (1.0 + rand::random::<f64>()),
                                0.5 * (1.0 + rand::random::<f64>()),
                                0.5 * (1.0 + rand::random::<f64>()),
                            ),
                            rand::random(),
                        )),
                    )));
                } else {
                    world.hitables.push(Box::new(Sphere::new(
                        center.clone(),
                        0.2,
                        Material::Dielectric(Dielectric::new(1.5)),
                    )));
                }
            }
        }
    }

    world.hitables.push(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Material::Dielectric(Dielectric::new(1.5)),
    )));

    world.hitables.push(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Material::Lambertian(Lambertian::new(Vec3::new(0.4, 0.2, 0.1))),
    )));

    world.hitables.push(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Material::Metal(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)),
    )));

    world
}

fn main() {
    let nx: i32 = 400;
    let ny: i32 = 200;
    let aa_ray_count = 100;
    println!("P3\n {} {}\n255", nx, ny);

    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let cam = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        nx as f64 / ny as f64,
        aperture,
        dist_to_focus,
    );

    let world = generate_scene();
    let pbar = ProgressBar::new((ny * nx) as u64);

    pbar.set_style(ProgressStyle::default_bar().template(
        "[{elapsed} elapsed] {wide_bar:.cyan/white} {percent}% [{eta} remaining] [rendering]",
    ));

    let result: Vec<Vec<(i64, i64, i64)>> = (0..ny)
        .into_par_iter()
        .map(|j: i32| {
            (0..nx)
                .into_par_iter()
                .map(|i: i32| {
                    let mut col = Vec3::new(0.0, 0.0, 0.0);

                    for _ in 0..aa_ray_count {
                        let u: f64 = (i as f64 + rand::random::<f64>()) as f64 / nx as f64;
                        let v: f64 = (j as f64 + rand::random::<f64>()) as f64 / ny as f64;

                        let r = cam.get_ray(u, v);

                        col += &calculate_color(&r, &world, 0);
                    }

                    col /= aa_ray_count as f64;
                    col = Vec3::new(col.x.sqrt(), col.y.sqrt(), col.z.sqrt());

                    let ir = (255.99 * col.x) as i64;
                    let ig = (255.99 * col.y) as i64;
                    let ib = (255.99 * col.z) as i64;
                    pbar.inc(1);
                    (ir, ig, ib)
                })
                .collect()
        })
        .collect();

    for row in result.iter().rev() {
        for pix in row {
            println!("{} {} {}", pix.0, pix.1, pix.2)
        }
    }
}