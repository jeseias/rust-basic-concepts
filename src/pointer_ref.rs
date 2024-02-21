// Reference - Point to a resource in memory

pub fn run() {  
  // Primitive array 
  let arr1 = vec![1,2,3];
  let arr2 = &arr1;

  // With non-primitives, if you assign another variable to a piece of data, the first variable will no longer hold
  // that value, hold that value. You'll need to use a reference (&) to point to the resource

  println!("Values: {:?}", (&arr1, arr2));
}