use std::env::{args, Args};

fn main() {
    println!("Hello, world!");
    let mut args: Args = args();
    println!("{:?}", args);


  // The first argument is the location of the compiled binary, so skip it
  let first: String = args.nth(1).unwrap();
  // After accessing the second argument, the iterator's next element becomes the first
  let operator: String = args.nth(0).unwrap();
  let second: String = args.nth(0).unwrap();

  println!("{} {} {}", first, operator, second);
}
