
extern crate clap;
use self::clap::{Arg, App};

use types::Config;
use constants::*;

pub fn config_from_args() -> Config {
    let default_sample_root = DEFAULT_SAMPLE_ROOT.to_string();
    let default_max_depth = DEFAULT_MAX_DEPTH.to_string();

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
             .default_value(default_sample_root.as_str())
             .takes_value(true))
        .arg(Arg::with_name("depth")
             .short("d")
             .long("depth")
             .value_name("DEPTH")
             .help("Maximum recursion depth")
             .default_value(default_max_depth.as_str())
             .takes_value(true))
        .arg(Arg::with_name("scene-name")
             .short("n")
             .long("scene-name")
             .value_name("NAME")
             .help("Scene name")
             .takes_value(true))
        .arg(Arg::with_name("output-file")
             .short("o")
             .long("output-file")
             .value_name("FILENAME")
             .help("Output filename path")
             .default_value(DEFAULT_OUTPUT_FILENAME)
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
        output_file: match ms.value_of("output-file") {
            Some(v) => String::from(v),
            None => panic!("Output file must be specified"),
        },
        scene_name: match ms.value_of("scene-name") {
            Some(v) => String::from(v),
            None => panic!("Scene name must be provided"),
        },
    };
}
