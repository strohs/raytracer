use crate::common::{Color, Point3, Vec3};
use crate::hittable::{HittableList, Sphere, Hittable};
use std::rc::Rc;
use crate::material::{Lambertian, Material, Metal, Dielectric};
use rand::Rng;

/// generates a random "world" containing 44 spheres of various colors and materials on a x/y plane
pub fn random_scene() -> HittableList {
    let mut rng = rand::thread_rng();

    let mut world = HittableList::new();

    // a big, Lambertian grey, sphere that will act at the ground
    let ground_material: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let ground_sphere = Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material);
    world.add(Rc::new(ground_sphere));

    // generate 484 sphere with random materials and colors, all of radius 0.2
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.gen();
            let center: Point3 = Point3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>());

            if (center - Vec3::new(4., 0.2, 0.)).length() > 0.9 {
                let sphere_mat: Rc<dyn Material>;

                if choose_mat < 0.8 {
                    // 80% will be diffuse spheres
                    let albedo = Color::random() * Color::random();
                    sphere_mat = Rc::new(Lambertian::new(albedo));
                } else if choose_mat < 0.95 {
                    // 15% will be metal sphere with random fuzziness
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = rng.gen_range(0.0, 0.5);
                    sphere_mat = Rc::new(Metal::new(albedo, fuzz));
                }  else {
                    // 5% will be glass spheres
                    sphere_mat = Rc::new(Dielectric::new(1.5));
                }
                world.add(Rc::new(Sphere::new(center, 0.2, Rc::clone(&sphere_mat))));
            }
        }
    }

    // add a single, larger glass sphere
    let mat1: Rc<dyn Material> = Rc::new(Dielectric::new(1.5));
    let sphere1: Rc<dyn Hittable> = Rc::new(Sphere::new(Point3::new(0., 1., 0.), 1.0, Rc::clone(&mat1)));
    world.add(sphere1);

    // add a single, lambertian brown colored sphere
    let mat2: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    let sphere2: Rc<dyn Hittable> = Rc::new(Sphere::new(Point3::new(-4., 1., 0.), 1.0, Rc::clone(&mat2)));
    world.add(sphere2);

    // add a single metal sphere (tan color)
    let mat3: Rc<dyn Material> = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    let sphere3: Rc<dyn Hittable> = Rc::new(Sphere::new(Point3::new(4., 1., 0.), 1.0, Rc::clone(&mat3)));
    world.add(sphere3);

    world
}