#![feature(allocator_api)]
#![feature(array_chunks)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

mod blake2_inner_verifier;
pub mod circuits;
pub mod transcript;
mod wrapper_inner_verifier;
use proof_compression::{
    PlonkSnarkWrapper, ProofSystemDefinition, SnarkWrapperProofSystem, SnarkWrapperSetupData,
    SnarkWrapperStep, hardcoded_canonical_g2_bases,
};
//pub mod wrapper_utils;
pub use wrapper_utils;

use proof_compression::serialization::{
    GenericWrapper, PlonkSnarkVerifierCircuitDeviceSetupWrapper,
};

#[cfg(test)]
mod tests;

#[cfg(feature = "wrap_final_machine")]
pub use final_risc_verifier as risc_verifier;
#[cfg(feature = "wrap_with_reduced_log_23")]
pub use reduced_risc_verifier as risc_verifier;

use circuits::*;
use shivini::boojum::algebraic_props::round_function::AbsorptionModeOverwrite;
use shivini::boojum::algebraic_props::sponge::GoldilocksPoseidon2Sponge;
use shivini::boojum::config::{DevCSConfig, ProvingCSConfig, SetupCSConfig};
use shivini::boojum::cs::cs_builder::new_builder;
use shivini::boojum::cs::cs_builder_reference::CsReferenceImplementationBuilder;
use shivini::boojum::cs::implementations::hints::DenseVariablesCopyHint;
use shivini::boojum::cs::implementations::hints::DenseWitnessCopyHint;
use shivini::boojum::cs::implementations::polynomial_storage::SetupBaseStorage;
use shivini::boojum::cs::implementations::polynomial_storage::SetupStorage;
use shivini::boojum::cs::implementations::pow::NoPow;
use shivini::boojum::cs::implementations::proof::Proof;
use shivini::boojum::cs::implementations::prover::ProofConfig;
use shivini::boojum::cs::implementations::setup::FinalizationHintsForProver;
use shivini::boojum::cs::implementations::transcript::GoldilocksPoisedon2Transcript;
use shivini::boojum::cs::implementations::verifier::{
    VerificationKey, VerificationKeyCircuitGeometry,
};
use shivini::boojum::cs::oracle::merkle_tree::MerkleTreeWithCap;
use shivini::boojum::cs::traits::circuit::CircuitBuilder;
use shivini::boojum::cs::traits::circuit::CircuitBuilderProxy;
use shivini::boojum::gadgets::recursion::recursive_transcript::CircuitAlgebraicSpongeBasedTranscript;
use shivini::boojum::gadgets::recursion::recursive_tree_hasher::CircuitGoldilocksPoseidon2Sponge;
use shivini::boojum::implementations::poseidon2::Poseidon2Goldilocks;
pub use shivini::boojum::worker::Worker as BoojumWorker;
use shivini::boojum::worker::*;
use shivini::cs::{GpuSetup, gpu_setup_and_vk_from_base_setup_vk_params_and_hints};
use shivini::gpu_proof_config::GpuProofConfig;
use shivini::{
    ProverContext, ProverContextConfig, gpu_proof_config, gpu_prove_from_external_witness_data,
};
use std::alloc::Global;
use std::io::Cursor;
use std::path::Path;
use wrapper_utils::verifier_traits::CircuitBlake2sForEverythingVerifier;
use zksync_gpu_prover::AsyncSetup;
use zksync_gpu_prover::bellman::plonk::better_better_cs::cs::{
    Assembly, SynthesisModeGenerateSetup, SynthesisModeProve,
};

pub type GL = shivini::boojum::field::goldilocks::GoldilocksField;
pub type GLExt2 = shivini::boojum::field::goldilocks::GoldilocksExt2;
pub type RiscLeafInclusionVerifier = CircuitBlake2sForEverythingVerifier<GL>;
pub type RiscWrapper = RiscWrapperCircuit<GL, RiscLeafInclusionVerifier>;
pub type RiscWrapperTranscript =
    shivini::boojum::cs::implementations::transcript::GoldilocksPoisedon2Transcript;
pub type RiscWrapperTreeHasher = GoldilocksPoseidon2Sponge<AbsorptionModeOverwrite>;
pub type RiscWrapperProof = Proof<GL, RiscWrapperTreeHasher, GLExt2>;
pub type RiscWrapperVK = VerificationKey<GL, RiscWrapperTreeHasher>;

pub type CircuitRiscWrapperTranscript =
    CircuitAlgebraicSpongeBasedTranscript<GL, 8, 12, 4, Poseidon2Goldilocks>;
pub type CircuitRiscWrapperTreeHasher = CircuitGoldilocksPoseidon2Sponge;

pub type RiscWrapperCircuitBuilder = CircuitBuilderProxy<GL, RiscWrapper>;

pub use shivini::circuit_definitions::snark_wrapper;
pub use shivini::circuit_definitions::snark_wrapper::franklin_crypto::bellman;
pub use shivini::circuit_definitions::snark_wrapper::rescue_poseidon;

pub use rescue_poseidon::franklin_crypto::bellman::pairing::bn256::{Bn256, Fr};
use rescue_poseidon::poseidon2::Poseidon2Sponge;
use rescue_poseidon::poseidon2::transcript::Poseidon2Transcript;

use snark_wrapper::implementations::poseidon2::tree_hasher::AbsorptionModeReplacement;

pub type CompressionPoW = Poseidon2Sponge<Bn256, GL, AbsorptionModeReplacement<Fr>, 2, 3>;
pub type CompressionTreeHasher = Poseidon2Sponge<Bn256, GL, AbsorptionModeReplacement<Fr>, 2, 3>;
pub type CompressionTranscript =
    Poseidon2Transcript<Bn256, GL, AbsorptionModeReplacement<Fr>, 2, 3>;

pub type CompressionProof = Proof<GL, CompressionTreeHasher, GLExt2>;
pub type CompressionVK = VerificationKey<GL, CompressionTreeHasher>;

pub type CompressionCircuitBuilder = CircuitBuilderProxy<GL, CompressionCircuit>;

