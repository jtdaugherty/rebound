
use std::fs::File;
use std::io::Write;
use std::io::stdout;

extern crate nalgebra;
use nalgebra::{Vector3};

extern crate rand;
use rand::Rng;

extern crate samplers;
mod types;
mod materials;
mod cameras;
mod sphere;
mod constants;
mod util;
mod scene;
mod args;

use types::*;

fn build_scene(config: &Config) -> scene::Scene {
    let s_right_front = sphere::Sphere {
        center: Vector3::new(1.2, 0.0, -0.6),
        radius: 0.5,
        material: Box::new(materials::Metal {
            albedo: Color::new(0.3, 0.3, 0.7),
            gloss: 0.3,
        }),
    };
    let s_left_front = sphere::Sphere {
        center: Vector3::new(-1.2, 0.0, -0.6),
        radius: 0.5,
        material: Box::new(materials::Metal {
            albedo: Color::new(0.9, 0.5, 0.5),
            gloss: 0.01,
        }),
    };
    let s_right_back = sphere::Sphere {
        center: Vector3::new(0.6, 0.0, -2.0),
        radius: 0.5,
        material: Box::new(materials::Metal {
            albedo: Color::new(0.4, 0.6, 0.1),
            gloss: 2.0,
        }),
    };
    let s_left_back = sphere::Sphere {
        center: Vector3::new(-0.6, 0.0, -2.0),
        radius: 0.5,
        material: Box::new(materials::Metal {
            albedo: Color::new(0.97, 0.56, 0.26),
            gloss: 0.01,
        }),
    };
    let s_light1 = sphere::Sphere {
        center: Vector3::new(10.0, 12.0, -2.0),
        radius: 5.0,
        material: Box::new(materials::Emissive {
            color: Color::all(1.0),
        }),
    };
    let s_light2 = sphere::Sphere {
        center: Vector3::new(-10.0, 12.0, -2.0),
        radius: 5.0,
        material: Box::new(materials::Emissive {
            color: Color::all(1.0),
        }),
    };

    let s_ground = sphere::Sphere {
        center: Vector3::new(0.0, -10000.5, -1.0),
        radius: 10000.0,
        material: Box::new(materials::Lambertian {
            albedo: Color::new(0.5, 0.5, 0.5),
        }),
    };

    let cam = cameras::SimpleCamera {
        lower_left: Vector3::new(-2.0, -1.0, -4.0),
        horizontal: Vector3::new(4.0, 0.0, 0.0),
        vertical: Vector3::new(0.0, 2.0, 0.0),
        origin: Vector3::new(0.0, 0.0, 3.0),
    };

    scene::Scene {
        objects: vec![
            Box::new(s_left_front),
            Box::new(s_right_front),
            Box::new(s_left_back),
            Box::new(s_right_back),
            Box::new(s_ground),
            Box::new(s_light1),
            Box::new(s_light2),
        ],
        background: Color::all(0.1),
        camera: Box::new(cam),
        config: config.clone(),
    }
}

fn main() {
    let config = args::config_from_args();

    if !config.quiet {
        config.show();
    }

    let mut output_file = File::create(config.output_file.clone()).unwrap();

    let w = build_scene(&config);
    let mut img = Image::new(800, 400, black());
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

    if !config.quiet {
        println!("Rendering...");
    }

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
                let r = w.camera.get_ray(u, v);

                color += w.color(&r, index, &hemi_sample_sets[sample_set_indexes[col]], 0);
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

    if !config.quiet {
        println!("Writing output file.");
    }

    img.write(&mut output_file);

    if !config.quiet {
        println!("Output written to {}", config.output_file);
    }
}
