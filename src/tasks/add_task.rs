use std::io;
use crate::Task;
use super::add_priority::add_priority;





//? This takes a mutable refrence because i need to 
//? to update the tasks
pub fn add_task(tasks: &mut Vec<Task>) {
    let mut title = String::new();
    println!("Enter task:");
    io::stdin().read_line(&mut title).expect("Failed to read");
    let priority =  add_priority() ;
    
    
    let title = title.trim().to_string();

   let task = Task {
    title,
    done: false,
    priority,
   };

   tasks.push(task);
   println!("✅ Task added successfully!!!");
 }