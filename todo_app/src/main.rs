use std::{fs,str,io, io::Write};
use serde::{Deserialize, Serialize};
use serde_json::to_writer;

const TODO_FILE: &str = "../../Documents/rust_todo.json";
const YES_LIST: [&str; 5] = ["y\n", "yes\n", "Y\n","YES\n","true\n"];

#[derive(Serialize, Deserialize)]
struct Todo {
    todo_list: Vec<String>,
}

fn main(){
    println!("This program reads/writes from/to ~/Documents/rust_todo.json\n");
    let mut todos = parse_json().todo_list;
    if let ask_user_continue(){}
        remove_todos(&mut todos);
        add_todos(&mut todos);
        save_todo_list(&todos);
    }
}

fn ask_user_continue() -> bool{
    let mut user_response = "";
    println!("If you want to edit your todo list, type y or yes then hit enter, else just hit enter.");
    io::stdin().read_line(&mut user_response).expect("Failed to read line"); 
    for val in YES_LIST{if user_response == *val{
        true
    }
    false
}

fn print_current_todo(todos: &[String]){
    println!("Your current todo's are:");
    todos.iter().enumerate().for_each(|(i,x)|println!("{}. {}",i+1,x));
}

fn print_flush(print_str: &str){
    print!("{}",print_str); 
    io::stdout().flush().unwrap();
}

fn parse_json() -> Todo{
    serde_json::from_str(&init_file()).unwrap()
}

fn init_file() -> String{
    str::from_utf8(&fs::read(TODO_FILE).expect("File not found")).unwrap().to_string()
}

fn add_todos(todos: &mut Vec<String>){
    let mut item: String = String::from("");
    loop{
        item.clear();
        print_flush("If you want to add a todo, please type it in here, otherwise hit enter: ");
        io::stdin().read_line(&mut item).expect("Failed to read line"); 
        if item == *"\n"{break}
        todos.push(item.clone());
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

    if remove_numchar != "0" && !todos.is_empty(){
        todos.remove(remove_numchar.trim().parse::<usize>().expect("Input not an integer")-1);
        if !todos.is_empty(){
            remove_todos(todos);
        }else{
            println!("You have no more todos!");
        }
    }
}

fn save_todo_list(todos: &Vec<String>){
    let mut save_string: String = String::from("");
    println!("Your todos are \n {:#?}.", todos);
    print_flush("Do you want to save the todo list (y, yes, Y or n, no, N)?: ");
    io::stdin().read_line(&mut save_string).expect("Failed to read line"); 

    for val in YES_LIST{if save_string == *val{
        let save_string = Todo{
            todo_list: todos.to_owned()
        };
        to_writer(fs::File::create_new(TODO_FILE).unwrap(),&save_string).unwrap();
    }}
}

