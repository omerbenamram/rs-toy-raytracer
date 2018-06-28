extern crate rs_raytracer;

use rs_raytracer::ray::Ray;
use rs_raytracer::vec3::Vec3;

fn calculate_color(r: &Ray) -> Vec3 {
    let v = Vec3::new(0.0, 0.1, -1.0);

    if hit_sphere(&v, 0.5, r) {
        return Vec3::new(1.0, 0.0, 0.0);
    }

    let unit_vec = r.direction.make_unit_vec();
    let t = (unit_vec.y + 1.0) / 2.0;

    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

fn hit_sphere(center: &Vec3, radius: f64, ray: &Ray) -> bool {
    let origin_to_center = &ray.origin - center;
    let a = ray.direction.dot(&ray.direction);
    let b = 2.0 * ray.direction.dot(&origin_to_center);
    let c = origin_to_center.dot(&origin_to_center) - radius * radius;

    b * b - 4.0 * a * c > 0.0
}

fn main() {
    let nx = 200;
    let ny = 100;
    println!("P3\n {} {}\n255", nx, ny);

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f64 / nx as f64;
            let v = j as f64 / ny as f64;

            let color = Ray::new(
                origin,
                &lower_left_corner + &(horizontal * u) + (vertical * v),
            );
            let color = calculate_color(&color);

            let ir = (255.99 * color.x) as i64;
            let ig = (255.99 * color.y) as i64;
            let ib = (255.99 * color.z) as i64;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
