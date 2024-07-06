pub(crate) mod simple;
#[cfg(not(feature = "no_std"))]
pub(crate) mod unix_epoch;
#[cfg(feature = "uuid")]
pub(crate) mod uuid;

/// A trait for generating unique IDs.
pub trait Generator<T> {
    /// Generates a new unique ID.
    fn generate(&self) -> T;

    fn with_offset(&self, offset: T) -> T;
}
