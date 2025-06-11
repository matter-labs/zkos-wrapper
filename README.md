# zkos-wrapper
Library for verifying zkos with boojum

It takes the zkos proof (in json format), and returns the snark proof going through the following steps:

![diagram](diagram.svg)

You can use it as a library, or as a cli tool:

```
cargo run --bin wrapper --release -- prove --input testing_data/risc_proof --output-dir tmp.json
```

Also you should update generated files:
```
cargo run --bin wrapper_generator --release
```

## CRS file

For production - please make sure to use the real CRS file:

https://storage.googleapis.com/matterlabs-setup-keys-us/setup-keys/setup_2^24.key

The tool also offers to use the 'fake' file - that works for local testing, but must NEVER be used for production cases.

## Generating verification key


```shell
cargo run --bin wrapper_generator --release generate-vk --input-binary ../air_compiler/examples/hashed_fibonacci/app.bin --output-dir /tmp --trusted-setup-file crs/setup.key --universal-verifier
```

This will generate the verification key (and verification key hash) - that can be put into the solidity code.
Or you can use era-boojum-verifier-cli tool to verify the proofs manually.

## Security

See [SECURITY.md](./SECURITY.md).

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

See [CONTRIBUTING.md](./CONTRIBUTING.md).