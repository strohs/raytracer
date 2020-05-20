use rand::{Rng};
use std::path::Path;
use threadpool::ThreadPool;

use raytracer::common::color;
use raytracer::common::{Color, Ray, Camera, Point3, Vec3};
use raytracer::hittable::{HittableList};
use raytracer::util::{scenes, ppm};
use std::sync::mpsc::{channel, Receiver};
use std::sync::Arc;
use std::time::Instant;

// max recursion depth allowed when bouncing rays of hittables
static MAX_BOUNCE_DEPTH: u32 = 50;
// maximum samples to use, per pixel, when anti-aliasing
static MAX_SAMPLES_PER_PIXEL: u32 = 100;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u32; // 576 if width is 1024 at 16:9
    let filename = format!("./raytrace_final_{}x{}.ppm", image_width, image_height);
    let path = Path::new(&filename);

    // camera settings
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let camera = Arc::new(Camera::new(
        look_from,
        look_at,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus));


    // generate a world with spheres in random locations
    let world = Arc::new(scenes::random_scene());
    let now = Instant::now();
    println!("rendering {}x{} image...", &image_width, &image_height);
    println!("using {} workers...", num_cpus::get_physical());
    let rx = render_multi_threaded(camera, world, num_cpus::get_physical(), image_height, image_width);


    // stores the final ray colors. image is stored from bottom to top, left to right
    let mut image: Vec<Color> = vec![Color::default(); (image_width * image_height) as usize];
    
    // read finished jobs from the channel
    for (pixel_row, pixel_col, pixel_color) in rx.iter() {
        // ppm images are stored in reverse row order (start from lower left of image to upper right)
        let index = ((image_height - 1 - pixel_col) * image_width + pixel_row) as usize;
        image[index] = pixel_color;
    }
    println!("done, total elapsed {:.3} secs", now.elapsed().as_secs_f64());

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
fn ray_color(r: &Ray, world: &HittableList, depth: u32) -> Color
{
    // exceeded the ray bounce limit, no more light is gathered
    if depth == 0 {
        Color::default()
    } else if let Some(ref rec) = world.check_for_hits(r, 0.001, f64::INFINITY) {
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

fn render_multi_threaded(camera: Arc<Camera>,
                         world: Arc<HittableList>,
                         num_workers: usize,
                         image_height: u32,
                         image_width: u32) -> Receiver<(u32, u32, Color)>
{
    // build a thread pool to render a pixel color per thread
    let pool = ThreadPool::new(num_workers);
    let (tx, rx) = channel();

    // traverse the image from lower left corner to upper right and generate pixel rendering jobs
    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let tx = tx.clone();
            let world = Arc::clone(&world);
            let camera = Arc::clone(&camera);

            pool.execute(move || {
                let mut rng = rand::thread_rng();

                let mut pixel_color: Color = Color::default(); // (0,0,0) color
                // multi-sample the pixels around the current pixel to compute an aliased pixel color
                for _ in 0..MAX_SAMPLES_PER_PIXEL {
                    // u,v are offsets that randomly choose a pixel close to the current pixel
                    let u = (i as f64 + rng.gen::<f64>()) / (image_width - 1) as f64;
                    let v = (j as f64 + rng.gen::<f64>()) / (image_height - 1) as f64;
                    let r: Ray = camera.get_ray(u, v);

                    pixel_color += ray_color(&r, &world, MAX_BOUNCE_DEPTH);
                }
                pixel_color = color::multi_sample(pixel_color, MAX_SAMPLES_PER_PIXEL);
                tx.send((i, j, pixel_color)).expect("error occurred rendering pixel");
            });
        }
    }
    println!("submitted {} pixel render jobs", image_width * image_height);
    rx
}