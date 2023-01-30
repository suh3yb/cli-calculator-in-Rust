use std::env::{args, Args};

fn main() {
    println!("Hello, world!");
    let mut args: Args = args();
    println!("{:?}", args);

    let first: String = args.nth(1).unwrap();
    println!("{}", first);

    println!("args after using nth method on it: {:?}", args);
}
