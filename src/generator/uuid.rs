use uuid::Uuid;

use crate::lib::Add;
use crate::prelude::Generator;

/// A unique ID generator that generates IDs based on a UUID.
///
/// The generator generates IDs based on a UUID. This means that the IDs are unique and sequential.
/// The generator is useful when you want to generate unique IDs for your data. This generator is
/// however not available in a `no_std` environment since it relies on the `uuid` crate to generate
/// the IDs. In no_std environments, you can use the `SimpleGenerator` provided by the
/// `sequential-gen` crate.
#[derive(Clone, Debug)]
pub struct UuidGenerator;

impl Generator<u128> for UuidGenerator {
    /// Generates a new unique ID based on a UUID.
    ///
    /// The ID is generated using a UUID. This means that the IDs are unique and sequential.
    ///
    /// # Returns
    ///
    /// A new unique ID.
    fn generate(&self) -> u128 {
        Uuid::now_v7().as_u128()
    }

    /// Generates a new unique ID based on a UUID and adds an offset.
    ///
    /// This is useful when you want to generate a value that is offset by a certain amount.
    ///
    /// # Arguments
    ///
    /// * `offset` - The offset to add to the generated value.
    ///
    /// # Returns
    ///
    /// A new unique ID.
    fn with_offset(&self, offset: u128) -> u128 {
        self.generate().add(offset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uuid_generator() {
        let generator = UuidGenerator;
        let value = generator.generate();
        let value_2 = generator.generate();
        assert_ne!(value, value_2);
    }

    #[test]
    fn test_uuid_generator_with_offset() {
        let generator = UuidGenerator;
        let value = generator.with_offset(10);
        let value_2 = generator.with_offset(10);
        assert_ne!(value, value_2);
    }
}