use bellman::kate_commitment::{Crs, CrsForMonomialForm};
use bellman::plonk::better_better_cs::cs::Circuit as SnarkCircuit;
use bellman::plonk::better_better_cs::cs::PlonkCsWidth4WithNextStepAndCustomGatesParams;
use bellman::plonk::better_better_cs::cs::{ProvingAssembly, SetupAssembly};
use bellman::plonk::better_better_cs::gates::selector_optimized_with_d_next::SelectorOptimizedWidth4MainGateWithDNext;
use bellman::plonk::better_better_cs::verifier::verify as verify_snark;
use shivini::boojum::ethereum_types::H256;
use snark_wrapper::implementations::poseidon2::CircuitPoseidon2Sponge;
use snark_wrapper::implementations::poseidon2::transcript::CircuitPoseidon2Transcript;

use bellman::worker::Worker as BellmanWorker;

pub type CircuitCompressionTreeHasher = CircuitPoseidon2Sponge<Bn256, 2, 3, 3, true>;
pub type CircuitCompressionTranscript = CircuitPoseidon2Transcript<Bn256, 2, 3, 3, true>;

pub type SnarkWrapperProof =
    bellman::plonk::better_better_cs::proof::Proof<Bn256, SnarkWrapperCircuit>;
pub type SnarkWrapperVK =
    bellman::plonk::better_better_cs::setup::VerificationKey<Bn256, SnarkWrapperCircuit>;
pub type SnarkWrapperSetup =
    bellman::plonk::better_better_cs::setup::Setup<Bn256, SnarkWrapperCircuit>;
pub type SnarkWrapperTranscript =
    bellman::plonk::commitments::transcript::keccak_transcript::RollingKeccakTranscript<Fr>;

pub use execution_utils::ProgramProof;
use execution_utils::{
    base_layer_verifier_vk, final_recursion_layer_verifier_vk,
    recursion_layer_no_delegation_verifier_vk, recursion_layer_verifier_vk,
    recursion_log_23_layer_verifier_vk, universal_circuit_no_delegation_verifier_vk,
    universal_circuit_verifier_vk,
};
//CircuitAlgebraicSpongeBasedTranscript<GoldilocksField, 8, 12, 4, R>,

// RiscV -> Stark Wrapper
pub fn get_risc_wrapper_setup(
    worker: &Worker,
    binary_commitment: BinaryCommitment,
) -> (
    FinalizationHintsForProver,
    SetupBaseStorage<GL>,
    //SetupStorage<GL>,
    //RiscWrapperVK,
    VerificationKeyCircuitGeometry,
    //MerkleTreeWithCap<GL, RiscWrapperTreeHasher>,
    DenseVariablesCopyHint,
    DenseWitnessCopyHint,
) {
    let verify_inner_proof: bool = false;
    let circuit = RiscWrapper::new(None, verify_inner_proof, binary_commitment);

    let geometry = RiscWrapper::geometry();
    let (max_trace_len, num_vars) = circuit.size_hint();

    let builder_impl = CsReferenceImplementationBuilder::<GL, GL, SetupCSConfig>::new(
        geometry,
        max_trace_len.unwrap(),
    );
    let builder = new_builder::<_, GL>(builder_impl);

    let builder = RiscWrapper::configure_builder(builder);
    let mut cs = builder.build(num_vars.unwrap());
    circuit.add_tables(&mut cs);
    circuit.synthesize_into_cs(&mut cs);
    let (_, finalization_hint) = cs.pad_and_shrink();

    let ProofConfig {
        fri_lde_factor,
        merkle_tree_cap_size,
        ..
    } = RiscWrapper::get_proof_config();
    let cs = cs.into_assembly::<std::alloc::Global>();

    // TODO: for gpu we don't need full setup (light setup is enough)

    let (setup_base, vk_params, vars_hint, witness_hints) =
        cs.get_light_setup(worker, fri_lde_factor, merkle_tree_cap_size);

    //let (setup_base, setup, vk, setup_tree, vars_hint, witness_hints) =
    //    cs.get_full_setup::<RiscWrapperTreeHasher>(worker, fri_lde_factor, merkle_tree_cap_size);

    (
        finalization_hint,
        setup_base,
        //setup,
        vk_params,
        //setup_tree,
        vars_hint,
        witness_hints,
    )
}

pub fn prove_risc_wrapper(
    risc_wrapper_witness: RiscWrapperWitness,
    finalization_hint: &FinalizationHintsForProver,
    setup_base: &SetupBaseStorage<GL>,
    //setup: &SetupStorage<GL>,
    //vk: &RiscWrapperVK,
    vk: &VerificationKeyCircuitGeometry,
    //setup_tree: &MerkleTreeWithCap<GL, RiscWrapperTreeHasher>,
    vars_hint: &DenseVariablesCopyHint,
    witness_hints: &DenseWitnessCopyHint,
    worker: &Worker,
    binary_commitment: BinaryCommitment,
) -> (RiscWrapperProof, RiscWrapperVK) {
    println!("======== STARTING GPU ===========");
    let verify_inner_proof = true;
    let circuit = RiscWrapper::new(
        Some(risc_wrapper_witness),
        verify_inner_proof,
        binary_commitment,
    );

    let geometry = RiscWrapper::geometry();
    let (max_trace_len, num_vars) = circuit.size_hint();

    let builder_impl = CsReferenceImplementationBuilder::<GL, GL, ProvingCSConfig>::new(
        geometry,
        max_trace_len.unwrap(),
    );
    let builder = new_builder::<_, GL>(builder_impl);

    let builder = RiscWrapper::configure_builder(builder);
    let mut cs = builder.build(num_vars.unwrap());
    circuit.add_tables(&mut cs);
    circuit.synthesize_into_cs(&mut cs);
    cs.pad_and_shrink_using_hint(finalization_hint);
    let cs = cs.into_assembly::<std::alloc::Global>();

    let proof_config = RiscWrapper::get_proof_config();

    /*let proof1 = cs
    .prove_from_precomputations::<GLExt2, RiscWrapperTranscript, RiscWrapperTreeHasher, NoPow>(
        proof_config,
        &setup_base,
        &setup,
        &setup_tree,
        &vk,
        &vars_hint,
        &witness_hints,
        (),
        worker,
    );*/

    let gpu_proof_config = GpuProofConfig::from_assembly(&cs);

    let external_witness_data = cs.witness.unwrap();

    //let vk_params = vk.fixed_parameters.clone();

    let (gpu_setup, gpu_vk) = gpu_setup_and_vk_from_base_setup_vk_params_and_hints(
        setup_base.clone(),
        vk.clone(),
        vars_hint.clone(),
        witness_hints.clone(),
        &worker,
    )
    .unwrap();

    type Transcript = GoldilocksPoisedon2Transcript;
    type Hasher = GoldilocksPoseidon2Sponge<AbsorptionModeOverwrite>;

    let proof2 = gpu_prove_from_external_witness_data::<Transcript, Hasher, NoPow, Global>(
        &gpu_proof_config,      // normally taken from 'verifier' - but I don't have any.
        &external_witness_data, // witness vector (I have it as struct, not bytes)
        proof_config,           // LDE factors and other stuff.
        &gpu_setup,
        &gpu_vk, // vk should be fine
        (),      // empty shoudl be ok
        worker,  // ok
    )
    .unwrap();

    (proof2.into(), gpu_vk)
}

