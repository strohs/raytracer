use std::sync::Arc;
use rand::Rng;
use crate::common::{Color, Point3, Vec3, Camera, CameraBuilder};
use crate::hittable::{HittableList, Sphere, Hittable, MovingSphere, XYRect, YZRect, XZRect, FlipFace, BoxInst, RotateY, Translate, ConstantMedium, BvhNode};
use crate::material::{Lambertian, Material, Metal, Dielectric, DiffuseLight};
use crate::texture::{Texture, SolidColor, CheckerTexture, Perlin, NoiseTexture, ImageTexture};

/// builds a "default" random sphere scene, containing 484 small spheres randomly positioned around
/// 3 bigger spheres. These are then positioned on top of an enormous sphere with a checkerboard
/// texture, which acts as the ground plane
pub fn build_random_sphere_scene(image_width: u32, aspect_ratio: f64)
    -> (Camera, HittableList, u32, u32)
{
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    // build the camera
    let camera = CameraBuilder::new()
        .look_from(Point3::new(13.0, 2.0, 3.0))
        .look_at(Point3::new(0.0, 0.0, 0.0))
        .up_direction(Vec3::new(0.0, 1.0, 0.0))
        .aspect_ratio(16.0 / 9.0)
        .focus_distance(10.0)
        .aperture(0.0)
        .vertical_field_of_view(20.0)
        .open_close_time(0.0, 1.0)
        .build();

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

    // build the camera
    let camera = CameraBuilder::new()
        .look_from(Point3::new(13.0, 2.0, 3.0))
        .look_at(Point3::new(0.0, 0.0, 0.0))
        .up_direction(Vec3::new(0.0, 1.0, 0.0))
        .aspect_ratio(16.0 / 9.0)
        .focus_distance(10.0)
        .aperture(0.0)
        .vertical_field_of_view(20.0)
        .open_close_time(0.0, 1.0)
        .build();

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

    // build the camera
    let camera = CameraBuilder::new()
        .look_from(Point3::new(13.0, 2.0, 3.0))
        .look_at(Point3::new(0.0, 0.0, 0.0))
        .up_direction(Vec3::new(0.0, 1.0, 0.0))
        .aspect_ratio(16.0 / 9.0)
        .focus_distance(10.0)
        .aperture(0.0)
        .vertical_field_of_view(20.0)
        .open_close_time(0.0, 1.0)
        .build();

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
pub fn build_earth_scene(image_width: u32, aspect_ratio: f64, file_path: &str)
                                -> (Camera, HittableList, u32, u32)
{
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    // build the camera
    let camera = CameraBuilder::new()
        .look_from(Point3::new(13.0, 2.0, 3.0))
        .look_at(Point3::new(0.0, 0.0, 0.0))
        .up_direction(Vec3::new(0.0, 1.0, 0.0))
        .aspect_ratio(16.0 / 9.0)
        .focus_distance(10.0)
        .aperture(0.0)
        .vertical_field_of_view(30.0)
        .open_close_time(0.0, 1.0)
        .build();

    // build a image mapped sphere
    let earth_tex: Arc<dyn Texture> = Arc::new(ImageTexture::from(file_path));
    let lamb: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::clone(&earth_tex)));
    let sphere = Sphere::new(Point3::new(0., 0., 0.), 2., Arc::clone(&lamb));

    let mut world = HittableList::new();
    world.add(Arc::new(sphere));

    (camera, world, image_width, image_height)
}

/// builds a scene with two perlin spheres, and a xy_rectangle light source
pub fn build_two_perlin_spheres_with_light_source(image_width: u32, aspect_ratio: f64)
                                                  -> (Camera, HittableList, u32, u32)
{
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    // build the camera
    let camera = CameraBuilder::new()
        .look_from(Point3::new(13.0, 2.0, 3.0))
        .look_at(Point3::new(0.0, 0.0, 0.0))
        .up_direction(Vec3::new(0.0, 1.0, 0.0))
        .aspect_ratio(16.0 / 9.0)
        .focus_distance(10.0)
        .aperture(0.0)
        .vertical_field_of_view(60.0)
        .open_close_time(0.0, 1.0)
        .build();

    // generate two spheres with a perlin noise texture
    let sphere1 = build_perlin_sphere(
        Point3::new(0.,-1000., 0.),
        1000.,
        0.1);
    let sphere2 = build_perlin_sphere(
        Point3::new(0.,2., 0.),
        2.,
        0.1);

    // build the rectangle light source, colors are brighter than 1,1,1 so that it's bright enough to light things
    let xy_rect = build_xy_diff_light(
        Color::new(4., 4., 4.),
        3., 4., 1., 3., -2.);
    let xz_rect = build_xz_diff_light(
        Color::new(4., 4., 4.),
        -2., 2., -2., 2., 6.);

    let mut world = HittableList::new();
    world.add(Arc::new(sphere1));
    world.add(Arc::new(sphere2));
    world.add(Arc::new(xy_rect));
    world.add(Arc::new(xz_rect));

    (camera, world, image_width, image_height)
}



