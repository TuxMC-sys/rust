use std::env;
use std::error::Error;
use std::fs;
pub struct Input {
    pub rel_path: String,
    pub search_string: String,
    pub ignore_case: bool,
}

impl Input {
    pub fn build(args: &[String]) -> Result<Input, &'static str> {
        if args.len() < 3 {
            return Err("You must specify the search string and the file to search in.");
        }
        let rel_path = args[1].clone();
        let search_string = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Input {
            search_string,
            rel_path,
            ignore_case
        })
    }
}
pub fn run(config: Input) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.rel_path)?;
    let results = if config.ignore_case {
        search_case_insensitive(&contents, &config.search_string)
    } else {
        search(&contents, &config.search_string)
    };
    for line in results {
        println!("{line}");
    }

    Ok(())
}
pub fn search<'a>(contents: &'a str, query: &str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines(){
        if line.contains(query){
            results.push(line);
        }
    }
    results
}
pub fn search_case_insensitive<'a>(
    contents: &'a str,
    query: &str
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}
/*#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(contents, query));
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
            search_case_insensitive(contents, query)
        );
    }
}*/