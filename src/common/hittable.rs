use crate::common::{Point3, Vec3, Ray};
use std::rc::Rc;

/// A trait for objects in our scene that can be *hit* by a Ray
pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;

}


/// holds a 'record' of where a Ray "hit" a "hittable" object
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct HitRecord {
    // point on the hittable that was hit by a ray
    pub p: Point3,

    // the normal vector at the point that was hit
    pub normal: Vec3,

    // position on the ray that hit the point, p
    pub t: f64,

    // true if ray hit a front face of a hittable (ray hit from outside the hittable),
    // false if a ray hit a backward face of a 'hittable' (ray hit from the inside of a hittable)
    pub front_face: bool,
}

impl HitRecord {

    /// create a new `HitRecord`
    pub fn new(p: Point3, normal: Vec3, t: f64, front_face: bool) -> Self {
        Self {
            p,
            normal,
            t,
            front_face,
        }
    }

    /// build a HitRecord with `front_face` and `normal` direction computed based on the
    /// given ray `r` and `outward_normal`
    /// `r` - the ray that hit a hittable
    /// `p` - the point where the ray hit the hittable
    /// `outward_normal` - the outward normal of the given point `p`
    /// `t` - the position along `r` that hit `p`
    pub fn with_face_normal(r: &Ray, p: Point3, outward_normal: &Vec3, t: f64) -> Self {
        let front_face = HitRecord::hit_front_face(r, outward_normal);
        let normal = match front_face {
            true => *outward_normal,
            false => -*outward_normal,
        };
        HitRecord::new(p, normal, t, front_face)
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
        self.normal = match self.front_face {
            true => *outward_normal,
            false => -*outward_normal,
        }
    }

}



// TODO may need a Rc over objects, see section 6.5
/// a list of all Hittable objects in the ray tracer's "world" (a.k.a scene)
pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>
}

impl HittableList {

    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    /// clear the list of all objects
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    /// add a hittable object to the hittable list
    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }

}

impl Hittable for HittableList {

    /// iterate through the hittable list to determine if a ray has hit some
    /// object in the world. If an object was hit, `Some(HitRecord)` is returned
    /// containing details of the closest "hit". If no object was hit by the ray,
    /// `None` is returned
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut hit_anything: Option<HitRecord> = None;

        for object in self.objects.iter() {
            if let Some(hit_record) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = hit_record.t;
                hit_anything = Some(hit_record);
            }
        }

        hit_anything
    }
}
