use std::io;
use crate::Task;
use self::{add_task::add_task, view_tasks::view_tasks,delete_task::delete_task,mark_as_done::mark_as_done};


mod add_task;
mod view_tasks;
mod delete_task;
mod mark_as_done;
mod add_priority;

pub fn tasks_control() {
     let mut tasks: Vec<Task> = Vec::new();

     loop {
         println!("\n 📌 To-Do Menu");
         println!("1. Add Task");
         println!("2. View Tasks");
         println!("3. Mark Task as Done");
         println!("4. Delete task");
         println!("5. Exit");
     
         let mut choice = String::new();
     
         io::stdin().read_line(&mut choice).expect("Failed to read");
     
         let choice = choice.trim();
     
         match choice {
             "1" => add_task(&mut tasks),
             "2" => view_tasks(&tasks),
             "3" => mark_as_done(&mut tasks),
             "4" => delete_task(&mut tasks),
             "5" => {
                 println!("Goodbye!");
                 break;
             },
             _ =>  println!("Invalid choice")
         }
     
     }
     
     

}