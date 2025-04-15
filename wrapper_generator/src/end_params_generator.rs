use proc_macro2::TokenStream;
use prover::merkle_trees::MerkleTreeCapVarLength;
use prover::transcript::Blake2sBufferingTranscript;
use prover::transcript::blake2s_u32::BLAKE2S_DIGEST_SIZE_U32_WORDS;
use quote::quote;

#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ExpectedFinalStateData {
    pub expected_final_pc: u32,
    pub setup_caps: Vec<MerkleTreeCapVarLength>,
}

pub fn generate_constants(
    base_layer_data: &ExpectedFinalStateData,
    first_recursion_layer_data: &ExpectedFinalStateData,
    next_recursion_layer_data: &ExpectedFinalStateData,
    final_circuit_data: &ExpectedFinalStateData,
) -> TokenStream {
    let aux_registers_values = compute_commitment_for_chain_of_programs(
        base_layer_data,
        first_recursion_layer_data,
        next_recursion_layer_data,
    );

    let end_params = compute_end_params_output(final_circuit_data);

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
    base_layer_data: &ExpectedFinalStateData,
    first_recursion_layer_data: &ExpectedFinalStateData,
    next_recursion_layer_data: &ExpectedFinalStateData,
) -> [u32; BLAKE2S_DIGEST_SIZE_U32_WORDS] {
    let base_layer_end_params = compute_end_params_output(base_layer_data);
    let first_recursion_layer_end_params = compute_end_params_output(first_recursion_layer_data);
    let next_recursion_layer_end_params = compute_end_params_output(next_recursion_layer_data);

    let mut hasher = Blake2sBufferingTranscript::new();
    hasher.absorb(&[0u32; BLAKE2S_DIGEST_SIZE_U32_WORDS]);
    hasher.absorb(&base_layer_end_params);
    let tmp = hasher.finalize_reset().0;

    hasher.absorb(&tmp);
    hasher.absorb(&first_recursion_layer_end_params);
    let tmp = hasher.finalize_reset().0;

    hasher.absorb(&tmp);
    hasher.absorb(&next_recursion_layer_end_params);
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
