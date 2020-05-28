use std::sync::Arc;
use std::sync::mpsc::{Sender, channel};
use threadpool::ThreadPool;
use rand::{Rng};

use crate::common::{Ray, Color, Camera, color};
use crate::hittable::{Hittable, BvhNode, HittableList};

// max recursion depth allowed when bouncing rays of hittables
const MAX_RAY_BOUNCE_DEPTH: u32 = 50;
// maximum samples to use, per pixel, when anti-aliasing
const MAX_SAMPLES_PER_PIXEL: u32 = 100;


/// Renders an image using the provided `Camera` and `World`. `num_workers` is the number of
/// **os threads** to use for rendering, this should be set to the number of physical or logical
/// cores on the machine you are rendering on.
/// `image_width`, `image_height` is the desired width/height of the final image.
///
/// # Returns
/// a Vec of `Color` where each Color represents a pixel color. The image is returned in reverse
/// row order, meaning the bottom rows of the image appear first in the Vec
pub fn render(camera: Camera,
              mut world: HittableList,
              num_workers: usize,
              image_width: u32,
              image_height: u32) -> Vec<Color>
{
    // build a thread pool to render a pixel color per thread
    let pool = ThreadPool::new(num_workers);
    
    // build the Bounded Volume Hierarchy
    let bvh: Arc<dyn Hittable> = Arc::new(BvhNode::from(&mut world, 0.0, 1.0));
    let camera = Arc::new(camera);

    let rx = {
        let (tx, rx) = channel();

        // traverse the image from lower left corner to upper right and generate pixel rendering jobs
        for j in (0..image_height).rev() {
            for i in 0..image_width {
                let tx = Sender::clone(&tx);
                let world = Arc::clone(&bvh);
                let camera = Arc::clone(&camera);

                pool.execute(move || {
                    let pixel_color = multi_sample_pixel(i, j, &camera, &*world, image_width, image_height);
                    tx.send((i, j, pixel_color)).expect("error occurred rendering pixel");
                });
            }
        }
        println!("submitted {} pixel render jobs with a thread pool size = {}", image_width * image_height, num_workers);
        rx
    };

    // allocate a vector to store the pixel colors of the image (in row major format)
    let mut image: Vec<Color> = vec![Color::default(); (image_width * image_height) as usize];

    // read finished jobs data from the channel
    for (pixel_row, pixel_col, pixel_color) in rx.iter() {
        // ppm images are stored in reverse row order (start from lower left of image to upper right)
        let index = ((image_height - 1 - pixel_col) * image_width + pixel_row) as usize;
        image[index] = pixel_color;
    }

    image
}

/// determine if a Ray has hit a `Hittable` object in the `world` and compute the pixel color
/// of the `Ray`. The Hittable's `Material` is taken into account when performing ray bouncing
/// (up to `MAX_RAY_BOUNCE_DEPTH` times) in order to get an accurate color determination. If nothing
/// was hit, than a linearly blended "sky" color is returned
fn ray_color<T: Hittable + ?Sized>(r: &Ray, world: &T, depth: u32) -> Color
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

/// determine the color of the specified pixel using multisampling. `pw,ph` are the width and
/// height of the pixel in the image
fn multi_sample_pixel<T: Hittable + ?Sized>(pw: u32,
                      ph: u32,
                      camera: &Camera,
                      world: &T,
                      image_width: u32,
                      image_height: u32) -> Color
{
    let mut rng = rand::thread_rng();
    let mut pixel_color = Color::default();

    for _ in 0..MAX_SAMPLES_PER_PIXEL {
        // u,v are offsets that randomly choose a pixel close to the current pixel
        let u = (pw as f64 + rng.gen::<f64>()) / (image_width - 1) as f64;
        let v = (ph as f64 + rng.gen::<f64>()) / (image_height - 1) as f64;

        let r: Ray = camera.get_ray(u, v);

        pixel_color += ray_color(&r, world, MAX_RAY_BOUNCE_DEPTH);
    }


    color::multi_sample(pixel_color, MAX_SAMPLES_PER_PIXEL)
}