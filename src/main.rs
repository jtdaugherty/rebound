extern crate samplers;
extern crate nalgebra;
extern crate clap;
extern crate rand;

use nalgebra::{Vector3};
use std::cmp::Ordering;
use clap::{Arg, App};

use std::fs::File;
use std::io::Write;
use std::io::stdout;
use std::io::BufWriter;

use std::ops::DivAssign;
use std::ops::AddAssign;
use std::ops::Mul;
use std::ops::Add;

use rand::Rng;

const T_MIN: f64 = 0.0005;

#[derive(Clone)]
#[derive(Copy)]
struct Color {
    r: f64,
    g: f64,
    b: f64,
}

fn reflect(v: &Vector3<f64>, n: &Vector3<f64>) -> Vector3<f64> {
    v - 2.0 * v.dot(&n) * n
}

// fn refract(v: &Vector3<f64>, n: &Vector3<f64>, ni_nt: f64) -> Option<Vector3<f64>> {
//     let uv = v.normalize();
//     let dt = uv.dot(n);
//     let desc = 1.0 - ni_nt * ni_nt * (1.0 - dt * dt);
//     if desc > 0.0 {
//         Some(ni_nt * (uv - n * dt) - n * desc.sqrt())
//     } else {
//         None
//     }
// }

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

    fn write(&self, f: &mut File) {
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

struct ScatterResult {
    ray: Ray,
    attenuate: Color,
}

trait Material {
    fn scatter(&self, r: &Ray, hit: &Hit, sv: &Vector3<f64>) -> Option<ScatterResult>;
    fn emitted(&self) -> Color;
}

struct Lambertian {
    albedo: Color,
}

impl Material for Lambertian {
    fn emitted(&self) -> Color {
        black()
    }

    fn scatter(&self, _r: &Ray, hit: &Hit, sv: &Vector3<f64>) -> Option<ScatterResult> {
        let target = hit.p + hit.normal + sv;
        Some(ScatterResult {
            ray: Ray {
                origin: hit.p,
                direction: target - hit.p,
            },
            attenuate: self.albedo,
        })
    }
}

// fn schlick(cosine: f64, ref_idx: f64) -> f64 {
//     let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
//     let r1 = r0 * r0;
//     r1 + (1.0 - r1) * (1.0 - cosine).powf(5.0)
// }

// struct Dielectric {
//     ri: f64,
//     reflect_gloss: f64,
//     refract_gloss: f64,
//     color: Color,
// }

// impl Material for Dielectric {
//     fn emitted(&self) -> Color {
//         black()
//     }
// 
//     fn scatter(&self, r: &Ray, hit: &Hit, sn: usize, ss: &Vector3<f64>) -> Option<ScatterResult> {
//         let refl = reflect(&r.direction, &hit.normal);
// 
//         let (outward_normal, ni_nt, cosine) = if r.direction.dot(&hit.normal) > 0.0 {
//             (-1.0 * hit.normal, self.ri,
//              self.ri * r.direction.dot(&hit.normal) / r.direction.norm())
//         } else {
//             (hit.normal, 1.0 / self.ri,
//              -1.0 * r.direction.dot(&hit.normal) / r.direction.norm())
//         };
// 
//         let (gloss, ray_dir) = match refract(&r.direction, &outward_normal, ni_nt) {
//             Some(refracted) => {
//                 let prob = schlick(cosine, self.ri);
//                 if s.next_f64() < prob {
//                     (self.reflect_gloss, refl)
//                 } else {
//                     (self.refract_gloss, refracted)
//                 }
//             },
//             None => (self.reflect_gloss, refl),
//         };
// 
//         let fuzz_vec = gloss * ss[sn];
//         Some(ScatterResult {
//             ray: Ray { origin: hit.p, direction: ray_dir + fuzz_vec },
//             attenuate: self.color,
//         })
//     }
// }

struct Emissive {
    color: Color,
}

impl Material for Emissive {
    fn emitted(&self) -> Color {
        self.color
    }

    fn scatter(&self, _r: &Ray, _hit: &Hit, _sv: &Vector3<f64>) -> Option<ScatterResult> {
        None
    }
}

struct Metal {
    albedo: Color,
    gloss: f64,
}

impl Material for Metal {
    fn emitted(&self) -> Color {
        black()
    }

    fn scatter(&self, r: &Ray, hit: &Hit, sv: &Vector3<f64>) -> Option<ScatterResult> {
        let reflected = reflect(&r.direction, &hit.normal);
        let fuzz_vec = self.gloss * sv;
        let dir = reflected + fuzz_vec;

        Some(ScatterResult {
            ray: Ray {
                origin: hit.p,
                direction: dir,
            },
            attenuate: self.albedo,
        })
    }
}

struct Sphere {
    center: Vector3<f64>,
    radius: f64,
    material: Box<Material>,
}

trait Intersectable {
    fn hit<'a>(&'a self, r: &Ray) -> Option<Hit<'a>>;
}

