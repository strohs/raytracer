use crate::common::{degrees_to_radians, Point3, Ray, Vec3};
use crate::hittable::{Aabb, HitRecord, Hittable};
use std::sync::Arc;

#[derive(Debug)]
pub struct RotateY {
    ptr: Arc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Option<Aabb>,
}

impl RotateY {
    pub fn from(p: Arc<dyn Hittable>, angle: f64) -> Self {
        let bbox = p
            .bounding_box(0.0, 1.0)
            .expect("can't rotate-y a Hittable that doesn't have a bounding box");

        let sin_theta = degrees_to_radians(angle).sin();
        let cos_theta = degrees_to_radians(angle).cos();

        let mut min = Point3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max = Point3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.max().x() + (1.0 - i as f64) * bbox.min().x();
                    let y = j as f64 * bbox.max().y() + (1.0 - j as f64) * bbox.min().y();
                    let z = k as f64 * bbox.max().z() + (1.0 - k as f64) * bbox.min().z();

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = (-sin_theta * x) + (cos_theta * z);

                    let tester = Vec3::new(newx, y, newz);

                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }

        Self {
            ptr: p,
            sin_theta,
            cos_theta,
            bbox: Some(Aabb::new(min, max)),
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut origin = r.origin();
        let mut direction = r.direction();

        origin[0] = self.cos_theta * r.origin()[0] - self.sin_theta * r.origin()[2];
        origin[2] = self.sin_theta * r.origin()[0] + self.cos_theta * r.origin()[2];

        direction[0] = self.cos_theta * r.direction()[0] - self.sin_theta * r.direction()[2];
        direction[2] = self.sin_theta * r.direction()[0] + self.cos_theta * r.direction()[2];

        let rotated_r = Ray::new(origin, direction, r.time());

        if let Some(mut rec) = self.ptr.hit(&rotated_r, t_min, t_max) {
            let mut p = rec.p;
            let mut normal = rec.normal;

            p[0] = self.cos_theta * rec.p[0] + self.sin_theta * rec.p[2];
            p[2] = -self.sin_theta * rec.p[0] + self.cos_theta * rec.p[2];
            normal[0] = self.cos_theta * rec.normal[0] + self.sin_theta * rec.normal[2];
            normal[2] = -self.sin_theta * rec.normal[0] + self.cos_theta * rec.normal[2];

            rec.p = p;
            rec.set_face_normal(&rotated_r, &normal);

            Some(rec)
        } else {
            None
        }
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<Aabb> {
        self.bbox
    }
}

#[cfg(test)]
mod tests {
    use crate::common::{Color, Point3};
    use crate::hittable::{BoxInst, RotateY};
    use crate::material::Metal;
    use std::sync::Arc;

    #[test]
    fn rotate_about_y_90_degrees() {
        let box_inst = BoxInst::from(
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(1., 1., 1.),
            Arc::new(Metal::new(Color::new(0., 0., 0.), 0.5)),
        );
        let roty = RotateY::from(Arc::new(box_inst), 90.0);
        dbg!(roty);
    }
}
