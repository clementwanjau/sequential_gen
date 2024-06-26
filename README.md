# Sequential_Gen

[![Build Status](https://github.com/clementwanjau/sequential-gen/actions/workflows/build.yaml/badge.svg)](https://github.com/clementwanjau/sequential-gen/actions/workflows/build.yaml)

[![Crates.io](https://img.shields.io/crates/v/sequential_gen)](https://crates.io/crates/sequential_gen)
[![Docs.rs](https://docs.rs/sequential_gen/badge.svg)](https://docs.rs/sequential_gen)
[![License](https://img.shields.io/crates/l/sequential_gen)](
https://opensource.org/licenses/MIT)

A simple crate to generate sequential ids in Rust. This crate is useful when you need to generate sequential ids for
your data structures.

I recently needed to generate sequential ids for a tree data structure that I was working on, and I couldn't find a
crate that provided this functionality with `no_std` capabilities. So I decided to create this crate to fill that gap.

## Usage

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
sequential_gen = "0.1"
```

### Generating Sequential Ids

To ensure that the generated ids are unique, we need to ensure only a single instance of the `Generator` trait is
available
in the program. Once you have your `Generator` instance, you can use the `generate` method to generate sequential ids.

```rust
extern crate lazy_static;

use lazy_static::lazy_static;
use sequential_gen::prelude::*;

lazy_static! {
    static ref GENERATOR: SimpleGenerator<usize> = SimpleGenerator::new(1usize);
}

fn main() -> Result<(), Error> {
	let id = GENERATOR.generate()?;
	// Use your ID
	Ok(())
}
 ```

## License

This project is licensed under the MIT license.
