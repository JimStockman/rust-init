use chrono::prelude::*;
pub struct Person {
    pub first_name: String,
    pub last_name: String,
    pub age: i8,
    pub birthdate: String
}

impl Person {

    pub fn new(first_name: &str, last_name: &str, age: i8, birthdate: &str) -> Self {
        Self { 
            first_name: first_name.to_string(), 
            last_name: last_name.to_string(), 
            age: age, 
            birthdate: birthdate.to_string() 
        }
    }

    pub fn greet(&self) {
        println!("Hello, my name is {} and I am {} years old.", self.first_name, self.age.to_string());
    }
}