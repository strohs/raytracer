use crate::common::{Camera, CameraBuilder, Point3, Vec3};
use crate::hittable::{HittableList, Sphere};
use crate::material::{Lambertian, Material};
use crate::texture::{ImageTexture, Texture};
use std::sync::Arc;

/// builds a scene with a single earth textured sphere
pub fn build_earth_scene(
    image_width: u32,
    aspect_ratio: f64,
    file_path: &str,
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
        .vertical_field_of_view(30.0)
        .open_close_time(0.0, 1.0)
        .build();

    // build a image mapped sphere
    let earth_tex: Arc<dyn Texture> = Arc::new(ImageTexture::from(file_path));
    let lamb: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::clone(&earth_tex)));
    let sphere = Sphere::new(Point3::new(0., 0., 0.), 2., Arc::clone(&lamb));

    let mut world = HittableList::new();
    world.add(Arc::new(sphere));

    (camera, world)
}
