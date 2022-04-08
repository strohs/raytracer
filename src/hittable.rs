pub mod primitive;
pub use primitive::*;

pub mod volume;
pub use volume::*;

pub mod aabb;
pub use aabb::*;

pub mod flip_face;
pub use flip_face::*;

pub mod bvh_node;
pub use bvh_node::*;

pub mod hit_record;
pub use hit_record::*;

pub mod hittable_list;
pub use hittable_list::*;

pub mod translate;
pub use translate::*;

pub mod rotate;
pub use rotate::*;

use crate::common::Ray;

/// A trait for primitives in a scene that can be *hit* by a Ray
pub trait Hittable: Send + Sync + std::fmt::Debug {
    /// returns `Some(HitRecord)` if the given `[Ray]` `r`, has *hit* this hittable.
    /// `t_min` and `t_max` are used to constrain the bounds of the "hit" so that the object
    /// hit must be between `t_min and t_max`. If the Ray did not hit then `None` is returned
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;

    /// Computes and returns the axis-aligned bounding box `Aabb` of this hittable
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<Aabb>;
}
