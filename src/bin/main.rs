use indicatif::ProgressBar;
use rayon::prelude::*;
use std::f64;

use clap::{App, Arg};
use indicatif::ProgressStyle;
use rs_raytracer::camera::Camera;
use rs_raytracer::hitable::{Hitable, HitableList};
use rs_raytracer::materials::dielectric::Dielectric;
use rs_raytracer::materials::lambertian::Lambertian;
use rs_raytracer::materials::metal::Metal;
use rs_raytracer::ray::Ray;
use rs_raytracer::sphere::Sphere;
use rs_raytracer::vec3::Vec3;

fn calculate_color(r: &Ray, world: &dyn Hitable, depth: i32) -> Vec3 {
    match world.hit(r, 0.001, f64::MAX) {
        Some(hit) => match hit.material.scatter(r, &hit) {
            Some((albedo, scattered)) => {
                if depth < 50 {
                    return albedo * calculate_color(&scattered, world, depth + 1);
                }
                Vec3::origin()
            }
            None => Vec3::origin(),
        },
        None => {
            let unit_vec = r.direction.make_unit_vec();
            let t = (unit_vec.y + 1.0) / 2.0;

            Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
        }
    }
}

fn generate_scene() -> HitableList {
    let mut world = HitableList::new();

    world.push(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Box::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5))),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let center = Vec3::new(
                f64::from(a) + 0.9 * rand::random::<f64>(),
                0.2,
                f64::from(b) + 0.9 * rand::random::<f64>(),
            );
            let choose_mat = rand::random::<f64>();
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    world.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Box::new(Lambertian::new(Vec3::new(
                            rand::random(),
                            rand::random(),
                            rand::random(),
                        ))),
                    )))
                //metal
                } else if choose_mat < 0.95 {
                    world.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Box::new(Metal::new(
                            Vec3::new(
                                0.5 * (1.0 + rand::random::<f64>()),
                                0.5 * (1.0 + rand::random::<f64>()),
                                0.5 * (1.0 + rand::random::<f64>()),
                            ),
                            rand::random(),
                        )),
                    )));
                } else {
                    world.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Box::new(Dielectric::new(1.5)),
                    )));
                }
            }
        }
    }

    world.push(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Box::new(Dielectric::new(1.5)),
    )));

    world.push(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Box::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1))),
    )));

    world.push(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Box::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)),
    )));

    world
}

fn main() {
    let app = App::new("Raytracer").args(&[
        Arg::with_name("x").required(true),
        Arg::with_name("y").required(true),
        Arg::with_name("aa")
            .required(false)
            .default_value("100")
            .help("Number of anti-aliasing rays"),
        Arg::with_name("dist-to-focus")
            .required(false)
            .help("Distance to focus"),
        Arg::with_name("output")
            .required(false)
            .default_value("output.png")
            .short("o")
            .help("Output path, defaults to `output.png`"),
    ]);

    let matches = app.get_matches();

    let nx: i32 = matches
        .value_of("x")
        .expect("a required argument")
        .parse()
        .expect("x should be a number");

    let ny: i32 = matches
        .value_of("y")
        .expect("a required argument")
        .parse()
        .expect("y should be a number");

    let aa_ray_count = matches
        .value_of("aa")
        .expect("has a default")
        .parse()
        .expect("number of aa rays should be a number");

    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let cam = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        f64::from(nx) / f64::from(ny),
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
                        let u: f64 = (f64::from(i) + rand::random::<f64>()) as f64 / f64::from(nx);
                        let v: f64 = (f64::from(j) + rand::random::<f64>()) as f64 / f64::from(ny);

                        let r = cam.get_ray(u, v);

                        col += calculate_color(&r, &world, 0);
                    }

                    col /= f64::from(aa_ray_count);
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

    let mut imgbuf = image::ImageBuffer::new(nx as u32, ny as u32);

    for (r, row) in result.iter().rev().zip(imgbuf.rows_mut()) {
        for (pixel_result, pix) in r.iter().zip(row) {
            *pix = image::Rgb([
                pixel_result.0 as u8,
                pixel_result.1 as u8,
                pixel_result.2 as u8,
            ]);
        }
    }

    let output_path = matches.value_of("output").expect("has a default value");

    imgbuf
        .save(output_path)
        .expect("failed to write to output path.");
}
