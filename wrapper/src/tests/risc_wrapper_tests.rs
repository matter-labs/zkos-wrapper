use crate::circuits::{BinaryCommitment, RiscWrapperWitness};

use super::*;

/// Commitment matching the checked-in `RISC_PROOF_PATH` fixture for the active
/// security level, mirroring the (non-check) production path so the derived VK
/// stays consistent with the checked-in fixtures.
///
/// - `security_80`: `risc_proof_80sb` is a proof over the `risc_app` program, so
///   the `end_params` are derived from the checked-in program bin/text.
/// - `security_100`: `risc_proof_100sb` is a proof over the default
///   unified-recursion verifier binary, matching `BinaryCommitment::default()`.
///
/// `aux_params` is left zeroed; these callers exercise the `check_aux_params =
/// false` path where it is never consumed.
fn binary_commitment_for_testing() -> BinaryCommitment {
    #[cfg(feature = "security_100")]
    {
        // Explicit `return` (not a tail expression) so the function still compiles
        // if both security features are somehow enabled together.
        return BinaryCommitment::default();
    }
    #[cfg(feature = "security_80")]
    {
        use std::io::Read;

        let mut binary = vec![];
        let mut file = std::fs::File::open(RISC_PROGRAM_BIN_PATH).unwrap();
        file.read_to_end(&mut binary).unwrap();

        let mut text = vec![];
        let mut file = std::fs::File::open(RISC_PROGRAM_TEXT_PATH).unwrap();
        file.read_to_end(&mut text).unwrap();

        let mut padded_binary = binary.to_vec();
        setups::pad_bytecode_bytes_for_proving(&mut padded_binary);
        let mut padded_text = text.to_vec();
        setups::pad_bytecode_bytes_for_proving(&mut padded_text);

        use execution_utils::unified_circuit::compute_unified_setup_for_machine_configuration;
        use risc_verifier::prover::riscv_transpiler::cycle::IWithoutByteAccessIsaConfigWithDelegation;

        let setup = compute_unified_setup_for_machine_configuration::<
            IWithoutByteAccessIsaConfigWithDelegation,
        >(&padded_binary, &padded_text);

        BinaryCommitment {
            end_params: setup.end_params,
            aux_params: [0; 8],
        }
    }
}

#[test]
pub(crate) fn risc_wrapper_full_test() {
    let worker = boojum::worker::Worker::new_with_num_threads(32);

    let binary_commitment = binary_commitment_for_testing();
    dbg!(binary_commitment);

    let program_proof: execution_utils::unrolled::UnrolledProgramProof =
        deserialize_from_bin_file(RISC_PROOF_PATH).unwrap();

    let risc_wrapper_witness =
        RiscWrapperWitness::from_full_proof(program_proof, &binary_commitment, false).unwrap();

    #[cfg(not(feature = "gpu"))]
    let (risc_wrapper_proof, risc_wrapper_vk) = {
        let (
            finalization_hint,
            setup_base,
            setup,
            risc_wrapper_vk,
            setup_tree,
            vars_hint,
            witness_hints,
        ) = crate::get_risc_wrapper_setup(&worker, binary_commitment.clone(), false);

        let risc_wrapper_proof = crate::prove_risc_wrapper(
            risc_wrapper_witness,
            &finalization_hint,
            &setup_base,
            &setup,
            &risc_wrapper_vk,
            &setup_tree,
            &vars_hint,
            &witness_hints,
            &worker,
            binary_commitment.clone(),
            false,
        );

        let is_valid = crate::verify_risc_wrapper_proof(&risc_wrapper_proof, &risc_wrapper_vk);
        assert!(is_valid);

        (risc_wrapper_proof, risc_wrapper_vk)
    };

    #[cfg(feature = "gpu")]
    let (risc_wrapper_proof, risc_wrapper_vk) = {
        let _prover_context = shivini::ProverContext::create().unwrap();
        let (gpu_setup, gpu_vk, finalization_hint) =
            crate::gpu::risc_wrapper::get_risc_wrapper_setup(
                &worker,
                binary_commitment.clone(),
                false,
            );

        let risc_wrapper_proof = crate::gpu::risc_wrapper::prove_risc_wrapper(
            risc_wrapper_witness,
            &finalization_hint,
            &gpu_setup,
            &gpu_vk,
            &worker,
            binary_commitment.clone(),
            false,
        );

        let is_valid = crate::verify_risc_wrapper_proof(&risc_wrapper_proof, &gpu_vk);
        assert!(is_valid);

        (risc_wrapper_proof, gpu_vk)
    };

    serialize_to_bin_file(&risc_wrapper_proof, RISC_WRAPPER_PROOF_PATH).unwrap();
    serialize_to_bin_file(&risc_wrapper_vk, RISC_WRAPPER_VK_PATH).unwrap();
}

