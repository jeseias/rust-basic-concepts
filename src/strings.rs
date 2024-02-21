// Primitive str = " Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure = use when you need to modify or own string data

pub fn run() {
  let mut hello = String::from("Super long hello ");

  println!("Length: {}", hello.len());

  // This will push a cha
  hello.push('w');

  // This will push another string
  hello.push_str("Jobs");


  // Capacity in bytes
  println!("Capacity: {}", hello.capacity());

  // Check if empty
  println!("Contains: {}", hello.is_empty());

  // Contains
  println!("Contains: {}", hello.contains("Jobs"));

  // Replace
  println!("Replace: {}", hello.replace("wJobs", "Super Job"));

  // for loop through string by whitespace
  for word in hello.split_whitespace() {
    println!("{}", word);
  }

  // Create string with capacity
  let mut s = String::with_capacity(10);
  s.push('a');
  s.push('b');

  // Assertion testing
  assert_eq!(2, s.len());
  assert_eq!(11, s.capacity());

  println!("{}", s);
  println!("{}", hello);
}