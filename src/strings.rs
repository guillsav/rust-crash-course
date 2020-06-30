// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run() {
  let mut hello = String::from("Hello ");

  // Get length
  println!("Length: {}", hello.len());

  // Push a char
  hello.push('W');

  // Push a string
  hello.push_str("orld!");

  println!("{}", hello);
}