pub fn run() {
  // print to console
  println!("Hello from the print.rs");

  // basic formatting
  println!("Number: {}", 1); // use {} as placeholder

  // Positional Arguments - use indexes to reference
  println!(
    "{0} is from {1} and {0} likes to {2}",
    "Rahul", "India", "Code"
  );

  // Named Arguments
  println!(
    "{name} is from {place} and {name} likes to {activity}",
    name = "Rahul",
    place = "India",
    activity = "Code"
  );

  // Placeholder Triats
  println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

  // Placeholder for debug trait
  println!("{:?}", (12, true, "hello"));

  // basic Math
  println!("10 + 10 = {}", 10 + 10);
}
