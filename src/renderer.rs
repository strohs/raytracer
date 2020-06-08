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
/// The colors of the image are stored in row major format, starting from top left
/// to the bottom right
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

        // traverse the image from upper left corner to lower right corner and generate pixel
        // render jobs
        for row in 0..image_height {
            let tx = Sender::clone(&tx);
            let world = Arc::clone(&bvh);
            let camera = Arc::clone(&camera);

            pool.execute(move || {
                let row_colors = render_scanline(row,
                                                     &camera,
                                                     &*world,
                                                     image_width, image_height);
                tx.send((row, row_colors)).expect("error occurred rendering pixel");
            });
        }
        println!("submitted {} scanline render jobs with a thread pool size = {}",
                 image_height,
                 num_workers);
        rx
    };

    // allocate a vector to store the pixel colors of the image (in row major format)
    let mut image: Vec<Color> = vec![Color::default(); (image_width * image_height) as usize];

    // read finished jobs data from the channel and store in image vector
    for (row, row_colors) in rx.iter() {
        println!("row {} of {} finished...", &row, &image_height);
        let ridx = (row * image_width) as usize;
        let image_slice = &mut image[ridx..(ridx + image_width as usize)];
        for (i, color) in row_colors.into_iter().enumerate() {
            image_slice[i] = color;
        }
    }

    image
}

/// determine if a Ray has hit a `Hittable` object in the `world` and compute the pixel color
/// of the Ray, `r`. The Hittable's `Material` is taken into account when performing ray bouncing
/// (up to `MAX_RAY_BOUNCE_DEPTH` times) in order to get an accurate color determination. If nothing
/// was hit then the `background` color is returned, than a linearly blended "sky" color is returned
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

    // if a hittable was hit, determine if its material will scatter the incoming
    // ray, AND how much light the material emits
    if let Some(ref rec) = world.hit(r, 0.001, f64::INFINITY) {
        let emitted = rec.mat_ptr.emitted(rec.u, rec.v, &rec.p);

        if let Some(scatter_rec) = rec.mat_ptr.scatter(r, rec) {
            emitted
                + scatter_rec.attenuation
                * ray_color(&scatter_rec.scattered, world, background, depth - 1)
        } else {
            emitted
        }

    } else {
        // nothing hit, return the background color
        *background
    }
}


/// Computes the color of a row (scanline) of pixels. `row` is the current row being rendered,
/// where row ranges from 0..image_height
/// Returns a Vector containing the final pixel colors of the row
fn render_scanline<T: Hittable + ?Sized>(
    row: u32,
    camera: &Camera,
    world: &T,
    image_width: u32,
    image_height: u32) -> Vec<Color>
{
    let mut rng = rand::thread_rng();
    let mut colors: Vec<Color> = Vec::with_capacity(image_width as usize);

    for col in 0..image_width {
        let mut pixel_color = Color::default();
        let background = Color::default();

        for _ in 0..MAX_SAMPLES_PER_PIXEL {
            // u,v are offsets that randomly choose a point close to the current pixel
            let u = (col as f64 + rng.gen::<f64>()) / (image_width - 1) as f64;
            let v = (row as f64 + rng.gen::<f64>()) / (image_height - 1) as f64;

            let r: Ray = camera.get_ray(u, v);

            pixel_color += ray_color(&r, world, &background, MAX_RAY_BOUNCE_DEPTH);
        }
        colors.push(color::multi_sample(&pixel_color, MAX_SAMPLES_PER_PIXEL));
    }
    colors
}