use minigrep::{run, Config};
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    //dbg!(args);

    let config = Config::build(args.into_iter()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    

    println!("Searching for: \"{}\"", config.querry);
    println!("In file: {}", config.file_path);
    if let Err(e) = run(config) {
        eprintln!("Application error:  {}", e);
        process::exit(1);
    }
}
