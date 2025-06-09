#![feature(allocator_api)]
#![feature(array_chunks)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

pub mod circuits;
pub mod transcript;
mod wrapper_inner_verifier;
pub mod wrapper_utils;

#[cfg(test)]
mod tests;

use std::alloc::Global;
use std::path::Path;
use boojum::algebraic_props::round_function::AbsorptionModeOverwrite;
use boojum::algebraic_props::sponge::GoldilocksPoseidon2Sponge;
use boojum::config::{DevCSConfig, ProvingCSConfig, SetupCSConfig};
use boojum::cs::cs_builder::new_builder;
use boojum::cs::cs_builder_reference::CsReferenceImplementationBuilder;
use boojum::cs::implementations::hints::DenseVariablesCopyHint;
use boojum::cs::implementations::hints::DenseWitnessCopyHint;
use boojum::cs::implementations::polynomial_storage::SetupBaseStorage;
use boojum::cs::implementations::polynomial_storage::SetupStorage;
use boojum::cs::implementations::pow::NoPow;
use boojum::cs::implementations::proof::Proof;
use boojum::cs::implementations::prover::ProofConfig;
use boojum::cs::implementations::setup::FinalizationHintsForProver;
use boojum::cs::implementations::verifier::VerificationKey;
use boojum::cs::oracle::merkle_tree::MerkleTreeWithCap;
use boojum::cs::traits::circuit::CircuitBuilder;
use boojum::cs::traits::circuit::CircuitBuilderProxy;
use boojum::gadgets::recursion::recursive_transcript::CircuitAlgebraicSpongeBasedTranscript;
use boojum::gadgets::recursion::recursive_tree_hasher::CircuitGoldilocksPoseidon2Sponge;
use boojum::implementations::poseidon2::Poseidon2Goldilocks;
pub use boojum::worker::Worker as BoojumWorker;
use boojum::worker::*;
use circuits::*;
use wrapper_utils::verifier_traits::CircuitBlake2sForEverythingVerifier;

pub type GL = boojum::field::goldilocks::GoldilocksField;
pub type GLExt2 = boojum::field::goldilocks::GoldilocksExt2;
pub type RiscLeafInclusionVerifier = CircuitBlake2sForEverythingVerifier<GL>;
pub type RiscWrapper = RiscWrapperCircuit<GL, RiscLeafInclusionVerifier>;
pub type RiscWrapperTranscript =
    boojum::cs::implementations::transcript::GoldilocksPoisedon2Transcript;
pub type RiscWrapperTreeHasher = GoldilocksPoseidon2Sponge<AbsorptionModeOverwrite>;
pub type RiscWrapperProof = Proof<GL, RiscWrapperTreeHasher, GLExt2>;
pub type RiscWrapperVK = VerificationKey<GL, RiscWrapperTreeHasher>;

pub type CircuitRiscWrapperTranscript =
    CircuitAlgebraicSpongeBasedTranscript<GL, 8, 12, 4, Poseidon2Goldilocks>;
pub type CircuitRiscWrapperTreeHasher = CircuitGoldilocksPoseidon2Sponge;

pub type RiscWrapperCircuitBuilder = CircuitBuilderProxy<GL, RiscWrapper>;

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
use boojum::ethereum_types::H256;
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
use execution_utils::{final_recursion_layer_verifier_vk, recursion_layer_no_delegation_verifier_vk, recursion_layer_verifier_vk, universal_circuit_no_delegation_verifier_vk, universal_circuit_verifier_vk};
//CircuitAlgebraicSpongeBasedTranscript<GoldilocksField, 8, 12, 4, R>,

