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

    for line in search(&c.querry, &contents) {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(querry: &str, contents: &'a str) -> Vec<&'a str> {
    let mut res = Vec::new();

    for line in contents.lines() {
        if line.contains(querry) {
            res.push(line.trim());
        }
    }
    res
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str,) -> Vec<&'a str>{
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query){
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let querry = "duct";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(querry, contents));
    }

    #[test]
    fn case_insensitive(){
        let querry = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(querry, contents))
    }
}
