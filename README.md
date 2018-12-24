# Cargo-toml2

[![Crates.io](https://img.shields.io/crates/v/cargo-toml2.svg)](https://crates.io/crates/cargo-toml2)
![maintenance-as-is](https://img.shields.io/badge/maintenance-as--is-yellow.svg)

Stuff for parsing Cargo.toml files.

Should be fully accurate as per the [reference](https://doc.rust-lang.org/cargo/reference/manifest.html).
In the event the reference and the implementation diverge, I will make no attempt to fix anything until
the reference is properly updated.
Keeping track of undocumented changes is not something I will do.

## Details

Minimal to no validation is performed on (De)serialization.
It is up to you to ensure inputs are valid if you intend to Serialize.

One thing this library will do for you, however, is setup default values.
That is, if you parse a `Cargo.toml` and it doesn't define every possible option itself, the default values that cargo assumes will be there.

## FAQ

* Q: What about `cargo-toml`?
* A: 🤷.

* Q: Why call it `cargo-toml2`?
* A: `Cargo-toml` was taken, and I'm unimaginative. It's a simple obvious name that clearly states its purpose, what else could I make it? Sorry. Bit late to change after it's published.

## License

Licensed under either of

* Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0)>
* MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT)>

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
