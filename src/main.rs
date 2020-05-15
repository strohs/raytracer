use raytracer::ppm;
use std::path::Path;
use raytracer::common::{Color, Point3, Vec3, Ray, Sphere};
use raytracer::common::hittable::{Hittable, HittableList};


/// linearly blends white and blue depending on the height of the y coordinate after
/// scaling the ray direction to unit length (so −1.0 < y < 1.0)
/// When `t=1.0` I want blue. When `t=0.0` I want white. In between, I want a blend
fn ray_color<T>(r: &Ray, world: &T) -> Color
    where T: Hittable
{

    // if a ray has hit something in the world, return the pixel color
    if let Some(rec) = world.hit(r, 0.0, f64::INFINITY) {
        0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0))
    } else {
        // return a linear interpolated Color from white to blue
        let unit_direction = r.direction().unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);

        (1.0 - t) * Color::new(1.0, 1.0, 1.0)
            + t * Color::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    println!("Raytracer....");
    let path = Path::new("./testGradient.ppm");
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 384;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let mut image: Vec<Color> = vec![];

    // camera is at the origin
    let origin: Point3 = Point3::new(0.0, 0.0, 0.0);
    let horizontal: Vec3 = Vec3::new(4.0, 0.0, 0.0);
    let vertical: Vec3 = Vec3::new(0.0, 2.25, 0.0);
    let lower_left_corner: Point3 = Point3::new(-2.0, -1.0, -1.0);

    // create the Hittable objects and store them in a HittableList
    let mut world = HittableList::new();
    let sphere1 = Sphere::from_coords(0.0, 0.0, -1.0, 0.5);
    let sphere2 = Sphere::from_coords(0.0, -100.5, -1.0, 100.0);
    world.add(sphere1);
    world.add(sphere2);

    // traverse the screen from lower left corner to upper right
    for j in (0..image_height).rev() {
        println!("Scanlines remaining {}", &j);
        for i in 0..image_width {
            // u,v are offset vectors that move the ray endpoint across the screen
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;
            let r: Ray = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical
            );
            let pixel_color: Color = ray_color(&r, &world);
            image.push(pixel_color);
        }
    }
    match ppm::write(path, image_width, image_height, &image) {
        Ok(()) => println!("test image created at {:?}", path),
        Err(e) => eprintln!("{}", e),
    }
}
