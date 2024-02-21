// Enums are types which have a few definite values

enum Movement {
  // Variants
  Up,
  Down,
  Left,
  Right
}

fn move_avatar(m: Movement) {
  // Perform action depending on info
  match m {
      Movement::Up => println!("Avatar moving up"),
      Movement::Down => println!("Avatar moving down"),
      Movement::Left => println!("Avatar moving left"),
      Movement::Right => println!("Avatar moving right"),
  }
}

pub fn run() {

}