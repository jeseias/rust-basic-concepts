// Structs - Used to create custom data types

// Traditional Strcut
// struct Color {
//   red: u8,
//   green: u8,
//   blue: u8
// }

// Tuple
// struct Color(u8,u8,u8);

struct Person {
  first_name: String,
  last_name: String
}

impl Person {
  fn new(first: &str, last: &str) -> Person {
    Person {
      first_name: first.to_string(),
      last_name: last.to_string()
    }
  }

  // Get FUll name
  fn full_name(&self) -> String {
    format!("{} {}", self.first_name, self.last_name)
  }

  // Set last name
  fn set_last_name(&mut self, last: &str) {
    self.last_name = last.to_string()
  }

  // Name to Tuple
  fn to_tuple(&self) -> (String, String) {
    (self.first_name.to_string(), self.last_name.to_string())
  }
}

pub fn run() {
  // let mut c = Color {
  //   red: 255,
  //   green: 0,
  //   blue: 0
  // };

  // let mut c = Color(255,0,0);

  // c.0 = 200;

  // println!("Color: {} {} {}", c.0, c.1, c.2)

  let person = Person::new("John","Doe");
  let mut mary = Person::new("Mary","Doe");

  println!("Person {}", person.full_name());
  println!("Person {}", mary.full_name());
  
  mary.set_last_name("William");
  println!("Person {}", mary.full_name());
  println!("Person {:?}", mary.to_tuple());
}