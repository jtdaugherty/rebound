
extern crate nalgebra;
use nalgebra::{Vector3, normalize};

extern crate rand;
use rand::IsaacRng;
use rand::Rng;

extern crate num;
use num::traits::Pow;

#[macro_use] extern crate itertools;

#[derive(Debug)]
pub struct Point2d {
    pub x: f64,
    pub y: f64,
}

pub struct SampleSource {
    pub rng: IsaacRng,
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

// Assumes input samples are all in [0..1]
pub fn to_hemisphere(points: Vec<Point2d>, e: f64) -> Vec<Vector3<f64>> {
    points.iter().map(
        |p| {
            let cos_phi = (2.0 * std::f64::consts::PI * p.x).cos();
            let sin_phi = (2.0 * std::f64::consts::PI * p.x).sin();
            let cos_theta = Pow::pow(1.0 - p.y, 1.0 / (e + 1.0));
            let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
            let pu = sin_theta * cos_phi;
            let pv = sin_theta * sin_phi;
            let pw = cos_theta;
            normalize(&Vector3::new(pu, pv, pw))
        }).collect()
}

// Assumes input samples are all in [0..1]
pub fn to_poisson_disc(points: Vec<Point2d>) -> Vec<Point2d> {
    points.iter().map(
        |p| {
            let spx = 2.0 * p.x - 1.0;
            let spy = 2.0 * p.y - 1.0;
            let mut phi: f64;
            let r: f64;

            if spx > -spy {
                if spx > spy {
                    r = spx;
                    phi = spy / spx;
                } else {
                    r = spy;
                    phi = 2.0 - spx / spy;
                }
            } else {
                if spx < spy {
                    r = -spx;
                    phi = 4.0 + spy / spx;
                } else {
                    r = -spy;
                    if spy != 0.0 {
                        phi = 6.0 - spx / spy;
                    } else {
                        phi = 0.0;
                    }
                }
            }

            phi *= std::f64::consts::PI / 4.0;

            Point2d {
                x: r * phi.cos(),
                y: r * phi.sin(),
            }
        }
        ).collect()
}
