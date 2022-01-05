// Vectors - resizable arrays

pub fn run() {
  let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
  println!("{:?}", numbers);

  // Re-assign value
  numbers[2] = 20;

  // Add to vectors
  numbers.push(6);
  println!("{:?}", numbers);

  // remove from vector
  numbers.pop();
  println!("{:?}", numbers);

  // single value
  println!("1 element: {}", numbers[0]);

  // get Vector  length
  println!("Vector  length: {}", numbers.len());

  // Vectors are stack allocated
  println!("Vector s occupies {} byte", std::mem::size_of_val(&numbers));

  // Vector  slice
  let slice: &[i32] = &numbers[0..2];
  println!("Slice: {:?}", slice);

  // Loop through vector values
  for x in numbers.iter() {
    println!("Number: {}", x);
  }

  // Loop and mutate values
  for x in numbers.iter_mut() {
    *x *= 2
  }
  println!("{:?}", numbers);
}
