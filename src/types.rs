/**
 * Primitive Types:
 *
 * Integers: u8, i8, u16, i16, u32, i32, u63, i64, u128, i128 (number of bits it can take in memory)
 * Floats: f32, f64
 * Boolean (bool)
 * Characters (char)
 * Tuples
 * Arrays
 */

// Rust is statically typed language though the rust compiler can infer what type based on value and how it is used

pub fn run() {
  let x = 1; // default to "i32"

  let y = 2.5; // default to  "f64"

  let z: i64 = 2343234; // explicit type

  // Find max size
  println!("Max i32: {}", std::i32::MAX);
  println!("Max i64: {}", std::i64::MAX);

  let is_active = true; // Boolean

  let is_greater = 10 > 5; // boolean from expression

  let a1 = 'a'; // Single quote for char
  let face: char = '\u{1F600}'; // unicode char

  println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}
