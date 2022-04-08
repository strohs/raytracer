use raytracer::common::Color;
use raytracer::renderer::{BackgroundColor, Renderer};
use raytracer::util::command::Command;
use raytracer::util::scenes::Scene;
use raytracer::util::{ppm, scenes};
use std::{env, process};

fn main() {
    // parse the command line options
    let args: Vec<String> = env::args().collect();
    let command = Command::new(args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    // number of worker threads to use for rendering
    let pool_size = num_cpus::get_physical();

    // build the camera, world and set the background color for each scene
    let (camera, world, renderer) = match command.scene {
        Scene::RandomSpheres => {
            let (c, w) = scenes::build_random_sphere_scene(command.width, command.aspect_ratio);
            let renderer = Renderer::new(
                50,
                command.samples_per_pixel,
                BackgroundColor::LinearInterp(Color::new(1., 1., 1.), Color::new(0.5, 0.5, 1.0)),
                pool_size,
            );
            (c, w, renderer)
        }
        Scene::CornellBox => {
            let (c, w) =
                scenes::build_cornell_box_with_two_boxes(command.width, command.aspect_ratio);
            let renderer = Renderer::new(
                50,
                command.samples_per_pixel,
                BackgroundColor::Solid(Color::default()),
                pool_size,
            );
            (c, w, renderer)
        }
        Scene::CornellSmokeBoxes => {
            let (c, w) = scenes::build_cornell_smoke_box(command.width, command.aspect_ratio);
            let renderer = Renderer::new(
                50,
                command.samples_per_pixel,
                BackgroundColor::Solid(Color::default()),
                pool_size,
            );
            (c, w, renderer)
        }
        Scene::Earth => {
            let (c, w) =
                scenes::build_earth_scene(command.width, command.aspect_ratio, "./earthmap.jpg");
            let renderer = Renderer::new(
                50,
                command.samples_per_pixel,
                BackgroundColor::LinearInterp(Color::new(1., 1., 1.), Color::new(0.5, 0.5, 1.0)),
                pool_size,
            );
            (c, w, renderer)
        }
        Scene::PerlinSpheres => {
            let (c, w) = scenes::build_two_perlin_spheres(command.width, command.aspect_ratio);
            let renderer = Renderer::new(
                50,
                command.samples_per_pixel,
                BackgroundColor::LinearInterp(Color::new(1., 1., 1.), Color::new(0.5, 0.5, 1.0)),
                pool_size,
            );
            (c, w, renderer)
        }
        _ => {
            let (c, w) = scenes::build_final_scene(command.width, command.aspect_ratio);
            let renderer = Renderer::new(
                50,
                command.samples_per_pixel,
                BackgroundColor::Solid(Color::default()),
                pool_size,
            );
            (c, w, renderer)
        }
    };

    let (width, height) = (camera.image_width, camera.image_height);
    let file_path = format!("./raytrace_{}x{}.ppm", width, height);
    println!("rendering scene: {:?}", &command.scene);

    let image = renderer.render(camera, world);
    // write the image data to a PPM file
    match ppm::write_file(&file_path, width, height, &image) {
        Ok(()) => println!("test image created at {:?}", file_path),
        Err(e) => eprintln!("{}", e),
    }
}
