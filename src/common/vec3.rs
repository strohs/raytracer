use crate::common::clamp;
use core::f64::consts::PI;
use rand::Rng;
use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub};

/// a 3 dimensional vector containing `x`,`y` and `z` coordinates
#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// returns Vec3.x
    pub fn x(&self) -> f64 {
        self.x
    }

    /// returns Vec3.y
    pub fn y(&self) -> f64 {
        self.y
    }

    /// returns Vec3.z
    pub fn z(&self) -> f64 {
        self.z
    }

    /// returns this Vec3's *magnitude* a.k.a *length*: `∥⃗v∥=√x2+y2+z2`
    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    /// returns the dot product of this Vec3 and other
    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// returns the square of this Vec3's length, which is equal to this Vec3 dotted with itself
    pub fn length_squared(&self) -> f64 {
        self.dot(self)
    }

    /// returns a new Vec3 that is the cross product of this Vec3 and other
    pub fn cross(&self, other: Self) -> Self {
        Self {
            x: (self.y * other.z) - (self.z * other.y),
            y: (self.z * other.x) - (self.x * other.z),
            z: (self.x * other.y) - (self.y * other.x),
        }
    }

    /// computes the unit vector of this Vec3 and returns a new Vec3
    pub fn unit_vector(&self) -> Self {
        *self / self.length()
    }

    /// returns a `Vec3` with it's `x,y,z` fields set to a random f64 in the range `0..1`
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            x: rng.gen(),
            y: rng.gen(),
            z: rng.gen(),
        }
    }

    /// returns a `Vec3` with it's `x,y,z` fields set to a random f64 in the range `min..max`
    pub fn random_range(min: f64, max: f64) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            x: rng.gen_range(min, max),
            y: rng.gen_range(min, max),
            z: rng.gen_range(min, max),
        }
    }

    /// returns a random `Vec3` that is within the bounds of a (imaginary) unit sphere.
    /// Uses "rejection method" algorithm that loops continuously until x,y,z coordinates
    /// are generated that lie within a unit sphere
    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Vec3::random_range(-1.0, 1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    /// returns a random `Vec3` using
    /// [Lambertian Diffuse](https://en.wikipedia.org/wiki/Lambert%27s_cosine_law) to generate
    /// a vector that is more uniformly distributed
    pub fn random_unit_vector() -> Self {
        let mut rng = rand::thread_rng();
        let a = rng.gen_range(0.0, 2.0 * PI);
        let z = rng.gen_range(-1.0, 1.0);
        let r = f64::sqrt(1.0 - z * z);

        Self {
            x: r * a.cos(),
            y: r * a.sin(),
            z,
        }
    }

    /// an alternative Vec3 generation function that returns a random vector within the same
    /// hemisphere of the passed in `normal`. This type of method was commonly used before
    /// Lambertian Diffuse implemented in [`Vec3::random_unit_vector`]
    pub fn random_in_hemisphere(normal: &Vec3) -> Self {
        let in_unit_sphere = Self::random_in_unit_sphere();

        if in_unit_sphere.dot(normal) > 0.0 {
            // in the same hemisphere as the normal
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    /// generates a random vector within an "unit disk". Essentially a unit vector with a
    /// a random x,y value and z=0.0
    pub fn random_in_unit_disk() -> Self {
        let mut rng = rand::thread_rng();
        loop {
            let p = Vec3::new(rng.gen_range(-1.0, 1.0), rng.gen_range(-1.0, 1.0), 0.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    /// clamps each `x,y,z` field of this `Vec3` to be between `min` and `max`
    pub fn clamped(&mut self, min: f64, max: f64) {
        self.x = clamp(self.x, min, max);
        self.y = clamp(self.y, min, max);
        self.z = clamp(self.z, min, max);
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    /// use the `-` (negation) sign to negate each coordinate of this Vec3
    /// and return a new Vec3
    ///
    /// # Example
    /// ```
    /// use raytracer::common::Vec3;
    ///
    /// let v3 = Vec3::new(1.0, -2.0, 3.0);
    /// let nv3 = -v3;
    /// assert_eq!(nv3.x(), -1.0);
    /// assert_eq!(nv3.y(), 2.0);
    /// assert_eq!(nv3.z(), -3.0);
    /// ```
    fn neg(self) -> Self::Output {
        Self {
            x: self.x * -1.0,
            y: self.y * -1.0,
            z: self.z * -1.0,
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    /// add corresponding fields of two Vec3s and return the result as a new Vec3
    ///
    /// # Example
    /// ```
    /// use raytracer::common::Vec3;
    ///
    /// let v1 = Vec3::new(1.0, 2.0, 3.0);
    /// let v2 = Vec3::new(2.0, 3.0, 4.0);
    /// let r = v1 + v2;
    /// assert_eq!(r.x(), 3.0);
    /// assert_eq!(r.y(), 5.0);
    /// assert_eq!(r.z(), 7.0);
    /// ```
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    /// subtract corresponding fields of two Vec3s and return the result as a new Vec3
    ///
    /// # Example
    /// ```
    /// use raytracer::common::Vec3;
    ///
    /// let v1 = Vec3::new(1.0, 2.0, 4.0);
    /// let v2 = Vec3::new(2.0, 3.0, 3.0);
    /// let r = v1 - v2;
    /// assert_eq!(r.x(), -1.0);
    /// assert_eq!(r.y(), -1.0);
    /// assert_eq!(r.z(), 1.0);
    /// ```
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl AddAssign for Vec3 {
    /// add two Vec3's together using the `+=` operator
    /// corresponding x,y,z are added together and the result is stored in this Vec3
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
        self.z = self.z + rhs.z;
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    /// multiply each field of this Vec3 by the corresponding field in another Vec3
    /// This is **not** the cross product.
    ///
    /// # Example
    /// ```
    /// use raytracer::common::Vec3;
    ///
    /// let v1 = Vec3::new(1.0, 2.0, 3.0);
    /// let v2 = Vec3::new(5.0, 6.0, 7.0);
    /// let res = v1 * v2;
    /// assert_eq!(res.x(), 5.0);
    /// assert_eq!(res.y(), 12.0);
    /// assert_eq!(res.z(), 21.0);
    /// ```
    fn mul(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    /// multiply each field of this Vec3 by a scalar (f64) and return the result as a new Vec3
    ///
    /// # Example
    /// ```
    /// use raytracer::common::Vec3;
    ///
    /// let v3 = Vec3::new(1.0, 2.0, 3.0);
    /// let res = v3 * 2.0;
    /// assert_eq!(res.x(), 2.0);
    /// assert_eq!(res.y(), 4.0);
    /// assert_eq!(res.z(), 6.0);
    /// ```
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    /// multiply a f64 by a Vec3 and return the result as a new Vec3
    ///
    /// # Example
    /// ```
    /// use raytracer::common::Vec3;
    ///
    /// let v3 = Vec3::new(1.0, 2.0, 3.0);
    /// let res = 2.0 * v3;
    /// assert_eq!(res.x(), 2.0);
    /// assert_eq!(res.y(), 4.0);
    /// assert_eq!(res.z(), 6.0);
    /// ```
    fn mul(self, rhs: Self::Output) -> Self::Output {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl MulAssign<f64> for Vec3 {
    /// multiply each x,y,z of this Vec3 by a scalar `f64` value and store the result in
    /// this Vec3
    fn mul_assign(&mut self, rhs: f64) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
        self.z = self.z * rhs;
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    /// divide each x,y,z field of this Vec3 by a scalar (f64) value and return a new Vec3
    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl DivAssign<f64> for Vec3 {
    /// divide each x,y,z field of this Vec3 by a scalar f64 value and store the result
    /// in this Vec3
    fn div_assign(&mut self, rhs: f64) {
        self.x = self.x / rhs;
        self.y = self.y / rhs;
        self.z = self.z / rhs;
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    /// returns the x,y or z value of this Vec3 using the index operator `[]`
    ///
    /// # Panics
    /// the `index` must be in the range `0..=2` or else this function panics
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("index must be in range 0..2 inclusive"),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    /// returns the x,y or z value of this Vec3 using the index operator `[]`
    ///
    /// # Panics
    /// the `index` must be in the range `0..=2` or else this function panics
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("index must be in range 0..2 inclusive"),
        }
    }
}

/// displays each coordinate of this Vec3 with a precision of 3 decimal places
impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({:.3}, {:.3}, {:.3})", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::Vec3;

    #[test]
    fn negate_vec3() {
        let v3 = Vec3::new(1.23, -2.45, 3.67);
        let nv3 = -v3;
        assert_eq!(nv3.x, -1.23);
        assert_eq!(nv3.y, 2.45);
        assert_eq!(nv3.z, -3.67);
    }

    #[test]
    fn index_vec3() {
        let v3 = Vec3::new(1.23, -2.45, 3.67);
        assert_eq!(v3[0], 1.23);
        assert_eq!(v3[1], -2.45);
        assert_eq!(v3[2], 3.67);
    }

    #[test]
    #[should_panic]
    fn out_of_bounds_index_vec3_panics() {
        let v3 = Vec3::new(1.23, -2.45, 3.67);
        let _x = v3[3];
    }

    #[test]
    fn dot_product() {
        let v1 = Vec3::new(2.0, 3.0, 4.0);
        let v2 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v1.dot(&v2), 20.0);
    }

    #[test]
    fn cross_product() {
        let v1 = Vec3::new(2.0, 3.0, 4.0);
        let v2 = Vec3::new(5.0, 6.0, 7.0);
        let cp = v1.cross(v2);
        assert_eq!(cp.x, -3.0);
        assert_eq!(cp.y, 6.0);
        assert_eq!(cp.z, -3.0);
    }

    #[test]
    fn add_two_vec3s() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        let r = v1 + v2;
        assert_eq!(r.x, 5.0);
        assert_eq!(r.y, 7.0);
        assert_eq!(r.z, 9.0);
    }

    #[test]
    fn subtract_two_vec3s() {
        let v1 = Vec3::new(1.0, 2.0, 4.0);
        let v2 = Vec3::new(2.0, 3.0, 3.0);
        let r = v1 - v2;
        assert_eq!(r.x, -1.0);
        assert_eq!(r.y, -1.0);
        assert_eq!(r.z, 1.0);
    }

    #[test]
    fn add_assign_two_vec3s() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        v1 += v2;
        assert_eq!(v1.x, 5.0);
        assert_eq!(v1.y, 7.0);
        assert_eq!(v1.z, 9.0);
    }

    #[test]
    fn multiply_assign_vec3_by_a_scalar_f64() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        v1 *= 5.0;
        assert_eq!(v1.x, 5.0);
        assert_eq!(v1.y, 10.0);
        assert_eq!(v1.z, 15.0);
    }

    #[test]
    fn multiply_vec3_by_another_vec3() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(5.0, 6.0, 7.0);
        let r = v1 * v2;
        assert_eq!(r.x, 5.0);
        assert_eq!(r.y, 12.0);
        assert_eq!(r.z, 21.0);
    }

    #[test]
    fn multiply_vec3_by_scalar_f64() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let r = v * 2.0;
        assert_eq!(r.x, 2.0);
        assert_eq!(r.y, 4.0);
        assert_eq!(r.z, 6.0);
    }

    #[test]
    fn multiply_scalar_f64_by_vec3() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let r = 2.0 * v;
        assert_eq!(r.x, 2.0);
        assert_eq!(r.y, 4.0);
        assert_eq!(r.z, 6.0);
    }

    #[test]
    fn divide_assign_vec3_by_a_scalar_f64() {
        let mut v1 = Vec3::new(4.0, 8.0, 12.0);
        v1 /= 4.0;
        assert_eq!(v1.x, 1.0);
        assert_eq!(v1.y, 2.0);
        assert_eq!(v1.z, 3.0);
    }

    #[test]
    fn divide_vec3_by_a_scalar_f64() {
        let v1 = Vec3::new(4.0, 8.0, 12.0);
        let v2 = v1 / 2.0;
        assert_eq!(v2.x, 2.0);
        assert_eq!(v2.y, 4.0);
        assert_eq!(v2.z, 6.0);
    }

    #[test]
    fn default_vec3_is_0_0() {
        let dv3 = Vec3::default();
        assert_eq!(dv3.x, 0.0);
        assert_eq!(dv3.y, 0.0);
        assert_eq!(dv3.z, 0.0);
    }

    #[test]
    fn vec3_length() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v1.length(), f64::sqrt(14.0));
    }

    #[test]
    fn vec3_length_squared() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v1.length_squared(), 14.0);
    }
}
