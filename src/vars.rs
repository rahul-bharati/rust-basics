// Variables hold primitive data or reference to data
// Variables are immutable by default
// Rust is blocked-scoped language

pub fn run() {
  let name = "Heathcliff";
  let mut age = 23; // mut keyword is required if data is supposed to change
  println!("My name is {} and I'm {}", name, age);
  age = 24;
  println!("My name is {} and I'm {}", name, age);

  // Define constant
  const ID: i32 = 001; // const needs to have type assigned to it and variable_name needs to uppercase
  println!("ID: {}", ID);

  // Assign multiple vars
  let (my_name, my_age) = ("Mars", 10);
  println!("My name is {} and I'm {}", my_name, my_age);
}
