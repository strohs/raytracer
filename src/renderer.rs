use rand::Rng;
use std::sync::mpsc::{channel, Sender};
use std::sync::Arc;
use std::time::Instant;
use threadpool::ThreadPool;

use crate::common;
use crate::common::{Camera, Color, Ray};
use crate::hittable::{BvhNode, Hittable, HittableList};

/// Indicates what background color should be used by a renderer
/// Currently only two options are supported:
/// `Solid` - a solid color should be used for the background
/// `LinearInterp(Color1, Color2)` - use linear interpolation to render the background color
///  between color1 and color2
#[derive(Debug, Copy, Clone)]
pub enum BackgroundColor {
    Solid(Color),
    LinearInterp(Color, Color),
}

/// A Renderer will use ray-tracing to render a scene using a Camera and a list of Hittables.
///
/// `ray_bounce_depth` limits the level of recursion performed when computing a ray's color.
/// 50 is a good default value
/// `sample_per_pixel` controls the amount of multi-sampling performed on each pixel in the
/// scene. Higher values will render a more accurate scene, but will drastically increase
/// the render time. `500` is a good initial value while `10_000` will produce some stunning
/// images
/// `background_color` sets the default color of the renderer. This color is used as the
/// default ray color when a ray does not hit something
/// `num_workers` is the number of **Operating System threads** to spawn for rendering. Ideally
/// this should be equal to the number of physical cores on your machine
///
#[derive(Debug, Copy, Clone)]
pub struct Renderer {
    background_color: BackgroundColor,
    ray_bounce_depth: u32,
    samples_per_pixel: u32,
    num_workers: usize,
}

impl Renderer {
    /// Returns a new renderer.
    pub fn new(
        ray_bounce_depth: u32,
        samples_per_pixel: u32,
        background_color: BackgroundColor,
        num_workers: usize,
    ) -> Self {
        Self {
            ray_bounce_depth,
            samples_per_pixel,
            background_color,
            num_workers,
        }
    }

    /// Returns this renderer's bounce depth setting
    pub fn ray_bounce_depth(&self) -> u32 {
        self.ray_bounce_depth
    }

    /// Returns this renderer's samples per pixel setting
    pub fn samples_per_pixel(&self) -> u32 {
        self.samples_per_pixel
    }

    /// Returns this renderer's background color setting
    pub fn background_color(&self) -> BackgroundColor {
        self.background_color
    }

    /// Renders an image using the provided `Camera` and `World`.
    ///
    /// # Returns
    /// a Vector of `Color`s representing the final color of each pixel in the image.
    /// The colors of the image are stored in row major format, starting from top left
    /// to the bottom right
    pub fn render(self, camera: Camera, mut world: HittableList) -> Vec<Color> {
        let now = Instant::now();
        println!(
            "rendering a {}x{} image. threads={}  bounce_depth={}  samples_per_pixel={}",
            &camera.image_width,
            &camera.image_height,
            &self.num_workers,
            &self.ray_bounce_depth,
            &self.samples_per_pixel
        );

        // build a thread pool to render a pixel color per thread
        let pool = ThreadPool::new(self.num_workers);

        // build a BVH
        let world: Arc<dyn Hittable> = Arc::new(BvhNode::from(&mut world, 0.0, 1.0));
        let camera = Arc::new(camera);

        let rx = {
            let (tx, rx) = channel();

            // traverse the image from upper left corner to lower right corner and generate pixel
            // render jobs
            for row in 0..camera.image_height {
                let tx = Sender::clone(&tx);
                let world = Arc::clone(&world);
                let camera = Arc::clone(&camera);

                pool.execute(move || {
                    let row_colors = self.render_scanline(row, &*world, &camera);
                    tx.send((row, row_colors))
                        .expect("error occurred rendering");
                });
            }
            println!(
                "submitted {} scanline render jobs with a thread pool size = {}",
                &camera.image_height, &self.num_workers
            );
            rx
        };

        // allocate a vector to store the pixel colors of the image (in row major format)
        let mut image: Vec<Color> =
            vec![Color::default(); (camera.image_width * camera.image_height) as usize];

        // read finished jobs data from the channel and store in image vector
        for (row, row_colors) in rx.iter() {
            println!("row {} of {} finished...", &row, &camera.image_height);
            let ridx = (row * camera.image_width) as usize;
            let image_slice = &mut image[ridx..(ridx + camera.image_width as usize)];
            for (i, color) in row_colors.into_iter().enumerate() {
                image_slice[i] = color;
            }
        }
        println!(
            "done rendering, total elapsed {:.3} secs",
            now.elapsed().as_secs_f64()
        );

        image
    }

