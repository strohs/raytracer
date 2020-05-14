use crate::common::{Point3, Vec3, Ray};

/// holds information on where a Ray "hit" a 3D object
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    // front_face=true if ray is outside the 'hittable', false of ray is inside the 'hittable'
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

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = match self.front_face {
            true => *outward_normal,
            false => -*outward_normal,
        }
    }

}

/// A trait for 3D objects that can be *hit* by a Ray
pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}


// TODO may need a Rc over objects, see section 6.5
// TODO may need to modify Hittable to return Option<HitRecord>
#[derive(Debug)]
pub struct HittableList<T: Hittable> {
    objects: Vec<T>
}

impl<T: Hittable> HittableList<T> {

    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: T) {
        self.objects.push(object);
    }

    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut temp_rec = HitRecord::default();
        let mut closest_so_far = t_max;
        let mut hit_anything: Option<HitRecord> = None;

        for object in self.objects.iter() {
            if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                closest_so_far = temp_rec.t;
                hit_anything = Some(temp_rec);
            }
        }

        hit_anything
    }
}
