use raytracer::ppm;
use std::path::Path;
use raytracer::common::{Color, Ray, Sphere, Camera, random_f64};
use raytracer::common::hittable::{Hittable, HittableList};
use std::rc::Rc;
use raytracer::common::color::multi_sample_color;


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
    let samples_per_pixel: u32 = 100;
    let mut image: Vec<Color> = vec![];

    // axis-aligned camera
    let camera = Camera::default();

    // create the Hittable objects and store them in a HittableList
    let mut world = HittableList::new();
    let sphere1 = Sphere::from_coords(0.0, 0.0, -1.0, 0.5);
    let sphere2 = Sphere::from_coords(0.0, -100.5, -1.0, 100.0);
    world.add(Rc::new(sphere1));
    world.add(Rc::new(sphere2));

    // traverse the screen from lower left corner to upper right
    for j in (0..image_height).rev() {
        println!("Scanlines remaining {}", &j);
        for i in 0..image_width {
            let mut pixel_color: Color = Color::default(); // (0,0,0) color
            // multi-sample the pixels around the current pixel to compute an aliased pixel color
            for _ in 0..samples_per_pixel {
                // u,v are offsets that move the ray endpoint across the screen
                let u = (i as f64 + random_f64()) / (image_width - 1) as f64;
                let v = (j as f64 + random_f64()) / (image_height - 1) as f64;
                let r: Ray = camera.get_ray(u, v);
                pixel_color += ray_color(&r, &world);
            }
            pixel_color = multi_sample_color(pixel_color, samples_per_pixel);
            image.push(pixel_color);
        }
    }

    // write the image to a file
    match ppm::write(path, image_width, image_height, &image) {
        Ok(()) => println!("test image created at {:?}", path),
        Err(e) => eprintln!("{}", e),
    }

}
