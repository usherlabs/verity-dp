# Verity Data Processor (VDP) - Internet Computer (IC) Libraries

![License](https://img.shields.io/crates/l/verity_dp_ic) [![verity_dp_ic on crates.io](https://img.shields.io/crates/v/verity_dp_ic)](https://crates.io/crates/verity_dp_ic) [![verity_dp_ic on docs.rs](https://docs.rs/verity_dp_ic/badge.svg)](https://docs.rs/verity_dp_ic)

`verity_dp_ic` provides libraries for verifying and processing data from Verity within the Internet Computer.

For more information about the Verity Data Processor (VDP), visit the [VDP repository](https://github.com/usherlabs/verity-dp).

## Publishing the Library to crates.io

1. **Update Version in `Cargo.toml`:**

   - Modify the `version` field in `Cargo.toml` to the new package version.

2. **Commit Changes:**

   - Ensure your git stage is clean by committing all recent code changes.

3. **Create a Git Tag:**

   - Use the format `vX.X.X` (e.g., `v1.1.2`, `v0.1.3`).
   - Ensure this version matches the one in `Cargo.toml`.

4. **Push the Tag:**

   - Push the tag to trigger the GitHub Action for publishing:
     ```bash
     git push origin vX.X.X
     ```

5. **Monitor the Process:**

   - Track the GitHub Action progress [here](https://github.com/usherlabs/verity-dp/actions).
