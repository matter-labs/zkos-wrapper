# zkos-wrapper
Library for verifying zkos with boojum

It takes the zkos proof (in json format), and returns the boojum proof.

Currently working only on the zkos delegation proofs.

You can use it as a library, or as a cli tool:

```
cargo run --release -- --input delegation_proof --output boojum_proof.json
```