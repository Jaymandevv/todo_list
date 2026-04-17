use std::io;
use crate::Task;

pub fn mark_as_done(tasks: &mut Vec<Task>) {
     let mut index = String::new();
    println!("Enter task number: ");

    io::stdin().read_line(&mut index).expect("Failed to read");
    let index: usize = match index.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number");
            return;
        }
    };

    if index > 0 && index <= tasks.len() {
        tasks[index - 1].done = true;
        println!("✅ Great you are done with Task {}", index)
    } else {
        println!("Task not found");
    }
}