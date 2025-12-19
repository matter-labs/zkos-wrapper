# zkos-wrapper

Library for verifying zkos with boojum

It takes the zkos proof (in json format), and returns the snark proof going through the following steps:

![diagram](diagram.svg)

## Docs

See the [docs](./docs/README.md).

## Security

See [SECURITY.md](./SECURITY.md).

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

See [CONTRIBUTING.md](./CONTRIBUTING.md).

### Updating airbender version

If you want to update the airbender dependency, please do these additional steps:

- update Cargo.toml
- re-generate quotients - this can be done by running wrapper_generator: `cargo run --release --bin wrapper_generator`
- update test inputs: from the corresponding airbender branch:

```shell
# From zksync-airbender directory
cargo run -p cli --release prove --bin examples/hashed_fibonacci/app.bin --input-file examples/hashed_fibonacci/input.txt  --until final-proof --output-dir /tmp/update

# From zkos-wrapper directory.
cp /tmp/update/final_program_proof.json wrapper/testing_data/risc_proof
RUST_MIN_STACK=67108864 cargo test all_layers_full_test --release -- --nocapture
```
