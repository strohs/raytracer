use raytracer::util::ppm;
use std::path::Path;
use raytracer::common::{Color, Ray, Camera, Point3, Vec3};
use raytracer::hittable::{Hittable};
use raytracer::common::color;
use raytracer::util::scenes;
use rand::{Rng};


// max recursion depth allowed when bouncing rays of hittables
static MAX_BOUNCE_DEPTH: u32 = 50;
// maximum samples to use, per pixel, when anti-aliasing
static MAX_SAMPLES_PER_PIXEL: u32 = 100;

fn main() {
    let num_cpus = num_cpus::get();
    println!("detected {} logical cores", num_cpus);
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 384;
    let image_height = (image_width as f64 / aspect_ratio) as u32; // 576 if width is 1024 at 16:9
    let filename = format!("./raytrace_final_{}x{}.ppm", image_width, image_height);
    let path = Path::new(&filename);

    // camera settings
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let camera = Camera::new(look_from, look_at,
                             vup,
                             20.0,
                             aspect_ratio,
                             aperture,
                             dist_to_focus);

    // generate a world with spheres in random locations
    let world = scenes::random_scene();

    let mut rng = rand::thread_rng();

    // stores the final image colors
    let mut image: Vec<Color> = Vec::with_capacity((image_width * image_height) as usize);
    
    // traverse the screen from lower left corner to upper right
    for j in (0..image_height).rev() {
        println!("Scanlines remaining {}", &j);
        for i in 0..image_width {
            let mut pixel_color: Color = Color::default(); // (0,0,0) color
            // multi-sample the pixels around the current pixel to compute an aliased pixel color
            for _ in 0..MAX_SAMPLES_PER_PIXEL {
                // u,v are offsets that randomly choose a pixel around the current pixel
                let u = (i as f64 + rng.gen::<f64>()) / (image_width - 1) as f64;
                let v = (j as f64 + rng.gen::<f64>()) / (image_height - 1) as f64;
                let r: Ray = camera.get_ray(u, v);
                pixel_color += ray_color(&r, &world, MAX_BOUNCE_DEPTH);
            }
            pixel_color = color::multi_sample(pixel_color, MAX_SAMPLES_PER_PIXEL);

            image.push(pixel_color);
        }
    }

    // write the image to a file
    match ppm::write(path, image_width, image_height, &image) {
        Ok(()) => println!("test image created at {:?}", path),
        Err(e) => eprintln!("{}", e),
    }

}

/// determine if a Ray has hit a `hittable` in the `world` and returns the final pixel color
/// of the `Ray`, taking into account the `Material` of the `Hittable`, performing ray bouncing
/// (up to `MAX_BOUNCE_DEPTH` times) in order to get an accurate color determination. If nothing
/// was hit, than a linearly blended "sky" color is returned
fn ray_color<T>(r: &Ray, world: &T, depth: u32) -> Color
    where T: Hittable
{
    // exceeded the ray bounce limit, no more light is gathered
    if depth == 0 {
        Color::default()
    } else if let Some(ref rec) = world.hit(r, 0.001, f64::INFINITY) {
        if let Some(scatter_rec) = rec.mat_ptr.scatter(r, rec) {
            scatter_rec.attenuation * ray_color(&scatter_rec.scattered, world, depth - 1)
        } else {
            Color::default()
        }

    } else {
        // nothing hit, return a linear interpolated sky Color from white to blue based on height
        // of Ray's y coordinate
        let unit_direction = r.direction().unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);

        (1.0 - t) * Color::new(1.0, 1.0, 1.0)
            + t * Color::new(0.5, 0.7, 1.0)
    }
}