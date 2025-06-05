use proc_macro2::TokenStream;
use prover::merkle_trees::MerkleTreeCapVarLength;
use prover::transcript::Blake2sBufferingTranscript;
use prover::transcript::blake2s_u32::BLAKE2S_DIGEST_SIZE_U32_WORDS;
use prover::worker::Worker;
use quote::quote;
use std::alloc::Global;

#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ExpectedFinalStateData {
    pub expected_final_pc: u32,
    pub setup_caps: Vec<MerkleTreeCapVarLength>,
}

pub fn generate_params_and_register_values(
    base_layer_bin: &[u8],
    first_recursion_layer_bin: &[u8],
    next_recursion_layer_bin: &[u8],
    first_final_recursion_bin: &[u8],
    next_final_recursion_bin: &[u8],
) -> (
    [u32; BLAKE2S_DIGEST_SIZE_U32_WORDS],
    [u32; BLAKE2S_DIGEST_SIZE_U32_WORDS],
) {
    let worker = Worker::new_with_num_threads(8);
    let end_params =
        generate_params_for_binary_and_machine(next_final_recursion_bin, MachineType::ReducedFinal);

    let aux_registers_values = compute_commitment_for_chain_of_programs(
        base_layer_bin,
        first_recursion_layer_bin,
        next_recursion_layer_bin,
        first_final_recursion_bin,
        &worker,
    );
    (end_params, aux_registers_values)
}

pub enum MachineType {
    Standard,
    Reduced,
    // Final reduced machine, used to generate a single proof at the end.
    ReducedFinal,
}

pub fn generate_params_for_binary_and_machine(
    bin: &[u8],
    machine: MachineType,
) -> [u32; BLAKE2S_DIGEST_SIZE_U32_WORDS] {
    let worker = Worker::new_with_num_threads(8);

    let expected_final_pc = execution_utils::find_binary_exit_point(&bin);
    let binary: Vec<u32> = execution_utils::get_padded_binary(&bin);
    match machine {
        MachineType::Standard => execution_utils::compute_end_parameters(
            expected_final_pc,
            &setups::get_main_riscv_circuit_setup::<Global, Global>(&binary, &worker),
        ),
        MachineType::Reduced => execution_utils::compute_end_parameters(
            expected_final_pc,
            &setups::get_reduced_riscv_circuit_setup::<Global, Global>(&binary, &worker),
        ),

        MachineType::ReducedFinal => execution_utils::compute_end_parameters(
            expected_final_pc,
            &setups::get_final_reduced_riscv_circuit_setup::<Global, Global>(&binary, &worker),
        ),
    }
}

pub fn generate_constants(
    base_layer_bin: &[u8],
    first_recursion_layer_bin: &[u8],
    next_recursion_layer_bin: &[u8],
    first_final_recursion_bin: &[u8],
    next_final_recursion_bin: &[u8],
) -> TokenStream {
    let (end_params, aux_registers_values) = generate_params_and_register_values(
        base_layer_bin,
        first_recursion_layer_bin,
        next_recursion_layer_bin,
        first_final_recursion_bin,
        next_final_recursion_bin,
    );

    let [
        end_params_0,
        end_params_1,
        end_params_2,
        end_params_3,
        end_params_4,
        end_params_5,
        end_params_6,
        end_params_7,
    ] = end_params;
    let [
        aux_registers_values_0,
        aux_registers_values_1,
        aux_registers_values_2,
        aux_registers_values_3,
        aux_registers_values_4,
        aux_registers_values_5,
        aux_registers_values_6,
        aux_registers_values_7,
    ] = aux_registers_values;

    quote! {
        pub(crate) const FINAL_RISC_CIRCUIT_END_PARAMS: [u32; #BLAKE2S_DIGEST_SIZE_U32_WORDS] = [
            #end_params_0,
            #end_params_1,
            #end_params_2,
            #end_params_3,
            #end_params_4,
            #end_params_5,
            #end_params_6,
            #end_params_7,
        ];

        pub(crate) const FINAL_RISC_CIRCUIT_AUX_REGISTERS_VALUES: [u32; #BLAKE2S_DIGEST_SIZE_U32_WORDS] = [
            #aux_registers_values_0,
            #aux_registers_values_1,
            #aux_registers_values_2,
            #aux_registers_values_3,
            #aux_registers_values_4,
            #aux_registers_values_5,
            #aux_registers_values_6,
            #aux_registers_values_7,
        ];
    }
}

/// blake(
///     blake(
///         blake([0u32; 8] || base_program_end_params)
///         || first_recursion_step_end_params)
///     || next_recursion_step_end_params
/// )
fn compute_commitment_for_chain_of_programs(
    base_layer_bin: &[u8],
    first_recursion_layer_bin: &[u8],
    next_recursion_layer_bin: &[u8],
    first_final_recursion_bin: &[u8],
    worker: &Worker,
) -> [u32; BLAKE2S_DIGEST_SIZE_U32_WORDS] {
    let base_layer_end_params =
        generate_params_for_binary_and_machine(base_layer_bin, MachineType::Standard);

    let first_recursion_layer_end_params =
        generate_params_for_binary_and_machine(first_recursion_layer_bin, MachineType::Reduced);

    let next_recursion_layer_end_params =
        generate_params_for_binary_and_machine(next_recursion_layer_bin, MachineType::Reduced);

    let first_final_recursion_end_params = generate_params_for_binary_and_machine(
        first_final_recursion_bin,
        MachineType::ReducedFinal,
    );

    compute_chain_encoding(vec![
        [0u32; BLAKE2S_DIGEST_SIZE_U32_WORDS],
        base_layer_end_params,
        first_recursion_layer_end_params,
        next_recursion_layer_end_params,
        first_final_recursion_end_params,
    ])
}

pub fn compute_chain_encoding(
    data: Vec<[u32; BLAKE2S_DIGEST_SIZE_U32_WORDS]>,
) -> [u32; BLAKE2S_DIGEST_SIZE_U32_WORDS] {
    let mut hasher = Blake2sBufferingTranscript::new();
    let mut previous = data[0];

    for index in 1..data.len() {
        // continue the chain, only if the data is different
        if data[index] != data[index - 1] {
            hasher.absorb(&previous);
            hasher.absorb(&data[index]);
            previous = hasher.finalize_reset().0;
        }
    }

    previous
}
