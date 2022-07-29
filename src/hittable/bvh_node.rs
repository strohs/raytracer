use crate::common::Ray;
use crate::hittable::{Aabb, HitRecord, Hittable, HittableList};
use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::fmt;
use std::fmt::Formatter;
use std::sync::Arc;

/// A Bounded Volume Hierarchy (BVH)
/// A BVH is `Hittable` but it’s really a container. It's a binary "tree like" structure that can
/// respond to the question, “does this ray hit you?”.
/// It recursively sorts and subdivides the `Hittable`s in the "world" into smaller and smaller
/// groups, based on a Hittable's bounding box. Each "level" of the BVH will contain Hittables
/// such that their bounding boxes are contained within their parent bounding box.
/// The "leaves" of the BVH contain a single primitive, such as a sphere or cube etc...
pub struct BvhNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    // a bounding box that surrounds the BVH Node and it's children
    bbox: Aabb,
}

impl BvhNode {
    /// Constructs a BVH from the `list` of Hittables. The returned BVH will be the "root" node
    /// of the BVH
    pub fn from(list: &mut HittableList, time0: f64, time1: f64) -> BvhNode {
        BvhNode::split_volumes(list.objects(), time0, time1)
    }

    /// Constructs a single `BvhNode`
    fn new(left: Arc<dyn Hittable>, right: Arc<dyn Hittable>, bbox: Aabb) -> Self {
        Self { left, right, bbox }
    }

    /// Constructs a BVH from a list of Hittables.
    /// As long as the list of objects in a BvhNode gets divided into two sub-lists, the hit
    /// function will work. It will work best if the division is done well, so that the two
    /// children have smaller bounding boxes than their parent’s bounding box, but that is for
    /// speed not correctness. This function chooses the middle ground, at each node, split
    /// the list along one axis.
    ///
    /// 1. randomly choose an axis
    /// 2. sort the (hittable) primitives
    /// 3. put half in each subtree
    fn split_volumes(objects: &mut [Arc<dyn Hittable>], time0: f64, time1: f64) -> BvhNode {
        // randomly choose an x,y, or z axis for sorting the list of hittable objects
        let axis: usize = thread_rng().gen_range(0..3);

        let mut node: BvhNode = if objects.len() == 1 {
            // if there's only one element, put a reference to it in each subtree and end recursion
            BvhNode::new(
                Arc::clone(&objects[0]),
                Arc::clone(&objects[0]),
                Aabb::default(),
            )
        } else if objects.len() == 2 {
            // if objects only has two elements, put one in each subtree and end recursion
            if BvhNode::box_compare(&*objects[0], &*objects[1], axis) == Ordering::Less {
                BvhNode::new(
                    Arc::clone(&objects[0]),
                    Arc::clone(&objects[1]),
                    Aabb::default(),
                )
            } else {
                BvhNode::new(
                    Arc::clone(&objects[1]),
                    Arc::clone(&objects[0]),
                    Aabb::default(),
                )
            }
        } else {
            // recursively partition the remaining hittables into BVH Nodes, using their
            // bounding box axis' to sort then into left and right children
            objects.sort_unstable_by(|a, b| BvhNode::box_compare(&**a, &**b, axis));
            let mid = objects.len() / 2;
            let left = BvhNode::split_volumes(objects[0..mid].as_mut(), time0, time1);
            let right = BvhNode::split_volumes(objects[mid..].as_mut(), time0, time1);

            BvhNode::new(Arc::new(left), Arc::new(right), Aabb::default())
        };

        // construct a bounding box encompassing this node's left and right children
        let box_left = node.left.bounding_box(time0, time1);
        let box_right = node.right.bounding_box(time0, time1);
        if box_left.is_none() || box_right.is_none() {
            panic!("a hittable did not have a bounding box during BVH construction");
        }
        node.bbox = Aabb::surrounding_box(&box_left.unwrap(), &box_right.unwrap());

        node
    }

    /// Compares the axis aligned bounding boxes of two `Hittable`s using their respective
    /// `Aabb.min()` parameters.
    /// `axis` indicates which axis to use in the comparison.
    /// 0 = x-axis,
    /// 1 = y-axis,
    /// 2 = z-axis
    fn box_compare<T: Hittable + ?Sized>(a: &T, b: &T, axis: usize) -> Ordering {
        let box_a = a
            .bounding_box(0.0, 0.0)
            .expect("Hittable 'a' doesn't have a bounding box");
        let box_b = b
            .bounding_box(0.0, 0.0)
            .expect("Hittable 'b' doesn't have a bounding box");

        box_a.min()[axis]
            .partial_cmp(&box_b.min()[axis])
            .unwrap_or_else(|| {
                panic!(
                    "could not compare axis {} of box 'a' {:?} against box 'b' {:?}",
                    axis,
                    box_a.min()[axis],
                    box_b.min()[axis]
                )
            })
    }
}

impl Hittable for BvhNode {
    /// Check if the bounding box for a node is hit, and if so, recursively check its children
    /// to determine which child was hit (if any).
    /// Returns a `HitRecord` for the deepest node that was hit
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // first check if the hittable's bounding box was hit
        self.bbox.hit(ray, t_min, t_max)?;

        // check if the left and right children are hit. The hittable being checked could be
        // a BvhNode, or some other Hittable, like a primitive (sphere etc...)
        let hit_left = self.left.hit(ray, t_min, t_max);
        let hit_right = if let Some(hit_rec) = &hit_left {
            self.right.hit(ray, t_min, hit_rec.t)
        } else {
            self.right.hit(ray, t_min, t_max)
        };

