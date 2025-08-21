# Running end to end wrapper

Firstly, you should get these three files:
- base_app.bin - file that contains your base program in binary format
- risc_proof.json - file that contains the proof of the last airbender circuit (use airbender cli to generate it)
- setup.key - file that contains the trusted setup for the final Snark (see [Crs](./overview.md##Crs) section)

### Running wrapper

Now you can generate the RiscWrapper proof and VK with the following command:
```bash
cargo run --bin wrapper --release -- prove-risc-wrapper --input-binary {path to base_app.bin} --input {path to risc_proof.json} --output-dir {output directory}
```

Also you can run the full pipeline (risc_wrapper + compression + snark_wrapper) with the following command:
```bash
cargo run --bin wrapper --release -- prove-full --input-binary {path to base_app.bin} --input {path to risc_proof.json} --trusted-setup-file {path to setup.key} --output-dir {output directory}
```
**Note:** if `--trusted-setup-file` is missing wrapper will use the fake trusted setup, which **must not** be used for production cases, but can be used **only** for local testing.

### Generating final Snark VK

With this command you can generate the RiscWrapper VK:
```bash
cargo run --bin wrapper --release -- generate-risc-wrapper-vk --input-binary {path to base_app.bin} --output-dir {output directory}
```

To generate the final Snark VK you can use the following command:
```bash
cargo run --bin wrapper --release -- generate-snark-vk --input-binary {path to base_app.bin} --trusted-setup-file {path to setup.key} --output-dir {output directory}
```

Generated VK can be put into the solidity code.
Or you can use era-boojum-verifier-cli tool to verify the proofs manually.

**Note:** there are two versions of verifier programs set you can use for recursion: common or universal. See [execution_utils](https://github.com/matter-labs/zksync-airbender/blob/6a49503916f046d091e1f7134d80fe037ace8ec6/execution_utils/src/lib.rs#L29C1-L49C72). To activate universal verifier, you should add `--universal-verifier` flag to the VK generation commands above. Proving commands can identify used type from `risc_proof.json`, so you only need to specify it during VK generation.

### Regenerating wrapper
If you need to regenerate the wrapper, due to changes in the last airbender recursion circuit you can use the following command:
```bash
cargo run --bin wrapper_generator --release
```


## GPU

To use gpu, you must:
* compile with gpu feature flag
* install BELLMAN_CUDA (and set BELLMAN_CUDA_DIR=)
* download the compact CRS file (4GB) (https://storage.googleapis.com/matterlabs-setup-keys-us/setup-keys/setup_compact.key)


For example, you can snark-wrap the airbender risk program proof:

```shell
time CUDA_VISIBLE_DEVICES=0 RUST_BACKTRACE=1 RUST_MIN_STACK=267108864 cargo run --bin wrapper --release  prove-full --input wrapper/testing_data/risc_proof --trusted-setup-file crs/setup_compact.key  --output-dir /tmp/snark_output
```