pub fn verify_risc_wrapper_proof(proof: &RiscWrapperProof, vk: &RiscWrapperVK) -> bool {
    let verifier_builder = RiscWrapperCircuitBuilder::dyn_verifier_builder();
    let verifier = verifier_builder.create_verifier();
    verifier.verify::<RiscWrapperTreeHasher, RiscWrapperTranscript, NoPow>((), vk, proof)
}

// Stark -> Stark Compression
pub fn get_compression_setup(
    risc_wrapper_vk: RiscWrapperVK,
    worker: &Worker,
) -> (
    FinalizationHintsForProver,
    SetupBaseStorage<GL>,
    SetupStorage<GL>,
    CompressionVK,
    MerkleTreeWithCap<GL, CompressionTreeHasher>,
    DenseVariablesCopyHint,
    DenseWitnessCopyHint,
) {
    let verify_inner_proof: bool = false;
    let circuit = CompressionCircuit::new(None, risc_wrapper_vk, verify_inner_proof);

    let geometry = CompressionCircuit::geometry();
    let (max_trace_len, num_vars) = circuit.size_hint();

    let builder_impl = CsReferenceImplementationBuilder::<GL, GL, SetupCSConfig>::new(
        geometry,
        max_trace_len.unwrap(),
    );
    let builder = new_builder::<_, GL>(builder_impl);

    let builder = CompressionCircuit::configure_builder(builder);
    let mut cs = builder.build(num_vars.unwrap());
    circuit.synthesize_into_cs(&mut cs);
    let (_, finalization_hint) = cs.pad_and_shrink();

    let ProofConfig {
        fri_lde_factor,
        merkle_tree_cap_size,
        ..
    } = RiscWrapper::get_proof_config();
    let cs = cs.into_assembly::<std::alloc::Global>();

    let (setup_base, setup, vk, setup_tree, vars_hint, witness_hints) =
        cs.get_full_setup::<CompressionTreeHasher>(worker, fri_lde_factor, merkle_tree_cap_size);

    (
        finalization_hint,
        setup_base,
        setup,
        vk,
        setup_tree,
        vars_hint,
        witness_hints,
    )
}

pub fn prove_compression(
    risc_wrapper_proof: RiscWrapperProof,
    risc_wrapper_vk: RiscWrapperVK,
    finalization_hint: &FinalizationHintsForProver,
    setup_base: &SetupBaseStorage<GL>,
    setup: &SetupStorage<GL>,
    vk: &CompressionVK,
    setup_tree: &MerkleTreeWithCap<GL, CompressionTreeHasher>,
    vars_hint: &DenseVariablesCopyHint,
    witness_hints: &DenseWitnessCopyHint,
    worker: &Worker,
) -> CompressionProof {
    let verify_inner_proof = true;
    let circuit = CompressionCircuit::new(
        Some(risc_wrapper_proof),
        risc_wrapper_vk,
        verify_inner_proof,
    );

    let geometry = CompressionCircuit::geometry();
    let (max_trace_len, num_vars) = circuit.size_hint();

    let builder_impl = CsReferenceImplementationBuilder::<GL, GL, ProvingCSConfig>::new(
        geometry,
        max_trace_len.unwrap(),
    );
    let builder = new_builder::<_, GL>(builder_impl);

    let builder = CompressionCircuit::configure_builder(builder);
    let mut cs = builder.build(num_vars.unwrap());
    circuit.synthesize_into_cs(&mut cs);
    cs.pad_and_shrink_using_hint(finalization_hint);
    let cs = cs.into_assembly::<std::alloc::Global>();

    let proof_config = CompressionCircuit::get_proof_config();

    cs.prove_from_precomputations::<GLExt2, CompressionTranscript, CompressionTreeHasher, NoPow>(
        proof_config,
        &setup_base,
        &setup,
        &setup_tree,
        vk,
        &vars_hint,
        &witness_hints,
        (),
        worker,
    )
}

