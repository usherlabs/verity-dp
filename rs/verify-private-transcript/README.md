# Verity Verify Private Transcript

This is essentially a fork of the TLSN Core library, with the following changes:

- Comment out functionality traditionally dependent on ring lib
- Add support for pre-computed encodings - see `encodings_precompute.rs`
  - Generation
  - Verification
