// Reference Pointers - Point to a ressource in memory

pub fn run() {
  // Primitive Array
  let arr1: [i32; 3] = [1, 2, 3];
  let arr2 = arr1;

  // With non-primitives, if you assign another variable to a piece of data, the first variable will no longer hold that value. You'll need to use a reference (&) to point to the ressource.

  // Vector
  let vec1: Vec<i32> = vec![1, 2, 3];
  let vec2 = &vec1;

  println!("Values: {:?}", (&vec1, vec2));
}