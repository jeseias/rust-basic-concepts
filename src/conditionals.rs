pub fn run() {
  let age = 18;
  let check_id = false;
  let knows_the_person = false;

  // If/Else
  if age >= 21  || knows_the_person {
    println!("Bartender: What would you like to drink?");
  } else if age < 21 && check_id {
    println!("Bartender: Sorry you have to leave!")
  } else {
    println!("Bartender: I'll need to see your ID")
  }

  // Shorthand if
  let is_of_age = if age >= 21 {"yep"} else {"nope"};
  println!("Is of age, {}", is_of_age);
}