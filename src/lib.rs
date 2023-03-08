use std::{error::Error, fs};

pub struct Config {
    pub querry: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments !");
        }

        let querry = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config { querry, file_path })
    }
}

pub fn run(c: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(c.file_path)?;
    println!("With text:\n{}", contents);

    Ok(())
}
