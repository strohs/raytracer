use std::sync::Arc;
use std::sync::mpsc::{Sender, channel};
use threadpool::ThreadPool;
use rand::{Rng};

use crate::common::{Ray, Color, Camera, color};
use crate::hittable::{Hittable, BvhNode, HittableList};

// max recursion depth allowed when bouncing rays of hittables
const MAX_RAY_BOUNCE_DEPTH: u32 = 50;
// maximum samples to use, per pixel, when anti-aliasing
const MAX_SAMPLES_PER_PIXEL: u32 = 1000;


/// Renders an image using the provided `Camera` and `World`. `num_workers` is the number of
/// **operating system threads** to use for rendering, this should be set to the number of
/// physical or logical cores on the machine you are rendering on.
/// `image_width`, `image_height` is the desired width and height of the final image.
///
/// # Returns
/// a Vector of `Color` representing the final color of each pixel in the image.
/// The image is stored row by row,  from top left of the image to the bottom right
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
                    let pixel_color = multi_sample_pixel(i, j,
                                                         &camera,
                                                         &*world,
                                                         image_width, image_height);
                    tx.send((i, j, pixel_color)).expect("error occurred rendering pixel");
                });
            }
        }
        println!("submitted {} pixel render jobs with a thread pool size = {}",
                 image_width * image_height,
                 num_workers);
        rx
    };

    // allocate a vector to store the pixel colors of the image (in row major format)
    let mut image: Vec<Color> = vec![Color::default(); (image_width * image_height) as usize];

    // read finished jobs data from the channel and store in image vector
    for (pixel_col, pixel_row, pixel_color) in rx.iter() {
        let idx = (pixel_row * image_width + pixel_col) as usize;
        image[idx] = pixel_color;
    }

    image
}

/// determine if a Ray has hit a `Hittable` object in the `world` and compute the pixel color
/// of the Ray, `r`. The Hittable's `Material` is taken into account when performing ray bouncing
/// (up to `MAX_RAY_BOUNCE_DEPTH` times) in order to get an accurate color determination. If nothing
/// was hit, than a linearly blended "sky" color is returned
fn ray_color<T: Hittable + ?Sized>(
    r: &Ray,
    world: &T,
    background: &Color,
    depth: u32) -> Color
{
    // exceeded the ray bounce limit, no more light is gathered
    if depth == 0 {
        return Color::default();
    }

    // if a hittable in the world was hit, determine if its material will scatter the incoming
    // ray, AND if its material emits light
    if let Some(ref rec) = world.hit(r, 0.001, f64::INFINITY) {
        let emitted = rec.mat_ptr.emitted(rec.u, rec.v, &rec.p);

        if let Some(scatter_rec) = rec.mat_ptr.scatter(r, rec) {
            return emitted
                + scatter_rec.attenuation
                * ray_color(&scatter_rec.scattered, world, background, depth - 1)
        } else {
            return emitted;
        }

    } else {
        // nothing hit, return the background color
        return *background;
    }
}

/// Computes the color of the pixel located at `pw,ph` (pixel width, pixel height) by sampling
/// the pixels around it.
fn multi_sample_pixel<T: Hittable + ?Sized>(
    pw: u32,
    ph: u32,
    camera: &Camera,
    world: &T,
    image_width: u32,
    image_height: u32) -> Color
{
    let mut rng = rand::thread_rng();
    let mut pixel_color = Color::default();
    let background = Color::default();

    for _ in 0..MAX_SAMPLES_PER_PIXEL {
        // u,v are offsets that randomly choose a point close to the current pixel
        let u = (pw as f64 + rng.gen::<f64>()) / (image_width - 1) as f64;
        let v = (ph as f64 + rng.gen::<f64>()) / (image_height - 1) as f64;

        let r: Ray = camera.get_ray(u, v);

        pixel_color += ray_color(&r, world, &background, MAX_RAY_BOUNCE_DEPTH);
    }

    color::multi_sample(&pixel_color, MAX_SAMPLES_PER_PIXEL)
}