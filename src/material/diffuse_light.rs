use std::sync::Arc;
use crate::texture::Texture;
use crate::material::{Material, ScatterRecord};
use crate::common::{Ray, Point3, Color};
use crate::hittable::HitRecord;

/// Models a diffuse light source that can emit light of a specific `Color`
#[derive(Debug)]
pub struct DiffuseLight {
    emit: Arc<dyn Texture>,
}

impl DiffuseLight {

    pub fn from(tex_ptr: Arc<dyn Texture>) -> Self {
        Self {
            emit: tex_ptr,
        }
    }
}

impl Material for DiffuseLight {

    /// this default implementation of diffuse light does not scatter.
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<ScatterRecord> {
        None
    }

    /// This default implementation of `emitted` call's the textures `value` function with
    /// the given `u,v` coordinates at point `p`
    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color {
        self.emit.value(u, v, p)
    }
}