![Poster](./SequentialGenPoster.png)
 
[![Build Status](https://github.com/clementwanjau/sequential_gen/actions/workflows/build.yaml/badge.svg)](https://github.com/clementwanjau/sequential_gen/actions/workflows/build.yaml)
[![Crates.io](https://img.shields.io/crates/v/sequential_gen)](https://crates.io/crates/sequential_gen)
[![Docs.rs](https://docs.rs/sequential_gen/badge.svg)](https://docs.rs/sequential_gen)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.71+-orange.svg)](https://www.rust-lang.org)

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

fn main() {
	let id = GENERATOR.generate();
	// Use your ID
}
 ```

> **Note:** You can also create your own generator by implementing the `Generator` trait if you need more control over
> the generation process or if the provided generators do not meet your requirements.

## `no_std` Support

The crate is `no_std` compatible, but you need to disable the default features in your `Cargo.toml` file. You then need
to
enable the `no_std` feature:

```toml
[dependencies]
sequential_gen = { version = "0.1", default-features = false, features = ["no_std"] }
```

> **Note:**
> When working in a `no_std` environment, you are confined to using the `SimpleGenerator` struct. In most cases this
> will suffice. The other generators require the `std` library.

The usage of the crate remains the same.

## License

This project is licensed under the [MIT](LICENSE) license.
