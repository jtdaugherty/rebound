extern crate samplers;

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

fn main() {
    let w = 40;
    let h = 40;
    let mut img = Image::new(w, h, black());

    for i in 0..w {
        img.set_pixel(i, i, white());
    }

    img.print();
}
