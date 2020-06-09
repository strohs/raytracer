
use raytracer::util::{scenes, ppm};
use std::time::Instant;
use raytracer::renderer;
use raytracer::renderer::{RenderJob, BackgroundColor};
use raytracer::common::Color;

fn main() {
    // aspect ratio for final image
    let aspect_ratio = 16.0 / 9.0;

    // desired image width
    let image_width = 640;


    // random sphere scene
    // let (camera, world) = scenes::build_random_sphere_scene(image_width, aspect_ratio);
    // let rjob = RenderJob::from(
    //     world, camera,
    //     BackgroundColor::LinearInterp(Color::new(1.,1.,1.,), Color::new(0.5, 0.5, 1.0)),
    //     50,
    //     1000);

    // two checkered spheres
    // let (camera, world) = scenes::build_two_checkered_spheres(image_width, aspect_ratio);
    // let rjob = RenderJob::from(
    //     world, camera,
    //     BackgroundColor::LinearInterp(Color::new(1.,1.,1.,), Color::new(0.5, 0.5, 1.0)),
    //     50,
    //     1000);


    // two perlin spheres stacked on top of each other
    // let (camera, world) = scenes::build_two_perlin_spheres(image_width, aspect_ratio);
    // let rjob = RenderJob::from(
    //     world, camera,
    //     BackgroundColor::LinearInterp(Color::new(1.,1.,1.,), Color::new(0.5, 0.5, 1.0)),
    //     50,
    //     1000);



    // build a textured earth sphere
    // let (camera, world) = scenes::build_earth_scene(image_width, aspect_ratio, "./earthmap.jpg");
    // let rjob = RenderJob::from(
    //     world, camera,
    //     BackgroundColor::LinearInterp(Color::new(1.,1.,1.,), Color::new(0.5, 0.5, 1.0)),
    //     50,
    //     500);


    // build two perlin spheres, lit by a rectangular light source against a black background
    // let (camera, world) =
    //     scenes::build_two_perlin_spheres_with_light_source(image_width, aspect_ratio);
    // let rjob = RenderJob::from(
    //     world, camera,
    //     BackgroundColor::Solid(Color::default()),
    //     50,
    //     500);

    // let (camera, world, image_width, image_height) =
    //     scenes::build_cornell_smoke_box(image_width, aspect_ratio);


    // let (camera, world) =
    //     scenes::build_cornell_box_with_two_boxes(image_width, aspect_ratio);
    // let rjob = RenderJob::from(
    //     world, camera,
    //     BackgroundColor::Solid(Color::default()),
    //     50,
    //     1000);

    // build final scene
    let (camera, world) =
        scenes::build_final_scene(image_width, aspect_ratio);
    let rjob = RenderJob::from(
        world, camera,
        BackgroundColor::Solid(Color::default()),
        50,
        500);

    // number of worker threads to spin up
    let pool_size = num_cpus::get_physical();
    
    let now = Instant::now();
    println!("rendering {}x{} image. bounce_depth:{}  samples_per_pixel:{}",
             &camera.image_width, &camera.image_height, &rjob.ray_bounce_depth, &rjob.samples_per_pixel);

    let image = renderer::render(rjob, pool_size);

    println!("done, total elapsed {:.3} secs", now.elapsed().as_secs_f64());

    // write the image data to a PPM file
    let file_path = format!("./raytrace_{}x{}.ppm", &camera.image_width, &camera.image_height);
    match ppm::write_file(&file_path, camera.image_width, camera.image_height, &image) {
        Ok(()) => println!("test image created at {:?}", file_path),
        Err(e) => eprintln!("{}", e),
    }
}


