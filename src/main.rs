use std::{env,process,};
use minigrep::{Config, run};

fn main() {
    let args: Vec<String> = env::args().collect();
    //dbg!(args);

    let config = Config::build(args)
        .unwrap_or_else(|err| {println!("Problem parsing arguments: {err}");
                                        process::exit(1)});

    println!("Searching for: \"{}\"", config.querry);
    println!("In file: {}", config.file_path);
    if let Err(e) = run(config) {
        println!("Application error:  {}", e);
        process::exit(1);
    }
}
