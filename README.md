# metalloprotein

[![Crates.io](https://img.shields.io/crates/v/metalloprotein.svg)](https://crates.io/crates/metalloprotein)
[![Docs.rs](https://docs.rs/metalloprotein/badge.svg)](https://docs.rs/metalloprotein)
[![CI](https://github.com/yoshanuikabundi/metalloprotein/workflows/CI/badge.svg)](https://github.com/yoshanuikabundi/metalloprotein/actions)
[![Coverage Status](https://coveralls.io/repos/github/yoshanuikabundi/metalloprotein/badge.svg?branch=main)](https://coveralls.io/github/yoshanuikabundi/metalloprotein?branch=main)

## Installation

Clone the repo, then build natively:

```sh
cargo build
```

or for WASM:

```
cargo build --target wasm32-unknown-unknown
```

If `wasm-server-runner` is installed, `cargo run --target wasm32-unknown-unknown` will use it to serve the WASM version.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

See [CONTRIBUTING.md](CONTRIBUTING.md).