// RiscV -> Stark Wrapper
pub fn get_risc_wrapper_setup(
    worker: &Worker,
    binary_commitment: BinaryCommitment,
) -> (
    FinalizationHintsForProver,
    SetupBaseStorage<GL>,
    SetupStorage<GL>,
    RiscWrapperVK,
    MerkleTreeWithCap<GL, RiscWrapperTreeHasher>,
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

    let (setup_base, setup, vk, setup_tree, vars_hint, witness_hints) =
        cs.get_full_setup::<RiscWrapperTreeHasher>(worker, fri_lde_factor, merkle_tree_cap_size);

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

pub fn prove_risc_wrapper(
    risc_wrapper_witness: RiscWrapperWitness,
    finalization_hint: &FinalizationHintsForProver,
    setup_base: &SetupBaseStorage<GL>,
    setup: &SetupStorage<GL>,
    vk: &RiscWrapperVK,
    setup_tree: &MerkleTreeWithCap<GL, RiscWrapperTreeHasher>,
    vars_hint: &DenseVariablesCopyHint,
    witness_hints: &DenseWitnessCopyHint,
    worker: &Worker,
    binary_commitment: BinaryCommitment,
) -> RiscWrapperProof {
    let verify_inner_proof = true;
    let circuit = RiscWrapper::new(
        Some(risc_wrapper_witness),
        verify_inner_proof,
        binary_commitment,
    );

    let geometry = RiscWrapper::geometry();
    let (max_trace_len, num_vars) = circuit.size_hint();

    let builder_impl = CsReferenceImplementationBuilder::<GL, GL, DevCSConfig>::new(
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

    cs.prove_from_precomputations::<GLExt2, RiscWrapperTranscript, RiscWrapperTreeHasher, NoPow>(
        proof_config,
        &setup_base,
        &setup,
        &setup_tree,
        &vk,
        &vars_hint,
        &witness_hints,
        (),
        worker,
    )
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

pub fn create_binary_commitment(
    binary_path: String,
    expected_end_params: &[u32; 8],
) -> BinaryCommitment {
    let bin = std::fs::read(binary_path).unwrap();

    let worker = risc_verifier::prover::worker::Worker::new_with_num_threads(8);

    let expected_final_pc = execution_utils::find_binary_exit_point(&bin);
    let binary: Vec<u32> = execution_utils::get_padded_binary(&bin);

    let base_params = execution_utils::compute_end_parameters(
        expected_final_pc,
        &setups::get_main_riscv_circuit_setup::<Global, Global>(&binary, &worker),
    );

    // Check which verifier was used.
    if universal_circuit_no_delegation_verifier_vk().params == *expected_end_params {
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
    } else if final_recursion_layer_verifier_vk().params == *expected_end_params {
        let layers = vec![
            [0u32; 8],
            base_params,
            recursion_layer_verifier_vk().params,
            recursion_layer_no_delegation_verifier_vk().params,
            final_recursion_layer_verifier_vk().params,
        ];
        BinaryCommitment {
            end_params: final_recursion_layer_verifier_vk().params,
            aux_params: execution_utils::compute_chain_encoding(layers),
        }
    } else {
        panic!(
            "Cannot find a verifier for the proof end parameters: {:?}",
            expected_end_params
        );
    }
}

pub fn prove(
    input: String,
    input_binary: Option<String>,
    output_dir: String,
    trusted_setup_file: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let crs_mons = match trusted_setup_file {
        Some(ref crs_file_str) => get_trusted_setup(crs_file_str),
        None => Crs::<Bn256, CrsForMonomialForm>::crs_42(
            1 << L1_VERIFIER_DOMAIN_SIZE_LOG,
            &BellmanWorker::new(),
        ),
    };

    println!("=== Phase 1: Creating the Risc wrapper proof");

    let worker = boojum::worker::Worker::new_with_num_threads(4);

    let program_proof: crate::ProgramProof = deserialize_from_file(&input);
    let binary_commitment = match input_binary {
        Some(binary_path) => create_binary_commitment(binary_path, &program_proof.end_params),
        None => BinaryCommitment::from_default_binary(),
    };
    let risc_wrapper_witness =
        RiscWrapperWitness::from_full_proof(program_proof, &binary_commitment);

    let (
        finalization_hint,
        setup_base,
        setup,
        risc_wrapper_vk,
        setup_tree,
        vars_hint,
        witness_hints,
    ) = get_risc_wrapper_setup(&worker, binary_commitment.clone());

    let risc_wrapper_proof = prove_risc_wrapper(
        risc_wrapper_witness,
        &finalization_hint,
        &setup_base,
        &setup,
        &risc_wrapper_vk,
        &setup_tree,
        &vars_hint,
        &witness_hints,
        &worker,
        binary_commitment,
    );
    let is_valid = verify_risc_wrapper_proof(&risc_wrapper_proof, &risc_wrapper_vk);

    assert!(is_valid);

    serialize_to_file(
        &risc_wrapper_proof,
        &Path::new(&output_dir.clone()).join("risc_proof.json"),
    );

    println!("=== Phase 2: Creating compression proof");

    let (
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
    );
    let is_valid = verify_compression_proof(&compression_proof, &compression_vk);

    assert!(is_valid);

    serialize_to_file(
        &compression_proof,
        &Path::new(&output_dir.clone()).join("compression_proof.json"),
    );

    println!("=== Phase 3: Creating SNARK proof");

    {
        let worker = BellmanWorker::new_with_cpus(4);

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

        assert!(is_valid);

        serialize_to_file(
            &snark_wrapper_proof,
            &Path::new(&output_dir.clone()).join("snark_proof.json"),
        );
        serialize_to_file(
            &snark_wrapper_vk,
            &Path::new(&output_dir.clone()).join("snark_vk.json"),
        );
    }

    Ok(())
}