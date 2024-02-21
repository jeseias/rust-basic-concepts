/*
  Primitive Types---
  Integers  : u8, u16,u32,u64,u182,i8,i16,i32,i64,i128 (number of bits they take in memory)
  Floats: f32, f64
  Boolean (bool)
  Characters (char)
  Tuples
  Arrays
*/

/*
Rust is a statically types language, which means that it must know the types of all variables at 
complie time, however the complier can usually infer what type we want to use based on the value and how we use it
*/

pub fn run() {
  // DEfault is "i32"
  let x = 1;

  // Default is "f64"
  let y = 2.5;

  // Add Explicit type
  let z: i64 = 4545445454545;

  println!("Max i32: {}", std::i32::MAX);
  println!("Max i64: {}", std::i64::MAX);

  // Boolean 
  let is_active = true;

  // get boolean from expression
  let is_greater = 10 < 5;

  let a1 = 'a';
  let face = '\u{1F600}';

  println!("{:?}", (x,y,z, is_active, is_greater, a1, face));
}