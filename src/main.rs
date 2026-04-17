use tasks::tasks_control;


mod tasks;
#[derive(Debug)]
pub struct Task {
    pub title: String,
    pub done: bool
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
//3. Add priority 