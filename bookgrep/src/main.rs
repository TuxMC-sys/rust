use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    
    let (search_string, rel_path) = parse_env(&args);
    
    let contents = fs::read_to_string(rel_path)
        .expect("Could not read file.");
    
    println!("Searching for {} in {}", search_string, rel_path);
    println!("The file contains:\n{}", contents)
}
fn parse_env(args: &[String]) -> (&str, &str){
    let search_string = &args[1];
    let rel_path = &args[2];

    (search_string, rel_path)
}