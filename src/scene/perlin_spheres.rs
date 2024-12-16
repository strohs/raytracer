use crate::common::{Camera, CameraBuilder, Color, Point3, Vec3};
use crate::hittable::builder::build_perlin_sphere;
use crate::hittable::{build_xy_diff_light, build_xz_diff_light, HittableList, Sphere};
use crate::material::{Lambertian, Material};
use crate::texture::{NoiseTexture, Texture};
use std::sync::Arc;

/// builds a scene with two perlin spheres on top of each other
pub fn build_perlin_spheres(image_width: u32, aspect_ratio: f64) -> (Camera, HittableList) {
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
    let perlin_tex: Arc<dyn Texture> = Arc::new(NoiseTexture::new(0.8));
    let lamb: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::clone(&perlin_tex)));
    let sphere1 = Sphere::new(Point3::new(0., -1000., 0.), 1000., Arc::clone(&lamb));
    let sphere2 = Sphere::new(Point3::new(0., 2., 0.), 2., Arc::clone(&lamb));

    let mut world = HittableList::new();
    world.add(Arc::new(sphere1));
    world.add(Arc::new(sphere2));

    (camera, world)
}

/// builds a scene with two perlin spheres, and a xy_rectangle light source
pub fn build_two_perlin_spheres_with_light_source(
    image_width: u32,
    aspect_ratio: f64,
) -> (Camera, HittableList) {
    // build the camera
    let camera = CameraBuilder::new()
        .look_from(Point3::new(13.0, 2.0, 3.0))
        .look_at(Point3::new(0.0, 0.0, 0.0))
        .up_direction(Vec3::new(0.0, 1.0, 0.0))
        .aspect_ratio(aspect_ratio)
        .image_width(image_width)
        .focus_distance(10.0)
        .aperture(0.0)
        .vertical_field_of_view(60.0)
        .open_close_time(0.0, 1.0)
        .build();

    // generate two spheres with a perlin noise texture
    let sphere1 = build_perlin_sphere(Point3::new(0., -1000., 0.), 1000., 0.1);
    let sphere2 = build_perlin_sphere(Point3::new(0., 2., 0.), 2., 0.1);

    // build the rectangle light source, colors are brighter than 1,1,1 so that it's bright enough to light things
    let xy_rect = build_xy_diff_light(Color::new(4., 4., 4.), 3., 4., 1., 3., -2.);
    let xz_rect = build_xz_diff_light(Color::new(4., 4., 4.), -2., 2., -2., 2., 6.);

    let mut world = HittableList::new();
    world.add(Arc::new(sphere1));
    world.add(Arc::new(sphere2));
    world.add(Arc::new(xy_rect));
    world.add(Arc::new(xz_rect));

    (camera, world)
}
