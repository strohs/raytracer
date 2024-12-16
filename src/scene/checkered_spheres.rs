use crate::common::{Camera, CameraBuilder, Color, Point3, Vec3};
use crate::hittable::builder::build_checker_sphere;
use crate::hittable::HittableList;
use std::sync::Arc;

/// builds a scene with two checkered spheres on top of each other
pub fn build_two_checkered_spheres(image_width: u32, aspect_ratio: f64) -> (Camera, HittableList) {
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

    // generate two checkered spheres
    let sphere1 = build_checker_sphere(
        Point3::new(0., -10., 0.),
        10.,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    );

    let sphere2 = build_checker_sphere(
        Point3::new(0., 10., 0.),
        10.,
        Color::new(0.2, 0.2, 0.2),
        Color::new(0.8, 0.8, 0.8),
    );

    let mut world = HittableList::new();
    world.add(Arc::new(sphere1));
    world.add(Arc::new(sphere2));

    (camera, world)
}
