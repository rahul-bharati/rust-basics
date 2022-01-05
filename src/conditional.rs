pub fn run() {
  let age = 18;
  let check_id = false;
  let met_before = true;

  if age >= 21 && check_id || met_before {
    println!("You can vote");
  } else if age < 21 && check_id {
    println!("You need to be of age 21 to vote");
  } else {
    println!("Provide with ID");
  }

  // Short hand if

  let is_of_age = if age >= 21 { true } else { false };
  println!("is of age: {}", is_of_age);
}
