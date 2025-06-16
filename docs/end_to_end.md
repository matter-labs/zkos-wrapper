# Running end to end wrapper

Firstly, you should get these three files:
- base.bin - file that contains your base program in binary format
- risc_proof.json - file that contains the proof of the last airbender circuit (use airbender cli to generate it)
- setup.key - file that contains the trusted setup for the final Snark (see [Crs](./overview.md##Crs) section)

### Running full wrapper

Now you can generate the final Snark proof and VK with the following command:
```bash
cargo run --bin wrapper --release -- prove --input-binary {path to base.bin} --input {path to risc_proof.json} --trusted-setup-file {path to setup.key} --output-dir {output directory}
```

### Generating final Snark VK

With this command you can generate the final Snark VK:
```bash
cargo run --bin wrapper --release -- generate-vk --input-binary {path to base.bin} --trusted-setup-file {path to setup.key} --output-dir {output directory}
```

Generated VK can be put into the solidity code.
Or you can use era-boojum-verifier-cli tool to verify the proofs manually.

### Regenerating wrapper
If you need to regenerate the wrapper, due to changes in the last airbender recursion circuit you can use the following command:
```bash
cargo run --bin wrapper_generator --release
```
