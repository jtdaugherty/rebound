
extern crate nalgebra;

use nalgebra::{Vector3};

use types::*;
use util;

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
