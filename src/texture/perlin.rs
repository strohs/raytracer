
use rand::seq::SliceRandom;
use crate::common::{Point3, Vec3};
use std::fmt::Formatter;

const POINT_COUNT: usize = 256;

/// A helper struct that can be used to generate Perlin noise via a call to the
/// `perlin_noise(p: Point3)` method of this struct
pub struct Perlin {
    perm_x: [i32; POINT_COUNT],
    perm_y: [i32; POINT_COUNT],
    perm_z: [i32; POINT_COUNT],
    rand_vecs: [Vec3; POINT_COUNT],
}

impl Default for Perlin {
    fn default() -> Self {
        Self {
            perm_x: [0_i32; POINT_COUNT],
            perm_y: [0_i32; POINT_COUNT],
            perm_z: [0_i32; POINT_COUNT],
            rand_vecs: [Vec3::default(); POINT_COUNT],
        }
    }
}

impl Perlin {

    /// Generates a new, randomized Perlin struct
    pub fn new() -> Self {
        let mut perlin = Perlin::default();

        for item in perlin.rand_vecs.iter_mut() {
            *item = Vec3::random_range(-1.0, 1.0).unit_vector();
        }

        Perlin::generate_perm(&mut perlin.perm_x);
        Perlin::generate_perm(&mut perlin.perm_y);
        Perlin::generate_perm(&mut perlin.perm_z);

        perlin
    }

    /// Returns a random perlin noise value.
    /// it takes a 3D point as input, `point`, and always returns the same "randomish number".
    /// Nearby points should return similar numbers. Another important part of Perlin noise is
    /// that it be simple and fast, so it’s usually done as a hack...
    pub fn perlin_noise(&self, point: &Point3) -> f64 {
        let u = point.x() - point.x().floor();
        let v = point.y() - point.y().floor();
        let w = point.z() - point.z().floor();

        let i = point.x().floor() as usize;
        let j = point.y().floor() as usize;
        let k = point.z().floor() as usize;

        let mut c = [Vec3::default(); 8];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let i1d = di + 2 * (dj + 2 * dk);
                    let idx = (
                        self.perm_x[(i + di) & 255] ^
                            self.perm_y[(j + dj) & 255] ^
                            self.perm_z[(k + dk) & 255]) as usize;

                    c[i1d] = self.rand_vecs[idx];
                }
            }
        }

        perlin_interp(&c, u, v, w)
    }

    /// Generates turbulence on a texture via repeated calls to `perlin_noise()`.
    /// `depth` controls the number of times to call perlin_noise()
    pub fn turb(&self, point: &Point3, depth: usize) -> f64 {
        let mut accum: f64 = 0.0;
        let mut temp_p = *point;
        let mut weight = 1.0;

        for _i in 0..depth {
            accum += weight * self.perlin_noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        accum.abs()
    }

    /// fills the input array with integers in the range 0..POINT_COUNT and then
    /// "shuffles" the array
    fn generate_perm(arr: &mut [i32; POINT_COUNT]) {
        let mut rng = rand::thread_rng();

        for (i, item) in arr.iter_mut().enumerate() {
            *item = i as i32;
        }
        
        arr.shuffle(&mut rng);
    }
}


/// trilinear interpolation used to smooth out perlin noise
fn perlin_interp(
    c: &[Vec3; 8],
    u: f64,
    v: f64,
    w: f64) -> f64
{
    // Hermitian cubic used to smooth out the noise
    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);
    
    let mut accum = 0.0;

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let idx = i + 2 * (j + 2 * k);
                let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                accum += (i as f64 * uu + (1.0 - i as f64) * (1.0 - uu))
                    * (j as f64 * vv + (1.0 - j as f64) * (1.0 - vv))
                    * (k as f64 * ww + (1.0 - k as f64) * (1.0 - ww))
                    * (c[idx]).dot(&weight_v);
            }
        }
    }
    accum
}


/// only shows the first 5 values for array in the Perlin struct
impl std::fmt::Debug for Perlin {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {

        f.debug_struct("Perlin")
            .field("rand_vecs", &format_args!("{:?}...", &self.rand_vecs[0..5]))
            .field("perm_x", &format_args!("{:?}...", &self.perm_x[0..5]))
            .field("perm_y", &format_args!("{:?}...", &self.perm_y[0..5]))
            .field("perm_z", &format_args!("{:?}...", &self.perm_z[0..5]))
            .finish()
    }
}