#[test]
pub(crate) fn risc_wrapper_setup_test() {
    let worker = boojum::worker::Worker::new();

    // Mirror the production non-check path (`check_aux_params = false` returns
    // `BinaryCommitment::default()`) so the checked-in VK fixture stays valid.
    let binary_commitment = BinaryCommitment::default();

    let (_finalization_hint, _setup_base, _setup, vk, _setup_tree, _vars_hint, _witness_hints) =
        crate::get_risc_wrapper_setup(&worker, binary_commitment, false);

    serialize_to_bin_file(&vk, RISC_WRAPPER_VK_PATH).unwrap();
}

/// Off-circuit `check_aux_params` validation in `from_full_proof`: a commitment
/// whose `aux_params` equals the proof's final registers 18..=25 is accepted,
/// and any mismatch is rejected up front with a clear error. This mirrors the
/// in-circuit aux constraint and guards against comparing against the wrong
/// value (e.g. the proof's `recursion_chain_hash`, which is one recursion fold
/// short of `aux_params`).
#[test]
fn check_aux_params_off_circuit_validation() {
    let program_proof: execution_utils::unrolled::UnrolledProgramProof =
        deserialize_from_bin_file(RISC_PROOF_PATH).unwrap();

    // The binary chain commitment the program exposes in its final registers.
    let mut aux_params = [0u32; 8];
    for i in 0..8 {
        aux_params[i] = program_proof.register_final_values[18 + i].value;
    }

    let end_params = binary_commitment_for_testing().end_params;

    // Positive: matching aux_params is accepted.
    let matching = BinaryCommitment {
        end_params,
        aux_params,
    };
    RiscWrapperWitness::from_full_proof(program_proof.clone(), &matching, true)
        .expect("matching aux_params must be accepted in check mode");

    // Negative: a single flipped word is rejected.
    let mut wrong_aux = aux_params;
    wrong_aux[0] ^= 1;
    let mismatched = BinaryCommitment {
        end_params,
        aux_params: wrong_aux,
    };
    let err = match RiscWrapperWitness::from_full_proof(program_proof, &mismatched, true) {
        Ok(_) => panic!("mismatched aux_params must be rejected in check mode"),
        Err(err) => err,
    };
    assert!(
        err.to_string().contains("aux_params"),
        "unexpected error message: {err}"
    );
}

/// `BinaryCommitment::from_base_binary` is the core of the `compute-aux-params`
/// subcommand: it derives `end_params` (unified layer) and the folded
/// recursion-chain `aux_params` (base -> unrolled) from the base program.
/// Smoke-test that it runs and produces a non-trivial commitment.
///
/// `#[ignore]`d because it computes three full recursion-layer setups (~minutes);
/// run explicitly with `cargo test from_base_binary_computes_aux_params -- --ignored`.
#[test]
#[ignore]
fn from_base_binary_computes_aux_params() {
    use std::io::Read;

    let mut binary = vec![];
    std::fs::File::open(RISC_PROGRAM_BIN_PATH)
        .unwrap()
        .read_to_end(&mut binary)
        .unwrap();
    let mut text = vec![];
    std::fs::File::open(RISC_PROGRAM_TEXT_PATH)
        .unwrap()
        .read_to_end(&mut text)
        .unwrap();

    let commitment = BinaryCommitment::from_base_binary(&binary, &text);

    assert_ne!(
        commitment.end_params, [0u32; 8],
        "end_params must be populated"
    );
    assert_ne!(
        commitment.aux_params, [0u32; 8],
        "aux_params (recursion-chain hash) must be populated"
    );
}

#[test]
fn test_verifier_inner_function() {
    run_verifier_inner_function(false);
}

/// In-circuit coverage of `--check-aux-params` mode: builds the wrapper circuit
/// with a commitment whose `aux_params` equals the proof's final registers
/// 18..=25 and asserts the constraint system is satisfied.
#[test]
fn test_verifier_inner_function_check_aux_params() {
    run_verifier_inner_function(true);
}

