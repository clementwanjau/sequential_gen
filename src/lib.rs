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
//!    static ref GENERATOR: SimpleGenerator<usize> = SimpleGenerator::new(1usize);
//! }
//!
//! fn main() {
//!   let id = GENERATOR.generate();
//!   // Use your ID
//! }
//! ```

#![cfg_attr(feature = "no_std", no_std)]

mod lib {
	pub use self::core::clone::Clone;
	pub use self::core::convert::{From, Into};
	pub use self::core::default::Default;
	pub use self::core::fmt::Debug;
	pub use self::core::marker::Copy;
	pub use self::core::sync::atomic::{AtomicUsize, Ordering};
	pub use self::core::usize;

	mod core {
		#[cfg(feature = "no_std")]
		pub use core::*;

		#[cfg(not(feature = "no_std"))]
		pub use std::*;
	}
}

mod generator;

pub mod prelude {
	pub use crate::generator::{Generator, SimpleGenerator};
}
