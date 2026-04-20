use std::convert::TryFrom;
use tasks::tasks_control;


mod tasks;
#[derive(Debug)]
pub struct Task {
    pub title: String,
    pub done: bool,
    pub priority: Priority,
}

#[derive(Debug)]
pub enum Priority {
    High,
    Medium,
    Low
    
} 

impl TryFrom<u32> for Priority {
    type Error = &'static str;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Priority::High),
            2 => Ok(Priority::Medium),
            3 => Ok(Priority::Low),
            _ => Err("Invalid Priority Number")
        }
    }
    
}


fn main() {
tasks_control();
}




// Todo:
//1. Add Task ✅
//2. View Tasks✅
//3. Mark task as done  ✅


// Improvement
//1. Add modules ✅
//2. Add delete task ✅
//3. Add priority ✅