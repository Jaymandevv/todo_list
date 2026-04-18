use crate::Task;
use crate::Priority;

pub fn view_tasks(tasks: &Vec<Task>) {


     if tasks.len() <= 0 {
          println!("❌ No task yet, please add tasks.");
      } else {
          println!("\n Tasks");
          for (i, task) in tasks.iter().enumerate() {
            let priority = match task.priority {
                Priority::High => "High",
                Priority::Medium => "Medium",
                Priority::Low => "Low",
              };
              let status = if task.done {"Done"} else {"Not done"};
              print!("\n {}. {} [{}] [{}]", i + 1, task.title, priority, status);
          }
      }
}