/// builds a cornell box containing two boxes
pub fn build_cornell_box_with_two_boxes(image_width: u32, aspect_ratio: f64)
                                        -> (Camera, HittableList, u32, u32)
{
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    // build the camera
    let camera = CameraBuilder::new()
        .look_from(Point3::new(278.0, 278.0, -800.0))
        .look_at(Point3::new(278.0, 278.0, 0.0))
        .up_direction(Vec3::new(0.0, 1.0, 0.0))
        .focus_distance(10.0)
        .aspect_ratio(16.0 / 9.0)
        .aperture(0.0)
        .vertical_field_of_view(40.0)
        .open_close_time(0.0, 1.0)
        .build();

    // build solid color materials
    let red: Arc<dyn Texture> = Arc::new(SolidColor::from_rgb(0.65, 0.05, 0.05));
    let white: Arc<dyn Texture> = Arc::new(SolidColor::from_rgb(0.73, 0.73, 0.73));
    let green: Arc<dyn Texture> = Arc::new(SolidColor::from_rgb(0.12,  0.45, 0.15));
    let red_mat: Arc<dyn Material> = Arc::new(Lambertian::new( Arc::clone(&red)));
    let white_mat: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::clone(&white)));
    let green_mat: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::clone(&green)));

    // build the walls of the room
    let green_wall = Arc::new(
        FlipFace::from(Arc::new(YZRect::from(0., 555., 0., 555., 555., Arc::clone(&green_mat)))));
    let red_wall = Arc::new(YZRect::from(0., 555., 0., 555., 0., Arc::clone(&red_mat)));
    let floor = Arc::new(FlipFace::from(Arc::new(XZRect::from(0., 555., 0., 555., 555., Arc::clone(&white_mat)))));
    let ceiling = Arc::new(XZRect::from(0., 555., 0., 555., 0., Arc::clone(&white_mat)));
    let back_wall = Arc::new(FlipFace::from(Arc::new(XYRect::from(0., 555., 0., 555., 555., Arc::clone(&white_mat)))));

    // build the rectangular light at the top
    let light = Arc::new(build_xz_diff_light(
        Color::new(16.,16.,16.),
        183., 373.,
        197., 362.,
        554.));

    // build a rectangular box
    let mut rect_box: Arc<dyn Hittable> = Arc::new(BoxInst::from(
        Point3::new(0., 0., 0.),
        Point3::new(165., 330., 165.),
        Arc::clone(&white_mat)));
    rect_box = Arc::new(RotateY::from(
        Arc::clone(&rect_box),
        15.0));
    rect_box = Arc::new(Translate::from(
        Arc::clone(&rect_box),
        Vec3::new(265., 0., 295.)));

    // build a square box
    let mut square_box: Arc<dyn Hittable> = Arc::new(BoxInst::from(
        Point3::new(0., 0., 0.),
        Point3::new(165., 165., 165.),
        Arc::clone(&white_mat)));
    square_box = Arc::new(RotateY::from(Arc::clone(&square_box), -18.0));
    square_box = Arc::new(Translate::from(
        Arc::clone(&square_box),
        Vec3::new(130., 0., 100.)));

    // build a perlin sphere on top of the square box
    let mut per_sphere: Arc<dyn Hittable> = Arc::new(build_perlin_sphere(
        Point3::new(0.0, 0.0, 0.0),
        60.0,
        0.2));
    per_sphere = Arc::new(Translate::from(
        Arc::clone(&per_sphere),
        Vec3::new(175., 225., 170.)));

    let mut world = HittableList::new();
    world.add(green_wall);
    world.add(red_wall);
    world.add(light);
    world.add(floor);
    world.add(ceiling);
    world.add(back_wall);
    world.add(rect_box);
    world.add(square_box);
    world.add(per_sphere);

    (camera, world, image_width, image_height)
}



