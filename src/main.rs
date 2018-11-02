
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

    let s_ground = plane::Plane {
        origin: Point3::new(0.0, -0.5, 0.0),
        normal: Vector3::new(0.0, 1.0, 0.0),
        material: Box::new(lambertian::Lambertian {
            albedo: Color::new(0.5, 0.5, 0.5),
        }),
    };

    let cam = cameras::PinholeCamera {
        core: CameraCore::new(
                  Vector3::new(0.0, 1.0, 2.0),
                  Vector3::new(0.0, 0.0, -1.0),
                  Vector3::new(0.0, 1.0, 0.0),
                  ),
        vp_distance: 400.0,
        zoom_factor: 1.0,
    };

    Scene {
        objects: vec![
            Box::new(s_left_front),
            Box::new(s_right_front),
            Box::new(s_left_back),
            Box::new(s_right_back),
            Box::new(s_ground),
            Box::new(s_light1),
            Box::new(s_light2),
        ],
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

    if !config.quiet {
        println!("Rendering...");
    }

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
