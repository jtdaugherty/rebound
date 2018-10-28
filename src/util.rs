
extern crate nalgebra;

use nalgebra::{Vector3};

pub fn reflect(v: &Vector3<f64>, n: &Vector3<f64>) -> Vector3<f64> {
    v - 2.0 * v.dot(&n) * n
}

// fn refract(v: &Vector3<f64>, n: &Vector3<f64>, ni_nt: f64) -> Option<Vector3<f64>> {
//     let uv = v.normalize();
//     let dt = uv.dot(n);
//     let desc = 1.0 - ni_nt * ni_nt * (1.0 - dt * dt);
//     if desc > 0.0 {
//         Some(ni_nt * (uv - n * dt) - n * desc.sqrt())
//     } else {
//         None
//     }
// }
