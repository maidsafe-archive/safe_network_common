# Safe Network Common - Change Log

## [0.5.1]
- Depend on routing 0.25.0.

## [0.5.0]
- Depend on sodiumoxide 0.0.10 because newer versions break the build.

## [0.4.1]
- Code a From trait to convert from anthing that can be converted to `String` via `Into` into either Get or Mutation Error.

## [0.4.0]
- Migrate to Routing 0.23.0.
- Migrate to maidsafe_utilities 0.8.0.

## [0.3.0]
- Migrate to Routing 0.22.0.

## [0.2.0]
- Depend on routing and reduce the `XorName` size to 256 bit.
- Errors implement `std::Error`.

## [0.1.1]
- `NetworkFull` error added to `MutationError`

## [0.0.1]
- Core-Vault error communication module
- Shared constants - type tags for session packet and DNS
- Mpid messaging infrastructure module
