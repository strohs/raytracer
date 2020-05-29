use std::sync::Arc;
use crate::common::{Point3, Ray, Vec3};
use crate::material::Material;
use crate::texture;
use crate::hittable::{Hittable, HitRecord, Aabb};
use std::fmt::{Formatter};


/// a 3D sphere "primitive" with a `center` and `radius`
pub struct Sphere {
    center: Point3,
    radius: f64,
    mat_ptr: Arc<dyn Material>,
}

impl Sphere {

    pub fn new(center: Point3, radius: f64, mat_ptr: Arc<dyn Material>) -> Self {
        Self { center, radius, mat_ptr }
    }

    /// convenience constructor to create a Sphere from x,y,z coordinates and a radius
    pub fn from_coords(cx: f64, cy: f64, cz: f64,
                       radius: f64,
                       mat_ptr: Arc<dyn Material>) -> Self
    {
        Self {
            center: Point3::new(cx, cy, cz),
            radius,
            mat_ptr,
        }
    }


    pub fn center(&self) -> Point3 { self.center }

    pub fn radius(&self) -> f64 { self.radius }

}

impl Hittable for Sphere {

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {

        // helper closure that builds a new HitRecord
        let build_hit_record = |t: f64| -> HitRecord {
            let hit_point = r.at(t);
            let outward_normal = (hit_point - self.center) / self.radius;
            let (u,v) = texture::get_sphere_uv(&outward_normal);
            HitRecord::with_face_normal(
                r,
                hit_point,
                &outward_normal,
                Arc::clone(&self.mat_ptr),
                t,
                u,
                v)
        };

        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = oc.dot(&r.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        // if the Ray hit some point on this Sphere
        if discriminant > 0.0 {
            let root = f64::sqrt(discriminant);
            let t_temp = (-half_b - root) / a;
            if t_temp < t_max && t_temp > t_min {
                return Some(build_hit_record(t_temp));
            }
            let t_temp = (-half_b + root) / a;
            if t_temp < t_max && t_temp > t_min {
                return Some(build_hit_record(t_temp));
            }
        }
        // ray did not hit this Sphere
        None
    }

    /// returns a bounding box for this sphere
    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<Aabb> {
        Some(Aabb::new(
            self.center() - Vec3::new(self.radius(), self.radius(), self.radius()),
            self.center() + Vec3::new(self.radius(), self.radius(), self.radius())
        ))
    }
}

impl std::fmt::Debug for Sphere {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        
        f.debug_struct("Sphere")
            .field("center", &self.center)
            .field("radius", &self.radius)
            .field("material", &self.mat_ptr)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use crate::hittable::{Sphere, Hittable};
    use crate::common::{Point3};
    use crate::material::{Lambertian, Material};
    use std::sync::Arc;
    use crate::texture::{SolidColor, Texture};

    #[test]
    fn has_a_bounding_box_of_min_0_and_max_2() {
        // create a test sphere located at 1,1,1  with a radius of 1
        let tex: Arc<dyn Texture> = Arc::new(SolidColor::from_rgb(0.5, 0.5, 0.5));
        let lamb_mat: Arc<dyn Material> = Arc::new(Lambertian::new(tex));
        let sphere = Sphere::new(Point3::new(1.0, 1.0, 1.0), 1.0, lamb_mat);

        let aabb = sphere.bounding_box(1.0, 1.0);

        assert!(aabb.is_some());
        assert_eq!(aabb.unwrap().min(), Point3::new(0.0, 0.0, 0.0));
        assert_eq!(aabb.unwrap().max(), Point3::new(2.0, 2.0, 2.0));
    }

    #[test]
    fn should_print_debug() {
        let tex: Arc<dyn Texture> = Arc::new(SolidColor::from_rgb(0.5, 0.5, 0.5));
        let lamb_mat: Arc<dyn Material> = Arc::new(Lambertian::new(tex));
        let sphere = Sphere::new(Point3::new(1.0, 1.0, 1.0), 1.0, lamb_mat);
        println!("{:#?}", sphere);
    }
}