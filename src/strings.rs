// Primitive str - Immutable fixed-length string somewhere in memory
// String - Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run() {
  let hello = "Hello"; // this is str (primitive)

  let mut hello_world = String::from("Hello World"); // String type

  // get length
  println!("Length: {}", hello.len());

  // push method on String not str(primitive)
  hello_world.push('!'); // accepts only characters
  hello_world.push_str(" Welcome"); // accepts string

  // Capacity in byte
  println!("Capacity: {}", hello_world.capacity());

  // is empty
  println!("Is Empty: {}", hello_world.is_empty());

  //Contains world or char
  println!("Contains 'World' {}", hello_world.contains("Hello"));

  //replace word or char
  println!("Replace: {}", hello_world.replace("Hello", "Hi"));

  // Loop through strings by whitespace
  for word in hello_world.split_whitespace() {
    println!("{}", word);
  }

  // Create string with Capacity
  let mut s = String::with_capacity(10);
  s.push('a');
  s.push('b');

  // Assertion testing
  assert_eq!(2, s.len());
  assert_eq!(10, s.capacity());

  println!("{:?}", (hello, hello_world));
}
