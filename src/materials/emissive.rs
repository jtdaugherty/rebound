
extern crate nalgebra;

use nalgebra::{Vector3};

use types::*;

pub struct Emissive {
    pub color: Color,
}

impl Material for Emissive {
    fn emitted(&self) -> Color {
        self.color
    }

    fn scatter(&self, _r: &Ray, _hit: &Hit, _sv: &Vector3<f64>) -> Option<ScatterResult> {
        None
    }
}
