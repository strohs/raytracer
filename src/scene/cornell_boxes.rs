use crate::common::{Camera, CameraBuilder, Color, Point3, Vec3};
use crate::hittable::{
    build_xz_diff_light, BoxInst, ConstantMedium, FlipFace, Hittable, HittableList, RotateY,
    Translate, XYRect, XZRect, YZRect,
};
use crate::material::{DiffuseLight, Lambertian, Material};
use crate::texture::{SolidColor, Texture};
use std::sync::Arc;

/// builds a cornell box containing two boxes
pub fn build_cornell_box_with_two_boxes(
    image_width: u32,
    aspect_ratio: f64,
) -> (Camera, HittableList) {
    // build the camera
    let camera = CameraBuilder::new()
        .look_from(Point3::new(278.0, 278.0, -800.0))
        .look_at(Point3::new(278.0, 278.0, 0.0))
        .up_direction(Vec3::new(0.0, 1.0, 0.0))
        .focus_distance(10.0)
        .aspect_ratio(aspect_ratio)
        .image_width(image_width)
        .aperture(0.0)
        .vertical_field_of_view(40.0)
        .open_close_time(0.0, 1.0)
        .build();

    // build solid color materials
    let red: Arc<dyn Texture> = Arc::new(SolidColor::from_rgb(0.65, 0.05, 0.05));
    let white: Arc<dyn Texture> = Arc::new(SolidColor::from_rgb(0.73, 0.73, 0.73));
    let green: Arc<dyn Texture> = Arc::new(SolidColor::from_rgb(0.12, 0.45, 0.15));
    let red_mat: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::clone(&red)));
    let white_mat: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::clone(&white)));
    let green_mat: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::clone(&green)));

    // build the walls of the room
    let green_wall = Arc::new(FlipFace::from(Arc::new(YZRect::from(
        0.,
        555.,
        0.,
        555.,
        555.,
        Arc::clone(&green_mat),
    ))));
    let red_wall = Arc::new(YZRect::from(0., 555., 0., 555., 0., Arc::clone(&red_mat)));
    let floor = Arc::new(FlipFace::from(Arc::new(XZRect::from(
        0.,
        555.,
        0.,
        555.,
        555.,
        Arc::clone(&white_mat),
    ))));
    let ceiling = Arc::new(XZRect::from(0., 555., 0., 555., 0., Arc::clone(&white_mat)));
    let back_wall = Arc::new(FlipFace::from(Arc::new(XYRect::from(
        0.,
        555.,
        0.,
        555.,
        555.,
        Arc::clone(&white_mat),
    ))));

    // build the rectangular light at the top
    let light = Arc::new(build_xz_diff_light(
        Color::new(16., 16., 16.),
        183.,
        373.,
        137.,
        302.,
        554.,
    ));

    // build a rectangular box
    let mut rect_box: Arc<dyn Hittable> = Arc::new(BoxInst::from(
        Point3::new(0., 0., 0.),
        Point3::new(165., 330., 165.),
        Arc::clone(&white_mat),
    ));
    rect_box = Arc::new(RotateY::from(Arc::clone(&rect_box), 15.0));
    rect_box = Arc::new(Translate::from(
        Arc::clone(&rect_box),
        Vec3::new(265., 0., 295.),
    ));

    // build a square box
    let mut square_box: Arc<dyn Hittable> = Arc::new(BoxInst::from(
        Point3::new(0., 0., 0.),
        Point3::new(165., 165., 165.),
        Arc::clone(&white_mat),
    ));
    square_box = Arc::new(RotateY::from(Arc::clone(&square_box), -18.0));
    square_box = Arc::new(Translate::from(
        Arc::clone(&square_box),
        Vec3::new(130., 0., 100.),
    ));

    // // build a perlin sphere on top of the square box
    // let mut per_sphere: Arc<dyn Hittable> = Arc::new(build_perlin_sphere(
    //     Point3::new(0.0, 0.0, 0.0),
    //     60.0,
    //     0.2));
    // per_sphere = Arc::new(Translate::from(
    //     Arc::clone(&per_sphere),
    //     Vec3::new(175., 225., 170.)));

    let mut world = HittableList::new();
    world.add(green_wall);
    world.add(red_wall);
    world.add(light);
    world.add(floor);
    world.add(ceiling);
    world.add(back_wall);
    world.add(rect_box);
    world.add(square_box);

    (camera, world)
}

