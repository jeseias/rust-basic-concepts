// Arrays - Fixed list where elements are the same types

use std::mem;

pub fn run() {  
  let mut numbers:[i32; 4] = [1,2,3,4];

  // Re-assign value
  numbers[2] = 20;

  println!("{:?}", numbers);

  println!("Array length: {}", numbers.len());

  // Arrays are stack allocated
  println!("Array occupies {} bytes", mem::size_of_val(&numbers));


  // Get slice = 
  let slice: &[i32] = &numbers[0..3];
  println!("Slice: {:?}", slice); 
}