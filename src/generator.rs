use std::sync::atomic::AtomicUsize;

use lazy_static::lazy_static;

lazy_static! {
    static ref GENERATOR: SequenceGenerator = SequenceGenerator::default();
}

/// A trait for generating unique IDs.
pub trait Generator<T> {
    /// Generates a new unique ID.
    fn generate(&self) -> Result<T, crate::error::Error>;
}

/// An internal counter that keeps track of the current value.
#[derive(Debug)]
struct SequenceGenerator {
    value: AtomicUsize,
}

impl Default for SequenceGenerator {
    /// Creates a new sequence generator with a default value of 0.
    fn default() -> Self {
        Self {
            value: AtomicUsize::new(0),
        }
    }
}

/// A simple generator that increments the value by a step.
///
/// # Examples
///
/// ```rust
/// # use sequential_gen::prelude::*;
///
/// let generator = SimpleGenerator::new(1);
///
/// assert_eq!(generator.generate().unwrap(), 1);
/// assert_eq!(generator.generate().unwrap(), 2);
/// ```
///
#[derive(Clone, Debug, Default)]
pub struct SimpleGenerator<T>
where
    T: Into<usize> + Copy,
{
    step: T,
}

impl<T> SimpleGenerator<T>
where
    T: Into<usize> + Copy,
{
    /// Creates a new simple generator with the specified step.
    ///
    /// # Arguments
    ///
    /// * `step` - The step to increment the value by.
    ///
    /// # Returns
    ///
    /// A new simple generator.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use sequential_gen::prelude::*;
    ///
    /// // Create a new simple generator with a step of 1.
    /// let generator = SimpleGenerator::new(1usize);
    /// ```
    pub fn new(step: T) -> Self {
        Self { step }
    }
}

impl<T> Generator<T> for SimpleGenerator<T>
where
    T: Into<usize> + Copy + Default + From<usize>,
{
    /// Generates a new value by incrementing the current value by the step.
    ///
    /// # Returns
    ///
    /// A new value.
    ///
    /// # Errors
    /// An error will be returned if the value cannot be locked.
    fn generate(&self) -> Result<T, crate::error::Error> {
        let val = GENERATOR
            .value
            .fetch_add(self.step.into(), std::sync::atomic::Ordering::SeqCst);
        Ok(T::from(val))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_generator() -> Result<(), crate::error::Error> {
        let generator = SimpleGenerator::new(1);
        assert_eq!(generator.generate()?, 1);
        assert_eq!(generator.generate()?, 2);
        Ok(())
    }
}
