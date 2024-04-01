use std::fs;
use std::str;
use serde_json::Value;
const TODO_FILE: &str = "../../Documents/rust_todo.json";
fn main(){
    println!("This program writes to ~/Documents/rust_todo.json");
    //println!("{:?}", init_file().iter().map(|x|*x as char).collect::<Vec<char>>());
    println!("{:?}", parse_json());
}
fn parse_json() -> Value{
    serde_json::from_str(&init_file()).unwrap()
}
fn init_file() -> String{
    str::from_utf8(&fs::read(TODO_FILE).expect("File not found")).unwrap().to_string()
}