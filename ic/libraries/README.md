# verity_dp_ic ![License](https://img.shields.io/crates/l/verity_dp_ic) [![verity_dp_ic on crates.io](https://img.shields.io/crates/v/verity_dp_ic)](https://crates.io/crates/verity_dp_ic) [![verity_dp_ic on docs.rs](https://docs.rs/verity_dp_ic/badge.svg)](https://docs.rs/verity_dp_ic)

## Publishing the Library to crates.io

1. **Update Version in `Cargo.toml`:**
   - Update the `version` field in the `Cargo.toml` file to reflect the new package version.

2. **Commit Changes:**
   - Ensure the git stage is clean by committing the latest code changes to the repository.

3. **Create a Git Tag:**
   - Use the format `vX.X.X` (e.g., `v1.1.2`, `v0.1.3`).
   - Ensure the version matches the one specified in the updated `Cargo.toml` file.

4. **Push the Tag:**
   - Push the tag to trigger the GitHub Action for publishing the package:
     ```bash
     git push origin vX.X.X
     ```

5. **Monitor the Process:**
   - Track the progress of the GitHub Action [here](https://github.com/usherlabs/verity-dp/actions).
