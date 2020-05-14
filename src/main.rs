use raytracer::ppm;
use std::path::Path;
use raytracer::common::{Color, Point3, Vec3, Ray};

/// linearly blends white and blue depending on the height of the y coordinate after
/// scaling the ray direction to unit length (so −1.0 < y < 1.0)
/// When `t=1.0` I want blue. When `t=0.0` I want white. In between, I want a blend
fn ray_color(r: &Ray) -> Color {
    // sphere centered at 0,0,-1, radius will be 0.5
    let sphere_center = Point3::new(0.0, 0.0, -1.0);

    // if the ray hits
    let mut t = hit_sphere(&sphere_center, 0.5, r);
    if t > 0.0 {
        let n: Vec3 = (r.at(t) - Vec3::new(0.0, 0.0, -1.0)).unit_vector();
        return 0.5 * Color::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
    }
    // scale the ray direction to unit length (so −1.0 < y < 1.0)
    let unit_direction = r.direction().unit_vector();
    t = 0.5 * (unit_direction.y() + 1.0);

    // linear interpolation from white to blue
    (1.0 - t) * Color::new(1.0, 1.0, 1.0)
        + t * Color::new(0.5, 0.7, 1.0)
}

/// if a ray hits the provided sphere, return the `t` value of the ray
/// else return -1.0
fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin() - *center;
    let a = r.direction().length_squared();
    let half_b = oc.dot(&(r.direction()));
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - f64::sqrt(discriminant)) / a
    }
}

fn main() {
    println!("Raytracer....");
    let path = Path::new("./testGradient.ppm");
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 384;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let mut image: Vec<Color> = vec![];

    // camera is at the origin
    let origin: Point3 = Point3::new(0.0, 0.0, 0.0);
    let horizontal: Vec3 = Vec3::new(4.0, 0.0, 0.0);
    let vertical: Vec3 = Vec3::new(0.0, 2.25, 0.0);
    let lower_left_corner: Point3 = origin
        - horizontal / 2.0
        - vertical / 2.0
        - Vec3::new(0.0,0.0,1.0);

    // traverse the screen from lower left corner to upper right
    for j in (0..image_height).rev() {
        println!("Scanlines remaining {}", &j);
        for i in 0..image_width {
            // u,v are offset vectors that move the ray endpoint across the screen
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;
            let r: Ray = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical
            );
            let pixel_color: Color = ray_color(&r);
            image.push(pixel_color);
        }
    }
    match ppm::write(path, image_width, image_height, &image) {
        Ok(()) => println!("test image created at {:?}", path),
        Err(e) => eprintln!("{}", e),
    }
}
