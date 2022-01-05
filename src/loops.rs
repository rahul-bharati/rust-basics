pub fn run() {
  let mut count = 0;

  // infinite loop
  loop {
    count += 1;
    println!("Number: {}", count);

    if count == 10 {
      break; // manually break loop
    }
  }

  // while loop (FizzBuzz)
  while count <= 100 {
    if count % 15 == 0 {
      println!("FizzBuzz");
    } else if count % 3 == 0 {
      println!("fizz");
    } else if count % 5 == 0 {
      println!("Buzz");
    } else {
      println!("{}", count)
    }
    count += 1;
  }

  // for range
  for x in 0..100 {
    if x % 15 == 0 {
      println!("FizzBuzz");
    } else if x % 3 == 0 {
      println!("fizz");
    } else if x % 5 == 0 {
      println!("Buzz");
    } else {
      println!("{}", x)
    }
  }
}
