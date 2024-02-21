mod print;

fn main() {
  print::run();

  let name = String::from("Brad");
  let planet = String::from("Mars");
  // Basic formatting
  println!("{} is from {}", name, planet);

  // Positional Arguments
  println!("{0} is from {1} and {0} likes to {2}", name, planet, "code");

  // Named Arguments
  println!("{name} likes to play {activity}", name = "John", activity = "baseball");

  // Placeholder traits
  println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

  // PLaceholder for debug trait
  println!("{:?}", (12, true, "hello"));

  // Basic Math
  println!("10 + 10 = {}", 10 + 10)
}
