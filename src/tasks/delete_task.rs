use std::io;
use crate::Task;

pub fn delete_task(tasks: &mut Vec<Task>) {
     let mut index = String::new();
    println!("Enter task number");

    io::stdin().read_line(&mut index).expect("Failed to read");
    let index : usize = match index.trim().parse()  {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number");
                return;  
            }
    };


    if index > 0 && index <= tasks.len() {
        tasks.remove(index - 1);
        println!("✅ Task number {} has been deleted successfully!!!", index)
    } else {
        println!("Task not found");
    }
}