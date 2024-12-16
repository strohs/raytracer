use crate::common::{Camera, CameraBuilder, Color, Point3, Vec3};
use crate::hittable::primitive::builder::build_solid_moving_sphere;
use crate::hittable::{
    build_constant_medium, build_dielectric_sphere, build_earth_sphere, build_metal_sphere,
    build_perlin_sphere, build_solid_lambertian, build_solid_sphere, build_xz_diff_light, BoxInst,
    BvhNode, Hittable, HittableList, RotateY, Translate,
};
use crate::material::Material;
use rand::Rng;
use std::sync::Arc;

/// Returns the camera and HittableList for the final scene from "Raytracing the Next Week".
pub fn build_final_scene(image_width: u32, aspect_ratio: f64) -> (Camera, HittableList) {
    // build the camera
    let camera = CameraBuilder::new()
        .look_from(Point3::new(178.0, 278.0, -800.0))
        .look_at(Point3::new(278.0, 278.0, 0.0))
        .up_direction(Vec3::new(0.0, 1.0, 0.0))
        .aspect_ratio(aspect_ratio)
        .image_width(image_width)
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
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1: f64 = rng.gen_range(1.0..101.0);
            let z1 = z0 + w;

            let box_inst = BoxInst::from(
                Point3::new(x0, y0, z0),
                Point3::new(x1, y1, z1),
                Arc::clone(&ground_mat),
            );
            boxes1.add(Arc::new(box_inst));
        }
    }

    // objects will hold all the hittables in this scene
    let mut objects = HittableList::new();

    // add the ground boxes into a BVH and then add that to the list of objects
    objects.add(Arc::new(BvhNode::from(&mut boxes1, 0., 1.)));

    // build a light source
    let light = build_xz_diff_light(Color::new(7., 7., 7.), 123., 423., 147., 412., 554.);
    objects.add(Arc::new(light));

    // build a moving sphere
    let center_start = Point3::new(400., 400., 200.);
    let mov_sphere = build_solid_moving_sphere(
        Color::new(0.7, 0.3, 0.1),
        center_start,
        center_start + Vec3::new(30., 0., 0.),
        0.0,
        1.0,
        50.0,
    );
    objects.add(Arc::new(mov_sphere));

    // build a glass sphere
    let glass_sphere = build_dielectric_sphere(Point3::new(260., 150., 45.), 50., 1.5);
    objects.add(Arc::new(glass_sphere));

    // build a metal sphere
    let metal_sphere = build_metal_sphere(
        Point3::new(0., 150., 145.),
        50.0,
        Color::new(0.8, 0.8, 0.9),
        10.,
    );
    objects.add(Arc::new(metal_sphere));

    // build a blueish, glass sphere, and make it foggy
    let sphere_boundary: Arc<dyn Hittable> = Arc::new(build_dielectric_sphere(
        Point3::new(360., 150., 145.),
        70.,
        1.5,
    ));
    objects.add(Arc::clone(&sphere_boundary));
    let fog_volume = build_constant_medium(sphere_boundary, 0.2, Color::new(0.2, 0.4, 0.9));
    objects.add(Arc::new(fog_volume));

    // build a spherical mist volume throughout the whole scene
    let sphere_boundary: Arc<dyn Hittable> =
        Arc::new(build_dielectric_sphere(Point3::new(0., 0., 0.), 5000., 1.5));
    let mist_volume = build_constant_medium(sphere_boundary, 0.0001, Color::new(1., 1., 1.));
    objects.add(Arc::new(mist_volume));

    // build a image mapped sphere with a earth texture
    let earth = build_earth_sphere(Point3::new(400., 200., 400.), 100.);
    objects.add(Arc::new(earth));

    // build a sphere with perlin noise texture
    let perlin_sphere = build_perlin_sphere(Point3::new(220., 280., 300.), 80.0, 0.1);
    objects.add(Arc::new(perlin_sphere));

    // build a box composed of ~1000 smaller spheres
    let ns = 1000; // number of internal spheres
    let mut box_of_sphere = HittableList::new();
    for _ in 0..ns {
        let sphere: Arc<dyn Hittable> = Arc::new(build_solid_sphere(
            Point3::random_range(0.0, 165.0),
            10.0,
            Color::new(0.73, 0.73, 0.73),
        ));
        box_of_sphere.add(sphere);
    }

    // add the box of spheres to a BVH and then rotate and translate the entire box of spheres
    let sphere_box = BvhNode::from(&mut box_of_sphere, 0.0, 1.0);
    let rotated_spheres: Arc<dyn Hittable> = Arc::new(RotateY::from(Arc::new(sphere_box), 15.0));
    let translated_spheres: Arc<dyn Hittable> = Arc::new(Translate::from(
        Arc::clone(&rotated_spheres),
        Vec3::new(-100., 270., 395.),
    ));
    objects.add(translated_spheres);

    (camera, objects)
}
