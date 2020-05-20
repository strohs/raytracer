
use std::path::Path;

use raytracer::util::{scenes, ppm};
use raytracer::renderer;
use std::time::Instant;
use std::sync::Arc;

fn main() {
    // aspect ratio for final image
    let aspect_ratio = 16.0 / 9.0;
    // desired image width
    let image_width = 1280;
    let pool_size = num_cpus::get_physical();

    let (camera, world, image_width, image_height) =
        scenes::build_default_sphere_scene(image_width, aspect_ratio);

    let now = Instant::now();
    println!("rendering {}x{} image...", &image_width, &image_height);

    let image = renderer::render(
        Arc::new(camera),
        Arc::new(world),
        pool_size,
        image_width,
        image_height);

    println!("done, total elapsed {:.3} secs", now.elapsed().as_secs_f64());

    let filename = format!("./raytrace_final_{}x{}.ppm", image_width, image_height);
    let path = Path::new(&filename);

    // write the image to a file
    match ppm::write(path, image_width, image_height, &image) {
        Ok(()) => println!("test image created at {:?}", path),
        Err(e) => eprintln!("{}", e),
    }
}


