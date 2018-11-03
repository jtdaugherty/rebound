
use std::fs::File;

extern crate nalgebra;
use nalgebra::{Point3, Vector3};

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
use shapes::plane;
use materials::metal;
use materials::lambertian;
use materials::emissive;

fn build_scene(config: &Config) -> Scene {
    let mut ss = (-10..10).map(|z|
        Box::new(sphere::Sphere {
            center: Vector3::new(0.0, 0.5, z as f64),
            radius: 0.5,
            material: Box::new(lambertian::Lambertian {
                albedo: Color::new(0.7, 0.8, 1.0),
            }),
        }) as Box<Intersectable>).collect();

    let s_light = sphere::Sphere {
        center: Vector3::new(10.0, 12.0, -2.0),
        radius: 5.0,
        material: Box::new(emissive::Emissive {
            color: Color::all(1.0),
        }),
    };

    let s_ground = plane::Plane {
        origin: Point3::new(0.0, 0.0, 0.0),
        normal: Vector3::new(0.0, 1.0, 0.0),
        material: Box::new(lambertian::Lambertian {
            albedo: Color::new(0.5, 0.5, 0.5),
        }),
    };

    let cam = cameras::ThinLensCamera {
        core: CameraCore::new(
                  Vector3::new(2.0, 0.5, 10.0),
                  Vector3::new(0.0, 0.0, -1.0),
                  Vector3::new(0.0, 1.0, 0.0),
                  ),
        vp_distance: 400.0,
        zoom_factor: 1.0,
        focal_plane_distance: 4.0,
        lens_radius: 0.1,
    };

    let mut all_objects: Vec<Box<Intersectable>> = vec![
        Box::new(s_ground),
        Box::new(s_light),
    ];

    all_objects.append(&mut ss);

    Scene {
        objects: all_objects,
        background: Color::all(0.5),
        camera: Box::new(cam),
        config: config.clone(),
        view_plane: ViewPlane {
            hres: 800,
            vres: 400,
            pixel_size: 1.0,
        },
    }
}

fn main() {
    let config = args::config_from_args();

    if !config.quiet {
        config.show();
    }

    let s = build_scene(&config);
    let img = s.camera.render(&s);

    if !config.quiet {
        println!("Writing output file.");
    }

    let mut output_file = File::create(config.output_file.clone()).unwrap();
    img.write(&mut output_file);

    if !config.quiet {
        println!("Output written to {}", config.output_file);
    }
}