/// builds a cornell box, containing two boxes, one made of smoke and the other of fog.
pub fn build_cornell_smoke_box(image_width: u32, aspect_ratio: f64)
                               -> (Camera, HittableList, u32, u32)
{
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    // build the camera
    let camera = CameraBuilder::new()
        .look_from(Point3::new(278.0, 278.0, -800.0))
        .look_at(Point3::new(278.0, 278.0, 0.0))
        .up_direction(Vec3::new(0.0, 1.0, 0.0))
        .aspect_ratio(16.0 / 9.0)
        .focus_distance(10.0)
        .aperture(0.0)
        .vertical_field_of_view(40.0)
        .open_close_time(0.0, 1.0)
        .build();

    // build solid color materials
    let red: Arc<dyn Texture> = Arc::new(SolidColor::from_rgb(0.65, 0.05, 0.05));
    let white: Arc<dyn Texture> = Arc::new(SolidColor::from_rgb(0.73, 0.73, 0.73));
    let green: Arc<dyn Texture> = Arc::new(SolidColor::from_rgb(0.12,  0.45, 0.15));
    let all_black: Arc<dyn Texture> = Arc::new(SolidColor::from_rgb(0., 0., 0.));
    let all_white: Arc<dyn Texture> = Arc::new(SolidColor::from_rgb(1., 1., 1.));
    let light_tex: Arc<dyn Texture> = Arc::new(SolidColor::from_rgb(7., 7., 7.));
    let red_mat: Arc<dyn Material> = Arc::new(Lambertian::new( Arc::clone(&red)));
    let white_mat: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::clone(&white)));
    let green_mat: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::clone(&green)));
    let light_mat: Arc<dyn Material> = Arc::new(DiffuseLight::from(Arc::clone(&light_tex)));

    // build the walls of the room
    let green_wall = Arc::new(FlipFace::from(Arc::new(YZRect::from(0., 555., 0., 555., 555., Arc::clone(&green_mat)))));
    let red_wall = Arc::new(YZRect::from(0., 555., 0., 555., 0., Arc::clone(&red_mat)));
    let light = Arc::new(XZRect::from(113., 443., 127., 432., 554., Arc::clone(&light_mat)));
    let floor = Arc::new(FlipFace::from(Arc::new(XZRect::from(0., 555., 0., 555., 555., Arc::clone(&white_mat)))));
    let ceiling = Arc::new(XZRect::from(0., 555., 0., 555., 0., Arc::clone(&white_mat)));
    let back_wall = Arc::new(FlipFace::from(Arc::new(XYRect::from(0., 555., 0., 555., 555., Arc::clone(&white_mat)))));

    // build a rectangular box
    let mut rect_box: Arc<dyn Hittable> = Arc::new(BoxInst::from(
        Point3::new(0., 0., 0.),
        Point3::new(165., 330., 165.),
        Arc::clone(&white_mat)));
    rect_box = Arc::new(RotateY::from(
        Arc::clone(&rect_box),
        15.0));
    rect_box = Arc::new(Translate::from(
        Arc::clone(&rect_box),
        Vec3::new(265., 0., 295.)));

    // build a square box
    let mut square_box: Arc<dyn Hittable> = Arc::new(BoxInst::from(
        Point3::new(0., 0., 0.),
        Point3::new(165., 165., 165.),
        Arc::clone(&white_mat)));
    square_box = Arc::new(RotateY::from(Arc::clone(&square_box), -18.0));
    square_box = Arc::new(Translate::from(
        Arc::clone(&square_box),
        Vec3::new(130., 0., 65.)));

    let fog_box = Arc::new(ConstantMedium::from(Arc::clone(&rect_box), 0.01, all_black));
    let smoke_box = Arc::new(ConstantMedium::from(Arc::clone(&square_box), 0.01, all_white));

    let mut world = HittableList::new();
    world.add(green_wall);
    world.add(red_wall);
    world.add(light);
    world.add(floor);
    world.add(ceiling);
    world.add(back_wall);
    world.add(fog_box);
    world.add(smoke_box);

    (camera, world, image_width, image_height)
}


