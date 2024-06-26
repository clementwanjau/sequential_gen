//! # Sequential ID Generator
//!
//! This crate provides a simple sequential ID generator that can be used to generate unique sequential IDs.
//! The generator allows you to specify the step to increment the value by. To use the generator, you need to
//! ensure that only one instance of the generator is created. This can be achieved by using a singleton pattern
//! or by using a global variable.
//!
//! The crate provides a `Generator` trait that you can implement to create your own generator. The crate also
//! provides a `SimpleGenerator` that you can use out of the box.
//!
//! ## Usage
//!
//! ```rust
//! # extern crate lazy_static;
//!
//! # use lazy_static::lazy_static;
//! use sequential_gen::prelude::*;
//!
//! lazy_static! {
//!    static ref GENERATOR: SimpleGenerator<usize> = SimpleGenerator::new(1);
//! }
//!
//! fn main() -> Result<(), Error> {
//!   let id = GENERATOR.generate()?;
//!   // Use your ID
//!   Ok(())
//! }
//! ```

mod error;
mod generator;

pub mod prelude {
	pub use crate::error::Error;
	pub use crate::generator::{Generator, SimpleGenerator};
}