pub fn prove_compression_with_gpu(
    risc_wrapper_proof: RiscWrapperProof,
    risc_wrapper_vk: RiscWrapperVK,
    worker: &Worker,
) -> (CompressionProof, CompressionVK) {
    let verify_inner_proof: bool = false;
    let circuit = CompressionCircuit::new(None, risc_wrapper_vk.clone(), verify_inner_proof);

    let geometry = CompressionCircuit::geometry();
    let (max_trace_len, num_vars) = circuit.size_hint();

    let builder_impl = CsReferenceImplementationBuilder::<GL, GL, SetupCSConfig>::new(
        geometry,
        max_trace_len.unwrap(),
    );
    let builder = new_builder::<_, GL>(builder_impl);

    let builder = CompressionCircuit::configure_builder(builder);
    let mut cs = builder.build(num_vars.unwrap());
    circuit.synthesize_into_cs(&mut cs);
    let (_, finalization_hint) = cs.pad_and_shrink();

    let ProofConfig {
        fri_lde_factor,
        merkle_tree_cap_size,
        ..
    } = RiscWrapper::get_proof_config();
    let cs = cs.into_assembly::<std::alloc::Global>();

    let (setup_base, vk_params, vars_hint, witness_hints) =
        cs.get_light_setup(worker, fri_lde_factor, merkle_tree_cap_size);

    let verify_inner_proof = true;
    let circuit = CompressionCircuit::new(
        Some(risc_wrapper_proof),
        risc_wrapper_vk,
        verify_inner_proof,
    );

    let geometry = CompressionCircuit::geometry();
    let (max_trace_len, num_vars) = circuit.size_hint();

    let builder_impl = CsReferenceImplementationBuilder::<GL, GL, ProvingCSConfig>::new(
        geometry,
        max_trace_len.unwrap(),
    );
    let builder = new_builder::<_, GL>(builder_impl);

    let builder = CompressionCircuit::configure_builder(builder);
    let mut cs = builder.build(num_vars.unwrap());
    circuit.synthesize_into_cs(&mut cs);
    cs.pad_and_shrink_using_hint(&finalization_hint);
    let cs = cs.into_assembly::<std::alloc::Global>();

    let gpu_proof_config = GpuProofConfig::from_assembly(&cs);

    let external_witness_data = cs.witness.unwrap();

    let proof_config = CompressionCircuit::get_proof_config();

    /*cs.prove_from_precomputations::<GLExt2, CompressionTranscript, CompressionTreeHasher, NoPow>(
        proof_config,
        &setup_base,
        &setup,
        &setup_tree,
        vk,
        &vars_hint,
        &witness_hints,
        (),
        worker,
    )*/

    let (gpu_setup, gpu_vk) =
        gpu_setup_and_vk_from_base_setup_vk_params_and_hints::<CompressionTreeHasher, _>(
            setup_base.clone(),
            vk_params.clone(),
            vars_hint.clone(),
            witness_hints.clone(),
            &worker,
        )
        .unwrap();

    type Transcript = CompressionTranscript;
    type Hasher = CompressionTreeHasher;

    let proof2 = gpu_prove_from_external_witness_data::<Transcript, Hasher, NoPow, Global>(
        &gpu_proof_config,      // normally taken from 'verifier' - but I don't have any.
        &external_witness_data, // witness vector (I have it as struct, not bytes)
        proof_config,           // LDE factors and other stuff.
        &gpu_setup,
        &gpu_vk, // vk should be fine
        (),      // empty shoudl be ok
        worker,  // ok
    )
    .unwrap();

    (proof2.into(), gpu_vk)
}

pub fn verify_compression_proof(proof: &CompressionProof, vk: &CompressionVK) -> bool {
    let verifier_builder = CompressionCircuitBuilder::dyn_verifier_builder();
    let verifier = verifier_builder.create_verifier();
    verifier.verify::<CompressionTreeHasher, CompressionTranscript, NoPow>((), vk, proof)
}

// Stark -> Snark Wrapper
pub const L1_VERIFIER_DOMAIN_SIZE_LOG: usize = 24;

pub fn get_snark_wrapper_setup(
    compression_vk: CompressionVK,
    crs_mons: &Crs<Bn256, CrsForMonomialForm>,
    worker: &BellmanWorker,
) -> (SnarkWrapperSetup, SnarkWrapperVK) {
    let mut assembly = SetupAssembly::<
        Bn256,
        PlonkCsWidth4WithNextStepAndCustomGatesParams,
        SelectorOptimizedWidth4MainGateWithDNext,
    >::new();

    let fixed_parameters = compression_vk.fixed_parameters.clone();

    let wrapper_function = SnarkWrapperFunction;
    let wrapper_circuit = SnarkWrapperCircuit {
        witness: None,
        vk: compression_vk,
        fixed_parameters,
        transcript_params: (),
        wrapper_function,
    };

    wrapper_circuit.synthesize(&mut assembly).unwrap();

    assembly.finalize_to_size_log_2(L1_VERIFIER_DOMAIN_SIZE_LOG);
    assert!(assembly.is_satisfied());

    let snark_setup = assembly
        .create_setup::<SnarkWrapperCircuit>(worker)
        .unwrap();

    let snark_vk = SnarkWrapperVK::from_setup(&snark_setup, &worker, &crs_mons).unwrap();

    (snark_setup, snark_vk)
}

pub fn prove_snark_wrapper(
    compression_proof: CompressionProof,
    compression_vk: CompressionVK,
    snark_setup: &SnarkWrapperSetup,
    crs_mons: &Crs<Bn256, CrsForMonomialForm>,
    worker: &BellmanWorker,
) -> SnarkWrapperProof {
    let mut assembly = ProvingAssembly::<
        Bn256,
        PlonkCsWidth4WithNextStepAndCustomGatesParams,
        SelectorOptimizedWidth4MainGateWithDNext,
    >::new();

    let fixed_parameters = compression_vk.fixed_parameters.clone();

    let wrapper_function = SnarkWrapperFunction;
    let wrapper_circuit = SnarkWrapperCircuit {
        witness: Some(compression_proof),
        vk: compression_vk,
        fixed_parameters,
        transcript_params: (),
        wrapper_function,
    };

    wrapper_circuit.synthesize(&mut assembly).unwrap();

    assembly.finalize_to_size_log_2(L1_VERIFIER_DOMAIN_SIZE_LOG);
    assert!(assembly.is_satisfied());

    let snark_proof = assembly
        .create_proof::<SnarkWrapperCircuit, SnarkWrapperTranscript>(
            worker,
            &snark_setup,
            &crs_mons,
            None,
        )
        .unwrap();

    snark_proof
}

pub fn verify_snark_wrapper_proof(proof: &SnarkWrapperProof, vk: &SnarkWrapperVK) -> bool {
    verify_snark::<Bn256, SnarkWrapperCircuit, SnarkWrapperTranscript>(vk, proof, None).unwrap()
}

