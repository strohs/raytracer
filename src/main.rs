
use raytracer::util::{scenes, ppm};
use std::time::Instant;
use raytracer::renderer;
use raytracer::renderer::{RenderJob, BackgroundColor};
use raytracer::common::Color;
use std::{env, process};
use raytracer::util::command::Command;
use raytracer::util::scenes::Scene;

fn main() {

    let args: Vec<String> = env::args().collect();

    let command = Command::new(args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let job = match command.scene {
        Scene::RandomSpheres => {
            let (c, w) = scenes::build_random_sphere_scene(command.width, command.aspect_ratio);
            RenderJob::from(
                w, c,
                BackgroundColor::LinearInterp(Color::new(1.,1.,1.,), Color::new(0.5, 0.5, 1.0)),
                50,
                command.samples_per_pixel)
        },
        Scene::CornellBox => {
            let (c, w) = scenes::build_cornell_box_with_two_boxes(command.width, command.aspect_ratio);
            RenderJob::from(
                w, c,
                BackgroundColor::Solid(Color::default()),
                50,
                command.samples_per_pixel)
        },
        Scene::CornellSmokeBoxes => {
            let (c, w) = scenes::build_cornell_smoke_box(command.width, command.aspect_ratio);
            RenderJob::from(
                w, c,
                BackgroundColor::Solid(Color::default()),
                50,
                command.samples_per_pixel)
        },
        Scene::Earth => {
            let (c, w) = scenes::build_earth_scene(command.width, command.aspect_ratio, "./earthmap.jpg");
            RenderJob::from(
                w, c,
                BackgroundColor::LinearInterp(Color::new(1.,1.,1.,), Color::new(0.5, 0.5, 1.0)),
                50,
                command.samples_per_pixel)
        },
        Scene::PerlinSpheres => {
            let (c, w) = scenes::build_two_perlin_spheres(command.width, command.aspect_ratio);
            RenderJob::from(
                w, c,
                BackgroundColor::LinearInterp(Color::new(1.,1.,1.,), Color::new(0.5, 0.5, 1.0)),
                50,
                command.samples_per_pixel)
        },
        _ => {
            let (c, w) = scenes::build_final_scene(command.width, command.aspect_ratio);
            RenderJob::from(
                w, c,
                BackgroundColor::Solid(Color::default()),
                50,
                command.samples_per_pixel)
        }
    };

    // number of worker threads to spin up
    let pool_size = num_cpus::get_physical();
    let (width, height) = (job.camera.image_width, job.camera.image_height);

    let now = Instant::now();
    println!("{:?} rendering as {}x{} image. aspect-ratio:{:.3}  bounce_depth:{}  samples_per_pixel:{}",
             &command.scene, &job.camera.image_width, &job.camera.image_height, &command.aspect_ratio, &job.ray_bounce_depth, &job.samples_per_pixel);

    let image = renderer::render(job, pool_size);

    println!("done, total elapsed {:.3} secs", now.elapsed().as_secs_f64());

    let file_path = format!("./raytrace_{}x{}.ppm", width, height);
    // write the image data to a PPM file
    match ppm::write_file(&file_path, width, height, &image) {
        Ok(()) => println!("test image created at {:?}", file_path),
        Err(e) => eprintln!("{}", e),
    }


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
    //     10_000);

    // build final scene
    // let (camera, world) =
    //     scenes::build_final_scene(image_width, aspect_ratio);
    // let rjob = RenderJob::from(
    //     world, camera,
    //     BackgroundColor::Solid(Color::default()),
    //     50,
    //     10_000);


}


