use std::sync::Arc;
use std::sync::mpsc::{Sender, channel};
use threadpool::ThreadPool;
use rand::{Rng};

use crate::common::{Ray, Color, Camera, color};
use crate::hittable::{Hittable, BvhNode, HittableList};

// max recursion depth allowed when bouncing rays of hittables
//const MAX_RAY_BOUNCE_DEPTH: u32 = 50;
// maximum samples to use, per pixel, when anti-aliasing
//const MAX_SAMPLES_PER_PIXEL: u32 = 500;

#[derive(Debug, Copy, Clone)]
pub enum BackgroundColor {
    Solid(Color),
    LinearInterp(Color, Color),
}

#[derive(Debug)]
pub struct RenderJob {
    pub world: BvhNode,
    pub camera: Camera,
    pub background_color: BackgroundColor,
    pub ray_bounce_depth: u32,
    pub samples_per_pixel: u32,
}

impl RenderJob {

    pub fn from(
        mut hittables: HittableList,
        camera: Camera,
        background_color: BackgroundColor,
        ray_bounce_depth: u32,
        samples_per_pixel: u32) -> Self
    {
        // convert hittable list into a BVH
        let world = BvhNode::from(&mut hittables, 0.0, 1.0);

        Self {
            world,
            camera,
            background_color,
            ray_bounce_depth,
            samples_per_pixel,
        }
    }
}

/// Renders an image using the provided `Camera` and `World`. `num_workers` is the number of
/// **operating system threads** to use for rendering, this should be set to the number of
/// physical or logical cores on the machine you are rendering on.
/// `image_width`, `image_height` is the desired width and height of the final image.
///
/// # Returns
/// a Vector of `Color` representing the final color of each pixel in the image.
/// The colors of the image are stored in row major format, starting from top left
/// to the bottom right
pub fn render(job: RenderJob, num_workers: usize) -> Vec<Color>
{
    // build a thread pool to render a pixel color per thread
    let pool = ThreadPool::new(num_workers);

    let (image_width, image_height) = (job.camera.image_width, job.camera.image_height);
    let world: Arc<dyn Hittable> = Arc::new(job.world);
    let camera = Arc::new(job.camera);

    let rx = {
        let (tx, rx) = channel();

        // traverse the image from upper left corner to lower right corner and generate pixel
        // render jobs
        for row in 0..image_height {
            let tx = Sender::clone(&tx);
            let world = Arc::clone(&world);
            let camera = Arc::clone(&camera);
            let background_color = job.background_color;
            let samples_per_pixel = job.samples_per_pixel;
            let ray_bounce_depth = job.ray_bounce_depth;

            pool.execute(move || {
                let row_colors = render_scanline(
                    row,
                    &*world,
                    &camera,
                    background_color,
                    samples_per_pixel,
                    ray_bounce_depth);
                tx.send((row, row_colors)).expect("error occurred rendering pixel");
            });
        }
        println!("submitted {} scanline render jobs with a thread pool size = {}",
                 &image_height,
                 &num_workers);
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


/// Computes the color of a row (scanline) of pixels. `row` is the current row being rendered,
/// where row ranges from 0..image_height
/// Returns a Vector containing the final pixel colors of the row
fn render_scanline<T: Hittable + ?Sized>(
    row: u32,
    world: &T,
    camera: &Camera,
    background_color: BackgroundColor,
    samples_per_pixel: u32,
    ray_bounce_depth: u32) -> Vec<Color>
{
    let mut rng = rand::thread_rng();
    let mut colors: Vec<Color> = Vec::with_capacity(camera.image_width as usize);

    for col in 0..camera.image_width {
        let mut pixel_color = Color::default();

        for _ in 0..samples_per_pixel {
            // u,v are offsets that randomly choose a point close to the current pixel
            let u = (col as f64 + rng.gen::<f64>()) / (camera.image_width - 1) as f64;
            let v = (row as f64 + rng.gen::<f64>()) / (camera.image_height - 1) as f64;

            let r: Ray = camera.get_ray(u, v);

            pixel_color += ray_color(&r, world, background_color, ray_bounce_depth);
        }
        colors.push(color::multi_sample(&pixel_color, samples_per_pixel));
    }
    colors
}



/// determine if a Ray has hit a `Hittable` object in the `world` and compute the pixel color
/// of the Ray, `r`. The Hittable's `Material` is taken into account when performing ray bouncing
/// (up to `MAX_RAY_BOUNCE_DEPTH` times) in order to get an accurate color determination. If nothing
/// was hit then the `background` color is returned, than a linearly blended "sky" color is returned
fn ray_color<T: Hittable + ?Sized>(
    ray: &Ray,
    world: &T,
    background: BackgroundColor,
    depth: u32) -> Color
{
    // exceeded the ray bounce limit, no more light is gathered
    if depth == 0 {
        return Color::default();
    }

    // if a hittable was hit, determine if its material will scatter the incoming
    // ray, AND how much light the material emits
    if let Some(ref rec) = world.hit(ray, 0.001, f64::INFINITY) {
        let emitted = rec.mat_ptr.emitted(rec.u, rec.v, &rec.p);

        if let Some(scatter_rec) = rec.mat_ptr.scatter(ray, rec) {
            emitted
                + scatter_rec.attenuation
                * ray_color(&scatter_rec.scattered, world, background, depth - 1)
        } else {
            emitted
        }

    } else {
        // nothing hit, return the background color
        match background {
            BackgroundColor::Solid(color) => color,
            BackgroundColor::LinearInterp(from, to) => linear_blend(ray, &from, &to)
        }
    }
}


fn linear_blend(ray: &Ray, from: &Color, to: &Color) -> Color {
    let unit_direction = ray.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    // blue is 0.5, 0.7, 1.0
    (1.0 - t) * *from + t * *to
}

