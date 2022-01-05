// Arrays - Fixed list where elemets are of same data type

pub fn run() {
  let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

  // Re-assign value
  numbers[2] = 20;

  println!("{:?}", numbers);

  // single value
  println!("1 element: {}", numbers[0]);

  // get array length
  println!("Array length: {}", numbers.len());

  // Arrays are stack allocated
  println!("Arrays occupies {} byte", std::mem::size_of_val(&numbers));

  // Array slice
  let slice: &[i32] = &numbers[0..2];
  println!("Slice: {:?}", slice);
}
