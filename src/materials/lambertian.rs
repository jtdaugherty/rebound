
extern crate nalgebra;

use nalgebra::{Vector3};

use types::*;

pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn emitted(&self) -> Color {
        black()
    }

    fn scatter(&self, _r: &Ray, hit: &Hit, sv: &Vector3<f64>) -> Option<ScatterResult> {
        let target = hit.point + hit.normal + sv;
        Some(ScatterResult {
            ray: Ray {
                origin: hit.point,
                direction: target - hit.point,
            },
            attenuate: self.albedo,
        })
    }
}
