# `await!` -> `.await`

Migration tool for replacing `await!` macro with `.await` syntax.

## Usage

In this example, `replace-await` is cloned in the parent directory of your project, but adjust the path if necessary.

```shell
git clone https://github.com/taiki-e/replace-await.git
cd your-project-dir
cargo run --manifest-path ../replace-await/Cargo.toml **/*.rs  **/*.md
```

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
