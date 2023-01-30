use std::env::{args, Args};

fn main() {
    let mut args: Args = args();

    let first: String = args.nth(1).unwrap();
    let operator: String = args.nth(0).unwrap();
    let second: String = args.nth(0).unwrap();
  
    let first_number = first.parse::<f32>().unwrap();
    let second_number = second.parse::<f32>().unwrap();
  
    println!("{} {} {}", first_number, operator, second_number);
}