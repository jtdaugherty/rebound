
extern crate nalgebra;

use nalgebra::{Vector3};

use types::*;
use constants::*;

pub struct Sphere {
    pub center: Vector3<f64>,
    pub radius: f64,
    pub material: Box<Material>,
}

impl Intersectable for Sphere {
    fn hit<'a>(&'a self, r: &Ray) -> Option<Hit<'a>> {
        let oc = r.origin - self.center;
        let a = r.direction.dot(&r.direction);
        let b = 2.0 * oc.dot(&r.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant > 0.0 {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);

            if t1 > T_MIN {
                let p = r.point_at_distance(t1);
                Some(Hit {
                    point: p,
                    distance: t1,
                    normal: (p - self.center) / self.radius,
                    material: self.material.as_ref(),
                })
            } else {
                let t2 = (-b + (b * b - a * c).sqrt()) / a;
                if t2 > T_MIN {
                    let p = r.point_at_distance(t2);
                    Some(Hit {
                        point: p,
                        distance: t2,
                        normal: (p - self.center) / self.radius,
                        material: self.material.as_ref(),
                    })
                } else {
                    None
                }
            }
        } else {
            None
        }
    }
}
