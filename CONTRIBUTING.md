# Contribution guidelines

Thank you for considering contributing to metalloprotein!

If you'd like to make a significant contribution, please open an issue to discuss the change with the maintainers and other developers.

## Pull requests

Please keep unrelated changes in separate pull requests.

### Updating the changelog

Before merging a PR, please update the [CHANGELOG] to reflect the changes you've
made. Please document your contribution under one of the following headings in
the **Unreleased** section:

- `Added` for new features.
- `Changed` for changes in existing functionality.
- `Deprecated` for soon-to-be removed features.
- `Removed` for now removed features.
- `Fixed` for any bug fixes.
- `Security` in case of vulnerabilities.

If the required subsection does not exist yet under **Unreleased**, create it!
For more information, see [Keep a changelog](https://keepachangelog.com/en/1.0.0/)

[CHANGELOG]: (https://github.com/yoshanuikabundi/metalloprotein/blob/main/CHANGELOG.md)

## Developing

### Set up

This is no different than other Rust projects.

```shell
git clone https://github.com/yoshanuikabundi/metalloprotein
cd metalloprotein
cargo test
```

### Useful Commands

- Build and run release version:

  ```shell
  cargo build --release && cargo run --release
  ```

- Run Clippy:

  ```shell
  cargo clippy --all-targets --all-features --workspace
  ```

- Run all tests:

  ```shell
  cargo test --all-features --workspace
  ```

- Check to see if there are code formatting issues

  ```shell
  cargo fmt --all -- --check
  ```

- Format the code in the project

  ```shell
  cargo fmt --all
  ```
