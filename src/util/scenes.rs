use std::sync::Arc;
use rand::Rng;
use crate::common::{Color, Point3, Vec3, Camera};
use crate::hittable::{HittableList, Sphere, Hittable, MovingSphere, XYRect, YZRect, XZRect, FlipFace, BoxInst};
use crate::material::{Lambertian, Material, Metal, Dielectric, DiffuseLight};
use crate::texture::{Texture, SolidColor, CheckerTexture, Perlin, NoiseTexture, ImageTexture};

/// builds a "default" random sphere scene, containing 484 small spheres randomly positioned around
/// 3 bigger spheres. These are then positioned on top of an enormous sphere with a checkerboard
/// texture, which acts as the ground plane
pub fn build_random_sphere_scene(image_width: u32, aspect_ratio: f64)
    -> (Camera, HittableList, u32, u32)
{
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    // camera settings
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;
    let vfov = 20.0;
    let camera = Camera::new(
        look_from, look_at,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
    0.0, 1.0);

    // generate a world with spheres in random locations
    let world = generate_random_spheres();

    (camera, world, image_width, image_height)
}

/// generates a random "world" containing 484 spheres of various colors and materials on top of
/// a gigantic checkerboard sphere
fn generate_random_spheres() -> HittableList {
    let mut rng = rand::thread_rng();

    let mut world = HittableList::new();

    // a big, Lambertian grey, sphere that will act at the ground
    let checker_tex: Arc<dyn Texture> = Arc::new(CheckerTexture::from(
        Arc::new(SolidColor::from_rgb(0.2, 0.3, 0.1)),
        Arc::new(SolidColor::from_rgb(0.9, 0.9, 0.9))));
    let ground_material: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::clone(&checker_tex)));
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
                let prob = rng.gen::<f64>();
                if prob < 0.1 {
                        // create movingSpheres with Lambertian material
                        let albedo: Arc<dyn Texture> = Arc::new(SolidColor::from(Color::random() * Color::random()));
                        let center2 = center + Vec3::new(0., rng.gen::<f64>(), 0.);
                        world.add(
                            Arc::new(
                                MovingSphere::new(
                                    center, center2,
                                    0.0, 1.0,
                                    0.2,
                                    Arc::new(Lambertian::new(Arc::clone(&albedo))))));
                // } else if prob < 0.4 {
                //     // create a marble textured sphere
                //     let marble_tex: Arc<dyn Texture> = Arc::new(Noise::new(Perlin::new(), 5.0));
                //     let center = center + Vec3::new(0., rng.gen::<f64>(), 0.);
                //     let mat: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::clone(&marble_tex)));
                //     let sphere = Sphere::new(center, 0.2, mat);
                //     world.add(Arc::new(sphere));
                } else if prob < 0.7 {
                    // create a solid color, Lambertian sphere
                    let solid_tex: Arc<dyn Texture> = Arc::new(SolidColor::from(Color::random() * Color::random()));
                    let center = center + Vec3::new(0., rng.gen::<f64>(), 0.);
                    let mat: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::clone(&solid_tex)));
                    let sphere = Sphere::new(center, 0.2, mat);
                    world.add(Arc::new(sphere));
                } else if prob < 0.95 {
                        let albedo = Color::random_range(0.5, 1.0);
                        let fuzz = rng.gen_range(0.0, 0.5);
                        world.add(Arc::new(Sphere::new(center, 0.2, Arc::new(Metal::new(albedo, fuzz)))));
                } else {
                        world.add(Arc::new(Sphere::new(center, 0.2, Arc::new(Dielectric::new(1.5)))));
                }
            }
        }
    }

    // add a single, larger glass sphere
    let mat1: Arc<dyn Material> = Arc::new(Dielectric::new(1.5));
    let sphere1: Arc<dyn Hittable> = Arc::new(Sphere::new(Point3::new(0., 1., 0.), 1.0, Arc::clone(&mat1)));
    world.add(sphere1);

    // add a single, lambertian reddish colored sphere
    let tex2: Arc<dyn Texture> = Arc::new(SolidColor::from_rgb(1.0, 0.1, 0.1));
    let mat2: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::clone(&tex2)));
    let sphere2: Arc<dyn Hittable> = Arc::new(Sphere::new(Point3::new(-4., 1., 0.), 1.0, Arc::clone(&mat2)));
    world.add(sphere2);

    // add a single metal sphere (tan color)
    let mat3: Arc<dyn Material> = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    let sphere3: Arc<dyn Hittable> = Arc::new(Sphere::new(Point3::new(4., 1., 0.), 1.0, Arc::clone(&mat3)));
    world.add(sphere3);

    world
}

