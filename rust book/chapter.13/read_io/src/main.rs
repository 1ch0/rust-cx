use std::{env, process};

use read_io::run;
use read_io::Config;

fn main() {
    // let args: Vec<String> = env::args().collect();
    // let query = &args[1];
    // let filename = &args[2];
    // let query = "the";
    // let filename = "poem.txt";

    // let (query, filename) = parse_config(&args);
    //
    // println!("Searching for {}", query);
    // println!("In file {}", filename);
    //
    // let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    // let config = parse_config(&args);

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
