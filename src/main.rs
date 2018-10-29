
use std::fs::File;

extern crate nalgebra;
use nalgebra::{Vector3};

extern crate samplers;
mod types;
mod materials;
mod cameras;
mod shapes;
mod constants;
mod util;
mod scene;
mod args;

use types::*;
use shapes::sphere;
use materials::metal;
use materials::lambertian;
use materials::emissive;

fn build_scene(config: &Config) -> scene::Scene {
    let s_right_front = sphere::Sphere {
        center: Vector3::new(1.2, 0.0, -0.6),
        radius: 0.5,
        material: Box::new(metal::Metal {
            albedo: Color::new(0.3, 0.3, 0.7),
            gloss: 0.3,
        }),
    };
    let s_left_front = sphere::Sphere {
        center: Vector3::new(-1.2, 0.0, -0.6),
        radius: 0.5,
        material: Box::new(metal::Metal {
            albedo: Color::new(0.9, 0.5, 0.5),
            gloss: 0.01,
        }),
    };
    let s_right_back = sphere::Sphere {
        center: Vector3::new(0.6, 0.0, -2.0),
        radius: 0.5,
        material: Box::new(metal::Metal {
            albedo: Color::new(0.4, 0.6, 0.1),
            gloss: 2.0,
        }),
    };
    let s_left_back = sphere::Sphere {
        center: Vector3::new(-0.6, 0.0, -2.0),
        radius: 0.5,
        material: Box::new(metal::Metal {
            albedo: Color::new(0.97, 0.56, 0.26),
            gloss: 0.01,
        }),
    };
    let s_light1 = sphere::Sphere {
        center: Vector3::new(10.0, 12.0, -2.0),
        radius: 5.0,
        material: Box::new(emissive::Emissive {
            color: Color::all(1.0),
        }),
    };
    let s_light2 = sphere::Sphere {
        center: Vector3::new(-10.0, 12.0, -2.0),
        radius: 5.0,
        material: Box::new(emissive::Emissive {
            color: Color::all(1.0),
        }),
    };

    let s_ground = sphere::Sphere {
        center: Vector3::new(0.0, -10000.5, -1.0),
        radius: 10000.0,
        material: Box::new(lambertian::Lambertian {
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

    let s = build_scene(&config);
    let mut img = Image::new(800, 400, black());

    if !config.quiet {
        println!("Rendering...");
    }

    s.render(&config, &mut img);

    if !config.quiet {
        println!("Writing output file.");
    }

    img.write(&mut output_file);

    if !config.quiet {
        println!("Output written to {}", config.output_file);
    }
}