/// builds a cornell box, containing two boxes, one made of smoke and the other of fog.
pub fn build_cornell_smoke_box(image_width: u32, aspect_ratio: f64) -> (Camera, HittableList) {
    // build the camera
    let camera = CameraBuilder::new()
        .look_from(Point3::new(278.0, 278.0, -800.0))
        .look_at(Point3::new(278.0, 278.0, 0.0))
        .up_direction(Vec3::new(0.0, 1.0, 0.0))
        .aspect_ratio(aspect_ratio)
        .image_width(image_width)
        .focus_distance(10.0)
        .aperture(0.0)
        .vertical_field_of_view(40.0)
        .open_close_time(0.0, 1.0)
        .build();

    // build solid color materials
    let red: Arc<dyn Texture> = Arc::new(SolidColor::from_rgb(0.65, 0.05, 0.05));
    let white: Arc<dyn Texture> = Arc::new(SolidColor::from_rgb(0.73, 0.73, 0.73));
    let green: Arc<dyn Texture> = Arc::new(SolidColor::from_rgb(0.12, 0.45, 0.15));
    let all_black: Arc<dyn Texture> = Arc::new(SolidColor::from_rgb(0., 0., 0.));
    let all_white: Arc<dyn Texture> = Arc::new(SolidColor::from_rgb(1., 1., 1.));
    let light_tex: Arc<dyn Texture> = Arc::new(SolidColor::from_rgb(7., 7., 7.));
    let red_mat: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::clone(&red)));
    let white_mat: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::clone(&white)));
    let green_mat: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::clone(&green)));
    let light_mat: Arc<dyn Material> = Arc::new(DiffuseLight::from(Arc::clone(&light_tex)));

    // build the walls of the room
    let green_wall = Arc::new(FlipFace::from(Arc::new(YZRect::from(
        0.,
        555.,
        0.,
        555.,
        555.,
        Arc::clone(&green_mat),
    ))));
    let red_wall = Arc::new(YZRect::from(0., 555., 0., 555., 0., Arc::clone(&red_mat)));
    let light = Arc::new(XZRect::from(
        113.,
        443.,
        127.,
        432.,
        554.,
        Arc::clone(&light_mat),
    ));
    let floor = Arc::new(FlipFace::from(Arc::new(XZRect::from(
        0.,
        555.,
        0.,
        555.,
        555.,
        Arc::clone(&white_mat),
    ))));
    let ceiling = Arc::new(XZRect::from(0., 555., 0., 555., 0., Arc::clone(&white_mat)));
    let back_wall = Arc::new(FlipFace::from(Arc::new(XYRect::from(
        0.,
        555.,
        0.,
        555.,
        555.,
        Arc::clone(&white_mat),
    ))));

    // build a rectangular box
    let mut rect_box: Arc<dyn Hittable> = Arc::new(BoxInst::from(
        Point3::new(0., 0., 0.),
        Point3::new(165., 330., 165.),
        Arc::clone(&white_mat),
    ));
    rect_box = Arc::new(RotateY::from(Arc::clone(&rect_box), 15.0));
    rect_box = Arc::new(Translate::from(
        Arc::clone(&rect_box),
        Vec3::new(265., 0., 295.),
    ));

    // build a square box
    let mut square_box: Arc<dyn Hittable> = Arc::new(BoxInst::from(
        Point3::new(0., 0., 0.),
        Point3::new(165., 165., 165.),
        Arc::clone(&white_mat),
    ));
    square_box = Arc::new(RotateY::from(Arc::clone(&square_box), -18.0));
    square_box = Arc::new(Translate::from(
        Arc::clone(&square_box),
        Vec3::new(130., 0., 65.),
    ));

    let fog_box = Arc::new(ConstantMedium::from(Arc::clone(&rect_box), 0.01, all_black));
    let smoke_box = Arc::new(ConstantMedium::from(
        Arc::clone(&square_box),
        0.01,
        all_white,
    ));

    let mut world = HittableList::new();
    world.add(green_wall);
    world.add(red_wall);
    world.add(light);
    world.add(floor);
    world.add(ceiling);
    world.add(back_wall);
    world.add(fog_box);
    world.add(smoke_box);

    (camera, world)
}