/// Returns the camera and HittableList for the final scene from "Raytracing the Next Week".
pub fn build_final_scene(image_width: u32, aspect_ratio: f64)
                               -> (Camera, HittableList, u32, u32)
{
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    // build the camera
    let camera = CameraBuilder::new()
        .look_from(Point3::new(178.0, 278.0, -800.0))
        .look_at(Point3::new(278.0, 278.0, 0.0))
        .up_direction(Vec3::new(0.0, 1.0, 0.0))
        .aspect_ratio(16.0 / 9.0)
        .focus_distance(10.0)
        .aperture(0.0)
        .vertical_field_of_view(40.0)
        .open_close_time(0.0, 1.0)
        .build();

    // build a ground layer consisting of ~400 boxes of various widths and heights
    let mut boxes1 = HittableList::new();
    let ground_mat: Arc<dyn Material> = Arc::new(build_solid_lambertian(0.48, 0.83, 0.53));
    let boxes_per_side = 20;
    let mut rng = rand::thread_rng();
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w as f64;
            let z0 = -1000.0 + j as f64 * w as f64;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1: f64 = rng.gen_range(1.0, 101.0);
            let z1 = z0 + w;

            let box_inst = BoxInst::from(
                Point3::new(x0,y0, z0),
                Point3::new(x1, y1, z1),
                Arc::clone(&ground_mat));
            boxes1.add(Arc::new(box_inst));
        }
    }

    // objects will hold all the hittables in this scene
    let mut objects = HittableList::new();

    // add the ground boxes into a BVH and then add that to the list of objects
    objects.add(Arc::new(BvhNode::from(&mut boxes1, 0., 1.)));

    // build a light source
    let light = build_xz_diff_light(
        Color::new(7.,7.,7.),
        123.,423.,
        147.,412.,554.);
    objects.add(Arc::new(light));

    // build a moving sphere
    let center_start = Point3::new(400., 400., 200.);
    let mov_sphere = build_solid_moving_sphere(
        Color::new(0.7, 0.3, 0.1),
        center_start,
        center_start + Vec3::new(30., 0., 0.),
        0.0, 1.0,
        50.0);
    objects.add(Arc::new(mov_sphere));

    // build a glass sphere
    let glass_sphere = build_dielectric_sphere(
        Point3::new(260., 150., 45.),
        50.,
        1.5);
    objects.add(Arc::new(glass_sphere));

    // build a metal sphere
    let metal_sphere = build_metal_sphere(
        Point3::new(0.,150.,145.),
        50.0,
        Color::new(0.8,0.8,0.9),
        10.);
    objects.add(Arc::new(metal_sphere));

    // build a blueish, glass sphere, and make it foggy
    let sphere_boundary: Arc<dyn Hittable> = Arc::new(build_dielectric_sphere(
        Point3::new(360., 150., 145.),
        70.,
        1.5));
    objects.add(Arc::clone(&sphere_boundary));
    let fog_volume = build_constant_medium(
        sphere_boundary, 0.2, Color::new(0.2, 0.4, 0.9));
    objects.add(Arc::new(fog_volume));

    // build a spherical mist volume throughout the whole scene
    let sphere_boundary: Arc<dyn Hittable> = Arc::new(build_dielectric_sphere(
        Point3::new(0., 0., 0.),
        5000.,
        1.5));
    let mist_volume = build_constant_medium(
        sphere_boundary, 0.0001, Color::new(1., 1., 1.));
    objects.add(Arc::new(mist_volume));

    // build a image mapped sphere with a earth texture
    let earth = build_earth_sphere(Point3::new(400., 200., 400.), 100.);
    objects.add(Arc::new(earth));

    // build a sphere with perlin noise texture
    let perlin_sphere = build_perlin_sphere(
        Point3::new(220., 280., 300.),
        80.0,
        0.5);
    objects.add(Arc::new(perlin_sphere));

    // build a box composed of ~1000 smaller spheres
    let ns = 1000; // number of internal spheres
    let mut box_of_sphere = HittableList::new();
    for _ in 0..ns {
        let sphere: Arc<dyn Hittable> = Arc::new(
            build_solid_sphere(
                Point3::random_range(0.0, 165.0), 10.0,
                Color::new(0.73, 0.73, 0.73)));
        box_of_sphere.add(sphere);
    }

    // add the box of spheres to a BVH and then rotate and translate the entire box of spheres
    let sphere_box = BvhNode::from(&mut box_of_sphere, 0.0, 1.0);
    let rotated_spheres: Arc<dyn Hittable> = Arc::new(
        RotateY::from(Arc::new(sphere_box), 15.0));
    let translated_spheres: Arc<dyn Hittable> = Arc::new(
        Translate::from(
            Arc::clone(&rotated_spheres),
            Vec3::new(-100., 270., 395.)));
    objects.add(translated_spheres);


    (camera, objects, image_width, image_height)
}







