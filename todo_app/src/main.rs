use std::{fs, io::{self, Write}, path::PathBuf,string::FromUtf8Error};
use serde::{Deserialize, Serialize};
use serde_json::to_writer;
use dirs::home_dir;
const YES_LIST: [&str; 5] = ["y\n", "yes\n", "Y\n","YES\n","true\n"];


#[derive(Serialize, Deserialize)]
struct Todo {
    todo_list: Vec<String>,
}


fn main(){
    println!("This is a program for making, managing, and reading todo lists.\n");
    let mut todos = parse_json().unwrap_or_else(|_|Todo{todo_list: vec![]}).todo_list;
    if ask_user_continue(){
        if !todos.is_empty(){remove_todos(&mut todos);}
        add_todos(&mut todos);
        save_todo_list(&todos);
    }else {print_current_todo(&todos)}
}

fn ask_user_continue() -> bool{
    let mut user_response = String::from("");
    println!("If you want to edit your todo list, type y or yes then hit enter, else if you just want to see it, hit enter.");
    io::stdin().read_line(&mut user_response).expect("Failed to read line; invalid input or var"); 
    for val in YES_LIST{if user_response == *val{
        return true;
    }}
    false
}

fn print_current_todo(todos: &[String]){
    if !todos.is_empty(){
        println!("Your current todos are: ");
        todos.iter().enumerate().for_each(|(i,x)|println!("{}. {}",i+1,x));
    }else{println!("Your todo list is empty!")};
}
fn print_flush(print_str: &str){
    print!("{}",print_str); 
    io::stdout().flush().unwrap_or_else(|error|println!("\nWarning: print! syntax not working due to {}, using println! instead.",error));
}
fn parse_json() -> Result<Todo, serde_json::Error>{
    serde_json::from_str(&init_file().unwrap_or_else(|_|String::from("")))
}
fn init_file() -> Result<String, FromUtf8Error>{
    String::from_utf8(fs::read(todo_file()).unwrap_or_else(|_|vec![]))
}
fn todo_file() -> PathBuf{
    PathBuf::from(&[home_dir().unwrap_or_else(||"".into()).display().to_string(),"/Documents/rust_todo.json".to_string()].join(""))
}
fn add_todos(todos: &mut Vec<String>){
    let mut item: String = String::from("");
    loop{
        item.clear();
        print_flush("If you want to add a todo, please type it in here, otherwise hit enter: ");
        io::stdin().read_line(&mut item).expect("Failed to read line"); 
        if item == *"\n"{break}
        todos.push((item[0..item.len()-1]).to_string());
        println!("Your new todo list is {:?}",todos);
    }
}


fn remove_todos(todos: &mut Vec<String>){
    let mut remove_numchar: String = "".to_string();
    println!("If you want to remove a todo from the list, please input the number of the todo.\n");
    print_current_todo(todos);
    print_flush("\nEnter the the number of the todo item to remove or enter 0 if you don't want to remove anything: ");
    io::stdin().read_line(&mut remove_numchar).expect("Failed to read line"); 
    remove_numchar.truncate(1);
    let remove_num: usize = remove_numchar.trim().parse().unwrap_or_else(|_|{
        remove_todos(todos);
        println!("You may only enter a number or numbers.");
        0});

    if remove_num != 0 && remove_num <= todos.len() && !todos.is_empty(){
        todos.remove(remove_num-1);
        if !todos.is_empty(){
            remove_todos(todos);
        }else{
            println!("You have no more todos!");
        }
    }
}


fn save_todo_list(todos: &Vec<String>){
    let mut save_string: String = String::from("");
    println!("Your todos are: ");
    print_current_todo(todos);
    print_flush("Do you want to save the todo list (y, yes, Y or n, no, N)?: ");
    io::stdin().read_line(&mut save_string).expect("Failed to read line"); 

    for val in YES_LIST{if save_string == *val{
        let save_string = Todo{
            todo_list: todos.to_owned()
        };
        to_writer(fs::File::create(todo_file()).expect("file couldn't create"),&save_string).expect("Passed var has invalid keys or serialize failed");
    }}
}

