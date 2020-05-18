use std::rc::Rc;
use raytracer::util::ppm;
use std::path::Path;
use raytracer::common::{Color, Ray, Camera, Point3, Vec3};
use raytracer::hittable::{Hittable, HittableList, Sphere};
use raytracer::common::color;
use raytracer::material::{Lambertian, Metal, Dielectric, Material};
use rand::{Rng};

// max recursion depth allowed when bouncing rays of hittables
static MAX_BOUNCE_DEPTH: u32 = 50;

/// linearly blends white and blue depending on the height of the y coordinate after
/// scaling the ray direction to unit length (so −1.0 < y < 1.0)
/// When `t=1.0` I want blue. When `t=0.0` I want white. In between, I want a blend
fn ray_color<T>(r: &Ray, world: &T, depth: u32) -> Color
    where T: Hittable
{
    // if we've exceeded the ray bounce limit, no more light is gathered
    if depth == 0 {
        Color::default()
    } else if let Some(ref rec) = world.hit(r, 0.001, f64::INFINITY) {
        if let Some(scatter_rec) = rec.mat_ptr.scatter(r, rec) {
            scatter_rec.attenuation * ray_color(&scatter_rec.scattered, world, depth - 1)
        } else {
            Color::default()
        }

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

    // camera settings
    let look_from = Point3::new(3.0, 3.0, 2.0);
    let look_at = Point3::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let aspect_ratio = (image_width / image_height) as f64;
    let dist_to_focus = (look_from - look_at).length();
    let aperture = 2.0;
    let camera = Camera::new(look_from, look_at,
                             vup,
                             20.0,
                             aspect_ratio,
                             aperture,
                             dist_to_focus);

    // materials
    let lambertian_r = Lambertian::new(Color::new(0.7, 0.3, 0.3));
    let lambertian_b = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let lambertian_y = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let metal_1 = Metal::new(Color::new(0.8, 0.6, 0.2), 0.3);
    let metal_grey = Metal::new(Color::new(0.8, 0.8, 0.8), 0.1);
    let glass: Rc<dyn Material> = Rc::new(Dielectric::new(1.5));

    // create Hittable objects
    let sphere_mid = Sphere::from_coords(0.0, 0.0, -1.0, 0.5, Rc::new(lambertian_b));
    let sphere_yel = Sphere::from_coords(0.0, -100.5, -1.0, 100.0, Rc::new(lambertian_y));
    let sphere_right = Sphere::from_coords(1.0, 0.0, -1.0, 0.5, Rc::new(metal_1));
    // next two spheres form a hollow glass sphere
    let sphere_left = Sphere::from_coords(-1.0, 0.0, -1.0, 0.5, Rc::clone(&glass));
    let sphere_bubble = Sphere::from_coords(-1.0, 0.0, -1.0, -0.45, Rc::clone(&glass));

    // create the world and add objects to it
    let mut world = HittableList::new();
    world.add(Rc::new(sphere_mid));
    world.add(Rc::new(sphere_yel));
    world.add(Rc::new(sphere_right));
    world.add(Rc::new(sphere_left));
    world.add(Rc::new(sphere_bubble));

    let mut rng = rand::thread_rng();
    // traverse the screen from lower left corner to upper right
    for j in (0..image_height).rev() {
        println!("Scanlines remaining {}", &j);
        for i in 0..image_width {
            let mut pixel_color: Color = Color::default(); // (0,0,0) color
            // multi-sample the pixels around the current pixel to compute an aliased pixel color
            for _ in 0..samples_per_pixel {
                // u,v are offsets that move the ray endpoint across the screen
                let u = (i as f64 + rng.gen::<f64>()) / (image_width - 1) as f64;
                let v = (j as f64 + rng.gen::<f64>()) / (image_height - 1) as f64;
                let r: Ray = camera.get_ray(u, v);
                pixel_color += ray_color(&r, &world, MAX_BOUNCE_DEPTH);
            }
            pixel_color = color::multi_sample_color(pixel_color, samples_per_pixel);
            image.push(pixel_color);
        }
    }

    // write the image to a file
    match ppm::write(path, image_width, image_height, &image) {
        Ok(()) => println!("test image created at {:?}", path),
        Err(e) => eprintln!("{}", e),
    }

}