/// Returns a lambertian material with a solid color texture consisting of the specified `r,g,b`
fn build_solid_lambertian(r: f64, g: f64, b: f64) -> impl Material {
    let solid_color= SolidColor::from_rgb(r,g,b);
    Lambertian::new(Arc::new(solid_color))
}

/// Returns a XZ-Rectangle diffuse light material with the specified Color and coordinates
fn build_xz_diff_light(light_color: Color, x0: f64, x1: f64, z0: f64, z1: f64, k: f64) -> XZRect {
    let light_color = SolidColor::from(light_color);
    let diff_light = DiffuseLight::from(Arc::new(light_color));
    XZRect::from(x0, x1, z0, z1, k, Arc::new(diff_light))
}

/// Returns a XY-Rectangle with a diffuse light material with the specified Color and coordinates
fn build_xy_diff_light(light_color: Color, x0: f64, x1: f64, y0: f64, y1: f64, k: f64) -> XYRect {
    let light_color = SolidColor::from(light_color);
    let diff_light = DiffuseLight::from(Arc::new(light_color));
    XYRect::from(x0, x1, y0, y1, k, Arc::new(diff_light))
}

/// Returns a YZ-Rectangle with a diffuse light material with the specified Color and coordinates
// fn build_yz_diff_light(light_color: Color, y0: f64, y1: f64, z0: f64, z1: f64, k: f64) -> YZRect {
//     let light_color = SolidColor::from(light_color);
//     let diff_light = DiffuseLight::from(Arc::new(light_color));
//     YZRect::from(y0, y1, z0, z1, k, Arc::new(diff_light))
// }

fn build_solid_moving_sphere(color: Color, c1: Point3, c2: Point3, t0: f64, t1: f64, rad: f64) -> MovingSphere {
    let solid_lamb = build_solid_lambertian(color.x(), color.y(), color.z());
    MovingSphere::new(c1, c2, t0, t1, rad, Arc::new(solid_lamb))
}

/// Returns a new sphere with a dielectric material with the specified refractive index `ref_idx`
fn build_dielectric_sphere(center: Point3, rad: f64, ref_idx: f64) -> Sphere {
    let dielectric = Dielectric::new(ref_idx);
    Sphere::new(center, rad, Arc::new(dielectric))
}

/// Returns a new sphere with a metal material with the specified color and fuzziness
fn build_metal_sphere(center: Point3, rad: f64, color: Color, fuzz: f64) -> Sphere {
    let metal = Metal::new(color, fuzz);
    Sphere::new(center, rad, Arc::new(metal))
}

/// Returns a new sphere with a solid lambertian material, with the specified color
fn build_solid_sphere(center: Point3, rad: f64, color: Color) -> Sphere {
    let solid_tex = SolidColor::from(color);
    let mat = Lambertian::new(Arc::new(solid_tex));
    Sphere::new(center, rad, Arc::new(mat))
}

/// Returns a new Constant Medium composed of the specified boundary, density and color
fn build_constant_medium(bound: Arc<dyn Hittable>, density: f64, color: Color) -> ConstantMedium {
    let solid_color = SolidColor::from(color);
    let boundary: Arc<dyn Hittable> = Arc::clone(&bound);
    ConstantMedium::from(boundary, density, Arc::new(solid_color))
}

/// Returns a sphere textured with the 'earthmap.jpg' texture
fn build_earth_sphere(center: Point3, rad: f64) -> Sphere {
    let etex = ImageTexture::from("./earthmap.jpg");
    let emat = Lambertian::new(Arc::new(etex));
    Sphere::new(center, rad, Arc::new(emat))
}

/// Returns a new sphere with a perlin noise texture
fn build_perlin_sphere(center: Point3, rad: f64, noise_scale: f64) -> Sphere {
    let pertex = NoiseTexture::new(Perlin::new(), noise_scale);
    let permat = Lambertian::new(Arc::new(pertex));
    Sphere::new(center, rad, Arc::new(permat))
}