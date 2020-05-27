use std::sync::Arc;
use std::cmp::Ordering;
use rand::{Rng, thread_rng};
use crate::hittable::{Hittable, Aabb, HitRecord, HittableList};
use crate::common::Ray;


/// A Bounded Volume Hierarchy (BVH)
/// A BVH is `Hittable` but it’s really a container. It can respond to the question,
/// “does this ray hit you?”.
/// It sorts the `Hittable`s in the "world" by a (random) bounding box axis, and then recursively
/// subdivides the hittables into smaller groups so that child groups have smaller bounding boxes
/// then their parent.
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
        Self { left, right, bbox, }
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
    fn split_volumes(
        objects: &mut [Arc<dyn Hittable>],
        time0: f64,
        time1: f64) -> BvhNode
    {
        // randomly choose an x,y, or z axis for sorting the list of hittable objects
        let axis: usize = thread_rng().gen_range(0, 3);

        let mut node: BvhNode = if objects.len() == 1 {
            // if there's only one element, put a reference to it in each subtree and end recursion
            BvhNode::new(
                Arc::clone(&objects[0]),
                Arc::clone(&objects[0]),
                Aabb::default())
        } else if objects.len() == 2 {
            // if objects only has two elements, put one in each subtree and end recursion
            if box_compare(&*objects[0], &*objects[1], axis) == Ordering::Less {
                BvhNode::new(
                    Arc::clone(&objects[0]),
                    Arc::clone(&objects[1]),
                    Aabb::default())
            } else {
                BvhNode::new(
                    Arc::clone(&objects[1]),
                    Arc::clone(&objects[0]),
                    Aabb::default())
            }
        } else {
            // sort the hittable objects by their bounding box axis'
            objects.sort_unstable_by(|a,b| box_compare(&**a, &**b, axis));
            let mid = objects.len() / 2;
            let left = BvhNode::split_volumes(objects[0..mid].as_mut(), time0, time1);
            let right = BvhNode::split_volumes(objects[mid..].as_mut(), time0, time1);

            BvhNode::new(
                Arc::new(left),
                Arc::new(right),
                Aabb::default())
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
}

impl Hittable for BvhNode {

    /// Check if the bounding box for a node is hit, and if so, recursively check its children
    /// to determine which child was hit.
    /// Returns a `HitRecord` for the deepest node that was hit
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // first check if the hittable's bounding box was hit
        if self.bbox.hit(r, t_min, t_max).is_none() {
            return None;
        }

        let hit_left = self.left.hit(r, t_min, t_max);
        let hit_right = if let Some(hit_rec) = &hit_left {
            self.right.hit(r, t_min, hit_rec.t)
        } else {
            self.right.hit(r, t_min, t_max)
        };

        return if hit_right.is_some() {
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


/// Compares the axis aligned bounding boxes of two `Hittable`s using their respective `Aabb.min()`
/// parameters.
/// `axis` indicates which axis to use in the comparison.
/// 0 = x-axis,
/// 1 = y-axis,
/// 2 = z-axis
fn box_compare<T: Hittable + ?Sized>(a: &T, b: &T, axis: usize) -> Ordering {
    let box_a = a.bounding_box(0.0, 0.0)
        .expect("Hittable 'a' doesn't have a bounding box");
    let box_b = b.bounding_box(0.0, 0.0)
        .expect("Hittable 'b' doesn't have a bounding box");

    box_a.min()[axis].partial_cmp(&box_b.min()[axis])
        .expect(
            format!("could not compare axis {} of box 'a' {:?} against box 'b' {:?}",
                    axis,
                    box_a.min()[axis],
                    box_b.min()[axis]).as_str())

}