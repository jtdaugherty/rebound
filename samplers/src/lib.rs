
extern crate nalgebra;
use nalgebra::{Vector3};

extern crate rand;
use rand::IsaacRng;
use rand::Rng;

#[macro_use] extern crate itertools;

#[derive(Debug)]
pub struct Point2d {
    pub x: f64,
    pub y: f64,
}

pub struct SampleSource {
    rng: IsaacRng,
}

impl SampleSource {
    pub fn next_f64(&mut self) -> f64 {
        self.rng.gen()
    }
}

pub fn new() -> SampleSource {
    let mut trng = rand::thread_rng();

    SampleSource {
        rng: IsaacRng::new_from_u64(trng.gen())
    }
}

pub fn u_grid_regular(root: usize) -> Vec<Point2d> {
    let increment = 1.0 / ((root as f64) + 1.0);
    let range: Vec<f64> = (0..root).map(|i| increment * (i as f64 + 1.0)).collect();

    iproduct!(&range, &range).map(
        |(x, y)| Point2d { x: x.clone(), y: y.clone(), }).collect()
}

pub fn u_grid_random(s: &mut SampleSource, num_samples: u32) -> Vec<Point2d> {
    (0..num_samples).map(
        |_| Point2d { x: s.rng.gen(), y: s.rng.gen(), }).collect()
}

pub fn u_grid_jittered(s: &mut SampleSource, root: usize) -> Vec<Point2d> {
    let increment = 1.0 / ((root as f64) + 1.0);
    let lo = -0.5 * increment;
    let hi = 0.5 * increment;
    let regular = u_grid_regular(root);
    regular.iter().map(
        |p| Point2d {
            x: p.x + s.rng.gen_range(lo, hi),
            y: p.y + s.rng.gen_range(lo, hi),
        }).collect()
}

pub fn u_sphere_random(s: &mut SampleSource) -> Vector3<f64> {
    let mut v = Vector3::new(5.0, 0.0, 0.0);

    while v.dot(&v) >= 1.0 {
        let xr: f64 = s.rng.gen();
        let yr: f64 = s.rng.gen();
        let zr: f64 = s.rng.gen();
        v.x = 2.0 * xr - 1.0;
        v.y = 2.0 * yr - 1.0;
        v.z = 2.0 * zr - 1.0;
    };

    v
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