fn run_verifier_inner_function(check_aux_params: bool) {
    // allocate CS
    let geometry = CSGeometry {
        num_columns_under_copy_permutation: 180,
        num_witness_columns: 0,
        num_constant_columns: 4,
        max_allowed_constraint_degree: 4,
    };

    use boojum::config::DevCSConfig;
    // use boojum::config::SetupCSConfig;
    use boojum::cs::cs_builder_reference::*;
    let builder_impl =
        CsReferenceImplementationBuilder::<F, F, DevCSConfig, StCircuitResolver<_, _>>::new(
            geometry,
            1 << 20,
        );
    use boojum::cs::cs_builder::new_builder;
    let builder = new_builder::<_, F>(builder_impl);

    let builder = builder.allow_lookup(
        LookupParameters::UseSpecializedColumnsWithTableIdAsConstant {
            width: 3,
            num_repetitions: 80,
            share_table_id: true,
        },
    );

    let builder = ConstantsAllocatorGate::configure_builder(
        builder,
        GatePlacementStrategy::UseGeneralPurposeColumns,
    );
    let builder = FmaGateInBaseFieldWithoutConstant::configure_builder(
        builder,
        GatePlacementStrategy::UseGeneralPurposeColumns,
    );
    let builder = ReductionGate::<F, 4>::configure_builder(
        builder,
        GatePlacementStrategy::UseGeneralPurposeColumns,
    );
    let builder = UIntXAddGate::<16>::configure_builder(
        builder,
        GatePlacementStrategy::UseGeneralPurposeColumns,
    );
    let builder = UIntXAddGate::<8>::configure_builder(
        builder,
        GatePlacementStrategy::UseGeneralPurposeColumns,
    );
    let builder =
        SelectionGate::configure_builder(builder, GatePlacementStrategy::UseGeneralPurposeColumns);
    let builder = U32TriAddCarryAsChunkGate::configure_builder(
        builder,
        GatePlacementStrategy::UseGeneralPurposeColumns,
    );
    // let builder = U32AddCarryAsChunkGate::configure_builder(
    //     builder,
    //     GatePlacementStrategy::UseGeneralPurposeColumns,
    // );
    let builder =
        NopGate::configure_builder(builder, GatePlacementStrategy::UseGeneralPurposeColumns);

    let builder = ReductionGate::<F, 2>::configure_builder(
        builder,
        GatePlacementStrategy::UseGeneralPurposeColumns,
    );
    let builder = ZeroCheckGate::configure_builder(
        builder,
        GatePlacementStrategy::UseGeneralPurposeColumns,
        false,
    );
    use boojum::cs::gates::PublicInputGate;
    let builder = PublicInputGate::configure_builder(
        builder,
        GatePlacementStrategy::UseGeneralPurposeColumns,
    );

    // let mut owned_cs = builder.build(CircuitResolverOpts::new(1 << 27));
    let mut owned_cs = builder.build(1 << 27);

    let cs = &mut owned_cs;

    let program_proof: execution_utils::unrolled::UnrolledProgramProof =
        deserialize_from_bin_file(RISC_PROOF_PATH).unwrap();

    // Commitment matching the checked-in proof fixture for the active security
    // level (see `binary_commitment_for_testing`).
    let mut binary_commitment = binary_commitment_for_testing();
    if check_aux_params {
        // The program exposes its binary chain commitment in final registers
        // 18..=25; bake exactly those values so both the off-circuit and
        // in-circuit aux_params checks are satisfiable.
        for i in 0..8 {
            binary_commitment.aux_params[i] = program_proof.register_final_values[18 + i].value;
        }
    }

    let risc_wrapper_witness =
        RiscWrapperWitness::from_full_proof(program_proof, &binary_commitment, check_aux_params)
            .unwrap();

    use crate::RiscWrapper;

    let circuit = RiscWrapper::new(
        Some(risc_wrapper_witness),
        true,
        binary_commitment,
        check_aux_params,
    );

    // Register the full canonical table set the circuit needs (the manual subset
    // this test used previously was incomplete for the checked-in proof fixture).
    circuit.add_tables(cs);

    circuit.synthesize_into_cs(cs);

    let worker = boojum::worker::Worker::new_with_num_threads(4);

    dbg!(cs.next_available_row());

    let _ = cs;
    owned_cs.pad_and_shrink();
    let mut owned_cs = owned_cs.into_assembly::<Global>();
    owned_cs.print_gate_stats();
    assert!(owned_cs.check_if_satisfied(&worker));
}