        if hit_right.is_some() {
            hit_right
        } else if hit_left.is_some() {
            hit_left
        } else {
            None
        }
    }

    /// Returns `Some(Aabb)` which is the axis-aligned bounding box that encompasses **all** of
    /// the `Hittables` contained by this `BvhNode`
    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<Aabb> {
        Some(self.bbox)
    }
}

impl std::fmt::Debug for BvhNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("BvhNode")
            .field("left", &self.left)
            .field("right", &self.right)
            .field("bbox", &self.bbox)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use crate::common::Point3;
    use crate::hittable::{BvhNode, Sphere};
    use crate::material::{Lambertian, Material};
    use crate::texture::{SolidColor, Texture};
    use std::cmp::Ordering;
    use std::sync::Arc;

    #[test]
    fn box_compare_sphere1_x_axis_lt_sphere2() {
        let tex: Arc<dyn Texture> = Arc::new(SolidColor::from_rgb(0.5, 0.5, 0.5));
        let lamb_mat: Arc<dyn Material> = Arc::new(Lambertian::new(tex));
        let sphere1 = Sphere::new(Point3::new(1.0, 1.0, 1.0), 1.0, Arc::clone(&lamb_mat));
        let sphere2 = Sphere::new(Point3::new(2.0, 2.0, 2.0), 1.0, Arc::clone(&lamb_mat));

        assert_eq!(BvhNode::box_compare(&sphere1, &sphere2, 0), Ordering::Less);
    }

    #[test]
    fn box_compare_sphere1_y_axis_lt_sphere2() {
        let tex: Arc<dyn Texture> = Arc::new(SolidColor::from_rgb(0.5, 0.5, 0.5));
        let lamb_mat: Arc<dyn Material> = Arc::new(Lambertian::new(tex));
        let sphere1 = Sphere::new(Point3::new(1.0, 1.0, 1.0), 1.0, Arc::clone(&lamb_mat));
        let sphere2 = Sphere::new(Point3::new(2.0, 2.0, 2.0), 1.0, Arc::clone(&lamb_mat));

        assert_eq!(BvhNode::box_compare(&sphere1, &sphere2, 1), Ordering::Less);
    }

    #[test]
    fn box_compare_sphere1_z_axis_lt_sphere2() {
        let tex: Arc<dyn Texture> = Arc::new(SolidColor::from_rgb(0.5, 0.5, 0.5));
        let lamb_mat: Arc<dyn Material> = Arc::new(Lambertian::new(tex));
        let sphere1 = Sphere::new(Point3::new(1.0, 1.0, 1.0), 1.0, Arc::clone(&lamb_mat));
        let sphere2 = Sphere::new(Point3::new(2.0, 2.0, 2.0), 1.0, Arc::clone(&lamb_mat));

        assert_eq!(BvhNode::box_compare(&sphere1, &sphere2, 2), Ordering::Less);
    }

    #[test]
    fn box_compare_sphere1_xaxis_to_sphere2_should_be_eq() {
        let tex: Arc<dyn Texture> = Arc::new(SolidColor::from_rgb(0.5, 0.5, 0.5));
        let lamb_mat: Arc<dyn Material> = Arc::new(Lambertian::new(tex));
        let sphere1 = Sphere::new(Point3::new(1.0, 1.0, 1.0), 1.0, Arc::clone(&lamb_mat));
        let sphere2 = Sphere::new(Point3::new(1.0, 2.0, 2.0), 1.0, Arc::clone(&lamb_mat));

        assert_eq!(BvhNode::box_compare(&sphere1, &sphere2, 0), Ordering::Equal);
    }

    #[test]
    fn box_compare_sphere1_xaxis_gt_sphere2() {
        let tex: Arc<dyn Texture> = Arc::new(SolidColor::from_rgb(0.5, 0.5, 0.5));
        let lamb_mat: Arc<dyn Material> = Arc::new(Lambertian::new(tex));
        let sphere1 = Sphere::new(Point3::new(2.0, 1.0, 1.0), 1.0, Arc::clone(&lamb_mat));
        let sphere2 = Sphere::new(Point3::new(1.0, 2.0, 2.0), 1.0, Arc::clone(&lamb_mat));

        assert_eq!(
            BvhNode::box_compare(&sphere1, &sphere2, 0),
            Ordering::Greater
        );
    }

    // #[test]
    // fn debug_bvh_node() {
    //     let tex: Arc<dyn Texture> = Arc::new(SolidColor::from_rgb(0.5, 0.5, 0.5));
    //     let lamb_mat: Arc<dyn Material> = Arc::new(Lambertian::new(tex));
    //     let sphere1 = Sphere::new(Point3::new(7.0, 0.0, 1.0), 1.0, Arc::clone(&lamb_mat));
    //     let sphere2 = Sphere::new(Point3::new(5.0, 0.0, 1.0), 1.0, Arc::clone(&lamb_mat));
    //     let sphere3 = Sphere::new(Point3::new(3.0, 0.0, 1.0), 1.0, Arc::clone(&lamb_mat));
    //     let sphere4 = Sphere::new(Point3::new(1.0, 0.0, 1.0), 1.0, Arc::clone(&lamb_mat));
    //     let mut hit_list = HittableList::new();
    //     hit_list.add(Arc::new(sphere1));
    //     hit_list.add(Arc::new(sphere2));
    //     hit_list.add(Arc::new(sphere3));
    //     hit_list.add(Arc::new(sphere4));
    //     let root = BvhNode::from(&mut hit_list, 0.0, 1.0);
    //     //dbg!(root);
    // }
}
