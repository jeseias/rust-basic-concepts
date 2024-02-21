// Arrays - Are resizable arrays

use std::mem;

pub fn run() {  
  let mut numbers: Vec<i32> = vec![1,2,3,4];

  // Re-assign value
  numbers[2] = 20;

  // Add onto vector
  numbers.push(5);
  numbers.push(6);

  // Pop of last elemnts
  numbers.pop();

  println!("{:?}", numbers);

  println!("Vector length: {}", numbers.len());

  // Arrays are stack allocated
  println!("Vector occupies {} bytes", mem::size_of_val(&numbers));


  // Get slice = 
  let slice: &[i32] = &numbers[0..3];
  println!("Slice: {:?}", slice); 

  // Loop trough vector values 
  for x in numbers.iter() {
    println!("Number: {}", x);
  }

  // Loop and mutate values
  for x in  numbers.iter_mut() {
    *x *= 2;
  }

  println!("Numbers multiplied by two Vec {:?}", numbers)
}