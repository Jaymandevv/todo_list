use std::{io::{self, Read}, task};

#[derive(Debug)]
struct Task {
    title: String,
    done: bool
}
fn main() {
let mut tasks: Vec<Task> = Vec::new();

loop {
    println!("\n 📌 To-Do Menu");
    println!("1. Add Task");
    println!("2. View Tasks");
    println!("3. Mark Task as Done");
    println!("4. Exit");

    let mut choice = String::new();

    io::stdin().read_line(&mut choice).expect("Failed to read");

    let choice = choice.trim();

    match choice {
        "1" => add_task(&mut tasks),
        "2" => view_tasks(&tasks),
        "3" => mark_as_done(&mut tasks),
        "4" => {
            println!("Goodbye!");
            break;
        },
        _ =>  println!("Invalid choice")
    }

}


}


//? This takes a mutable refrence because i need to 
//? to update the tasks

fn add_task(tasks: &mut Vec<Task>) {
    let mut title = String::new();
    println!("Enter task:");
    io::stdin().read_line(&mut title).expect("Failed to read");

    let title = title.trim().to_string();

   let task = Task {
    title,
    done: false
   };

   tasks.push(task);
}

fn view_tasks(tasks: &Vec<Task>) {
    for (i, task) in tasks.iter().enumerate() {
        let status = if task.done {"Done"} else {"Not done"};
        print!("\n {}. {} [{}]", i + 1, task.title, status);
    }
}

fn mark_as_done(tasks: &mut Vec<Task>) { 
    let mut index = String::new();
    println!("Enter task number: ");

    io::stdin().read_line(&mut index).expect("Failed to read");
    let index: usize = match index.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number");
            0
        }
    };

    if index > 0 && index <= tasks.len() {
        tasks[index - 1].done = true;
    } else {
        println!("Task not found");
    }

}


// Todo:
//1. Add Task
//2. View Tasks
//3. Mark task as done  


// Improvement
//1. Add modules
//2. Add delete task
//3. Add priority 