// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
  let name = "Guillaume";
  let mut age = 37;
  println!("My name is {} and I am {}", name, age);
  age = 38;
  println!("My name is {} and I am {}", name, age);

  // Define constant
  const ID: i32 = 001;
  println!("ID: {}", ID);

  // Assigne multiple vars
  let (my_name, my_age) = ("Guillaume", 37);
  println!("{} is {}", my_name, my_age);
}