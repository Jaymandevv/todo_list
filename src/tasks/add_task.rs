use std::io;
use crate::Task;



//? This takes a mutable refrence because i need to 
//? to update the tasks
pub fn add_task(tasks: &mut Vec<Task>) {
    let mut title = String::new();
    println!("Enter task:");
    io::stdin().read_line(&mut title).expect("Failed to read");

    let title = title.trim().to_string();

   let task = Task {
    title,
    done: false
   };

   tasks.push(task);
   println!("✅ Task added successfully!!!");
 }