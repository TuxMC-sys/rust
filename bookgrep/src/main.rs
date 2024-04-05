use std::{env, fs, process};

struct Input {
    search_string: String,
    rel_path: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Input::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    println!(
        "Searching for {} in {}",
        config.search_string, config.rel_path
    );
    let contents = fs::read_to_string(config.rel_path).expect("Could not read file.");
    println!("The file contains:\n{}", contents)
}

impl Input {
    fn build(args: &[String]) -> Result<Input, &'static str> {
        if args.len() < 3 {
            return Err("You must specify the search string and the file to search in.");
        }
        let search_string = args[1].clone();
        let rel_path = args[2].clone();
        Ok(Input {
            search_string,
            rel_path,
        })
    }
}
