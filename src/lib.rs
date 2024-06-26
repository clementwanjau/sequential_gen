mod error;
mod generator;

pub mod prelude {
	pub use crate::error::Error;
	pub use crate::generator::{Generator, SimpleGenerator};
}
