use crate::common::{Camera, CameraBuilder, Color, Point3, Vec3};
use crate::hittable::builder::{
    build_checker_sphere, build_dielectric_sphere, build_metal_sphere, build_perlin_sphere,
};
use crate::hittable::{HittableList, MovingSphere, Sphere};
use crate::material::{Dielectric, Lambertian, Material, Metal};
use crate::texture::{SolidColor, Texture};
use rand::Rng;
use std::sync::Arc;

/// builds and returns the Camera and HittableList for the random sphere scene.
/// This scene contains 484 small spheres randomly positioned around
/// 3 bigger spheres. These are then positioned on top of an enormous sphere with a checkerboard
/// texture, which acts as the ground plane
pub fn build_random_sphere_scene(image_width: u32, aspect_ratio: f64) -> (Camera, HittableList) {
    // build the camera
    let camera = CameraBuilder::new()
        .look_from(Point3::new(13.0, 2.0, 3.0))
        .look_at(Point3::new(0.0, 0.0, 0.0))
        .up_direction(Vec3::new(0.0, 1.0, 0.0))
        .aspect_ratio(aspect_ratio)
        .image_width(image_width)
        .focus_distance(10.0)
        .aperture(0.0)
        .vertical_field_of_view(20.0)
        .open_close_time(0.0, 1.0)
        .build();

    // generate a world with spheres in random locations
    let world = generate_random_spheres();

    (camera, world)
}

/// performs the actual generation of the spheres in the scene
fn generate_random_spheres() -> HittableList {
    let mut rng = rand::thread_rng();

    let mut world = HittableList::new();

    // a big, checkered sphere that will act at the ground
    let ground_sphere = build_checker_sphere(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Color::new(0.1, 0.2, 0.1),
        Color::new(0.8, 0.8, 0.8),
    );
    world.add(Arc::new(ground_sphere));

    // generate 484 spheres with random materials and colors, all of radius 0.2
    for a in -11..11 {
        for b in -11..11 {
            let center: Point3 = Point3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );

            if (center - Vec3::new(4., 0.2, 0.)).length() > 0.9 {
                // randomly select a material for a sphere
                let prob = rng.gen::<f64>();
                if prob < 0.1 {
                    // create movingSpheres with Lambertian material
                    let albedo: Arc<dyn Texture> =
                        Arc::new(SolidColor::from(Color::random() * Color::random()));
                    let center2 = center + Vec3::new(0., rng.gen::<f64>(), 0.);
                    world.add(Arc::new(MovingSphere::new(
                        center,
                        center2,
                        0.0,
                        1.0,
                        0.2,
                        Arc::new(Lambertian::new(Arc::clone(&albedo))),
                    )));
                    // } else if prob < 0.4 {
                    //     // create a marble textured sphere
                    //     let marble_tex: Arc<dyn Texture> = Arc::new(Noise::new(Perlin::new(), 5.0));
                    //     let center = center + Vec3::new(0., rng.gen::<f64>(), 0.);
                    //     let mat: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::clone(&marble_tex)));
                    //     let sphere = Sphere::new(center, 0.2, mat);
                    //     world.add(Arc::new(sphere));
                } else if prob < 0.7 {
                    // create a solid color, Lambertian sphere
                    let solid_tex: Arc<dyn Texture> =
                        Arc::new(SolidColor::from(Color::random() * Color::random()));
                    let center = center + Vec3::new(0., rng.gen::<f64>(), 0.);
                    let mat: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::clone(&solid_tex)));
                    let sphere = Sphere::new(center, 0.2, mat);
                    world.add(Arc::new(sphere));
                } else if prob < 0.95 {
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
                    world.add(Arc::new(Sphere::new(
                        center,
                        0.2,
                        Arc::new(Metal::new(albedo, fuzz)),
                    )));
                } else {
                    world.add(Arc::new(Sphere::new(
                        center,
                        0.2,
                        Arc::new(Dielectric::new(1.5)),
                    )));
                }
            }
        }
    }

    // add a single, larger glass sphere
    let glass_shere = build_dielectric_sphere(Point3::new(0.0, 1.0, 0.0), 1.0, 1.5);
    world.add(Arc::new(glass_shere));

    // add a sphere with perlin noise
    let perlin_sphere = build_perlin_sphere(Point3::new(-4., 1., 0.), 1.0, 0.9);
    //let tex2: Arc<dyn Texture> = Arc::new(SolidColor::from_rgb(1.0, 0.1, 0.1));
    //let mat2: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::clone(&tex2)));
    //let sphere2: Arc<dyn Hittable> = Arc::new(Sphere::new(Point3::new(-4., 1., 0.), 1.0, Arc::clone(&mat2)));
    world.add(Arc::new(perlin_sphere));

    // add a single metal sphere (tan color)
    let metal_sphere =
        build_metal_sphere(Point3::new(4., 1., 0.), 1.0, Color::new(0.7, 0.6, 0.5), 0.0);
    world.add(Arc::new(metal_sphere));

    world
}
