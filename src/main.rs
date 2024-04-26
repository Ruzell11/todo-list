use std::io;
use std::process::ExitCode;

#[derive(Debug)]
struct UserObject {
    name: String,
    age: String,
}


fn get_user_input(prompt: &str) -> String {
    
    println!("{}", prompt);

    let mut input = String::new();

    
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    input
}


fn add_todo(task_array: &mut Vec<String>) {

    let input = get_user_input("Type the task that you want to add");

    task_array.push(input);

    println!("Successfully added!");

    main_menu(task_array);
}

fn get_todo(task_array: &mut Vec<String>){

    for (index, task) in task_array.iter().enumerate() {
        println!("{}: {}", index, task);
    }

    let input = get_user_input("Press M to show main menu");

    println!("This is the input {}", input);

    if input.trim().to_uppercase() == "M" {
        main_menu(task_array);
    }
}

fn update_todo(task_array: &mut Vec<String>){

}

fn delete_todo(){

}

fn get_user_details() -> UserObject {
    let name: String = get_user_input("Please enter your name:");
    let age: String = get_user_input("Please enter your age:");

    

    UserObject {
        name: name,
        age: age
    }
}


fn exit_program() {
    std::process::exit(200)
}


fn main_menu(task_array: &mut Vec<String>){
    
    println!("Welcome to main menu.");

    println!("Options:");
    println!("A: Add todo");
    println!("G: Get todo list");
    println!("E: End Program");

    let input = get_user_input("Please enter the action you want to do");

    match input.trim().to_uppercase().as_str(){
        "A" => add_todo(task_array),
        "G" => get_todo(task_array),
        "U" => update_todo(task_array),
        "E" => exit_program(),
        _=>println!("Press valid keys"),
    }
}


fn main() {
    let mut task_array = Vec::new();
    println!("Welcome on to-do list application!");

    get_user_details();
    main_menu(&mut task_array);
}