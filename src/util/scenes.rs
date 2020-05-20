use std::sync::Arc;
use crate::common::{Color, Point3, Vec3, Camera};
use crate::hittable::{HittableList, Sphere, Hittable};
use crate::material::{Lambertian, Material, Metal, Dielectric};
use rand::Rng;

/// builds a "default" random sphere scene, containing 484 small spheres randomly positioned around
/// 3 bigger spheres. These are then positioned on top of an enormous sphere, which acts as the
/// ground plane
pub fn build_default_sphere_scene(image_width: u32, aspect_ratio: f64)
    -> (Camera, HittableList, u32, u32)
{
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    // camera settings
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let vfov = 20.0;
    let camera = Camera::new(
        look_from,
        look_at,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus);

    // generate a world with spheres in random locations
    let world = generate_random_spheres();

    (camera, world, image_width, image_height)
}

/// generates a random "world" containing 484 spheres of various colors and materials
fn generate_random_spheres() -> HittableList {
    let mut rng = rand::thread_rng();

    let mut world = HittableList::new();

    // a big, Lambertian grey, sphere that will act at the ground
    let ground_material: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let ground_sphere = Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material);
    world.add(Arc::new(ground_sphere));

    // generate 484 spheres with random materials and colors, all of radius 0.2
    for a in -11..11 {
        for b in -11..11 {
            let center: Point3 = Point3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>());

            if (center - Vec3::new(4., 0.2, 0.)).length() > 0.9 {
                // randomly select a material for a sphere
                let sphere_mat: Arc<dyn Material> = match rng.gen::<f64>() {
                    p if p < 0.8 => {
                        let albedo = Color::random() * Color::random();
                        Arc::new(Lambertian::new(albedo))
                    },
                    p if p < 0.95 => {
                        let albedo = Color::random_range(0.5, 1.0);
                        let fuzz = rng.gen_range(0.0, 0.5);
                        Arc::new(Metal::new(albedo, fuzz))
                    },
                    _ => Arc::new(Dielectric::new(1.5)),
                };
                world.add(Arc::new(Sphere::new(center, 0.2, Arc::clone(&sphere_mat))));
            }
        }
    }

    // add a single, larger glass sphere
    let mat1: Arc<dyn Material> = Arc::new(Dielectric::new(1.5));
    let sphere1: Arc<dyn Hittable> = Arc::new(Sphere::new(Point3::new(0., 1., 0.), 1.0, Arc::clone(&mat1)));
    world.add(sphere1);

    // add a single, lambertian reddish colored sphere
    let mat2: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(1.0, 0.1, 0.1)));
    let sphere2: Arc<dyn Hittable> = Arc::new(Sphere::new(Point3::new(-4., 1., 0.), 1.0, Arc::clone(&mat2)));
    world.add(sphere2);

    // add a single metal sphere (tan color)
    let mat3: Arc<dyn Material> = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    let sphere3: Arc<dyn Hittable> = Arc::new(Sphere::new(Point3::new(4., 1., 0.), 1.0, Arc::clone(&mat3)));
    world.add(sphere3);

    world
}