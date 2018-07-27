extern crate rand;

use rand::prelude::*;

pub struct Robot {
  name: String
}

impl Robot {
    pub fn new() -> Robot { 
      let mut random_name = "".to_string();
      let mut i = 0;
      while i < 3 {
        let mut ran_num = thread_rng().gen::<u8>();
        while ran_num < 65 || ran_num > 90 {  //Random number needs to be an uppercase letter
          ran_num = thread_rng().gen::<u8>();
        }
        if i < 2 {  //Add a new char to the robots name
          random_name.push(ran_num as char);
        }
        else {      //Add a new number to the robots name
          random_name.push_str(&ran_num.to_string());
          random_name.push('0');
        }
        i = i + 1;
      }
      Robot {name: random_name}
    }

    pub fn name<'a>(&'a self) -> &'a str { self.name.as_str() }
    pub fn reset_name(&mut self) {
      let mut i = 0;
      let mut random_name = "".to_string();
      while i < 3 {
        let mut ran_num = thread_rng().gen::<u8>();
        while ran_num < 65 || ran_num > 90 {  //Random number needs to be an uppercase letter
          ran_num = thread_rng().gen::<u8>();
        }
        if i < 2 {  //Add a new char to the robots name
          random_name.push(ran_num as char);
        }
        else {      //Add a new number to the robots name
          random_name.push_str(&ran_num.to_string());
          random_name.push('0');
        }
        i = i + 1;
      }
      self.name = random_name;
    }
}