// TODO: this should probably be moved somewhere into crypto.
pub fn calculate_verification_key_hash(verification_key: SnarkWrapperVK) -> H256 {
    use bellman::compact_bn256::Fq;
    use bellman::{CurveAffine, PrimeField, PrimeFieldRepr};
    use sha3::{Digest, Keccak256};

    let mut res = vec![];

    // gate setup commitments
    assert_eq!(8, verification_key.gate_setup_commitments.len());

    for gate_setup in verification_key.gate_setup_commitments {
        let (x, y) = gate_setup.as_xy();
        x.into_repr().write_be(&mut res).unwrap();
        y.into_repr().write_be(&mut res).unwrap();
    }

    // gate selectors commitments
    assert_eq!(2, verification_key.gate_selectors_commitments.len());

    for gate_selector in verification_key.gate_selectors_commitments {
        let (x, y) = gate_selector.as_xy();
        x.into_repr().write_be(&mut res).unwrap();
        y.into_repr().write_be(&mut res).unwrap();
    }

    // permutation commitments
    assert_eq!(4, verification_key.permutation_commitments.len());

    for permutation in verification_key.permutation_commitments {
        let (x, y) = permutation.as_xy();
        x.into_repr().write_be(&mut res).unwrap();
        y.into_repr().write_be(&mut res).unwrap();
    }

    // lookup selector commitment
    let lookup_selector = verification_key.lookup_selector_commitment.unwrap();
    let (x, y) = lookup_selector.as_xy();
    x.into_repr().write_be(&mut res).unwrap();
    y.into_repr().write_be(&mut res).unwrap();

    // lookup tables commitments
    assert_eq!(4, verification_key.lookup_tables_commitments.len());

    for table_commit in verification_key.lookup_tables_commitments {
        let (x, y) = table_commit.as_xy();
        x.into_repr().write_be(&mut res).unwrap();
        y.into_repr().write_be(&mut res).unwrap();
    }

    // table type commitment
    let lookup_table = verification_key.lookup_table_type_commitment.unwrap();
    let (x, y) = lookup_table.as_xy();
    x.into_repr().write_be(&mut res).unwrap();
    y.into_repr().write_be(&mut res).unwrap();

    // flag for using recursive part
    Fq::default().into_repr().write_be(&mut res).unwrap();

    let mut hasher = Keccak256::new();
    hasher.update(&res);
    let computed_vk_hash = hasher.finalize();

    H256::from_slice(&computed_vk_hash)
}

/// Uploads trusted setup file to the RAM
pub fn get_trusted_setup(crs_file_str: &String) -> Crs<Bn256, CrsForMonomialForm> {
    let crs_file_path = std::path::Path::new(crs_file_str);
    let crs_file = std::fs::File::open(&crs_file_path)
        .expect(format!("Trying to open CRS FILE: {:?}", crs_file_path).as_str());
    Crs::read(&crs_file).expect(format!("Trying to read CRS FILE: {:?}", crs_file_path).as_str())
}

pub fn serialize_to_file<T: serde::ser::Serialize>(content: &T, filename: &Path) {
    let src = std::fs::File::create(filename).unwrap();
    serde_json::to_writer_pretty(src, content).unwrap();
}

fn deserialize_from_file<T: serde::de::DeserializeOwned>(filename: &str) -> T {
    let src = std::fs::File::open(filename).unwrap();
    serde_json::from_reader(src).unwrap()
}

// pub fn create_binary_commitment(
//     binary_path: String,
//     expected_end_params: &[u32; 8],
// ) -> BinaryCommitment {
//     let bin = std::fs::read(binary_path).unwrap();

//     let worker = risc_verifier::prover::worker::Worker::new();

//     let expected_final_pc = execution_utils::find_binary_exit_point(&bin);
//     let binary: Vec<u32> = execution_utils::get_padded_binary(&bin);

//     let base_params = execution_utils::compute_end_parameters(
//         expected_final_pc,
//         &setups::get_main_riscv_circuit_setup::<Global, Global>(&binary, &worker),
//     );

//     // Check which verifier was used.
//     if universal_circuit_no_delegation_verifier_vk().params == *expected_end_params {
//         let layers = vec![
//             [0u32; 8],
//             base_params,
//             universal_circuit_verifier_vk().params,
//             universal_circuit_no_delegation_verifier_vk().params,
//         ];
//         BinaryCommitment {
//             end_params: universal_circuit_no_delegation_verifier_vk().params,
//             aux_params: execution_utils::compute_chain_encoding(layers),
//         }
//     } else if final_recursion_layer_verifier_vk().params == *expected_end_params {
//         let layers = vec![
//             [0u32; 8],
//             base_params,
//             recursion_layer_verifier_vk().params,
//             recursion_layer_no_delegation_verifier_vk().params,
//             final_recursion_layer_verifier_vk().params,
//         ];
//         BinaryCommitment {
//             end_params: final_recursion_layer_verifier_vk().params,
//             aux_params: execution_utils::compute_chain_encoding(layers),
//         }
//     } else {
//         panic!(
//             "Cannot find a verifier for the proof end parameters: {:?}",
//             expected_end_params
//         );
//     }
// }

pub(crate) fn write_crs_into_raw_compact_form<W: std::io::Write>(
    original_crs: &Crs<bellman::bn256::Bn256, CrsForMonomialForm>,
    mut dst_raw_compact_crs: W,
) -> std::io::Result<()> {
    use bellman::CurveAffine;
    use bellman::{PrimeField, PrimeFieldRepr};
    use byteorder::{BigEndian, WriteBytesExt};
    let num_points = original_crs.g1_bases.len();
    dst_raw_compact_crs.write_u32::<BigEndian>(num_points as u32)?;
    for g1_base in original_crs.g1_bases.iter() {
        let (x, y) = g1_base.as_xy();
        x.into_raw_repr().write_le(&mut dst_raw_compact_crs)?;
        y.into_raw_repr().write_le(&mut dst_raw_compact_crs)?;
    }
    assert_eq!(original_crs.g2_monomial_bases.len(), 2);
    for g2_base in original_crs.g2_monomial_bases.iter() {
        let (x, y) = g2_base.as_xy();
        x.c0.into_raw_repr().write_le(&mut dst_raw_compact_crs)?;
        x.c1.into_raw_repr().write_le(&mut dst_raw_compact_crs)?;
        y.c0.into_raw_repr().write_le(&mut dst_raw_compact_crs)?;
        y.c1.into_raw_repr().write_le(&mut dst_raw_compact_crs)?;
    }

    Ok(())
}

