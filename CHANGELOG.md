# Change Log

## v0.1.2

- Added `UuidGenerator` which generates UUIDs using the `uuid` crate. This generator is only available when the `std`
  feature is enabled.
- Bug fixes.

## v0.1.1

- Added an epoch based generator. `EpochBasedGenerator` only available when the `std` feature is enabled which is
  enabled by default.

## v0.1.0

- Initial release
- Added `no_std` support.
- Added `SimpleGenerator` struct
- Added `Generator` trait
