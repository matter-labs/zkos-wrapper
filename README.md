# zkos-wrapper
Library for verifying zkos with boojum

It takes the zkos proof (in json format), and returns the boojum proof.

Currently it also requires you to provide the file with final register inputs.

You can use it as a library, or as a cli tool:

```
cargo run --release -- --input testing_data/risc_proof  --registers-input testing_data/register_final_values --output tmp.json
```