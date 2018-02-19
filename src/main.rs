extern crate samplers;

extern crate nalgebra;
use nalgebra::{Vector3};

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
}

impl Sphere {
    fn hit(&self, r: &Ray) -> bool {
        let oc = r.origin - self.center;
        let a = r.direction.dot(&r.direction);
        let b = 2.0 * oc.dot(&r.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        discriminant > 0.0
    }
}

fn hit(s: &Sphere, r: &Ray) -> Color {
    if s.hit(r) {
        white()
    } else {
        black()
    }
}

fn main() {
    let s = Sphere {
        center: Vector3::new(0.0, 0.0, -1.0),
        radius: 0.5,
    };

    let mut img = Image::new(200, 100, black());

    let lower_left = Vector3::new(-2.0, -1.0, -1.0);
    let horizontal = Vector3::new(4.0, 0.0, 0.0);
    let vertical = Vector3::new(0.0, 2.0, 0.0);
    let o = Vector3::new(0.0, 0.0, 0.0);

    for row in (0..img.height).rev() {
        for col in 0..img.width {
            let u = (col as f64) / (img.width as f64);
            let v = (row as f64) / (img.height as f64);
            let r = Ray {
                origin: o,
                direction: lower_left + horizontal * u + vertical * v,
            };
            img.set_pixel(col, row, hit(&s, &r));
        }
    }

    img.print();
}
