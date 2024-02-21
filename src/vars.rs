// Variables hold primitive data or references to data
// Variables are immutable by default 
// Rust is a blocked-scoped language

pub fn run() {
  let name = "Brad";
  let mut age = 37;

  println!("Name: {}, and I am {}", name, age);
  age = 38;

  println!("Name: {}, and I am {}", name, age);

  // Define constant 
  const ID: i32 = 001;
  println!("ID: {}", ID);

  let (my_name, my_age) = ("Brad", 37);
  println!("My new name is {}, and I am {} years old", my_name, my_age);
} 