pub fn prove_risc_wrapper_with_snark(
    risc_wrapper_proof: RiscWrapperProof,
    risc_wrapper_vk: RiscWrapperVK,
    trusted_setup_file: Option<String>,
) -> Result<
    (
        <PlonkSnarkWrapper as ProofSystemDefinition>::Proof,
        <PlonkSnarkWrapper as ProofSystemDefinition>::VK,
    ),
    Box<dyn std::error::Error>,
> {
    let worker = shivini::boojum::worker::Worker::new();
    println!("=== Phase 2: Creating compression proof");

    let config = ProverContextConfig::default().with_smallest_supported_domain_size(1 << 15);

    let prover_context = ProverContext::create_with_config(config).unwrap();

    /*let (
        finalization_hint,
        setup_base,
        setup,
        compression_vk,
        setup_tree,
        vars_hint,
        witness_hints,
    ) = get_compression_setup(risc_wrapper_vk.clone(), &worker);
    let compression_proof = prove_compression(
        risc_wrapper_proof,
        risc_wrapper_vk,
        &finalization_hint,
        &setup_base,
        &setup,
        &compression_vk,
        &setup_tree,
        &vars_hint,
        &witness_hints,
        &worker,
    );*/

    let (compression_proof, compression_vk) =
        prove_compression_with_gpu(risc_wrapper_proof, risc_wrapper_vk, &worker);

    let is_valid = verify_compression_proof(&compression_proof, &compression_vk);

    if !is_valid {
        return Err("Compression proof is not valid".into());
    }

    drop(prover_context);

    println!("=== Phase 3: Creating SNARK proof");

    let crs_mons = match trusted_setup_file {
        Some(ref crs_file_str) => get_trusted_setup(crs_file_str),
        None => Crs::<Bn256, CrsForMonomialForm>::crs_42(
            1 << L1_VERIFIER_DOMAIN_SIZE_LOG,
            &BellmanWorker::new(),
        ),
    };

    let mut dst_raw_compact_crs = Vec::new();
    println!("Writing CRS into compact form");

    write_crs_into_raw_compact_form(&crs_mons, &mut dst_raw_compact_crs).unwrap();
    println!("Finished writing CRS into compact form");

    let crs_mons = <PlonkSnarkWrapper as SnarkWrapperProofSystem>::load_compact_raw_crs(Box::new(
        Cursor::new(dst_raw_compact_crs.clone()),
    ))
    .unwrap();
    println!("Finished reading CRS into compact form");

    let finalization_hint = PlonkSnarkWrapper::load_finalization_hint().unwrap();

    /*let (precomputation, vk) = PlonkSnarkWrapper::precompute_plonk_snark_wrapper_circuit(
        compression_vk.clone(),
        finalization_hint,
        crs_mons,
    )
    .unwrap();*/

    type PlonkAssembly<CSConfig> = Assembly<
        Bn256,
        PlonkCsWidth4WithNextStepAndCustomGatesParams,
        SelectorOptimizedWidth4MainGateWithDNext,
        CSConfig,
        zksync_gpu_prover::cuda_bindings::CudaAllocator,
    >;

    let (precomputation, vk) = {
        // reimplementing stuff in precompute_plonk_wrapper_circuit (as we have different compression circuit).
        let fixed_parameters = compression_vk.fixed_parameters.clone();
        let wrapper_function = SnarkWrapperFunction;
        let wrapper_circuit = SnarkWrapperCircuit {
            witness: None,
            vk: compression_vk.clone(),
            fixed_parameters,
            transcript_params: (),
            wrapper_function,
        };
        /*let mut assembly = SetupAssembly::<
            Bn256,
            PlonkCsWidth4WithNextStepAndCustomGatesParams,
            SelectorOptimizedWidth4MainGateWithDNext,
        >::new();*/

        let mut setup_assembly = PlonkAssembly::<SynthesisModeGenerateSetup>::new();

        wrapper_circuit
            .synthesize(&mut setup_assembly)
            .expect("must work");

        let hardcoded_finalization_hint = L1_VERIFIER_DOMAIN_SIZE_LOG;

        // TODO: synthesize vs synthesize for setup
        //wrapper_circuit.synthesize(&mut assembly).unwrap();
        // It used finalization hint instead (fine - it is 24 for plonk, and 23 for fflonk).
        setup_assembly.finalize_to_size_log_2(hardcoded_finalization_hint);
        assert!(setup_assembly.is_satisfied());

        // now gpu part.
        let mut ctx = PlonkSnarkWrapper::init_context(&crs_mons)?.into_inner();

        let worker = bellman::worker::Worker::new();
        let mut precomputation = zksync_gpu_prover::AsyncSetup::<
            <PlonkSnarkWrapper as ProofSystemDefinition>::Allocator,
        >::allocate(1 << hardcoded_finalization_hint);
        precomputation
            .generate_from_assembly(&worker, &setup_assembly, &mut ctx)
            .unwrap();

        let hardcoded_g2_bases = hardcoded_canonical_g2_bases();
        let mut dummy_crs = Crs::<bellman::bn256::Bn256, CrsForMonomialForm>::dummy_crs(1);
        dummy_crs.g2_monomial_bases = std::sync::Arc::new(hardcoded_g2_bases.to_vec());
        let vk = zksync_gpu_prover::compute_vk_from_assembly::<
            _,
            _,
            PlonkCsWidth4WithNextStepAndCustomGatesParams,
            SynthesisModeGenerateSetup,
        >(&mut ctx, &setup_assembly, &dummy_crs)
        .unwrap();

        ctx.free_all_slots();

        (
            PlonkSnarkVerifierCircuitDeviceSetupWrapper::from_inner(precomputation),
            vk,
        )
    };

    println!("Precomputations ready");

    let snark_wrapper_vk = vk.clone();

    let crs_mons = <PlonkSnarkWrapper as SnarkWrapperProofSystem>::load_compact_raw_crs(Box::new(
        Cursor::new(dst_raw_compact_crs),
    ))
    .unwrap();

    let setup_data_cache: SnarkWrapperSetupData<PlonkSnarkWrapper> = SnarkWrapperSetupData {
        precomputation,
        vk,
        finalization_hint,
        previous_vk: compression_vk.clone(),
        crs: crs_mons,
    };

    /*let snark_wrapper_proof =
    PlonkSnarkWrapper::prove_plonk_snark_wrapper_step(compression_proof, setup_data_cache)
        .expect("Failed to prove Plonk snark wrapper step");*/

    let snark_wrapper_proof = {
        let input_proof = compression_proof;
        // Recreate stuff from prove_plonk_snark_wrapper_step

        let input_vk = setup_data_cache.previous_vk;
        let mut ctx = PlonkSnarkWrapper::init_context(&setup_data_cache.crs)?.into_inner();
        let finalization_hint = setup_data_cache.finalization_hint;
        let fixed_parameters = input_vk.fixed_parameters.clone();

        let wrapper_function = SnarkWrapperFunction;
        let circuit = SnarkWrapperCircuit {
            witness: Some(input_proof),
            vk: compression_vk.clone(),
            fixed_parameters,
            transcript_params: (),
            wrapper_function,
        };

        //let circuit = Self::build_circuit(input_vk.clone(), Some(input_proof));

        let mut proving_assembly = PlonkAssembly::<SynthesisModeProve>::new();

        circuit
            .synthesize(&mut proving_assembly)
            .expect("must work");

        //let mut proving_assembly =
        //<Self as SnarkWrapperProofSystem>::synthesize_for_proving(circuit);
        let vk = setup_data_cache.vk;
        let mut precomputation: AsyncSetup = setup_data_cache.precomputation.into_inner();

        assert!(proving_assembly.is_satisfied());
        assert!(finalization_hint.is_power_of_two());
        proving_assembly.finalize_to_size_log_2(finalization_hint.trailing_zeros() as usize);
        let domain_size = proving_assembly.n() + 1;
        assert!(domain_size.is_power_of_two());
        assert!(domain_size == finalization_hint.clone());

        let worker = bellman::worker::Worker::new();
        let start = std::time::Instant::now();
        let proof = zksync_gpu_prover::create_proof::<
            _,
            _,
            <PlonkSnarkWrapper as ProofSystemDefinition>::Transcript,
            _,
        >(
            &proving_assembly,
            &mut ctx,
            &worker,
            &mut precomputation,
            None,
        )
        .unwrap();

        println!("plonk proving takes {} s", start.elapsed().as_secs());
        ctx.free_all_slots();

        let result = <PlonkSnarkWrapper as ProofSystemDefinition>::verify(&proof, &vk);
        if !result {
            println!("*** WARNING - SNARK FAILED TO VERIFY ****");
        }
        proof
    };

    /*let is_valid = verify_snark_wrapper_proof(&snark_wrapper_proof, &snark_wrapper_vk);

    if !is_valid {
        return Err("Snark wrapper proof is not valid".into());
    }*/
    Ok((snark_wrapper_proof, snark_wrapper_vk))

    /*{
        let worker = BellmanWorker::new();

        let (snark_setup, snark_wrapper_vk) =
            get_snark_wrapper_setup(compression_vk.clone(), &crs_mons, &worker);

        let snark_wrapper_proof = prove_snark_wrapper(
            compression_proof,
            compression_vk,
            &snark_setup,
            &crs_mons,
            &worker,
        );

        let is_valid = verify_snark_wrapper_proof(&snark_wrapper_proof, &snark_wrapper_vk);

        if !is_valid {
            return Err("Snark wrapper proof is not valid".into());
        }
        Ok((snark_wrapper_proof, snark_wrapper_vk))
    }*/
}

