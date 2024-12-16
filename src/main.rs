use clap::Parser;
use raytracer::common::Color;
use raytracer::renderer::{BackgroundColor, Renderer};
use raytracer::scene::cornell_boxes::{build_cornell_box_with_two_boxes, build_cornell_smoke_box};
use raytracer::scene::earth::build_earth_scene;
use raytracer::scene::final_scene::build_final_scene;
use raytracer::scene::perlin_spheres::build_perlin_spheres;
use raytracer::scene::random_spheres::build_random_sphere_scene;
use raytracer::scene::Scene;
use raytracer::util::png;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(author, version, about = "rust raytracer")]
struct Args {
    #[clap(
        short,
        long,
        value_parser,
        default_value_t = 1.77,
        help = "aspect ratio for the rendered image. 16:9 = 1.77, 16:10 = 1.6"
    )]
    aspect_ratio: f64,
    #[clap(
        short,
        long,
        value_parser,
        default_value_t = 1024,
        help = "width of the image in pixels. Image height will be computed automatically from the aspect ratio"
    )]
    width: u32,
    #[clap(
        short,
        long,
        value_parser,
        default_value_t = 500,
        help = "number of samples to render per pixel. Higher values will increase render times but will produce a 'sharper' image"
    )]
    samples_per_pixel: u32,
    #[clap(arg_enum, value_parser, help = "the name of the scene to render")]
    scene: Scene,
}

fn main() {
    // parse the command line options using clap
    let args = Args::parse();

    // number of worker threads to use for rendering
    let pool_size = num_cpus::get_physical();

    // build the camera, world and set the background color for each scene
    let (camera, world, renderer) = match args.scene {
        Scene::RandomSpheres => {
            let (c, w) = build_random_sphere_scene(args.width, args.aspect_ratio);
            let renderer = Renderer::new(
                50,
                args.samples_per_pixel,
                BackgroundColor::LinearInterp(Color::new(1., 1., 1.), Color::new(0.5, 0.5, 1.0)),
                pool_size,
            );
            (c, w, renderer)
        }
        Scene::CornellBox => {
            let (c, w) = build_cornell_box_with_two_boxes(args.width, args.aspect_ratio);
            let renderer = Renderer::new(
                50,
                args.samples_per_pixel,
                BackgroundColor::Solid(Color::default()),
                pool_size,
            );
            (c, w, renderer)
        }
        Scene::CornellSmokeBoxes => {
            let (c, w) = build_cornell_smoke_box(args.width, args.aspect_ratio);
            let renderer = Renderer::new(
                50,
                args.samples_per_pixel,
                BackgroundColor::Solid(Color::default()),
                pool_size,
            );
            (c, w, renderer)
        }
        Scene::Earth => {
            let (c, w) = build_earth_scene(args.width, args.aspect_ratio, "./earthmap.jpg");
            let renderer = Renderer::new(
                50,
                args.samples_per_pixel,
                BackgroundColor::LinearInterp(Color::new(1., 1., 1.), Color::new(0.5, 0.5, 1.0)),
                pool_size,
            );
            (c, w, renderer)
        }
        Scene::PerlinSpheres => {
            let (c, w) = build_perlin_spheres(args.width, args.aspect_ratio);
            let renderer = Renderer::new(
                50,
                args.samples_per_pixel,
                BackgroundColor::LinearInterp(Color::new(1., 1., 1.), Color::new(0.5, 0.5, 1.0)),
                pool_size,
            );
            (c, w, renderer)
        }
        _ => {
            let (c, w) = build_final_scene(args.width, args.aspect_ratio);
            let renderer = Renderer::new(
                50,
                args.samples_per_pixel,
                BackgroundColor::Solid(Color::default()),
                pool_size,
            );
            (c, w, renderer)
        }
    };

    let (width, height) = (camera.image_width, camera.image_height);
    let file_path = PathBuf::from(format!(
        "./raytrace_{:?}_{}x{}.png",
        args.scene, width, height
    ));
    println!("rendering scene: {:?}", &args.scene);

    let image = renderer.render(camera, world);
    // write the image data to a PNG file
    match png::write_file(&file_path, width, height, &image) {
        Ok(()) => println!("test image created at {:?}", file_path),
        Err(e) => eprintln!("{}", e),
    }
}
