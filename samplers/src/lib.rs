
extern crate rand;
#[macro_use] extern crate itertools;

use rand::IsaacRng;
use rand::Rng;
use rand::SeedableRng;

#[derive(Debug)]
pub struct Point2d {
    pub x: f64,
    pub y: f64,
}

pub struct Sampler {
    rng: IsaacRng,
}

pub fn new() -> Sampler {
    let sz = 8;
    let mut trng = rand::thread_rng();
    let seed: Vec<u32> = (0..sz).map(|_| trng.gen()).collect();

    Sampler {
        rng: IsaacRng::from_seed(&seed[0..sz]),
    }
}

pub fn u_grid_regular(root: usize) -> Vec<Point2d> {
    let increment = 1.0 / ((root as f64) + 1.0);
    let range: Vec<f64> = (0..root).map(|i| increment * (i as f64 + 1.0)).collect();

    iproduct!(&range, &range).map(
        |(x, y)| Point2d { x: x.clone(), y: y.clone(), }).collect()
}

pub fn u_grid_random(s: &mut Sampler, num_samples: u32) -> Vec<Point2d> {
    (0..num_samples).map(
        |_| Point2d { x: s.rng.next_f64(), y: s.rng.next_f64(), }).collect()
}

pub fn u_grid_jittered(s: &mut Sampler, root: usize) -> Vec<Point2d> {
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