pub fn prove_fri_risc_wrapper(
    program_proof: ProgramProof,
) -> Result<(RiscWrapperProof, RiscWrapperVK), Box<dyn std::error::Error>> {
    println!("=== Phase 1: Creating the Risc wrapper proof");

    let worker = shivini::boojum::worker::Worker::new();

    let binary_commitment = BinaryCommitment {
        end_params: program_proof.end_params,
        aux_params: program_proof
            .recursion_chain_hash
            .expect("Recursion chain hash is missing only in base layer"),
    };
    let risc_wrapper_witness =
        RiscWrapperWitness::from_full_proof(program_proof, &binary_commitment);

    let (
        finalization_hint,
        setup_base,
        //setup,
        risc_wrapper_vk,
        //setup_tree,
        vars_hint,
        witness_hints,
    ) = get_risc_wrapper_setup(&worker, binary_commitment.clone());

    let (risc_wrapper_proof, risc_wrapper_vk) = prove_risc_wrapper(
        risc_wrapper_witness,
        &finalization_hint,
        &setup_base,
        //&setup,
        &risc_wrapper_vk,
        //&setup_tree,
        &vars_hint,
        &witness_hints,
        &worker,
        binary_commitment,
    );
    let is_valid = verify_risc_wrapper_proof(&risc_wrapper_proof, &risc_wrapper_vk);
    if !is_valid {
        return Err("Risc wrapper proof is not valid".into());
    }

    Ok((risc_wrapper_proof, risc_wrapper_vk))
}

