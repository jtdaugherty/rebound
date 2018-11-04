
use std::fs::File;

extern crate nalgebra;
extern crate rayon;
extern crate samplers;

mod types;
mod materials;
mod cameras;
mod shapes;
mod constants;
mod util;
mod scene;
mod scenes;
mod args;

fn main() {
    let config = args::config_from_args();

    if !config.quiet {
        config.show();
    }

    let build_scene = scenes::lookup_scene(&config.scene_name).expect("Invalid scene name");

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
