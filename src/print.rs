pub fn run() {
  // Print to console
  println!("Hello from the the print.rs file");

  // Basic Formatting
  println!("Number: {}", 1);

  println!("{} lives in {}", "Guillaume", "New Jersey");

  // Positional Arguments
  println!("{0} lives in {1} and {0} likes to {2}", "Guillaume", "New Jersey", "code");

  // Named Arguments
  println!("{name} likes to play {activity}", name = "John", activity = "baseball" );

  // Placeholder traits
  println!("Binary: {:b} Hex: {:x} Octal {:o}", 10, 10, 10);

  // Placeholder for debug trait
  println!("{:?}", (12, true, "hello"));

  // Basic math
  println!("10 + 10 = {}", 10 + 10);
}