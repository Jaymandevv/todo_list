use std::{io, convert::TryFrom};
use crate::Priority;


pub fn add_priority() -> Priority {
    loop {
     println!("Select priority:");
     println!("1. High");
     println!("2. Medium");
     println!("3. Low");


     let mut index = String::new();

     io::stdin().read_line(&mut index).expect("Failed to read");

     let index: u32 = match index.trim().parse() {
          Ok(num) => num,
          Err(_) => {
               println!("Invalid number");
               continue;
          }
     };

     match Priority::try_from(index) {
          Ok(priority) => return priority,
          Err(err) => {
               println!("{}", err);
               continue;
          }
     }

   
}


}