pub fn prove(
    input: String,
    output_dir: String,
    trusted_setup_file: Option<String>,
    risc_wrapper_only: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    //let prover_context = ProverContext::create().unwrap();

    // 1<<17 was taken from proof compression file.
    let config = ProverContextConfig::default().with_smallest_supported_domain_size(1 << 15);

    let prover_context = ProverContext::create_with_config(config).unwrap();

    let program_proof: crate::ProgramProof = deserialize_from_file(&input);
    let (risc_wrapper_proof, risc_wrapper_vk) = prove_fri_risc_wrapper(program_proof).unwrap();

    if risc_wrapper_only {
        serialize_to_file(
            &risc_wrapper_vk,
            &Path::new(&output_dir.clone()).join("risc_wrapper_vk.json"),
        );
        serialize_to_file(
            &risc_wrapper_proof,
            &Path::new(&output_dir.clone()).join("risc_wrapper_proof.json"),
        );
        return Ok(());
    }

    drop(prover_context);

    let (snark_wrapper_proof, snark_wrapper_vk) = prove_risc_wrapper_with_snark(
        risc_wrapper_proof,
        risc_wrapper_vk,
        trusted_setup_file.clone(),
    )
    .unwrap();

    serialize_to_file(
        &snark_wrapper_proof,
        &Path::new(&output_dir.clone()).join("snark_proof.json"),
    );
    serialize_to_file(
        &snark_wrapper_vk,
        &Path::new(&output_dir.clone()).join("snark_vk.json"),
    );

    Ok(())
}

pub fn generate_and_save_risc_wrapper_vk(
    input_binary: String,
    output_dir: String,
    universal_verifier: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let boojum_worker = BoojumWorker::new();
    let risc_wrapper_vk =
        generate_risk_wrapper_vk(input_binary, universal_verifier, &boojum_worker)?;

    serialize_to_file(
        &risc_wrapper_vk,
        &Path::new(&output_dir.clone()).join("risc_wrapper_vk_expected.json"),
    );
    Ok(())
}

fn generate_risk_wrapper_vk(
    input_binary: String,
    universal_verifier: bool,
    boojum_worker: &Worker,
) -> Result<RiscWrapperVK, Box<dyn std::error::Error>> {
    // let verifier_params = if universal_verifier {
    //     universal_circuit_no_delegation_verifier_vk().params
    // } else {
    //     final_recursion_layer_verifier_vk().params
    // };

    // let binary_commitment = create_binary_commitment(input_binary, &verifier_params);

    let bin = std::fs::read(input_binary).unwrap();

    let worker = risc_verifier::prover::worker::Worker::new();

    let expected_final_pc = execution_utils::find_binary_exit_point(&bin);
    let binary: Vec<u32> = execution_utils::get_padded_binary(&bin);

    let base_params = execution_utils::compute_end_parameters(
        expected_final_pc,
        &setups::get_main_riscv_circuit_setup::<Global, Global>(&binary, &worker),
    );

    #[cfg(feature = "wrap_final_machine")]
    let binary_commitment = if universal_verifier {
        let layers = vec![
            [0u32; 8],
            base_params,
            universal_circuit_verifier_vk().params,
            universal_circuit_no_delegation_verifier_vk().params,
        ];
        BinaryCommitment {
            end_params: universal_circuit_no_delegation_verifier_vk().params,
            aux_params: execution_utils::compute_chain_encoding(layers),
        }
    } else {
        let layers = vec![
            [0u32; 8],
            base_params,
            base_layer_verifier_vk().params,
            recursion_layer_verifier_vk().params,
            recursion_layer_no_delegation_verifier_vk().params,
        ];
        BinaryCommitment {
            end_params: final_recursion_layer_verifier_vk().params,
            aux_params: execution_utils::compute_chain_encoding(layers),
        }
    };

    #[cfg(feature = "wrap_with_reduced_log_23")]
    let binary_commitment = if universal_verifier {
        let layers = vec![
            [0u32; 8],
            base_params,
            universal_circuit_verifier_vk().params,
        ];
        BinaryCommitment {
            end_params: recursion_log_23_layer_verifier_vk().params,
            aux_params: execution_utils::compute_chain_encoding(layers),
        }
    } else {
        let layers = vec![
            [0u32; 8],
            base_params,
            base_layer_verifier_vk().params,
            recursion_layer_verifier_vk().params,
        ];
        BinaryCommitment {
            end_params: recursion_log_23_layer_verifier_vk().params,
            aux_params: execution_utils::compute_chain_encoding(layers),
        }
    };

    let (_, _, risc_wrapper_vk, _, _) =
        get_risc_wrapper_setup(boojum_worker, binary_commitment.clone());
    // FIXME
    let risc_wrapper_vk = Default::default();
    Ok(risc_wrapper_vk)
}

pub fn generate_vk(
    input_binary: String,
    output_dir: String,
    trusted_setup_file: Option<String>,
    universal_verifier: bool,
) -> Result<H256, Box<dyn std::error::Error>> {
    let worker = BellmanWorker::new();
    let boojum_worker = shivini::boojum::worker::Worker::new();

    println!("=== Phase 1: Creating the Risc wrapper key");

    let risc_wrapper_vk =
        generate_risk_wrapper_vk(input_binary, universal_verifier, &boojum_worker)?;

    println!("=== Phase 2: Creating the Compression key");
    let (_, _, _, compression_vk, _, _, _) =
        get_compression_setup(risc_wrapper_vk.clone(), &boojum_worker);

    println!("=== Phase 3: Creating the SNARK key");

    let crs_mons = match trusted_setup_file {
        Some(ref crs_file_str) => get_trusted_setup(crs_file_str),
        None => Crs::<Bn256, CrsForMonomialForm>::crs_42(
            1 << L1_VERIFIER_DOMAIN_SIZE_LOG,
            &BellmanWorker::new(),
        ),
    };

    let (_, snark_wrapper_vk) = get_snark_wrapper_setup(compression_vk.clone(), &crs_mons, &worker);

    serialize_to_file(
        &snark_wrapper_vk,
        &Path::new(&output_dir.clone()).join("snark_vk_expected.json"),
    );

    let verification_key = calculate_verification_key_hash(snark_wrapper_vk);
    println!("VK key hash: {:?}", verification_key);

    Ok(verification_key)
}
