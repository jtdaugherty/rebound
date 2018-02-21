extern crate samplers;

extern crate nalgebra;
use nalgebra::{Vector3};
use std::cmp::Ordering;

use std::ops::DivAssign;
use std::ops::AddAssign;
use std::ops::Mul;
use std::ops::Add;

const T_MIN: f64 = 0.0005;

#[derive(Clone)]
#[derive(Copy)]
struct Color {
    r: f64,
    g: f64,
    b: f64,
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
    fn new(r: f64, g: f64, b: f64) -> Color {
        Color { r, g, b }
    }

    fn all(v: f64) -> Color {
       Color::new(v, v, v)
    }
}

fn black() -> Color { Color::all(0.0) }

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

struct Image {
    height: usize,
    width: usize,
    pixels: Vec<Vec<Color>>,
}

impl Image {
    fn new(w: usize, h: usize, initial_color: Color) -> Image {
        Image {
            pixels: (0..h).map(|_| (0..w).map(|_| initial_color.clone()).collect()).collect(),
            width: w,
            height: h,
        }
    }

    fn set_pixel(&mut self, w: usize, h: usize, val: Color) {
        self.pixels[h][w] = val;
    }

    fn print(&self) {
        println!("P3\n{} {}\n255", self.width, self.height);
        for row in &self.pixels {
            for pixel in row {
                println!("{} {} {}",
                         (pixel.r * 255.0) as u8,
                         (pixel.g * 255.0) as u8,
                         (pixel.b * 255.0) as u8);
            }
        }
    }
}

#[derive(Debug)]
struct Ray {
    origin: Vector3<f64>,
    direction: Vector3<f64>,
}

impl Ray {
    fn point_at_distance(&self, t: f64) -> Vector3<f64> {
        self.origin + (self.direction * t)
    }
}

struct Sphere {
    center: Vector3<f64>,
    radius: f64,
    color: Color,
}

trait Intersectable {
    fn hit(&self, r: &Ray, s: &mut samplers::Sampler) -> Option<Hit>;
}

#[derive(Clone)]
struct Hit {
    t: f64,
    p: Vector3<f64>,
    normal: Vector3<f64>,
    color: Color,
}

impl Hit {
    fn compare(&self, other: &Hit) -> Ordering {
        if self.t.le(&other.t) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

impl Intersectable for Sphere {
    fn hit(&self, r: &Ray, _: &mut samplers::Sampler) -> Option<Hit> {
        let oc = r.origin - self.center;
        let a = r.direction.dot(&r.direction);
        let b = 2.0 * oc.dot(&r.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant > 0.0 {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);

            if t1 > T_MIN {
                let p = r.point_at_distance(t1);
                Some(Hit {
                    p, t: t1, normal: (p - self.center) / self.radius,
                    color: self.color,
                })
            } else {
                let t2 = (-b + (b * b - a * c).sqrt()) / a;
                if t2 > T_MIN {
                    let p = r.point_at_distance(t2);
                    Some(Hit {
                        p, t: t2, normal: (p - self.center) / self.radius,
                        color: self.color,
                    })
                } else {
                    None
                }
            }
        } else {
            None
        }
    }
}

struct World {
    objects: Vec<Sphere>,
    background: Color,
}

impl Intersectable for World {
    fn hit(&self, r: &Ray, s: &mut samplers::Sampler) -> Option<Hit> {
        let hits: Vec<Hit> = self.objects.iter()
              .filter_map(|o| o.hit(r, s))
              .collect();

        if let Some(h) = hits.into_iter().min_by(Hit::compare) {
            let target = h.p + h.normal + samplers::u_sphere_random(s);

            let shadow_ray = Ray {
                origin: h.p,
                direction: target - h.p,
            };

            let new_color = match self.hit(&shadow_ray, s) {
                None => h.color,
                Some(_) => black(),
            };

            Some(Hit { color: new_color, ..h })
        } else {
            None
        }
    }
}

struct SimpleCamera {
    lower_left: Vector3<f64>,
    horizontal: Vector3<f64>,
    vertical: Vector3<f64>,
    origin: Vector3<f64>,
}

trait Camera {
    fn get_ray(&self, u: f64, v: f64) -> Ray;
}

impl Camera for SimpleCamera {
    fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left + self.horizontal * u + self.vertical * v,
        }
    }
}

fn main() {
    let s1 = Sphere {
        center: Vector3::new(0.0, 0.0, -1.0),
        radius: 0.5,
        color: Color::new(1.0, 0.0, 0.0),
    };
    let s2 = Sphere {
        center: Vector3::new(0.0, -100.5, -1.0),
        radius: 100.0,
        color: Color::new(0.0, 0.0, 1.0),
    };
    let w = World {
        objects: vec![s1, s2],
        background: Color::new(0.2, 0.3, 0.5),
    };

    let mut img = Image::new(400, 200, black());
    let cam = SimpleCamera {
        lower_left: Vector3::new(-2.0, -1.0, -1.0),
        horizontal: Vector3::new(4.0, 0.0, 0.0),
        vertical: Vector3::new(0.0, 2.0, 0.0),
        origin: Vector3::new(0.0, 0.0, 0.0),
    };

    let mut sampler = samplers::new();
    let samples = samplers::u_grid_regular(2);

    for row in 0..img.height {
        for col in 0..img.width {
            let mut color = black();

            for point in &samples {
                let u = (col as f64 + point.x) / (img.width as f64);
                let v = ((img.height - 1 - row) as f64 + point.y) / (img.height as f64);
                let r = cam.get_ray(u, v);

                color += match w.hit(&r, &mut sampler) {
                    None => w.background,
                    Some(h) => h.color,
                };
            }

            color /= samples.len() as f64;
            img.set_pixel(col, row, color);
        }
    }

    img.print();
}
