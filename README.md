# Cargo-toml2

[![Crates.io](https://img.shields.io/crates/v/cargo-toml2.svg)](https://crates.io/crates/cargo-toml2)
![maintenance-as-is](https://img.shields.io/badge/maintenance-as--is-yellow.svg)

Stuff for parsing Cargo.toml files.

Should be fully accurate as per the [reference](https://doc.rust-lang.org/cargo/reference/manifest.html).

In the event the reference and the implementation diverge, I will make no attempt to fix anything until
the reference is properly updated.

Keeping track of undocumented changes is never a fun thing and I don't plan to do it. And it benefits no one to have a bunch of undocumented behaviour for `Cargo.toml`.

## Details

No validation or other work is performed on (De)Serialization.
It is up to you to:

* Ensure inputs are valid if you intend to Serialize.
* Set Default values for missing fields, if you want them.

Note that this is a pretty direct wrapper around the Cargo Manifest, and you should see the [Manifest Reference](https://doc.rust-lang.org/cargo/reference/manifest.html) if you want to know what things mean.

## Limitations

Due to current limitations in `toml-rs`, Dependencies may fail to write out with a `ValueAfterTable` error.
As a workaround, if possible remove or convert the `Dependency::Simple` variants to `Dependency::Full` ones.
The relevant issue is [#256](https://github.com/alexcrichton/toml-rs/issues/265)

## FAQ

* Q: What about `cargo-toml`?
* A: 🤷. It didn't work for my needs, so I wrote my own.

----

* Q: Why call it `cargo-toml2`?
* A: `Cargo-toml` was taken, and I'm unimaginative. It's a simple obvious name that clearly states its purpose, what else could I make it? Sorry. Bit late to change after it's published.

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
