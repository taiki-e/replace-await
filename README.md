# `await!` -> `.await`

Migration tool for replacing `await!` macro with `await` syntax.

## Usage

In this example, `replace-await` is cloned in the parent directory of
`your-project-dir`, but adjust the path if necessary.

```shell
git clone https://github.com/taiki-e/replace-await.git
cd your-project-dir
cargo run --release --manifest-path ../replace-await/Cargo.toml **/*.rs **/*.md
```

See [rust-lang/futures-rs#1583](https://github.com/rust-lang/futures-rs/pull/1583)
for the conversion that this tool did.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
