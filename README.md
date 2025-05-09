# zkos-wrapper
Library for verifying zkos with boojum

It takes the zkos proof (in json format), and returns the boojum proof.

You can use it as a library, or as a cli tool:

```
cargo run --release -- --input testing_data/risc_proof --output-dir tmp.json
```