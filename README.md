# zkos-wrapper
Library for verifying zkos with boojum

It takes the zkos proof (in json format), and returns the boojum proof.

You can use it as a library, or as a cli tool:

```
cargo run --release -- prove --input testing_data/risc_proof --output-dir tmp.json
```

## CRS file

For production - please make sure to use the real CRS file:

https://storage.googleapis.com/matterlabs-setup-keys-us/setup-keys/setup_2^24.key

The tool also offers to use the 'fake' file - that works for local testing, but must NEVER be used for production cases.

## Generating verification key


```shell
cargo run --release generate-vk --input-binary ../zksync-airbender/examples/hashed_fibonacci/app.bin --output-dir /tmp --trusted-setup-file crs/setup.key
```

This will generate the verification key (and verification key hash) - that can be put into the solidity code.
Or you can use era-boojum-verifier-cli tool to verify the proofs manually.