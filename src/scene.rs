
extern crate nalgebra;

use nalgebra::{Vector3};

use types::*;

pub struct Scene {
    pub objects: Vec<Box<Intersectable>>,
    pub background: Color,
    pub camera: Box<Camera>,
    pub config: Config,
}

impl Intersectable for Scene {
    fn hit<'a>(&'a self, r: &Ray) -> Option<Hit<'a>> {
        let hits: Vec<Hit> = self.objects.iter()
              .filter_map(|o| o.hit(r))
              .collect();

        hits.into_iter().min_by(Hit::compare)
    }
}

impl Scene {
    pub fn color(&self, r: &Ray, sn: usize, ss: &Vec<Vec<Vector3<f64>>>, depth: usize) -> Color {
        match self.hit(r) {
            None => self.background,
            Some(h) => {
                let emitted = h.material.emitted();
                if depth < self.config.max_depth {
                    if let Some(sr) = h.material.scatter(r, &h, &ss[depth][sn]) {
                        emitted + self.color(&sr.ray, sn, &ss, depth + 1) * sr.attenuate
                    } else {
                        emitted
                    }
                } else {
                    emitted
                }
            },
        }
    }
}

