
extern crate nalgebra;

use nalgebra::{Vector3};

use types::*;

pub struct SimpleCamera {
    pub lower_left: Vector3<f64>,
    pub horizontal: Vector3<f64>,
    pub vertical: Vector3<f64>,
    pub origin: Vector3<f64>,
}

impl Camera for SimpleCamera {
    fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left + self.horizontal * u + self.vertical * v,
        }
    }
}
