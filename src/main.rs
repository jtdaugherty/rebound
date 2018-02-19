extern crate samplers;

extern crate nalgebra;
use nalgebra::{Vector3};
use std::cmp::Ordering;

const T_MIN: f64 = 0.0005;

#[derive(Clone)]
struct Color {
    r: f64,
    g: f64,
    b: f64,
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
fn white() -> Color { Color::all(1.0) }

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
                         pixel.r * 255.0,
                         pixel.g * 255.0,
                         pixel.b * 255.0);
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
    fn hit(&self, r: &Ray) -> Option<Hit>;
}

#[derive(Clone)]
struct Hit {
    t: f64,
    p: Vector3<f64>,
    normal: Vector3<f64>,
    color: Color,
}

impl Intersectable for Sphere {
    fn hit(&self, r: &Ray) -> Option<Hit> {
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
                    color: self.color.clone(),
                })
            } else {
                let t2 = (-b + (b * b - a * c).sqrt()) / a;
                if t2 > T_MIN {
                    let p = r.point_at_distance(t2);
                    Some(Hit {
                        p, t: t2, normal: (p - self.center) / self.radius,
                        color: self.color.clone(),
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
}

impl Intersectable for World {
    fn hit(&self, r: &Ray) -> Option<Hit> {
        let mut hits: Vec<Hit> = self.objects.iter()
              .map(|o| o.hit(r))
              .filter(|h| h.is_some())
              .map(|h| h.unwrap())
              .collect();

        if !hits.is_empty() {
            hits.sort_by(|a, b| if a.t.le(&b.t) { Ordering::Less } else { Ordering::Greater });
            Some(hits[0].clone())
        } else {
            None
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

    let mut img = Image::new(400, 200, black());

    let lower_left = Vector3::new(-2.0, -1.0, -1.0);
    let horizontal = Vector3::new(4.0, 0.0, 0.0);
    let vertical = Vector3::new(0.0, 2.0, 0.0);
    let o = Vector3::new(0.0, 0.0, 0.0);

    let w = World {
        objects: vec![s1, s2],
    };

    for row in 0..img.height {
        for col in 0..img.width {
            let u = (col as f64) / (img.width as f64);
            let v = ((img.height - 1 - row) as f64) / (img.height as f64);
            let r = Ray {
                origin: o,
                direction: lower_left + horizontal * u + vertical * v,
            };

            match w.hit(&r) {
                None => (),
                Some(h) => img.set_pixel(col, row, h.color),
            }
        }
    }

    img.print();
}
