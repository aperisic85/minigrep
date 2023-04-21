use std::{env, error::Error, fs};

pub struct Config {
    pub querry: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>,) -> Result<Config, &'static str> {
        
        args.next(); // skkip first element
        let querry = match args.next() {
            Some(arg) => arg,
            None => return Err("Didnt get querry string"),
            
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didnt get file path"),
            
        };
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config {
            querry,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(c: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(c.file_path)?;

    let results = if c.ignore_case {
        search_case_insensitive(&c.querry, &contents)
    } else {
        search(&c.querry, &contents)
    };

    for line in results {
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

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
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
    fn case_insensitive() {
        let querry = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(querry, contents)
        )
    }
}
