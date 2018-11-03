
extern crate nalgebra;
use nalgebra::{Point3, Vector3};

use types::*;
use cameras;
use shapes::sphere;
use shapes::plane;
use materials::lambertian;
use materials::emissive;

pub fn lookup_scene(name: &String) -> Option<&Fn(&Config) -> Scene> {
    let scenes: Vec<(String, &Fn(&Config) -> Scene)> = vec![
        (String::from("one"), &build_scene1),
    ];

    scenes.iter()
        .find(|t| t.0.eq(name))
        .map(|t| t.1)
}

fn build_scene1(config: &Config) -> Scene {
    let mut ss = (-10..10).map(|z|
        Box::new(sphere::Sphere {
            center: Vector3::new(0.0, 0.5, z as f64),
            radius: 0.5,
            material: Box::new(lambertian::Lambertian {
                albedo: Color::new(0.7, 0.8, 1.0),
            }),
        }) as Box<Intersectable>).collect();

    let s_light = sphere::Sphere {
        center: Vector3::new(0.0, 23.0, 0.0),
        radius: 15.0,
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
        lens_radius: 0.05,
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
