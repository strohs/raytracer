//! utility functions for building different types of primitives

use crate::common::{Color, Point3};
use crate::hittable::{ConstantMedium, Hittable, MovingSphere, Sphere, XYRect, XZRect};
use crate::material::{Dielectric, DiffuseLight, Lambertian, Material, Metal};
use crate::texture::{CheckerTexture, ImageTexture, NoiseTexture, SolidColor};
use std::sync::Arc;

pub fn build_solid_moving_sphere(
    color: Color,
    c1: Point3,
    c2: Point3,
    t0: f64,
    t1: f64,
    rad: f64,
) -> MovingSphere {
    let solid_lamb = build_solid_lambertian(color.x(), color.y(), color.z());
    MovingSphere::new(c1, c2, t0, t1, rad, Arc::new(solid_lamb))
}

/// Returns a new sphere with a dielectric material with the specified refractive index `ref_idx`
pub fn build_dielectric_sphere(center: Point3, rad: f64, ref_idx: f64) -> Sphere {
    let dielectric = Dielectric::new(ref_idx);
    Sphere::new(center, rad, Arc::new(dielectric))
}

/// Returns a new sphere with a metal material with the specified color and fuzziness
pub fn build_metal_sphere(center: Point3, rad: f64, color: Color, fuzz: f64) -> Sphere {
    let metal = Metal::new(color, fuzz);
    Sphere::new(center, rad, Arc::new(metal))
}

/// Returns a new sphere with a solid lambertian material, with the specified color
pub fn build_solid_sphere(center: Point3, rad: f64, color: Color) -> Sphere {
    let solid_tex = SolidColor::from(color);
    let mat = Lambertian::new(Arc::new(solid_tex));
    Sphere::new(center, rad, Arc::new(mat))
}

/// Returns a sphere textured with the 'earthmap.jpg' texture
pub fn build_earth_sphere(center: Point3, rad: f64) -> Sphere {
    let etex = ImageTexture::from("./earthmap.jpg");
    let emat = Lambertian::new(Arc::new(etex));
    Sphere::new(center, rad, Arc::new(emat))
}

/// Returns a new sphere with a perlin noise texture
pub fn build_perlin_sphere(center: Point3, rad: f64, noise_scale: f64) -> Sphere {
    let pertex = NoiseTexture::new(noise_scale);
    let permat = Lambertian::new(Arc::new(pertex));
    Sphere::new(center, rad, Arc::new(permat))
}

/// Returns a new sphere with a checker board texture
pub fn build_checker_sphere(center: Point3, rad: f64, even: Color, odd: Color) -> Sphere {
    let even = SolidColor::from(even);
    let odd = SolidColor::from(odd);
    let tex = CheckerTexture::from(Arc::new(even), Arc::new(odd));
    let mat = Lambertian::new(Arc::new(tex));

    Sphere::new(center, rad, Arc::new(mat))
}

/// Returns a new Constant Medium composed of the specified boundary, density and color
pub fn build_constant_medium(
    bound: Arc<dyn Hittable>,
    density: f64,
    color: Color,
) -> ConstantMedium {
    let solid_color = SolidColor::from(color);
    let boundary: Arc<dyn Hittable> = Arc::clone(&bound);
    ConstantMedium::from(boundary, density, Arc::new(solid_color))
}

/// Returns a XZ-Rectangle diffuse light material with the specified Color and coordinates
pub fn build_xz_diff_light(
    light_color: Color,
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64,
) -> XZRect {
    let light_color = SolidColor::from(light_color);
    let diff_light = DiffuseLight::from(Arc::new(light_color));
    XZRect::from(x0, x1, z0, z1, k, Arc::new(diff_light))
}

/// Returns a XY-Rectangle with a diffuse light material with the specified Color and coordinates
pub fn build_xy_diff_light(
    light_color: Color,
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
) -> XYRect {
    let light_color = SolidColor::from(light_color);
    let diff_light = DiffuseLight::from(Arc::new(light_color));
    XYRect::from(x0, x1, y0, y1, k, Arc::new(diff_light))
}

/// Returns a lambertian material with a solid color texture specified by the  `r,g,b` values
pub fn build_solid_lambertian(r: f64, g: f64, b: f64) -> impl Material {
    let solid_color = SolidColor::from_rgb(r, g, b);
    Lambertian::new(Arc::new(solid_color))
}
