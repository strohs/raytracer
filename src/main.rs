
use raytracer::util::{scenes, ppm};
use raytracer::renderer;
use std::time::Instant;

fn main() {
    // aspect ratio for final image
    let aspect_ratio = 16.0 / 9.0;

    // desired image width
    let image_width = 640;

    // number of worker threads to spin up
    let pool_size = num_cpus::get_physical();

    // build the random "spheres on sphere" scene
    // let (camera, world, image_width, image_height) =
    //     scenes::build_random_sphere_scene(image_width, aspect_ratio);

    // let (camera, world, image_width, image_height) =
    //     scenes::build_two_perlin_spheres(image_width, aspect_ratio);

    // let (camera, world, image_width, image_height) =
    //     scenes::build_earth_sphere(image_width, aspect_ratio, "./earthmap.jpg");

    // let (camera, world, image_width, image_height) =
    //     scenes::build_two_perlin_spheres_with_light_source(image_width, aspect_ratio);

    // let (camera, world, image_width, image_height) =
    //     scenes::build_cornell_smoke_box(image_width, aspect_ratio);

    // let (camera, world, image_width, image_height) =
    //      scenes::build_cornell_box_with_two_boxes(image_width, aspect_ratio);

    // build final scene
    let (camera, world, image_width, image_height) =
        scenes::build_final_scene(image_width, aspect_ratio);
    
    let now = Instant::now();
    println!("rendering {}x{} image...", &image_width, &image_height);
    let image = renderer::render(
        camera,
        world,
        pool_size,
        image_width,
        image_height);
    println!("done, total elapsed {:.3} secs", now.elapsed().as_secs_f64());

    // write the image data to a PPM file
    let file_path = format!("./raytrace_{}x{}.ppm", image_width, image_height);
    match ppm::write_file(&file_path, image_width, image_height, &image) {
        Ok(()) => println!("test image created at {:?}", file_path),
        Err(e) => eprintln!("{}", e),
    }
}


