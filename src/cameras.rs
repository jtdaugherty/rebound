
extern crate nalgebra;
use nalgebra::{Vector3};

extern crate rand;
use self::rand::Rng;

extern crate samplers;
use types::*;

use std::io::stdout;
use std::io::Write;

pub struct PinholeCamera {
    pub core: CameraCore,
    pub vp_distance: f64,
    pub zoom_factor: f64,
}

impl PinholeCamera {
    fn ray_direction(&self, x: f64, y: f64) -> Vector3<f64> {
        (x * self.core.u + y * self.core.v - self.vp_distance * self.core.w).normalize()
    }
}

impl Camera for PinholeCamera {
    fn render(&self, scene: &Scene) -> Image {
        let mut img = Image::new(scene.view_plane.hres, scene.view_plane.vres, black());
        let mut sampler = samplers::new();

        let pixel_sample_sets: Vec<Vec<samplers::Point2d>> =
            if scene.config.sample_root == 1 {
                vec!(samplers::u_grid_regular(scene.config.sample_root))
            } else {
                (0..img.width).map(|_|
                    samplers::u_grid_jittered(&mut sampler, scene.config.sample_root)).collect()
            };

        let hemi_sample_sets: Vec<Vec<Vec<Vector3<f64>>>> =
            (0..img.width).map(|_|
                (0..scene.config.max_depth).map(|_|
                    samplers::to_hemisphere(
                        samplers::u_grid_jittered(&mut sampler, scene.config.sample_root),
                        0.0)
                    ).collect()
                ).collect();

        let total_pixels = (img.height * img.width) as f64;
        let half_img_h = img.height as f64 * 0.5;
        let half_img_w = img.width as f64 * 0.5;
        let mut sample_set_indexes: Vec<usize> = (0..img.width).collect();
        let pixel_denom = 1.0 / ((scene.config.sample_root * scene.config.sample_root) as f64);
        let adjusted_pixel_size = scene.view_plane.pixel_size / self.zoom_factor;

        for row in 0..img.height {
            sampler.rng.shuffle(&mut sample_set_indexes);

            for col in 0..img.width {
                let mut color = black();
                let pixel_samples = &pixel_sample_sets[sample_set_indexes[col] % pixel_sample_sets.len()];

                for (index, point) in pixel_samples.iter().enumerate() {
                    let u = adjusted_pixel_size * (col as f64 - half_img_w + point.x);
                    let v = adjusted_pixel_size * ((img.height - row) as f64 - half_img_h + point.y);
                    let r = Ray {
                        direction: self.ray_direction(u, v),
                        origin: self.core.eye,
                    };

                    color += scene.color(&r, index, &hemi_sample_sets[sample_set_indexes[col]], 0);
                }

                color *= pixel_denom;
                color.max_to_one();

                img.set_pixel(col, row, color);
            }

            let progress = 100.0 * (((row + 1) * img.width) as f64) / total_pixels;
            print!("  {} %\r", progress as u32);
            stdout().flush().unwrap();
        }

        println!("");

        img
    }
}
