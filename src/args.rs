
extern crate clap;
use self::clap::{Arg, App};

use types::Config;
use constants::*;

pub fn config_from_args() -> Config {
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
