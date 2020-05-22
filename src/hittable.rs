pub mod sphere;
pub use sphere::*;

pub mod moving_sphere;
pub use moving_sphere::*;

pub mod aabb;
pub use aabb::*;

pub mod hit_record;
pub use hit_record::*;

pub mod hittable_list;
pub use hittable_list::*;

use crate::common::{Ray};

/// A trait for objects in our scene that can be *hit* by a Ray
pub trait Hittable: Send + Sync {

    /// returns `Some(HitRecord)` if the given `[Ray]` `r`, has *hit* this hittable.
    /// `t_min` and `t_max` are used to constrain the bounds of the "hit" so that the object
    ///hit must be between `t_min and t_max`. If the Ray did not hit then `None` is returned
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;

    /// Computes and returns the axis-aligned bounding box `Aabb` of this hittable
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<Aabb>;
}
