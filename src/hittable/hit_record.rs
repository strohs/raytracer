use std::sync::Arc;
use crate::common::{Point3, Vec3, Ray};
use crate::material::Material;

/// holds a 'record' containing the detials of where a Ray "hit" a "hittable" object
pub struct HitRecord {
    // point on the hittable that was hit by a ray
    pub p: Point3,

    // the normal vector at the point that was hit
    pub normal: Vec3,

    // a (shared) pointer to the material that was hit
    pub mat_ptr: Arc<dyn Material>,

    // position along the ray that hit the point, `p`
    pub t: f64,

    // true if ray hit a front face of a hittable (ray hit from outside the hittable),
    // false if a ray hit a backward face of a 'hittable' (ray hit from the inside of a hittable)
    pub front_face: bool,
}

impl HitRecord {

    /// create a new `HitRecord`
    pub fn new(p: Point3,
               normal: Vec3,
               mat_ptr: Arc<dyn Material>,
               t: f64,
               front_face: bool) -> Self
    {
        Self {
            p,
            normal,
            mat_ptr,
            t,
            front_face,
        }
    }

    /// build a HitRecord with `front_face` and `normal` direction computed based on the
    /// given ray `r` and `outward_normal`
    /// `r` - the ray that hit a hittable
    /// `p` - the point where the ray hit the hittable
    /// `outward_normal` - the outward normal of the given point `p`
    /// `mat_ptr` - pointer the type of material that was hit
    /// `t` - the position along `r` that hit `p`
    pub fn with_face_normal(r: &Ray,
                            p: Point3,
                            outward_normal: &Vec3,
                            mat_ptr: Arc<dyn Material>,
                            t: f64) -> Self
    {
        let front_face = HitRecord::hit_front_face(r, outward_normal);
        let normal = if front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
        HitRecord::new(p, normal, mat_ptr, t, front_face)
    }

    /// returns true if the given ray has "hit" a front face of a Hittable, returns false
    /// if the ray hit an "inner face" of a hittable
    fn hit_front_face(r: &Ray, outward_normal: &Vec3) -> bool {
        r.direction().dot(outward_normal) < 0.0
    }

    /// compute and set the `front_face` and `normal` fields of this HitRecord given
    /// a Ray and outward_normal
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = HitRecord::hit_front_face(r, outward_normal);
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        }
    }

}