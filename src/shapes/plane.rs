
extern crate nalgebra;

use nalgebra::{Vector3, Point3};

use types::*;
use constants::*;

pub struct Plane {
    pub origin: Point3<f64>,
    pub normal: Vector3<f64>,
    pub material: Box<Material>,
}

impl Intersectable for Plane {
    fn hit<'a>(&'a self, r: &Ray) -> Option<Hit<'a>> {
        let t = (self.origin.coords - r.origin).dot(&self.normal) / (r.direction.dot(&self.normal));

        if t > T_MIN {
            Some(Hit {
                point: r.origin + t * r.direction,
                distance: t,
                normal: self.normal,
                material: self.material.as_ref(),
            })
        } else {
            None
        }
    }
}
