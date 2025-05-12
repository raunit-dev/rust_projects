mod models;
mod store;
use std::io::{self,Write};
use crate::store::{load_from_file,save_to_file};
use crate::models::Todo;

fn main() {
    let mut todos = load_from_file().unwrap_or_else(|_| Vec::new());
    
    loop {
        println!("\n--- TODO MENU ---");
        println!("1. Add task");
        println!("2. View tasks");
        println!("3. Mark task as done");
        println!("4. Delete task");
        println!("5. Exit");

        print!("Choose an option: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let choice = input.trim();

        match choice {
            "1" => add_task(&mut todos),
            "2" => view_tasks(&todos),
            "3" => mark_done(&mut todos),
            "4" => delete_task(&mut todos),
            "5" => {
                save_to_file(&todos).unwrap();
                println!("Goodbye mate!");
                break;
            }
            _ => println!("Invalid choice, try again baka."),
    }
}
}

fn add_task(todos: &mut Vec<Todo>) {

    println!("Enter task details:");
    let mut task_input = String::new();
    io::stdin().read_line(&mut task_input).unwrap();
    let task = task_input.trim().to_string();

    println!("Enter deadline: ");
    let mut deadline_input = String::new();
    io::stdin().read_line(&mut deadline_input).unwrap();
    let deadline = deadline_input.trim().to_string();

    let id = if let Some(last) = todos.last() {
        last.id + 1
    } else {
        1
    };

    let new_todo = Todo {
        id,
        task,
        deadline,
        done: false
    };

    todos.push(new_todo);
    println!("Task added success")
}

fn view_tasks(todos: &Vec<Todo>) {

    if todos.is_empty() {
        println!("No tasks available");
        return;
    }
    println!("\n Current Todo List:");
    for todo in todos.iter(){
        println!(
            "[{}] {} | Deadline: {} | Status: {}",
            todo.id,
            todo.task,
            todo.deadline,
            if todo.done {"Done"} else {"Not Done"}
        );
    }
}

fn mark_done(todos: &mut Vec<Todo>) {
    println!("Enter the task ID to mark as done: ");
    let mut id_input = String::new();
    io::stdin().read_line(&mut id_input).unwrap();

    let id: i32 = id_input.trim().parse().unwrap_or(0);

    if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
        todo.done = true;
        println!("✔ Task marked as done!");
    } else {
        println!("❌Task with ID {} not found.", id);
    }
}

fn delete_task(todos: &mut Vec<Todo>) {
    println!("Enter the task ID to delete: ");
    let mut id_input = String::new();
    io::stdin().read_line(&mut id_input).unwrap_or(0);
    let id: i32 = id_input.trim().parse().unwrap_or(0);

     if let Some(pos) = todos.iter().position(|t| t.id == id) {
        todos.remove(pos);
        println!("✔ Task with ID {} deleted!", id);
    } else {
        println!("❌ Task with ID {} not found!", id);
    }
}
