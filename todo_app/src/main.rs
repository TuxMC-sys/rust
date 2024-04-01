use std::fs;
use std::str;
use std::io;
use serde::{Deserialize, Serialize};
use std::io::Write; 
const TODO_FILE: &str = "../../Documents/rust_todo.json";

#[derive(Serialize, Deserialize)]
struct Todo {
    todo_list: Vec<String>,
}


fn main(){
    println!("This program reads/writes from/to ~/Documents/rust_todo.json\n");
    let mut todos = parse_json().todo_list;
    remove_todos(&mut todos);
}
fn remove_todos(todos: &mut Vec<String>){
    let mut remove_numchar: String = "".to_string();
    let print_flush = |x|{print!("{}",x); io::stdout().flush().unwrap();};
    println!("If you want to remove a todo from the list, please input the number of the todo.\n");
    print_current_todo(&todos);
    print_flush("\nEnter the the number of the todo item to remove or enter 0 if you don't want to remove anything: ");
    io::stdin().read_line(&mut remove_numchar).expect("Failed to read line"); 
    remove_numchar.truncate(1);
    if remove_numchar != "0" && todos.len() != 0{
        todos.remove(remove_numchar.trim().parse::<usize>().expect("Input not an integer")-1);
        if todos.len()>=1{
            remove_todos(todos);
        }else{
            println!("You have no more todos!");
        }
    }
}
fn print_current_todo(todos: &Vec<String>){
    println!("Your current todo's are:");
    todos.iter().enumerate().for_each(|(i,x)|println!("{}. {}",i+1,x));
}
fn parse_json() -> Todo{
    serde_json::from_str(&init_file()).unwrap()
}
fn init_file() -> String{
    str::from_utf8(&fs::read(TODO_FILE).expect("File not found")).unwrap().to_string()
}