/// builds a scene with two checkered spheres on top of each other
pub fn build_two_checkered_spheres(image_width: u32, aspect_ratio: f64)
    -> (Camera, HittableList, u32, u32)
{
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    // camera settings
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;
    let vfov = 20.0;
    let camera = Camera::new(
        look_from, look_at,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0, 1.0);

    // generate two checkered spheres
    let check_tex: Arc<dyn Texture> = Arc::new(
        CheckerTexture::from(
            Arc::new(SolidColor::from_rgb(0.2, 0.3, 0.1)),
            Arc::new(SolidColor::from_rgb(0.9, 0.9, 0.9))));
    let lamb: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::clone(&check_tex)));
    let sphere1 = Sphere::new(Point3::new(0., -10., 0.), 10., Arc::clone(&lamb));
    let sphere2 = Sphere::new(Point3::new(0., 10., 0.), 10., Arc::clone(&lamb));

    let mut world = HittableList::new();
    world.add(Arc::new(sphere1));
    world.add(Arc::new(sphere2));

    (camera, world, image_width, image_height)
}

/// builds a scene with two checkered spheres on top of each other
pub fn build_two_perlin_spheres(image_width: u32, aspect_ratio: f64)
                                   -> (Camera, HittableList, u32, u32)
{
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    // camera settings
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;
    let vfov = 20.0;
    let camera = Camera::new(
        look_from, look_at,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0, 1.0);

    // generate two checkered spheres
    let perlin_tex: Arc<dyn Texture> = Arc::new(NoiseTexture::new(Perlin::new(), 2.));
    let lamb: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::clone(&perlin_tex)));
    let sphere1 = Sphere::new(Point3::new(0., -1000., 0.), 1000., Arc::clone(&lamb));
    let sphere2 = Sphere::new(Point3::new(0., 2., 0.), 2., Arc::clone(&lamb));

    let mut world = HittableList::new();
    world.add(Arc::new(sphere1));
    world.add(Arc::new(sphere2));

    (camera, world, image_width, image_height)
}

/// builds a scene with two checkered spheres on top of each other
///
pub fn build_earth_sphere(image_width: u32, aspect_ratio: f64, file_path: &str)
                                -> (Camera, HittableList, u32, u32)
{
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    // camera settings
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;
    let vfov = 30.0;
    let camera = Camera::new(
        look_from, look_at,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0, 1.0);

    // build a image mapped sphere
    let earth_tex: Arc<dyn Texture> = Arc::new(ImageTexture::from(file_path));
    let lamb: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::clone(&earth_tex)));
    let sphere = Sphere::new(Point3::new(0., 0., 0.), 2., Arc::clone(&lamb));

    let mut world = HittableList::new();
    world.add(Arc::new(sphere));

    (camera, world, image_width, image_height)
}

