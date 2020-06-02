use crate::common::{Point3, Ray};
use crate::hittable::{HittableList, XYRect, FlipFace, XZRect, YZRect, Hittable, HitRecord, Aabb};
use crate::material::Material;
use std::sync::Arc;

/// BoxInst is a 3D box made up of six rectangles
#[derive(Default, Debug)]
pub struct BoxInst {
    box_min: Point3,
    box_max: Point3,
    sides: HittableList,
}

impl BoxInst {

    /// Returns an axis-aligned Box consisting of six sides. The passed in `Material` will
    /// be applied to all sides of the box
    pub fn from(p0: Point3, p1: Point3, ptr: Arc<dyn Material>) -> Self {
        let mut box_inst = BoxInst::default();
        box_inst.box_min = p0;
        box_inst.box_max = p1;

        box_inst.sides.add(Arc::new(XYRect::from(
            p0.x(), p1.x(),
            p0.y(), p1.y(),
            p1.z(), Arc::clone(&ptr))));
        box_inst.sides.add(Arc::new(
            FlipFace::from(
                Arc::new(XYRect::from(
                    p0.x(), p1.x(),
                    p0.y(), p1.y(),
                    p0.z(), Arc::clone(&ptr))))));

        box_inst.sides.add(Arc::new(XZRect::from(
            p0.x(), p1.x(),
            p0.z(), p1.z(),
            p1.y(), Arc::clone(&ptr))));
        box_inst.sides.add(Arc::new(
            FlipFace::from(
                Arc::new(XZRect::from(
                    p0.x(), p1.x(),
                    p0.z(), p1.z(),
                    p0.y(), Arc::clone(&ptr))))));

        box_inst.sides.add(Arc::new(YZRect::from(
            p0.y(), p1.y(),
            p0.z(), p1.z(),
            p1.x(), Arc::clone(&ptr))));
        box_inst.sides.add(Arc::new(
            FlipFace::from(
                Arc::new(YZRect::from(
                    p0.y(), p1.y(),
                    p0.z(), p1.z(),
                    p0.x(), Arc::clone(&ptr))))));

        box_inst
    }
}

impl Hittable for BoxInst {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.hit(r, t_min, t_max)
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<Aabb> {
        Some(Aabb::new(self.box_min, self.box_max))
    }
}