#[derive(Clone)]
struct Hit<'a> {
    t: f64,
    p: Vector3<f64>,
    normal: Vector3<f64>,
    material: &'a Material,
}

impl<'a> Hit<'a> {
    fn compare(&self, other: &Hit) -> Ordering {
        if self.t.le(&other.t) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

impl Intersectable for Sphere {
    fn hit<'a>(&'a self, r: &Ray) -> Option<Hit<'a>> {
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
                    material: self.material.as_ref(),
                })
            } else {
                let t2 = (-b + (b * b - a * c).sqrt()) / a;
                if t2 > T_MIN {
                    let p = r.point_at_distance(t2);
                    Some(Hit {
                        p, t: t2, normal: (p - self.center) / self.radius,
                        material: self.material.as_ref(),
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
    objects: Vec<Box<Intersectable>>,
    background: Color,
    camera: Box<Camera>,
    config: Config,
}

impl Intersectable for World {
    fn hit<'a>(&'a self, r: &Ray) -> Option<Hit<'a>> {
        let hits: Vec<Hit> = self.objects.iter()
              .filter_map(|o| o.hit(r))
              .collect();

        hits.into_iter().min_by(Hit::compare)
    }
}

impl World {
    fn color(&self, r: &Ray, sn: usize, ss: &Vec<Vec<Vector3<f64>>>, depth: usize) -> Color {
        match self.hit(r) {
            None => self.background,
            Some(h) => {
                let emitted = h.material.emitted();
                if depth < self.config.max_depth {
                    if let Some(sr) = h.material.scatter(r, &h, &ss[depth][sn]) {
                        emitted + self.color(&sr.ray, sn, &ss, depth + 1) * sr.attenuate
                    } else {
                        emitted
                    }
                } else {
                    emitted
                }
            },
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

fn build_scene(config: &Config) -> World {
    let s_right_front = Sphere {
        center: Vector3::new(1.2, 0.0, -0.6),
        radius: 0.5,
        material: Box::new(Metal {
            albedo: Color::new(0.3, 0.3, 0.7),
            gloss: 0.3,
        }),
    };
    // let s_middle_front_1 = Sphere {
    //     center: Vector3::new(0.3, -0.25, -1.0),
    //     radius: 0.25,
    //     material: Box::new(Dielectric {
    //         ri: 1.05,
    //         reflect_gloss: 0.0,
    //         refract_gloss: 0.0,
    //         color: Color::new(0.2588, 0.702, 0.9567),
    //     }),
    // };
    // let s_middle_front_2 = Sphere {
    //     center: Vector3::new(-0.3, -0.25, -1.0),
    //     radius: 0.25,
    //     material: Box::new(Dielectric {
    //         ri: 1.31,
    //         reflect_gloss: 0.1,
    //         refract_gloss: 0.03,
    //         color: Color::new(0.2588, 0.702, 0.9567),
    //     }),
    // };
    let s_left_front = Sphere {
        center: Vector3::new(-1.2, 0.0, -0.6),
        radius: 0.5,
        material: Box::new(Metal {
            albedo: Color::new(0.9, 0.5, 0.5),
            gloss: 0.01,
        }),
    };
    let s_right_back = Sphere {
        center: Vector3::new(0.6, 0.0, -2.0),
        radius: 0.5,
        material: Box::new(Metal {
            albedo: Color::new(0.4, 0.6, 0.1),
            gloss: 2.0,
        }),
    };
    let s_left_back = Sphere {
        center: Vector3::new(-0.6, 0.0, -2.0),
        radius: 0.5,
        material: Box::new(Metal {
            albedo: Color::new(0.97, 0.56, 0.26),
            gloss: 0.01,
        }),
    };
    let s_light1 = Sphere {
        center: Vector3::new(10.0, 12.0, -2.0),
        radius: 5.0,
        material: Box::new(Emissive {
            color: Color::all(1.0),
        }),
    };
    let s_light2 = Sphere {
        center: Vector3::new(-10.0, 12.0, -2.0),
        radius: 5.0,
        material: Box::new(Emissive {
            color: Color::all(1.0),
        }),
    };

    let s_ground = Sphere {
        center: Vector3::new(0.0, -10000.5, -1.0),
        radius: 10000.0,
        material: Box::new(Lambertian {
            albedo: Color::new(0.5, 0.5, 0.5),
        }),
    };

    let cam = SimpleCamera {
        lower_left: Vector3::new(-2.0, -1.0, -4.0),
        horizontal: Vector3::new(4.0, 0.0, 0.0),
        vertical: Vector3::new(0.0, 2.0, 0.0),
        origin: Vector3::new(0.0, 0.0, 3.0),
    };

    World {
        objects: vec![
            Box::new(s_left_front),
            // Box::new(s_middle_front_1),
            // Box::new(s_middle_front_2),
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

#[derive(Debug)]
#[derive(Clone)]
struct Config {
    sample_root: usize,
    quiet: bool,
    max_depth: usize,
    output_file: String,
}

static DEFAULT_OUTPUT_FILENAME: &'static str = "output.ppm";
static DEFAULT_SAMPLE_ROOT: usize = 1;
static DEFAULT_MAX_DEPTH: usize = 3;

impl Config {
    fn new() -> Config {
        let ms = App::new("rebound")
            .version("0.1")
            .author("Jonathan Daugherty")
            .arg(Arg::with_name("quiet")
                 .short("q")
                 .long("quiet")
                 .help("Suppress all console output"))
            .arg(Arg::with_name("sample-root")
                 .short("r")
                 .long("sample-root")
                 .value_name("ROOT")
                 .help("Sample root")
                 .takes_value(true))
            .arg(Arg::with_name("depth")
                 .short("d")
                 .long("depth")
                 .value_name("DEPTH")
                 .help("Maximum recursion depth")
                 .takes_value(true))
            .arg(Arg::with_name("output-file")
                 .short("o")
                 .long("output-file")
                 .value_name("FILENAME")
                 .help("Output filename path")
                 .takes_value(true))
            .get_matches();

        return Config {
            quiet: ms.occurrences_of("quiet") > 0,
            sample_root: match ms.value_of("sample-root") {
                Some(v) => { v.parse().unwrap() },
                None => DEFAULT_SAMPLE_ROOT,
            },
            max_depth: match ms.value_of("depth") {
                Some(v) => { v.parse().unwrap() },
                None => DEFAULT_MAX_DEPTH,
            },
            output_file: match ms.occurrences_of("output-file") {
                0 => String::from(DEFAULT_OUTPUT_FILENAME),
                1 => String::from(ms.value_of("output-file").unwrap()),
                _ => panic!("BUG: output file specified more than once"),
            },
        };
    }

    fn show(&self) {
        println!("Renderer configuration:");
        println!("  Sample root:    {} ({} pixel sample{})",
           self.sample_root, self.sample_root * self.sample_root,
           if self.sample_root == 1 { "" } else { "s" });
        println!("  Maximum depth:  {}", self.max_depth);
        println!("  Output path:    {}", self.output_file);
    }
}

fn main() {
    let config = Config::new();

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
