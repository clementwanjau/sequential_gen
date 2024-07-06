use lazy_static::lazy_static;

use crate::lib::*;
use crate::prelude::Generator;

lazy_static! {
    static ref GENERATOR: SequenceGenerator = SequenceGenerator::default();
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
/// assert_eq!(generator.generate(), 1);
/// assert_eq!(generator.generate(), 2);
/// ```
///
#[derive(Clone, Debug, Default)]
pub struct SimpleGenerator<T>
where
    T: Into<u128> + Copy,
{
    step: T,
}

impl<T> SimpleGenerator<T>
where
    T: Into<u128> + Copy,
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
    /// let generator = SimpleGenerator::new(1u128);
    /// ```
    pub fn new(step: T) -> Self {
        Self { step }
    }
}

impl<T> Generator<T> for SimpleGenerator<T>
where
    T: Into<u128> + Copy + Default + From<u128> + Add<Output = T>,
{
    /// Generates a new value by incrementing the current value by the step.
    ///
    /// # Returns
    ///
    /// A new value.
    fn generate(&self) -> T {
        let _ = GENERATOR
            .value
            .fetch_add(self.step.into() as usize, Ordering::SeqCst);
        let val = GENERATOR.value.load(Ordering::SeqCst);
        T::from(val as u128)
    }

    /// Generates a new value by incrementing the current value by the step and adding an offset.
    ///
    /// This is useful when you want to generate a value that is offset by a certain amount.
    ///
    /// # Arguments
    ///
    /// * `offset` - The offset to add to the generated value.
    ///
    /// # Returns
    ///
    /// A new value.
    fn with_offset(&self, offset: T) -> T {
        let value = self.generate();
        value + offset
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_generator() {
        let generator = SimpleGenerator::new(1);
        assert_eq!(generator.generate(), 1);
        assert_eq!(generator.generate(), 2);
    }
}
