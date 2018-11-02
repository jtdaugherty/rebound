
extern crate nalgebra;

use nalgebra::{Vector3};

use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::ops::AddAssign;
use std::ops::DivAssign;
use std::ops::MulAssign;
use std::ops::Mul;
use std::ops::Add;
use std::cmp::Ordering;

#[derive(Clone)]
pub struct Hit<'a> {
    pub distance: f64,
    pub point: Vector3<f64>,
    pub normal: Vector3<f64>,
    pub material: &'a Material,
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Config {
    pub sample_root: usize,
    pub quiet: bool,
    pub max_depth: usize,
    pub output_file: String,
}

#[derive(Clone)]
#[derive(Copy)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

pub struct Image {
    pub height: usize,
    pub width: usize,
    pixels: Vec<Vec<Color>>,
}

#[derive(Debug)]
pub struct Ray {
    pub origin: Vector3<f64>,
    pub direction: Vector3<f64>,
}

pub struct ViewPlane {
    pub hres: usize,
    pub vres: usize,
    pub pixel_size: f64,
}

pub struct ScatterResult {
    pub ray: Ray,
    pub attenuate: Color,
}

pub trait Material {
    fn scatter(&self, r: &Ray, hit: &Hit, sv: &Vector3<f64>) -> Option<ScatterResult>;
    fn emitted(&self) -> Color;
}

pub trait Intersectable {
    fn hit<'a>(&'a self, r: &Ray) -> Option<Hit<'a>>;
}

pub struct Scene {
    pub objects: Vec<Box<Intersectable>>,
    pub background: Color,
    pub camera: Box<Camera>,
    pub config: Config,
    pub view_plane: ViewPlane,
}

pub struct CameraCore {
    pub eye: Vector3<f64>,
    pub look_at: Vector3<f64>,
    pub up: Vector3<f64>,
    pub u: Vector3<f64>,
    pub v: Vector3<f64>,
    pub w: Vector3<f64>,
}

impl CameraCore {
    pub fn new(eye: Vector3<f64>, look_at: Vector3<f64>, up: Vector3<f64>) -> CameraCore {
        let mut core = CameraCore {
            eye, look_at, up,
            u: Vector3::new(0.0, 0.0, 0.0),
            v: Vector3::new(0.0, 0.0, 0.0),
            w: Vector3::new(0.0, 0.0, 0.0),
        };
        core.compute_uvw();
        core
    }

    pub fn compute_uvw(&mut self) {
        self.w = (self.eye - self.look_at).normalize();
        self.u = self.up.cross(&self.w).normalize();
        self.v = self.w.cross(&self.u);
    }
}

pub trait Camera {
    fn render(&self, scene: &Scene) -> Image;
}

impl Image {
    pub fn new(w: usize, h: usize, initial_color: Color) -> Image {
        Image {
            pixels: (0..h).map(|_| (0..w).map(|_| initial_color.clone()).collect()).collect(),
            width: w,
            height: h,
        }
    }

    pub fn set_pixel(&mut self, w: usize, h: usize, val: Color) {
        self.pixels[h][w] = val;
    }

    pub fn write(&self, f: &mut File) {
        let mut buf = BufWriter::new(f);

        write!(buf, "P3\n{} {}\n65535\n", self.width, self.height);
        for row in &self.pixels {
            for pixel in row {
                write!(buf, "{} {} {}\n",
                       (pixel.r * 65535.99) as u16,
                       (pixel.g * 65535.99) as u16,
                       (pixel.b * 65535.99) as u16);
            }
        }
    }
}

impl DivAssign<f64> for Color {
    fn div_assign(&mut self, d: f64) {
        self.r /= d;
        self.g /= d;
        self.b /= d;
    }
}

impl MulAssign<f64> for Color {
    fn mul_assign(&mut self, d: f64) {
        self.r *= d;
        self.g *= d;
        self.b *= d;
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, other: Color) {
        self.r += other.r;
        self.g += other.g;
        self.b += other.b;
    }
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color { r, g, b }
    }

    pub fn all(v: f64) -> Color {
       Color::new(v, v, v)
    }

    pub fn max_to_one(&mut self) -> () {
        let mx1 = if self.r > self.g { self.r } else { self.g };
        let mx2 = if mx1 > self.b { mx1 } else { self.b };
        if mx2 > 1.0 {
            let i = 1.0 / mx2;
            self.r *= i;
            self.g *= i;
            self.b *= i;
        }
    }
}

pub fn black() -> Color { Color::all(0.0) }

impl Mul<Color> for Color {
    type Output = Self;

    fn mul(self, other: Color) -> Color {
        Color {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, other: f64) -> Color {
        Color {
            r: self.r * other,
            g: self.g * other,
            b: self.b * other,
        }
    }
}

impl Add<Color> for Color {
    type Output = Self;

    fn add(self, other: Color) -> Color {
        Color {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl Ray {
    pub fn point_at_distance(&self, t: f64) -> Vector3<f64> {
        self.origin + (self.direction * t)
    }
}

impl<'a> Hit<'a> {
    pub fn compare(&self, other: &Hit) -> Ordering {
        if self.distance.le(&other.distance) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

impl Config {
    pub fn show(&self) {
        println!("Renderer configuration:");
        println!("  Sample root:    {} ({} pixel sample{})",
           self.sample_root, self.sample_root * self.sample_root,
           if self.sample_root == 1 { "" } else { "s" });
        println!("  Maximum depth:  {}", self.max_depth);
        println!("  Output path:    {}", self.output_file);
    }
}