    /// Computes the color of a row (scanline) of pixels. `row` is the current row being rendered,
    /// where row ranges from 0..image_height
    /// Returns a Vector containing the final pixel colors of the row
    fn render_scanline<T: Hittable + ?Sized>(
        &self,
        row: u32,
        world: &T,
        camera: &Camera,
    ) -> Vec<Color> {
        let mut rng = rand::thread_rng();
        let mut colors: Vec<Color> = Vec::with_capacity(camera.image_width as usize);

        for col in 0..camera.image_width {
            let mut pixel_color = Color::default();

            for _ in 0..self.samples_per_pixel {
                // u,v are offsets that randomly choose a point close to the current pixel
                let u = (col as f64 + rng.gen::<f64>()) / (camera.image_width - 1) as f64;
                let v = (row as f64 + rng.gen::<f64>()) / (camera.image_height - 1) as f64;

                let r: Ray = camera.get_ray(u, v);

                pixel_color += self.ray_color(&r, world, self.ray_bounce_depth);
            }
            colors.push(Renderer::multi_sample(&pixel_color, self.samples_per_pixel));
        }
        colors
    }

    /// determine if a Ray has hit a `Hittable` object in the `world` and compute the pixel color
    /// of the Ray, `r`. The Hittable's `Material` is taken into account when performing ray bouncing
    /// (up to `MAX_RAY_BOUNCE_DEPTH` times) in order to get an accurate color determination. If nothing
    /// was hit then the `background` color is returned, than a linearly blended "sky" color is returned
    fn ray_color<T: Hittable + ?Sized>(&self, ray: &Ray, world: &T, depth: u32) -> Color {
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
                    + scatter_rec.attenuation * self.ray_color(&scatter_rec.scattered, world, depth - 1)
            } else {
                emitted
            }
        } else {
            // nothing hit, return the background color
            match self.background_color {
                BackgroundColor::Solid(color) => color,
                BackgroundColor::LinearInterp(from, to) => Renderer::linear_blend(ray, &from, &to),
            }
        }
    }

    /// Returns a linearly blended color between `from` and `to`. The input `ray`s
    /// y coordinate to determine how much of `from` or `to` to apply.
    fn linear_blend(ray: &Ray, from: &Color, to: &Color) -> Color {
        let unit_direction = ray.direction().unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        // blue is 0.5, 0.7, 1.0
        (1.0 - t) * *from + t * *to
    }

    /// Returns a new pixel color using multi-sample color computation
    fn multi_sample(pixel_color: &Color, samples_per_pixel: u32) -> Color {
        let mut r = pixel_color.x();
        let mut g = pixel_color.y();
        let mut b = pixel_color.z();

        // divide the color total by the number of samples and gamma correct for gamma=2.0
        let scale = 1.0 / samples_per_pixel as f64;
        r = f64::sqrt(scale * r);
        g = f64::sqrt(scale * g);
        b = f64::sqrt(scale * b);

        // compute a translated [0..=255] color value for each color's R,G,B
        Color::new(
            256.0 * common::clamp(r, 0.0, 0.999),
            256.0 * common::clamp(g, 0.0, 0.999),
            256.0 * common::clamp(b, 0.0, 0.999),
        )
    }
}
