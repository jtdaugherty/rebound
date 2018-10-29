
extern crate nalgebra;

use nalgebra::{Vector3};

use types::*;
use util;

pub struct Metal {
    pub albedo: Color,
    pub gloss: f64,
}

impl Material for Metal {
    fn emitted(&self) -> Color {
        black()
    }

    fn scatter(&self, r: &Ray, hit: &Hit, sv: &Vector3<f64>) -> Option<ScatterResult> {
        let reflected = util::reflect(&r.direction, &hit.normal);
        let fuzz_vec = self.gloss * sv;
        let dir = reflected + fuzz_vec;

        Some(ScatterResult {
            ray: Ray {
                origin: hit.point,
                direction: dir,
            },
            attenuate: self.albedo,
        })
    }
}
