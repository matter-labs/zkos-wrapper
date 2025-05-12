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

    let expected_final_pc = execution_utils::find_binary_exit_point(next_final_recursion_bin);
    let binary = execution_utils::get_padded_binary(next_final_recursion_bin);
    let main_circuit_precomputations =
        setups::get_final_reduced_riscv_circuit_setup::<Global>(&binary, &worker);

    let end_params =
        execution_utils::compute_end_parameters(expected_final_pc, &main_circuit_precomputations);

    let aux_registers_values = compute_commitment_for_chain_of_programs(
        base_layer_bin,
        first_recursion_layer_bin,
        next_recursion_layer_bin,
        first_final_recursion_bin,
        &worker,
    );
    (end_params, aux_registers_values)
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
    let expected_final_pc = execution_utils::find_binary_exit_point(base_layer_bin);
    let binary = execution_utils::get_padded_binary(base_layer_bin);
    let main_circuit_precomputations =
        setups::get_main_riscv_circuit_setup::<Global>(&binary, &worker);
    let base_layer_end_params =
        execution_utils::compute_end_parameters(expected_final_pc, &main_circuit_precomputations);

    let expected_final_pc = execution_utils::find_binary_exit_point(first_recursion_layer_bin);
    let binary = execution_utils::get_padded_binary(first_recursion_layer_bin);
    let main_circuit_precomputations =
        setups::get_reduced_riscv_circuit_setup::<Global>(&binary, &worker);
    let first_recursion_layer_end_params =
        execution_utils::compute_end_parameters(expected_final_pc, &main_circuit_precomputations);

    let expected_final_pc = execution_utils::find_binary_exit_point(next_recursion_layer_bin);
    let binary = execution_utils::get_padded_binary(next_recursion_layer_bin);
    let main_circuit_precomputations =
        setups::get_reduced_riscv_circuit_setup::<Global>(&binary, &worker);
    let next_recursion_layer_end_params =
        execution_utils::compute_end_parameters(expected_final_pc, &main_circuit_precomputations);

    let expected_final_pc = execution_utils::find_binary_exit_point(first_final_recursion_bin);
    let binary = execution_utils::get_padded_binary(first_final_recursion_bin);
    let main_circuit_precomputations =
        setups::get_final_reduced_riscv_circuit_setup::<Global>(&binary, &worker);
    let first_final_recursion_end_params =
        execution_utils::compute_end_parameters(expected_final_pc, &main_circuit_precomputations);

    let mut hasher = Blake2sBufferingTranscript::new();
    hasher.absorb(&[0u32; BLAKE2S_DIGEST_SIZE_U32_WORDS]);
    hasher.absorb(&base_layer_end_params);
    let tmp = hasher.finalize_reset().0;

    hasher.absorb(&tmp);
    hasher.absorb(&first_recursion_layer_end_params);
    let tmp = hasher.finalize_reset().0;

    /// If second recursion layer is matching first - we should not apply the hash anymore.
    let tmp = if next_recursion_layer_end_params != first_recursion_layer_end_params {
        hasher.absorb(&tmp);
        hasher.absorb(&next_recursion_layer_end_params);
        hasher.finalize_reset().0
    } else {
        tmp
    };

    hasher.absorb(&tmp);
    hasher.absorb(&first_final_recursion_end_params);
    hasher.finalize_reset().0
}

/// blake(expected_final_pc || setup_caps_flattened)
fn compute_end_params_output(
    circuit_data: &ExpectedFinalStateData,
) -> [u32; BLAKE2S_DIGEST_SIZE_U32_WORDS] {
    let mut hasher = Blake2sBufferingTranscript::new();
    hasher.absorb(&[circuit_data.expected_final_pc]);

    unsafe {
        for cap in circuit_data.setup_caps.iter() {
            hasher.absorb(cap.cap.align_to::<u32>().1);
        }
    }

    hasher.finalize_reset().0
}