/// builds a scene with two perlin spheres, and a xy_rectangle light source
pub fn build_two_perlin_spheres_with_light_rect(image_width: u32, aspect_ratio: f64)
                                -> (Camera, HittableList, u32, u32)
{
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    // camera settings
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;
    let vfov = 60.0;
    let camera = Camera::new(
        look_from, look_at,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0, 1.0);

    // generate two checkered spheres
    let perlin_tex: Arc<dyn Texture> = Arc::new(NoiseTexture::new(Perlin::new(), 2.));
    let lamb: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::clone(&perlin_tex)));
    let sphere1 = Sphere::new(Point3::new(0., -1000., 0.), 1000., Arc::clone(&lamb));
    let sphere2 = Sphere::new(Point3::new(0., 2., 0.), 2., Arc::clone(&lamb));

    // build the rectangle light source, it's brighter than 1,1,1 so that it's bright enough to light things
    let solid_tex: Arc<dyn Texture> = Arc::new(SolidColor::from_rgb(4.0, 4.0, 4.0));
    let diff_light: Arc<dyn Material> = Arc::new(DiffuseLight::from( Arc::clone(&solid_tex)));
    let sphere3 = Sphere::new(Point3::new(0.0, 7.0, 0.0), 2.0, Arc::clone(&diff_light));
    let xy_rect = XYRect::from(3., 4., 1., 3., -2., Arc::clone(&diff_light));

    let mut world = HittableList::new();
    world.add(Arc::new(sphere1));
    world.add(Arc::new(sphere2));
    world.add(Arc::new(sphere3));
    world.add(Arc::new(xy_rect));

    (camera, world, image_width, image_height)
}



/// builds a scene with an empty cornell box
pub fn build_empty_cornell_box(image_width: u32, aspect_ratio: f64)
                                                -> (Camera, HittableList, u32, u32)
{
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    // camera settings
    let look_from = Point3::new(278.0, 278.0, -800.0);
    let look_at = Point3::new(278.0, 278.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;
    let vfov = 40.0;
    let camera = Camera::new(
        look_from, look_at,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0, 1.0);

    // build solid color materials
    let red: Arc<dyn Texture> = Arc::new(SolidColor::from_rgb(0.65, 0.05, 0.05));
    let white: Arc<dyn Texture> = Arc::new(SolidColor::from_rgb(0.73, 0.73, 0.73));
    let green: Arc<dyn Texture> = Arc::new(SolidColor::from_rgb(0.12,  0.45, 0.15));
    let bright_light: Arc<dyn Texture> = Arc::new(SolidColor::from_rgb(15.,  15., 15.));
    let red_mat: Arc<dyn Material> = Arc::new(Lambertian::new( Arc::clone(&red)));
    let white_mat: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::clone(&white)));
    let green_mat: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::clone(&green)));
    let light_mat: Arc<dyn Material> = Arc::new(DiffuseLight::from(Arc::clone(&bright_light)));

    // build the walls of the room
    let green_wall = Arc::new(FlipFace::from(Arc::new(YZRect::from(0., 555., 0., 555., 555., Arc::clone(&green_mat)))));
    let red_wall = Arc::new(YZRect::from(0., 555., 0., 555., 0., Arc::clone(&red_mat)));
    let light = Arc::new(XZRect::from(213., 343., 227., 332., 554., Arc::clone(&light_mat)));
    let floor = Arc::new(FlipFace::from(Arc::new(XZRect::from(0., 555., 0., 555., 555., Arc::clone(&white_mat)))));
    let ceiling = Arc::new(XZRect::from(0., 555., 0., 555., 0., Arc::clone(&white_mat)));
    let back_wall = Arc::new(FlipFace::from(Arc::new(XYRect::from(0., 555., 0., 555., 555., Arc::clone(&white_mat)))));

    // build two boxes
    let rect_box = Arc::new(BoxInst::from(
        Point3::new(130., 0., 65.),
        Point3::new(295., 165., 230.),
        Arc::clone(&white_mat)));
    let square_box = Arc::new(BoxInst::from(
        Point3::new(265., 0., 295.),
        Point3::new(430., 330., 460.),
        Arc::clone(&white_mat)));

    let mut world = HittableList::new();
    world.add(green_wall);
    world.add(red_wall);
    world.add(light);
    world.add(floor);
    world.add(ceiling);
    world.add(back_wall);
    world.add(rect_box);
    world.add(square_box);

    (camera, world, image_width, image_height)
}