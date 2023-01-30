# Simple CLI Calculator in Rust

This is a very basic and simple cli calculator built with Rust.  
The application expects a number, an operator, and another number as env arguments.

**Usage:**

```bash

$ ./calculator <number> <operator> <number>

```

**Supported operators:**

`+`, `-`, `* | x | X`, `/`

### How to build

Simply run `cargo build --release`.

A binary file will be created here: `./target/release/calculator`

### How to run

#### Development

In your terminal run `cargo run -- ` followed by a number, an operator and the second number.

Eg. `cargo run -- 2 + 2`

#### Production

After building the application, you can locate the file in `./target/release` folder.

Navigate to `./target/release` and run `./calculator <number> <operator> <number>`

Eg.

```bash
	$ ./target/release/calculator 10 / 8
```
