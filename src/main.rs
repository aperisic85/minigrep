use std::{env, fs, process, error::Error};

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

struct Config {
    querry: String,
    file_path: String,
}

impl Config{
    fn build (args: Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments !");
        }
        
        let querry =  args[1].clone();
        let file_path=  args[2].clone();
        Ok(Config { querry, file_path })
    }
}

fn run(c: Config) -> Result<(), Box<dyn Error>>{
    
    let contents = fs::read_to_string(c.file_path)?;
    println!("With text:\n{}", contents);

    Ok(())
}