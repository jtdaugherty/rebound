
extern crate nalgebra;

use nalgebra::{Vector3};

use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::ops::AddAssign;
use std::ops::DivAssign;
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

pub trait Camera {
    fn get_ray(&self, u: f64, v: f64) -> Ray;
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

        write!(buf, "P3\n{} {}\n255\n", self.width, self.height);
        for row in &self.pixels {
            for pixel in row {
                write!(buf, "{} {} {}\n",
                       (pixel.r * 255.99) as u8,
                       (pixel.g * 255.99) as u8,
                       (pixel.b * 255.99) as u8);
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
