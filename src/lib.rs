use std::{error::Error, fs};

pub struct Config {
    query: String,
    file_path: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, String> {
        if !(args.len() == 3) {
            let given_args: usize = args.len() - 1;
            return Err(format!("Require exactly 2 args but given {given_args}"));
        }

        let query: String = args.get(1).unwrap().clone();
        let file_path: String = args.get(2).unwrap().clone();

        let returned_config = Config { query, file_path };

        Ok(returned_config)
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    println!("The content of the file is, {}", contents);
    Ok(())
}

pub fn search<'grep_res>(query: &str, contents: &'grep_res str) -> Vec<&'grep_res str> {
    let mut results: Vec<&'grep_res str> = Vec::new();

    for line in contents.lines() {
        if line.contains(query){
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'grep_res>(
    query: &str,
    contents: &'grep_res str,
) -> Vec<&'grep_res str> {
    let mut results: Vec<&'grep_res str> = Vec::new();
    let query = query.to_lowercase();

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
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
