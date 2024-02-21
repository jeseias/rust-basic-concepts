// Functions - Used to store blocks of code for re-use

pub fn run() {
  greeting("Hello", "Jane");
}

fn greeting(greet: &str, name: &str) {
  println!("{} {}, nice to meet you", greet, name);

  // Bind function values to variables
  let get_sum = add(5,5);
  println!("Sum: {}", get_sum);

  // Closures
  let n3 = 10;
  let add_numbers = |n1: i32, n2: i32 | n1+ n2 + n3;
  println!("C Sum: {}", add_numbers(3,3))
}

fn add(n1: i32, n2: i32) -> i32 {
  n1 + n2
}