
extern crate nalgebra;
use nalgebra::{Vector3};

extern crate rand;
use self::rand::Rng;

use std::io::stdout;
use std::io::Write;

use samplers;
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
    fn color(&self, r: &Ray, sn: usize, ss: &Vec<Vec<Vector3<f64>>>, depth: usize) -> Color {
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

    pub fn render(&self, config: &Config, img: &mut Image) {
        let mut sampler = samplers::new();

        let pixel_samples = if config.sample_root == 1 {
            samplers::u_grid_regular(config.sample_root)
        } else {
            samplers::u_grid_jittered(&mut sampler, config.sample_root)
        };

        let hemi_sample_sets: Vec<Vec<Vec<Vector3<f64>>>> =
            (0..img.width).map(|_|
                (0..config.max_depth).map(|_|
                    samplers::to_hemisphere(
                        samplers::u_grid_jittered(&mut sampler, config.sample_root),
                        0.0)
                    ).collect()
                ).collect();

        let total_pixels = (img.height * img.width) as f64;
        let img_h = img.height as f64;
        let img_w = img.width as f64;
        let mut sample_set_indexes: Vec<usize> = (0..img.width).collect();

        for row in 0..img.height {
            sampler.rng.shuffle(&mut sample_set_indexes);

            for col in 0..img.width {
                let mut color = black();

                for (index, point) in pixel_samples.iter().enumerate() {
                    let u = (col as f64 + point.x) / img_w;
                    let v = ((img.height - 1 - row) as f64 + point.y) / img_h;
                    let r = self.camera.get_ray(u, v);

                    color += self.color(&r, index, &hemi_sample_sets[sample_set_indexes[col]], 0);
                }

                color /= pixel_samples.len() as f64;

                color.r = color.r.sqrt();
                color.g = color.g.sqrt();
                color.b = color.b.sqrt();

                img.set_pixel(col, row, color);
            }

            let progress = 100.0 * (((row + 1) * img.width) as f64) / total_pixels;
            print!("  {} %\r", progress as u32);
            stdout().flush().unwrap();
        }

        println!("");
    }
}
