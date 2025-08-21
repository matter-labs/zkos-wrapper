#[allow(
    unused_braces,
    unused_mut,
    unused_variables,
    unsafe_op_in_unsafe_fn,
    unused_assignments
)]
unsafe fn evaluate_every_row_except_last<F: SmallField, CS: ConstraintSystem<F>>(
    cs: &mut CS,
    random_point: MersenneQuartic<F>,
    witness: &[MersenneQuartic<F>],
    memory: &[MersenneQuartic<F>],
    setup: &[MersenneQuartic<F>],
    stage_2: &[MersenneQuartic<F>],
    witness_next_row: &[MersenneQuartic<F>],
    memory_next_row: &[MersenneQuartic<F>],
    stage_2_next_row: &[MersenneQuartic<F>],
    quotient_alpha: MersenneQuartic<F>,
    quotient_beta: MersenneQuartic<F>,
    divisors: &[MersenneQuartic<F>; 6usize],
    lookup_argument_linearization_challenges: [MersenneQuartic<F>;
        NUM_LOOKUP_ARGUMENT_LINEARIZATION_CHALLENGES],
    lookup_argument_gamma: MersenneQuartic<F>,
    lookup_argument_two_gamma: MersenneQuartic<F>,
    memory_argument_linearization_challenges: [MersenneQuartic<F>;
        NUM_MEM_ARGUMENT_LINEARIZATION_CHALLENGES],
    memory_argument_gamma: MersenneQuartic<F>,
    delegation_argument_linearization_challenges : [MersenneQuartic < F > ; NUM_DELEGATION_ARGUMENT_LINEARIZATION_CHALLENGES],
    delegation_argument_gamma: MersenneQuartic<F>,
    public_inputs: &[MersenneField<F>; 4usize],
    aux_proof_values: &WrappedProofAuxValues<F>,
    aux_boundary_values: WrappedAuxArgumentsBoundaryValues<F>,
    memory_timestamp_high_from_sequence_idx: MersenneField<F>,
    delegation_type: MersenneField<F>,
    delegation_argument_interpolant_linear_coeff: MersenneQuartic<F>,
) -> MersenneQuartic<F> {
    let every_row_except_last_contribution = {
        let mut accumulated_contribution = {
            let individual_term = {
                let value = unsafe { *(witness.get_unchecked(13usize)) };
                let one = MersenneField::one(cs);
                let mut t = value;
                t = t.sub_base(cs, &one);
                t = t.mul(cs, &value);
                t
            };
            individual_term
        };
        {
            let contribution = {
                let individual_term = {
                    let value = unsafe { *(witness.get_unchecked(14usize)) };
                    let one = MersenneField::one(cs);
                    let mut t = value;
                    t = t.sub_base(cs, &one);
                    t = t.mul(cs, &value);
                    t
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let value = unsafe { *(witness.get_unchecked(15usize)) };
                    let one = MersenneField::one(cs);
                    let mut t = value;
                    t = t.sub_base(cs, &one);
                    t = t.mul(cs, &value);
                    t
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let value = unsafe { *(witness.get_unchecked(16usize)) };
                    let one = MersenneField::one(cs);
                    let mut t = value;
                    t = t.sub_base(cs, &one);
                    t = t.mul(cs, &value);
                    t
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let value = unsafe { *(witness.get_unchecked(17usize)) };
                    let one = MersenneField::one(cs);
                    let mut t = value;
                    t = t.sub_base(cs, &one);
                    t = t.mul(cs, &value);
                    t
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let value = unsafe { *(witness.get_unchecked(18usize)) };
                    let one = MersenneField::one(cs);
                    let mut t = value;
                    t = t.sub_base(cs, &one);
                    t = t.mul(cs, &value);
                    t
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let value = unsafe { *(witness.get_unchecked(19usize)) };
                    let one = MersenneField::one(cs);
                    let mut t = value;
                    t = t.sub_base(cs, &one);
                    t = t.mul(cs, &value);
                    t
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let value = unsafe { *(witness.get_unchecked(20usize)) };
                    let one = MersenneField::one(cs);
                    let mut t = value;
                    t = t.sub_base(cs, &one);
                    t = t.mul(cs, &value);
                    t
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let value = unsafe { *(witness.get_unchecked(21usize)) };
                    let one = MersenneField::one(cs);
                    let mut t = value;
                    t = t.sub_base(cs, &one);
                    t = t.mul(cs, &value);
                    t
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let value = unsafe { *(witness.get_unchecked(22usize)) };
                    let one = MersenneField::one(cs);
                    let mut t = value;
                    t = t.sub_base(cs, &one);
                    t = t.mul(cs, &value);
                    t
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let value = unsafe { *(witness.get_unchecked(23usize)) };
                    let one = MersenneField::one(cs);
                    let mut t = value;
                    t = t.sub_base(cs, &one);
                    t = t.mul(cs, &value);
                    t
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let value = unsafe { *(witness.get_unchecked(24usize)) };
                    let one = MersenneField::one(cs);
                    let mut t = value;
                    t = t.sub_base(cs, &one);
                    t = t.mul(cs, &value);
                    t
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let value = unsafe { *(witness.get_unchecked(25usize)) };
                    let one = MersenneField::one(cs);
                    let mut t = value;
                    t = t.sub_base(cs, &one);
                    t = t.mul(cs, &value);
                    t
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let value = unsafe { *(witness.get_unchecked(26usize)) };
                    let one = MersenneField::one(cs);
                    let mut t = value;
                    t = t.sub_base(cs, &one);
                    t = t.mul(cs, &value);
                    t
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let value = unsafe { *(witness.get_unchecked(27usize)) };
                    let one = MersenneField::one(cs);
                    let mut t = value;
                    t = t.sub_base(cs, &one);
                    t = t.mul(cs, &value);
                    t
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let value = unsafe { *(witness.get_unchecked(28usize)) };
                    let one = MersenneField::one(cs);
                    let mut t = value;
                    t = t.sub_base(cs, &one);
                    t = t.mul(cs, &value);
                    t
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let value = unsafe { *(witness.get_unchecked(29usize)) };
                    let one = MersenneField::one(cs);
                    let mut t = value;
                    t = t.sub_base(cs, &one);
                    t = t.mul(cs, &value);
                    t
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let value = unsafe { *(witness.get_unchecked(30usize)) };
                    let one = MersenneField::one(cs);
                    let mut t = value;
                    t = t.sub_base(cs, &one);
                    t = t.mul(cs, &value);
                    t
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let value = unsafe { *(witness.get_unchecked(31usize)) };
                    let one = MersenneField::one(cs);
                    let mut t = value;
                    t = t.sub_base(cs, &one);
                    t = t.mul(cs, &value);
                    t
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let value = unsafe { *(witness.get_unchecked(32usize)) };
                    let one = MersenneField::one(cs);
                    let mut t = value;
                    t = t.sub_base(cs, &one);
                    t = t.mul(cs, &value);
                    t
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let value = unsafe { *(witness.get_unchecked(33usize)) };
                    let one = MersenneField::one(cs);
                    let mut t = value;
                    t = t.sub_base(cs, &one);
                    t = t.mul(cs, &value);
                    t
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let value = unsafe { *(witness.get_unchecked(34usize)) };
                    let one = MersenneField::one(cs);
                    let mut t = value;
                    t = t.sub_base(cs, &one);
                    t = t.mul(cs, &value);
                    t
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let value = unsafe { *(witness.get_unchecked(35usize)) };
                    let one = MersenneField::one(cs);
                    let mut t = value;
                    t = t.sub_base(cs, &one);
                    t = t.mul(cs, &value);
                    t
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let value = unsafe { *(witness.get_unchecked(36usize)) };
                    let one = MersenneField::one(cs);
                    let mut t = value;
                    t = t.sub_base(cs, &one);
                    t = t.mul(cs, &value);
                    t
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let value = unsafe { *(witness.get_unchecked(37usize)) };
                    let one = MersenneField::one(cs);
                    let mut t = value;
                    t = t.sub_base(cs, &one);
                    t = t.mul(cs, &value);
                    t
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let value = unsafe { *(witness.get_unchecked(38usize)) };
                    let one = MersenneField::one(cs);
                    let mut t = value;
                    t = t.sub_base(cs, &one);
                    t = t.mul(cs, &value);
                    t
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let value = unsafe { *(witness.get_unchecked(39usize)) };
                    let one = MersenneField::one(cs);
                    let mut t = value;
                    t = t.sub_base(cs, &one);
                    t = t.mul(cs, &value);
                    t
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let value = unsafe { *(witness.get_unchecked(40usize)) };
                    let one = MersenneField::one(cs);
                    let mut t = value;
                    t = t.sub_base(cs, &one);
                    t = t.mul(cs, &value);
                    t
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let value = unsafe { *(witness.get_unchecked(41usize)) };
                    let one = MersenneField::one(cs);
                    let mut t = value;
                    t = t.sub_base(cs, &one);
                    t = t.mul(cs, &value);
                    t
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let value = unsafe { *(witness.get_unchecked(42usize)) };
                    let one = MersenneField::one(cs);
                    let mut t = value;
                    t = t.sub_base(cs, &one);
                    t = t.mul(cs, &value);
                    t
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let value = unsafe { *(witness.get_unchecked(43usize)) };
                    let one = MersenneField::one(cs);
                    let mut t = value;
                    t = t.sub_base(cs, &one);
                    t = t.mul(cs, &value);
                    t
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let value = unsafe { *(witness.get_unchecked(44usize)) };
                    let one = MersenneField::one(cs);
                    let mut t = value;
                    t = t.sub_base(cs, &one);
                    t = t.mul(cs, &value);
                    t
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let value = unsafe { *(witness.get_unchecked(45usize)) };
                    let one = MersenneField::one(cs);
                    let mut t = value;
                    t = t.sub_base(cs, &one);
                    t = t.mul(cs, &value);
                    t
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let value = unsafe { *(witness.get_unchecked(46usize)) };
                    let one = MersenneField::one(cs);
                    let mut t = value;
                    t = t.sub_base(cs, &one);
                    t = t.mul(cs, &value);
                    t
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let value = unsafe { *(witness.get_unchecked(47usize)) };
                    let one = MersenneField::one(cs);
                    let mut t = value;
                    t = t.sub_base(cs, &one);
                    t = t.mul(cs, &value);
                    t
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(13usize)) };
                        let b = unsafe { *(witness.get_unchecked(13usize)) };
                        a = a.mul(cs, &b);
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(65536u32));
                        a = a.mul_by_base(cs, &coeff);
                        a
                    };
                    {
                        let mut a = unsafe { *(witness.get_unchecked(13usize)) };
                        let b = unsafe { *(witness.get_unchecked(51usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2147483643u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(13usize)) };
                        let b = unsafe { *(witness.get_unchecked(53usize)) };
                        a = a.mul(cs, &b);
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(1024u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(13usize)) };
                        let b = unsafe { *(witness.get_unchecked(56usize)) };
                        a = a.mul(cs, &b);
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(4u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(13usize)) };
                        let b = unsafe { *(witness.get_unchecked(57usize)) };
                        a = a.mul(cs, &b);
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(16384u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(51usize)) };
                        let b = unsafe { *(witness.get_unchecked(51usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(131072u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(51usize)) };
                        let b = unsafe { *(witness.get_unchecked(53usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2080374783u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(51usize)) };
                        let b = unsafe { *(witness.get_unchecked(56usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2147221503u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(51usize)) };
                        let b = unsafe { *(witness.get_unchecked(57usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(1073741823u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(53usize)) };
                        let b = unsafe { *(witness.get_unchecked(53usize)) };
                        a = a.mul(cs, &b);
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(4u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(53usize)) };
                        let b = unsafe { *(witness.get_unchecked(56usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(67108864u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(53usize)) };
                        let b = unsafe { *(witness.get_unchecked(57usize)) };
                        a = a.mul(cs, &b);
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(128u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(56usize)) };
                        let b = unsafe { *(witness.get_unchecked(56usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(131072u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(56usize)) };
                        let b = unsafe { *(witness.get_unchecked(57usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(1073741824u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(57usize)) };
                        let b = unsafe { *(witness.get_unchecked(57usize)) };
                        a = a.mul(cs, &b);
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(1024u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(13usize)) };
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(256u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(51usize)) };
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2130706431u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(53usize)) };
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(2u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(56usize)) };
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(16777216u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(57usize)) };
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(32u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(14usize)) };
                        let b = unsafe { *(witness.get_unchecked(14usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(4194304u32));
                        a = a.mul_by_base(cs, &coeff);
                        a
                    };
                    {
                        let mut a = unsafe { *(witness.get_unchecked(14usize)) };
                        let b = unsafe { *(witness.get_unchecked(52usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2147483391u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(14usize)) };
                        let b = unsafe { *(witness.get_unchecked(54usize)) };
                        a = a.mul(cs, &b);
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(256u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(14usize)) };
                        let b = unsafe { *(witness.get_unchecked(55usize)) };
                        a = a.mul(cs, &b);
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(8192u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(14usize)) };
                        let b = unsafe { *(witness.get_unchecked(58usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(131072u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(52usize)) };
                        let b = unsafe { *(witness.get_unchecked(52usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(8388608u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(52usize)) };
                        let b = unsafe { *(witness.get_unchecked(54usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2130706431u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(52usize)) };
                        let b = unsafe { *(witness.get_unchecked(55usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(1610612735u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(52usize)) };
                        let b = unsafe { *(witness.get_unchecked(58usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2147483643u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(54usize)) };
                        let b = unsafe { *(witness.get_unchecked(54usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(8388608u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(54usize)) };
                        let b = unsafe { *(witness.get_unchecked(55usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(536870912u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(54usize)) };
                        let b = unsafe { *(witness.get_unchecked(58usize)) };
                        a = a.mul(cs, &b);
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(4u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(55usize)) };
                        let b = unsafe { *(witness.get_unchecked(55usize)) };
                        a = a.mul(cs, &b);
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(4u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(55usize)) };
                        let b = unsafe { *(witness.get_unchecked(58usize)) };
                        a = a.mul(cs, &b);
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(128u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(58usize)) };
                        let b = unsafe { *(witness.get_unchecked(58usize)) };
                        a = a.mul(cs, &b);
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(1024u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(14usize)) };
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(2048u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(52usize)) };
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2013265919u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(54usize)) };
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(134217728u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(55usize)) };
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(2u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(58usize)) };
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(32u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(13usize)) };
                        let b = unsafe { *(witness.get_unchecked(18usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2147483391u32));
                        a = a.mul_by_base(cs, &coeff);
                        a
                    };
                    {
                        let mut a = unsafe { *(witness.get_unchecked(13usize)) };
                        let b = unsafe { *(witness.get_unchecked(19usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2146959359u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(13usize)) };
                        let b = unsafe { *(witness.get_unchecked(20usize)) };
                        a = a.mul(cs, &b);
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(32768u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(13usize)) };
                        let b = unsafe { *(witness.get_unchecked(21usize)) };
                        a = a.mul(cs, &b);
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(32768u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(14usize)) };
                        let b = unsafe { *(witness.get_unchecked(17usize)) };
                        a = a.mul(cs, &b);
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(61440u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(14usize)) };
                        let b = unsafe { *(witness.get_unchecked(18usize)) };
                        a = a.mul(cs, &b);
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(63488u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(14usize)) };
                        let b = unsafe { *(witness.get_unchecked(19usize)) };
                        a = a.mul(cs, &b);
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(61440u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(14usize)) };
                        let b = unsafe { *(witness.get_unchecked(21usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2143289343u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(17usize)) };
                        let b = unsafe { *(witness.get_unchecked(52usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(134217728u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(17usize)) };
                        let b = unsafe { *(witness.get_unchecked(54usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2013265919u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(17usize)) };
                        let b = unsafe { *(witness.get_unchecked(58usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2147483615u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(18usize)) };
                        let b = unsafe { *(witness.get_unchecked(51usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(16777216u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(18usize)) };
                        let b = unsafe { *(witness.get_unchecked(56usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2130706431u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(18usize)) };
                        let b = unsafe { *(witness.get_unchecked(57usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2147483615u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(19usize)) };
                        let b = unsafe { *(witness.get_unchecked(51usize)) };
                        a = a.mul(cs, &b);
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(16u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(19usize)) };
                        let b = unsafe { *(witness.get_unchecked(53usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2147479553u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(19usize)) };
                        let b = unsafe { *(witness.get_unchecked(56usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2147483631u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(19usize)) };
                        let b = unsafe { *(witness.get_unchecked(57usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2147418111u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(20usize)) };
                        let b = unsafe { *(witness.get_unchecked(57usize)) };
                        a = a.mul(cs, &b);
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(4096u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(20usize)) };
                        let b = unsafe { *(witness.get_unchecked(58usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2147483615u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(21usize)) };
                        let b = unsafe { *(witness.get_unchecked(52usize)) };
                        a = a.mul(cs, &b);
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(128u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(21usize)) };
                        let b = unsafe { *(witness.get_unchecked(54usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2147483519u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(21usize)) };
                        let b = unsafe { *(witness.get_unchecked(55usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2147479553u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(21usize)) };
                        let b = unsafe { *(witness.get_unchecked(57usize)) };
                        a = a.mul(cs, &b);
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(4096u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(21usize)) };
                        let b = unsafe { *(witness.get_unchecked(58usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2147418111u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(58usize)) };
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(32u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let a = unsafe { *(witness.get_unchecked(80usize)) };
                        individual_term = individual_term.sub(cs, &a);
                    }
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(14usize)) };
                        let b = unsafe { *(witness.get_unchecked(20usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2147418112u32));
                        a = a.mul_by_base(cs, &coeff);
                        a
                    };
                    {
                        let mut a = unsafe { *(witness.get_unchecked(14usize)) };
                        let b = unsafe { *(witness.get_unchecked(21usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2147483632u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(20usize)) };
                        let b = unsafe { *(witness.get_unchecked(52usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(21usize)) };
                        let b = unsafe { *(witness.get_unchecked(54usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(14usize)) };
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(65535u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let a = unsafe { *(witness.get_unchecked(81usize)) };
                        individual_term = individual_term.sub(cs, &a);
                    }
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(16usize)) };
                        let b = unsafe { *(memory.get_unchecked(13usize)) };
                        a = a.mul(cs, &b);
                        a
                    };
                    {
                        let mut a = unsafe { *(witness.get_unchecked(17usize)) };
                        let b = unsafe { *(witness.get_unchecked(80usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(18usize)) };
                        let b = unsafe { *(memory.get_unchecked(13usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(19usize)) };
                        let b = unsafe { *(memory.get_unchecked(13usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let a = unsafe { *(witness.get_unchecked(82usize)) };
                        individual_term = individual_term.sub(cs, &a);
                    }
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(16usize)) };
                        let b = unsafe { *(memory.get_unchecked(14usize)) };
                        a = a.mul(cs, &b);
                        a
                    };
                    {
                        let mut a = unsafe { *(witness.get_unchecked(17usize)) };
                        let b = unsafe { *(witness.get_unchecked(81usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(18usize)) };
                        let b = unsafe { *(memory.get_unchecked(14usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(19usize)) };
                        let b = unsafe { *(memory.get_unchecked(14usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let a = unsafe { *(witness.get_unchecked(61usize)) };
                        individual_term = individual_term.sub(cs, &a);
                    }
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(3usize)) };
                        let b = unsafe { *(witness.get_unchecked(3usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(1073741824u32));
                        a = a.mul_by_base(cs, &coeff);
                        a
                    };
                    {
                        let mut a = unsafe { *(witness.get_unchecked(3usize)) };
                        let b = unsafe { *(witness.get_unchecked(4usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.sub(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(4usize)) };
                        let b = unsafe { *(witness.get_unchecked(4usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(1073741824u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(3usize)) };
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2147450883u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(4usize)) };
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(32764u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    let constant_term =
                        MersenneField::allocate_constant(cs, Mersenne31Field(2147352583u32));
                    individual_term = individual_term.add_base(cs, &constant_term);
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(3usize)) };
                        let b = unsafe { *(witness.get_unchecked(83usize)) };
                        a = a.mul(cs, &b);
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(32768u32));
                        a = a.mul_by_base(cs, &coeff);
                        a
                    };
                    {
                        let mut a = unsafe { *(witness.get_unchecked(4usize)) };
                        let b = unsafe { *(witness.get_unchecked(83usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2147450879u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(48usize)) };
                        let b = unsafe { *(witness.get_unchecked(83usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(83usize)) };
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(65536u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    let constant_term =
                        MersenneField::allocate_constant(cs, Mersenne31Field(2147483646u32));
                    individual_term = individual_term.add_base(cs, &constant_term);
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(86usize)) };
                        let b = unsafe { *(witness.get_unchecked(88usize)) };
                        a = a.mul(cs, &b);
                        a
                    };
                    {
                        let a = unsafe { *(witness.get_unchecked(90usize)) };
                        individual_term = individual_term.sub(cs, &a);
                    }
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(25usize)) };
                        let b = unsafe { *(witness.get_unchecked(90usize)) };
                        a = a.mul(cs, &b);
                        a
                    };
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(4usize)) };
                        let b = unsafe { *(witness.get_unchecked(88usize)) };
                        a = a.mul(cs, &b);
                        a = a.negated(cs);
                        a
                    };
                    {
                        let mut a = unsafe { *(witness.get_unchecked(7usize)) };
                        let b = unsafe { *(witness.get_unchecked(88usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let a = unsafe { *(witness.get_unchecked(4usize)) };
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let a = unsafe { *(witness.get_unchecked(91usize)) };
                        individual_term = individual_term.sub(cs, &a);
                    }
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(3usize)) };
                        let b = unsafe { *(witness.get_unchecked(88usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2147450879u32));
                        a = a.mul_by_base(cs, &coeff);
                        a
                    };
                    {
                        let mut a = unsafe { *(witness.get_unchecked(4usize)) };
                        let b = unsafe { *(witness.get_unchecked(88usize)) };
                        a = a.mul(cs, &b);
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(32768u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(8usize)) };
                        let b = unsafe { *(witness.get_unchecked(88usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(48usize)) };
                        let b = unsafe { *(witness.get_unchecked(88usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.sub(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(3usize)) };
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(32768u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(4usize)) };
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2147450879u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let a = unsafe { *(witness.get_unchecked(48usize)) };
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(88usize)) };
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2147352575u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let a = unsafe { *(witness.get_unchecked(92usize)) };
                        individual_term = individual_term.sub(cs, &a);
                    }
                    let constant_term =
                        MersenneField::allocate_constant(cs, Mersenne31Field(131072u32));
                    individual_term = individual_term.add_base(cs, &constant_term);
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(35usize)) };
                        let b = unsafe { *(witness.get_unchecked(93usize)) };
                        a = a.mul(cs, &b);
                        a
                    };
                    {
                        let a = unsafe { *(witness.get_unchecked(87usize)) };
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let a = unsafe { *(witness.get_unchecked(94usize)) };
                        individual_term = individual_term.sub(cs, &a);
                    }
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(35usize)) };
                        let b = unsafe { *(witness.get_unchecked(88usize)) };
                        a = a.mul(cs, &b);
                        a = a.negated(cs);
                        a
                    };
                    {
                        let a = unsafe { *(witness.get_unchecked(88usize)) };
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let a = unsafe { *(witness.get_unchecked(89usize)) };
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let a = unsafe { *(witness.get_unchecked(95usize)) };
                        individual_term = individual_term.sub(cs, &a);
                    }
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(3usize)) };
                        let b = unsafe { *(witness.get_unchecked(34usize)) };
                        a = a.mul(cs, &b);
                        a
                    };
                    {
                        let mut a = unsafe { *(witness.get_unchecked(34usize)) };
                        let b = unsafe { *(memory.get_unchecked(8usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.sub(cs, &a);
                    }
                    {
                        let a = unsafe { *(witness.get_unchecked(98usize)) };
                        individual_term = individual_term.sub(cs, &a);
                    }
                    {
                        let a = unsafe { *(memory.get_unchecked(8usize)) };
                        individual_term = individual_term.add(cs, &a);
                    }
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(34usize)) };
                        let b = unsafe { *(witness.get_unchecked(48usize)) };
                        a = a.mul(cs, &b);
                        a
                    };
                    {
                        let mut a = unsafe { *(witness.get_unchecked(34usize)) };
                        let b = unsafe { *(memory.get_unchecked(9usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.sub(cs, &a);
                    }
                    {
                        let a = unsafe { *(witness.get_unchecked(99usize)) };
                        individual_term = individual_term.sub(cs, &a);
                    }
                    {
                        let a = unsafe { *(memory.get_unchecked(9usize)) };
                        individual_term = individual_term.add(cs, &a);
                    }
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(27usize)) };
                        let b = unsafe { *(witness.get_unchecked(86usize)) };
                        a = a.mul(cs, &b);
                        a
                    };
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(61usize)) };
                        let b = unsafe { *(memory.get_unchecked(8usize)) };
                        a = a.mul(cs, &b);
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(65536u32));
                        a = a.mul_by_base(cs, &coeff);
                        a
                    };
                    {
                        let mut a = unsafe { *(witness.get_unchecked(61usize)) };
                        let b = unsafe { *(memory.get_unchecked(9usize)) };
                        a = a.mul(cs, &b);
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(2u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(82usize)) };
                        let b = unsafe { *(memory.get_unchecked(8usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(82usize)) };
                        let b = unsafe { *(memory.get_unchecked(9usize)) };
                        a = a.mul(cs, &b);
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(65536u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let a = unsafe { *(witness.get_unchecked(100usize)) };
                        individual_term = individual_term.sub(cs, &a);
                    }
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(34usize)) };
                        let b = unsafe { *(witness.get_unchecked(61usize)) };
                        a = a.mul(cs, &b);
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(65536u32));
                        a = a.mul_by_base(cs, &coeff);
                        a
                    };
                    {
                        let mut a = unsafe { *(witness.get_unchecked(34usize)) };
                        let b = unsafe { *(witness.get_unchecked(82usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(34usize)) };
                        let b = unsafe { *(memory.get_unchecked(8usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(34usize)) };
                        let b = unsafe { *(memory.get_unchecked(9usize)) };
                        a = a.mul(cs, &b);
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(65536u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(35usize)) };
                        let b = unsafe { *(witness.get_unchecked(100usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(36usize)) };
                        let b = unsafe { *(witness.get_unchecked(61usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2147418111u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(36usize)) };
                        let b = unsafe { *(witness.get_unchecked(82usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.sub(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(36usize)) };
                        let b = unsafe { *(memory.get_unchecked(8usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(36usize)) };
                        let b = unsafe { *(memory.get_unchecked(9usize)) };
                        a = a.mul(cs, &b);
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(65536u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let a = unsafe { *(witness.get_unchecked(101usize)) };
                        individual_term = individual_term.sub(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(102usize)) };
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2147418111u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(5usize)) };
                        let b = unsafe { *(witness.get_unchecked(30usize)) };
                        a = a.mul(cs, &b);
                        a = a.negated(cs);
                        a
                    };
                    {
                        let mut a = unsafe { *(witness.get_unchecked(30usize)) };
                        let b = unsafe { *(witness.get_unchecked(101usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(6usize)) };
                        let b = unsafe { *(witness.get_unchecked(30usize)) };
                        a = a.mul(cs, &b);
                        a = a.negated(cs);
                        a
                    };
                    {
                        let mut a = unsafe { *(witness.get_unchecked(30usize)) };
                        let b = unsafe { *(witness.get_unchecked(102usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(30usize)) };
                        let b = unsafe { *(witness.get_unchecked(37usize)) };
                        a = a.mul(cs, &b);
                        a = a.negated(cs);
                        a
                    };
                    {
                        let a = unsafe { *(witness.get_unchecked(30usize)) };
                        individual_term = individual_term.add(cs, &a);
                    }
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(29usize)) };
                        let b = unsafe { *(witness.get_unchecked(86usize)) };
                        a = a.mul(cs, &b);
                        a = a.negated(cs);
                        a
                    };
                    {
                        let a = unsafe { *(witness.get_unchecked(29usize)) };
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let a = unsafe { *(witness.get_unchecked(103usize)) };
                        individual_term = individual_term.sub(cs, &a);
                    }
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(29usize)) };
                        let b = unsafe { *(witness.get_unchecked(86usize)) };
                        a = a.mul(cs, &b);
                        a
                    };
                    {
                        let a = unsafe { *(witness.get_unchecked(104usize)) };
                        individual_term = individual_term.sub(cs, &a);
                    }
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(103usize)) };
                        let b = unsafe { *(memory.get_unchecked(16usize)) };
                        a = a.mul(cs, &b);
                        a
                    };
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(103usize)) };
                        let b = unsafe { *(memory.get_unchecked(17usize)) };
                        a = a.mul(cs, &b);
                        a
                    };
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(103usize)) };
                        let b = unsafe { *(memory.get_unchecked(13usize)) };
                        a = a.mul(cs, &b);
                        a
                    };
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(103usize)) };
                        let b = unsafe { *(memory.get_unchecked(14usize)) };
                        a = a.mul(cs, &b);
                        a
                    };
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(5usize)) };
                        let b = unsafe { *(witness.get_unchecked(104usize)) };
                        a = a.mul(cs, &b);
                        a
                    };
                    {
                        let mut a = unsafe { *(witness.get_unchecked(104usize)) };
                        let b = unsafe { *(memory.get_unchecked(16usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.sub(cs, &a);
                    }
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(6usize)) };
                        let b = unsafe { *(witness.get_unchecked(104usize)) };
                        a = a.mul(cs, &b);
                        a
                    };
                    {
                        let mut a = unsafe { *(witness.get_unchecked(104usize)) };
                        let b = unsafe { *(memory.get_unchecked(17usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.sub(cs, &a);
                    }
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(14usize)) };
                        let b = unsafe { *(witness.get_unchecked(29usize)) };
                        a = a.mul(cs, &b);
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(2048u32));
                        a = a.mul_by_base(cs, &coeff);
                        a
                    };
                    {
                        let mut a = unsafe { *(witness.get_unchecked(29usize)) };
                        let b = unsafe { *(witness.get_unchecked(52usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2013265919u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(29usize)) };
                        let b = unsafe { *(witness.get_unchecked(54usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(134217728u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(29usize)) };
                        let b = unsafe { *(witness.get_unchecked(58usize)) };
                        a = a.mul(cs, &b);
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(32u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(29usize)) };
                        let b = unsafe { *(memory.get_unchecked(16usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(14usize)) };
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2147481599u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(52usize)) };
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(134217728u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(54usize)) };
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2013265919u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(58usize)) };
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2147483615u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let a = unsafe { *(memory.get_unchecked(16usize)) };
                        individual_term = individual_term.sub(cs, &a);
                    }
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(29usize)) };
                        let b = unsafe { *(memory.get_unchecked(17usize)) };
                        a = a.mul(cs, &b);
                        a = a.negated(cs);
                        a
                    };
                    {
                        let a = unsafe { *(memory.get_unchecked(17usize)) };
                        individual_term = individual_term.add(cs, &a);
                    }
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(33usize)) };
                        let b = unsafe { *(witness.get_unchecked(86usize)) };
                        a = a.mul(cs, &b);
                        a
                    };
                    {
                        let mut a = unsafe { *(witness.get_unchecked(33usize)) };
                        let b = unsafe { *(witness.get_unchecked(87usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(33usize)) };
                        let b = unsafe { *(witness.get_unchecked(88usize)) };
                        a = a.mul(cs, &b);
                        a = a.negated(cs);
                        a
                    };
                    {
                        let a = unsafe { *(witness.get_unchecked(33usize)) };
                        individual_term = individual_term.add(cs, &a);
                    }
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(5usize)) };
                        let b = unsafe { *(witness.get_unchecked(33usize)) };
                        a = a.mul(cs, &b);
                        a
                    };
                    {
                        let mut a = unsafe { *(witness.get_unchecked(33usize)) };
                        let b = unsafe { *(memory.get_unchecked(23usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.sub(cs, &a);
                    }
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(6usize)) };
                        let b = unsafe { *(witness.get_unchecked(33usize)) };
                        a = a.mul(cs, &b);
                        a
                    };
                    {
                        let mut a = unsafe { *(witness.get_unchecked(33usize)) };
                        let b = unsafe { *(memory.get_unchecked(24usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.sub(cs, &a);
                    }
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(33usize)) };
                        let b = unsafe { *(witness.get_unchecked(82usize)) };
                        a = a.mul(cs, &b);
                        a
                    };
                    {
                        let mut a = unsafe { *(witness.get_unchecked(33usize)) };
                        let b = unsafe { *(memory.get_unchecked(25usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.sub(cs, &a);
                    }
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(33usize)) };
                        let b = unsafe { *(witness.get_unchecked(61usize)) };
                        a = a.mul(cs, &b);
                        a
                    };
                    {
                        let mut a = unsafe { *(witness.get_unchecked(33usize)) };
                        let b = unsafe { *(memory.get_unchecked(26usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.sub(cs, &a);
                    }
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(26usize)) };
                        let b = unsafe { *(witness.get_unchecked(86usize)) };
                        a = a.mul(cs, &b);
                        a = a.negated(cs);
                        a
                    };
                    {
                        let a = unsafe { *(witness.get_unchecked(26usize)) };
                        individual_term = individual_term.add(cs, &a);
                    }
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(26usize)) };
                        let b = unsafe { *(witness.get_unchecked(87usize)) };
                        a = a.mul(cs, &b);
                        a
                    };
                    {
                        let a = unsafe { *(memory.get_unchecked(27usize)) };
                        individual_term = individual_term.sub(cs, &a);
                    }
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(memory.get_unchecked(9usize)) };
                        let b = unsafe { *(memory.get_unchecked(27usize)) };
                        a = a.mul(cs, &b);
                        a
                    };
                    {
                        let a = unsafe { *(memory.get_unchecked(29usize)) };
                        individual_term = individual_term.sub(cs, &a);
                    }
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(52usize)) };
                        let b = unsafe { *(memory.get_unchecked(27usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(134217728u32));
                        a = a.mul_by_base(cs, &coeff);
                        a
                    };
                    {
                        let mut a = unsafe { *(witness.get_unchecked(54usize)) };
                        let b = unsafe { *(memory.get_unchecked(27usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2013265919u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let a = unsafe { *(memory.get_unchecked(28usize)) };
                        individual_term = individual_term.sub(cs, &a);
                    }
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(9usize)) };
                        let b = unsafe { *(witness.get_unchecked(87usize)) };
                        a = a.mul(cs, &b);
                        a
                    };
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(10usize)) };
                        let b = unsafe { *(witness.get_unchecked(87usize)) };
                        a = a.mul(cs, &b);
                        a
                    };
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(3usize)) };
                        let b = unsafe { *(witness.get_unchecked(23usize)) };
                        a = a.mul(cs, &b);
                        a
                    };
                    {
                        let mut a = unsafe { *(witness.get_unchecked(5usize)) };
                        let b = unsafe { *(witness.get_unchecked(22usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.sub(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(5usize)) };
                        let b = unsafe { *(witness.get_unchecked(23usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.sub(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(5usize)) };
                        let b = unsafe { *(witness.get_unchecked(25usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(5usize)) };
                        let b = unsafe { *(witness.get_unchecked(27usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.sub(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(5usize)) };
                        let b = unsafe { *(witness.get_unchecked(29usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.sub(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(5usize)) };
                        let b = unsafe { *(witness.get_unchecked(30usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.sub(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(5usize)) };
                        let b = unsafe { *(witness.get_unchecked(32usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(5usize)) };
                        let b = unsafe { *(witness.get_unchecked(33usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.sub(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(7usize)) };
                        let b = unsafe { *(witness.get_unchecked(30usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(22usize)) };
                        let b = unsafe { *(witness.get_unchecked(82usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(22usize)) };
                        let b = unsafe { *(memory.get_unchecked(8usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(23usize)) };
                        let b = unsafe { *(witness.get_unchecked(80usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(25usize)) };
                        let b = unsafe { *(witness.get_unchecked(82usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(25usize)) };
                        let b = unsafe { *(memory.get_unchecked(8usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.sub(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(27usize)) };
                        let b = unsafe { *(witness.get_unchecked(80usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(27usize)) };
                        let b = unsafe { *(witness.get_unchecked(98usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(29usize)) };
                        let b = unsafe { *(witness.get_unchecked(80usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(29usize)) };
                        let b = unsafe { *(memory.get_unchecked(8usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(32usize)) };
                        let b = unsafe { *(witness.get_unchecked(82usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(32usize)) };
                        let b = unsafe { *(memory.get_unchecked(8usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.sub(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(33usize)) };
                        let b = unsafe { *(witness.get_unchecked(80usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(33usize)) };
                        let b = unsafe { *(memory.get_unchecked(8usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(30usize)) };
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(65535u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(40usize)) };
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2147418111u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(6usize)) };
                        let b = unsafe { *(witness.get_unchecked(22usize)) };
                        a = a.mul(cs, &b);
                        a = a.negated(cs);
                        a
                    };
                    {
                        let mut a = unsafe { *(witness.get_unchecked(6usize)) };
                        let b = unsafe { *(witness.get_unchecked(23usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.sub(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(6usize)) };
                        let b = unsafe { *(witness.get_unchecked(25usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(6usize)) };
                        let b = unsafe { *(witness.get_unchecked(27usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.sub(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(6usize)) };
                        let b = unsafe { *(witness.get_unchecked(29usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.sub(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(6usize)) };
                        let b = unsafe { *(witness.get_unchecked(30usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.sub(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(6usize)) };
                        let b = unsafe { *(witness.get_unchecked(32usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(6usize)) };
                        let b = unsafe { *(witness.get_unchecked(33usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.sub(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(8usize)) };
                        let b = unsafe { *(witness.get_unchecked(30usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(22usize)) };
                        let b = unsafe { *(witness.get_unchecked(61usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(22usize)) };
                        let b = unsafe { *(memory.get_unchecked(9usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(23usize)) };
                        let b = unsafe { *(witness.get_unchecked(48usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(23usize)) };
                        let b = unsafe { *(witness.get_unchecked(81usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(25usize)) };
                        let b = unsafe { *(witness.get_unchecked(61usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(25usize)) };
                        let b = unsafe { *(memory.get_unchecked(9usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.sub(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(27usize)) };
                        let b = unsafe { *(witness.get_unchecked(81usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(27usize)) };
                        let b = unsafe { *(witness.get_unchecked(99usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(29usize)) };
                        let b = unsafe { *(witness.get_unchecked(81usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(29usize)) };
                        let b = unsafe { *(memory.get_unchecked(9usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(32usize)) };
                        let b = unsafe { *(witness.get_unchecked(61usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(32usize)) };
                        let b = unsafe { *(memory.get_unchecked(9usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.sub(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(33usize)) };
                        let b = unsafe { *(witness.get_unchecked(81usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(33usize)) };
                        let b = unsafe { *(memory.get_unchecked(9usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(30usize)) };
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(32767u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(37usize)) };
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2147418111u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let a = unsafe { *(witness.get_unchecked(40usize)) };
                        individual_term = individual_term.add(cs, &a);
                    }
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(3usize)) };
                        let b = unsafe { *(witness.get_unchecked(25usize)) };
                        a = a.mul(cs, &b);
                        a
                    };
                    {
                        let mut a = unsafe { *(witness.get_unchecked(7usize)) };
                        let b = unsafe { *(witness.get_unchecked(25usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.sub(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(25usize)) };
                        let b = unsafe { *(witness.get_unchecked(80usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(41usize)) };
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2147418111u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(8usize)) };
                        let b = unsafe { *(witness.get_unchecked(25usize)) };
                        a = a.mul(cs, &b);
                        a = a.negated(cs);
                        a
                    };
                    {
                        let mut a = unsafe { *(witness.get_unchecked(25usize)) };
                        let b = unsafe { *(witness.get_unchecked(48usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(25usize)) };
                        let b = unsafe { *(witness.get_unchecked(81usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(39usize)) };
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2147418111u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let a = unsafe { *(witness.get_unchecked(41usize)) };
                        individual_term = individual_term.add(cs, &a);
                    }
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(5usize)) };
                        let b = unsafe { *(witness.get_unchecked(25usize)) };
                        a = a.mul(cs, &b);
                        a
                    };
                    {
                        let a = unsafe { *(witness.get_unchecked(105usize)) };
                        individual_term = individual_term.sub(cs, &a);
                    }
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(6usize)) };
                        let b = unsafe { *(witness.get_unchecked(25usize)) };
                        a = a.mul(cs, &b);
                        a
                    };
                    {
                        let a = unsafe { *(witness.get_unchecked(106usize)) };
                        individual_term = individual_term.sub(cs, &a);
                    }
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(105usize)) };
                        let b = unsafe { *(witness.get_unchecked(107usize)) };
                        a = a.mul(cs, &b);
                        a
                    };
                    {
                        let mut a = unsafe { *(witness.get_unchecked(106usize)) };
                        let b = unsafe { *(witness.get_unchecked(107usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let a = unsafe { *(witness.get_unchecked(38usize)) };
                        individual_term = individual_term.add(cs, &a);
                    }
                    let constant_term =
                        MersenneField::allocate_constant(cs, Mersenne31Field(2147483646u32));
                    individual_term = individual_term.add_base(cs, &constant_term);
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(38usize)) };
                        let b = unsafe { *(witness.get_unchecked(105usize)) };
                        a = a.mul(cs, &b);
                        a
                    };
                    {
                        let mut a = unsafe { *(witness.get_unchecked(38usize)) };
                        let b = unsafe { *(witness.get_unchecked(106usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(5usize)) };
                        let b = unsafe { *(witness.get_unchecked(27usize)) };
                        a = a.mul(cs, &b);
                        a
                    };
                    {
                        let mut a = unsafe { *(witness.get_unchecked(5usize)) };
                        let b = unsafe { *(witness.get_unchecked(33usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(6usize)) };
                        let b = unsafe { *(witness.get_unchecked(29usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(7usize)) };
                        let b = unsafe { *(witness.get_unchecked(25usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(24usize)) };
                        let b = unsafe { *(witness.get_unchecked(84usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(26usize)) };
                        let b = unsafe { *(witness.get_unchecked(52usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(134217728u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(26usize)) };
                        let b = unsafe { *(witness.get_unchecked(54usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2013265919u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(31usize)) };
                        let b = unsafe { *(witness.get_unchecked(85usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let a = unsafe { *(witness.get_unchecked(64usize)) };
                        individual_term = individual_term.sub(cs, &a);
                    }
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(24usize)) };
                        let b = unsafe { *(witness.get_unchecked(85usize)) };
                        a = a.mul(cs, &b);
                        a
                    };
                    {
                        let mut a = unsafe { *(witness.get_unchecked(25usize)) };
                        let b = unsafe { *(witness.get_unchecked(86usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(26usize)) };
                        let b = unsafe { *(witness.get_unchecked(86usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(27usize)) };
                        let b = unsafe { *(witness.get_unchecked(86usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(29usize)) };
                        let b = unsafe { *(witness.get_unchecked(86usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(33usize)) };
                        let b = unsafe { *(witness.get_unchecked(86usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(31usize)) };
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(31u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let a = unsafe { *(witness.get_unchecked(65usize)) };
                        individual_term = individual_term.sub(cs, &a);
                    }
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(24usize)) };
                        let b = unsafe { *(witness.get_unchecked(86usize)) };
                        a = a.mul(cs, &b);
                        a
                    };
                    {
                        let mut a = unsafe { *(witness.get_unchecked(25usize)) };
                        let b = unsafe { *(witness.get_unchecked(87usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(26usize)) };
                        let b = unsafe { *(witness.get_unchecked(87usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(27usize)) };
                        let b = unsafe { *(witness.get_unchecked(87usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(29usize)) };
                        let b = unsafe { *(witness.get_unchecked(87usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(31usize)) };
                        let b = unsafe { *(witness.get_unchecked(86usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(33usize)) };
                        let b = unsafe { *(witness.get_unchecked(87usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let a = unsafe { *(witness.get_unchecked(66usize)) };
                        individual_term = individual_term.sub(cs, &a);
                    }
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(24usize)) };
                        let b = unsafe { *(witness.get_unchecked(57usize)) };
                        a = a.mul(cs, &b);
                        a
                    };
                    {
                        let mut a = unsafe { *(witness.get_unchecked(25usize)) };
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(17u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(26usize)) };
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(25u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(27usize)) };
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(17u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(29usize)) };
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(23u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(31usize)) };
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(7u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(33usize)) };
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(18u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let a = unsafe { *(witness.get_unchecked(67usize)) };
                        individual_term = individual_term.sub(cs, &a);
                    }
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(5usize)) };
                        let b = unsafe { *(witness.get_unchecked(29usize)) };
                        a = a.mul(cs, &b);
                        a
                    };
                    {
                        let mut a = unsafe { *(witness.get_unchecked(6usize)) };
                        let b = unsafe { *(witness.get_unchecked(33usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(24usize)) };
                        let b = unsafe { *(witness.get_unchecked(84usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2139095039u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(24usize)) };
                        let b = unsafe { *(memory.get_unchecked(8usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(8388608u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(25usize)) };
                        let b = unsafe { *(witness.get_unchecked(37usize)) };
                        a = a.mul(cs, &b);
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(8u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(25usize)) };
                        let b = unsafe { *(witness.get_unchecked(38usize)) };
                        a = a.mul(cs, &b);
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(16u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(25usize)) };
                        let b = unsafe { *(witness.get_unchecked(57usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(25usize)) };
                        let b = unsafe { *(witness.get_unchecked(59usize)) };
                        a = a.mul(cs, &b);
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(32u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(25usize)) };
                        let b = unsafe { *(witness.get_unchecked(62usize)) };
                        a = a.mul(cs, &b);
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(64u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(29usize)) };
                        let b = unsafe { *(witness.get_unchecked(87usize)) };
                        a = a.mul(cs, &b);
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(65536u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(31usize)) };
                        let b = unsafe { *(witness.get_unchecked(35usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2097152u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(31usize)) };
                        let b = unsafe { *(witness.get_unchecked(86usize)) };
                        a = a.mul(cs, &b);
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(65536u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(31usize)) };
                        let b = unsafe { *(memory.get_unchecked(8usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let a = unsafe { *(witness.get_unchecked(68usize)) };
                        individual_term = individual_term.sub(cs, &a);
                    }
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(24usize)) };
                        let b = unsafe { *(witness.get_unchecked(82usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(8388608u32));
                        a = a.mul_by_base(cs, &coeff);
                        a
                    };
                    {
                        let mut a = unsafe { *(witness.get_unchecked(24usize)) };
                        let b = unsafe { *(witness.get_unchecked(85usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2139095039u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(25usize)) };
                        let b = unsafe { *(witness.get_unchecked(88usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(29usize)) };
                        let b = unsafe { *(witness.get_unchecked(88usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(31usize)) };
                        let b = unsafe { *(witness.get_unchecked(87usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(33usize)) };
                        let b = unsafe { *(witness.get_unchecked(88usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let a = unsafe { *(witness.get_unchecked(69usize)) };
                        individual_term = individual_term.sub(cs, &a);
                    }
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(24usize)) };
                        let b = unsafe { *(witness.get_unchecked(87usize)) };
                        a = a.mul(cs, &b);
                        a
                    };
                    {
                        let mut a = unsafe { *(witness.get_unchecked(25usize)) };
                        let b = unsafe { *(witness.get_unchecked(89usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(29usize)) };
                        let b = unsafe { *(witness.get_unchecked(89usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(31usize)) };
                        let b = unsafe { *(witness.get_unchecked(88usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(33usize)) };
                        let b = unsafe { *(witness.get_unchecked(89usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let a = unsafe { *(witness.get_unchecked(70usize)) };
                        individual_term = individual_term.sub(cs, &a);
                    }
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(24usize)) };
                        let b = unsafe { *(witness.get_unchecked(57usize)) };
                        a = a.mul(cs, &b);
                        a
                    };
                    {
                        let mut a = unsafe { *(witness.get_unchecked(25usize)) };
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(22u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(29usize)) };
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(24u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(31usize)) };
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(37u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(33usize)) };
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(23u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let a = unsafe { *(witness.get_unchecked(71usize)) };
                        individual_term = individual_term.sub(cs, &a);
                    }
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(24usize)) };
                        let b = unsafe { *(witness.get_unchecked(60usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2147483391u32));
                        a = a.mul_by_base(cs, &coeff);
                        a
                    };
                    {
                        let mut a = unsafe { *(witness.get_unchecked(24usize)) };
                        let b = unsafe { *(memory.get_unchecked(9usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(31usize)) };
                        let b = unsafe { *(witness.get_unchecked(35usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2097152u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(31usize)) };
                        let b = unsafe { *(witness.get_unchecked(86usize)) };
                        a = a.mul(cs, &b);
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(65536u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(31usize)) };
                        let b = unsafe { *(memory.get_unchecked(9usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let a = unsafe { *(witness.get_unchecked(72usize)) };
                        individual_term = individual_term.sub(cs, &a);
                    }
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(24usize)) };
                        let b = unsafe { *(witness.get_unchecked(61usize)) };
                        a = a.mul(cs, &b);
                        a
                    };
                    {
                        let mut a = unsafe { *(witness.get_unchecked(24usize)) };
                        let b = unsafe { *(witness.get_unchecked(63usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2147483391u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(31usize)) };
                        let b = unsafe { *(witness.get_unchecked(89usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let a = unsafe { *(witness.get_unchecked(73usize)) };
                        individual_term = individual_term.sub(cs, &a);
                    }
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(24usize)) };
                        let b = unsafe { *(witness.get_unchecked(88usize)) };
                        a = a.mul(cs, &b);
                        a
                    };
                    {
                        let mut a = unsafe { *(witness.get_unchecked(31usize)) };
                        let b = unsafe { *(witness.get_unchecked(93usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let a = unsafe { *(witness.get_unchecked(74usize)) };
                        individual_term = individual_term.sub(cs, &a);
                    }
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(24usize)) };
                        let b = unsafe { *(witness.get_unchecked(57usize)) };
                        a = a.mul(cs, &b);
                        a
                    };
                    {
                        let mut a = unsafe { *(witness.get_unchecked(31usize)) };
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(37u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let a = unsafe { *(witness.get_unchecked(75usize)) };
                        individual_term = individual_term.sub(cs, &a);
                    }
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(24usize)) };
                        let b = unsafe { *(witness.get_unchecked(60usize)) };
                        a = a.mul(cs, &b);
                        a
                    };
                    {
                        let mut a = unsafe { *(witness.get_unchecked(31usize)) };
                        let b = unsafe { *(witness.get_unchecked(34usize)) };
                        a = a.mul(cs, &b);
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(2u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(31usize)) };
                        let b = unsafe { *(witness.get_unchecked(59usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(31usize)) };
                        let b = unsafe { *(witness.get_unchecked(86usize)) };
                        a = a.mul(cs, &b);
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(4u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let a = unsafe { *(witness.get_unchecked(76usize)) };
                        individual_term = individual_term.sub(cs, &a);
                    }
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(24usize)) };
                        let b = unsafe { *(witness.get_unchecked(63usize)) };
                        a = a.mul(cs, &b);
                        a
                    };
                    {
                        let mut a = unsafe { *(witness.get_unchecked(31usize)) };
                        let b = unsafe { *(witness.get_unchecked(96usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let a = unsafe { *(witness.get_unchecked(77usize)) };
                        individual_term = individual_term.sub(cs, &a);
                    }
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(24usize)) };
                        let b = unsafe { *(witness.get_unchecked(89usize)) };
                        a = a.mul(cs, &b);
                        a
                    };
                    {
                        let mut a = unsafe { *(witness.get_unchecked(31usize)) };
                        let b = unsafe { *(witness.get_unchecked(97usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let a = unsafe { *(witness.get_unchecked(78usize)) };
                        individual_term = individual_term.sub(cs, &a);
                    }
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(24usize)) };
                        let b = unsafe { *(witness.get_unchecked(57usize)) };
                        a = a.mul(cs, &b);
                        a
                    };
                    {
                        let mut a = unsafe { *(witness.get_unchecked(31usize)) };
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(20u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let a = unsafe { *(witness.get_unchecked(79usize)) };
                        individual_term = individual_term.sub(cs, &a);
                    }
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(4usize)) };
                        let b = unsafe { *(witness.get_unchecked(27usize)) };
                        a = a.mul(cs, &b);
                        a
                    };
                    {
                        let mut a = unsafe { *(witness.get_unchecked(5usize)) };
                        let b = unsafe { *(witness.get_unchecked(22usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(5usize)) };
                        let b = unsafe { *(witness.get_unchecked(23usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(5usize)) };
                        let b = unsafe { *(witness.get_unchecked(30usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(5usize)) };
                        let b = unsafe { *(witness.get_unchecked(32usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(9usize)) };
                        let b = unsafe { *(witness.get_unchecked(26usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(24usize)) };
                        let b = unsafe { *(witness.get_unchecked(86usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(24usize)) };
                        let b = unsafe { *(witness.get_unchecked(87usize)) };
                        a = a.mul(cs, &b);
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(256u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(25usize)) };
                        let b = unsafe { *(witness.get_unchecked(89usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(28usize)) };
                        let b = unsafe { *(witness.get_unchecked(80usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(31usize)) };
                        let b = unsafe { *(witness.get_unchecked(94usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(31usize)) };
                        let b = unsafe { *(witness.get_unchecked(96usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(88usize)) };
                        let b = unsafe { *(witness.get_unchecked(103usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(104usize)) };
                        let b = unsafe { *(memory.get_unchecked(13usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let a = unsafe { *(witness.get_unchecked(108usize)) };
                        individual_term = individual_term.sub(cs, &a);
                    }
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(3usize)) };
                        let b = unsafe { *(witness.get_unchecked(27usize)) };
                        a = a.mul(cs, &b);
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(32768u32));
                        a = a.mul_by_base(cs, &coeff);
                        a
                    };
                    {
                        let mut a = unsafe { *(witness.get_unchecked(4usize)) };
                        let b = unsafe { *(witness.get_unchecked(27usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2147450879u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(6usize)) };
                        let b = unsafe { *(witness.get_unchecked(22usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(6usize)) };
                        let b = unsafe { *(witness.get_unchecked(23usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(6usize)) };
                        let b = unsafe { *(witness.get_unchecked(30usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(6usize)) };
                        let b = unsafe { *(witness.get_unchecked(32usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(10usize)) };
                        let b = unsafe { *(witness.get_unchecked(26usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(24usize)) };
                        let b = unsafe { *(witness.get_unchecked(88usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(24usize)) };
                        let b = unsafe { *(witness.get_unchecked(89usize)) };
                        a = a.mul(cs, &b);
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(256u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(27usize)) };
                        let b = unsafe { *(witness.get_unchecked(48usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(28usize)) };
                        let b = unsafe { *(witness.get_unchecked(81usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(31usize)) };
                        let b = unsafe { *(witness.get_unchecked(95usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(31usize)) };
                        let b = unsafe { *(witness.get_unchecked(97usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(89usize)) };
                        let b = unsafe { *(witness.get_unchecked(103usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(104usize)) };
                        let b = unsafe { *(memory.get_unchecked(14usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(27usize)) };
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(131072u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let a = unsafe { *(witness.get_unchecked(109usize)) };
                        individual_term = individual_term.sub(cs, &a);
                    }
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(13usize)) };
                        let b = unsafe { *(witness.get_unchecked(42usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2147483391u32));
                        a = a.mul_by_base(cs, &coeff);
                        a
                    };
                    {
                        let mut a = unsafe { *(witness.get_unchecked(42usize)) };
                        let b = unsafe { *(witness.get_unchecked(51usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(16777216u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(42usize)) };
                        let b = unsafe { *(witness.get_unchecked(56usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2130706431u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(42usize)) };
                        let b = unsafe { *(witness.get_unchecked(57usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2147483615u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(13usize)) };
                        let b = unsafe { *(witness.get_unchecked(110usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2147483391u32));
                        a = a.mul_by_base(cs, &coeff);
                        a
                    };
                    {
                        let mut a = unsafe { *(witness.get_unchecked(51usize)) };
                        let b = unsafe { *(witness.get_unchecked(110usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(16777216u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(56usize)) };
                        let b = unsafe { *(witness.get_unchecked(110usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2130706431u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(57usize)) };
                        let b = unsafe { *(witness.get_unchecked(110usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2147483615u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let a = unsafe { *(witness.get_unchecked(42usize)) };
                        individual_term = individual_term.add(cs, &a);
                    }
                    let constant_term =
                        MersenneField::allocate_constant(cs, Mersenne31Field(2147483646u32));
                    individual_term = individual_term.add_base(cs, &constant_term);
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(42usize)) };
                        let b = unsafe { *(witness.get_unchecked(108usize)) };
                        a = a.mul(cs, &b);
                        a = a.negated(cs);
                        a
                    };
                    {
                        let a = unsafe { *(witness.get_unchecked(108usize)) };
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let a = unsafe { *(witness.get_unchecked(111usize)) };
                        individual_term = individual_term.sub(cs, &a);
                    }
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(42usize)) };
                        let b = unsafe { *(witness.get_unchecked(109usize)) };
                        a = a.mul(cs, &b);
                        a = a.negated(cs);
                        a
                    };
                    {
                        let a = unsafe { *(witness.get_unchecked(109usize)) };
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let a = unsafe { *(witness.get_unchecked(112usize)) };
                        individual_term = individual_term.sub(cs, &a);
                    }
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(13usize)) };
                        let b = unsafe { *(witness.get_unchecked(16usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2147483391u32));
                        a = a.mul_by_base(cs, &coeff);
                        a
                    };
                    {
                        let mut a = unsafe { *(witness.get_unchecked(13usize)) };
                        let b = unsafe { *(witness.get_unchecked(17usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2147483391u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(13usize)) };
                        let b = unsafe { *(witness.get_unchecked(20usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2147483391u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(13usize)) };
                        let b = unsafe { *(witness.get_unchecked(21usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2147483391u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(16usize)) };
                        let b = unsafe { *(witness.get_unchecked(51usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(16777216u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(16usize)) };
                        let b = unsafe { *(witness.get_unchecked(56usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2130706431u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(16usize)) };
                        let b = unsafe { *(witness.get_unchecked(57usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2147483615u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(16usize)) };
                        let b = unsafe { *(memory.get_unchecked(23usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.sub(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(17usize)) };
                        let b = unsafe { *(witness.get_unchecked(51usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(16777216u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(17usize)) };
                        let b = unsafe { *(witness.get_unchecked(56usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2130706431u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(17usize)) };
                        let b = unsafe { *(witness.get_unchecked(57usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2147483615u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(17usize)) };
                        let b = unsafe { *(memory.get_unchecked(23usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.sub(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(20usize)) };
                        let b = unsafe { *(witness.get_unchecked(51usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(16777216u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(20usize)) };
                        let b = unsafe { *(witness.get_unchecked(56usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2130706431u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(20usize)) };
                        let b = unsafe { *(witness.get_unchecked(57usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2147483615u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(20usize)) };
                        let b = unsafe { *(memory.get_unchecked(23usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.sub(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(21usize)) };
                        let b = unsafe { *(witness.get_unchecked(51usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(16777216u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(21usize)) };
                        let b = unsafe { *(witness.get_unchecked(56usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2130706431u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(21usize)) };
                        let b = unsafe { *(witness.get_unchecked(57usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2147483615u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(21usize)) };
                        let b = unsafe { *(memory.get_unchecked(23usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.sub(cs, &a);
                    }
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(16usize)) };
                        let b = unsafe { *(memory.get_unchecked(24usize)) };
                        a = a.mul(cs, &b);
                        a
                    };
                    {
                        let mut a = unsafe { *(witness.get_unchecked(17usize)) };
                        let b = unsafe { *(memory.get_unchecked(24usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(20usize)) };
                        let b = unsafe { *(memory.get_unchecked(24usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(21usize)) };
                        let b = unsafe { *(memory.get_unchecked(24usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(19usize)) };
                        let b = unsafe { *(memory.get_unchecked(23usize)) };
                        a = a.mul(cs, &b);
                        a
                    };
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(19usize)) };
                        let b = unsafe { *(memory.get_unchecked(24usize)) };
                        a = a.mul(cs, &b);
                        a
                    };
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(16usize)) };
                        let b = unsafe { *(witness.get_unchecked(111usize)) };
                        a = a.mul(cs, &b);
                        a
                    };
                    {
                        let mut a = unsafe { *(witness.get_unchecked(16usize)) };
                        let b = unsafe { *(memory.get_unchecked(25usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.sub(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(17usize)) };
                        let b = unsafe { *(witness.get_unchecked(111usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(17usize)) };
                        let b = unsafe { *(memory.get_unchecked(25usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.sub(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(20usize)) };
                        let b = unsafe { *(witness.get_unchecked(111usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(20usize)) };
                        let b = unsafe { *(memory.get_unchecked(25usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.sub(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(21usize)) };
                        let b = unsafe { *(witness.get_unchecked(111usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(21usize)) };
                        let b = unsafe { *(memory.get_unchecked(25usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.sub(cs, &a);
                    }
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(16usize)) };
                        let b = unsafe { *(witness.get_unchecked(112usize)) };
                        a = a.mul(cs, &b);
                        a
                    };
                    {
                        let mut a = unsafe { *(witness.get_unchecked(16usize)) };
                        let b = unsafe { *(memory.get_unchecked(26usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.sub(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(17usize)) };
                        let b = unsafe { *(witness.get_unchecked(112usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(17usize)) };
                        let b = unsafe { *(memory.get_unchecked(26usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.sub(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(20usize)) };
                        let b = unsafe { *(witness.get_unchecked(112usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(20usize)) };
                        let b = unsafe { *(memory.get_unchecked(26usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.sub(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(21usize)) };
                        let b = unsafe { *(witness.get_unchecked(112usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(21usize)) };
                        let b = unsafe { *(memory.get_unchecked(26usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.sub(cs, &a);
                    }
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(19usize)) };
                        let b = unsafe { *(memory.get_unchecked(25usize)) };
                        a = a.mul(cs, &b);
                        a
                    };
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(19usize)) };
                        let b = unsafe { *(memory.get_unchecked(26usize)) };
                        a = a.mul(cs, &b);
                        a
                    };
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(4usize)) };
                        let b = unsafe { *(witness.get_unchecked(22usize)) };
                        a = a.mul(cs, &b);
                        a
                    };
                    {
                        let mut a = unsafe { *(witness.get_unchecked(4usize)) };
                        let b = unsafe { *(witness.get_unchecked(23usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(4usize)) };
                        let b = unsafe { *(witness.get_unchecked(24usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(4usize)) };
                        let b = unsafe { *(witness.get_unchecked(26usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(4usize)) };
                        let b = unsafe { *(witness.get_unchecked(28usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(4usize)) };
                        let b = unsafe { *(witness.get_unchecked(29usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(4usize)) };
                        let b = unsafe { *(witness.get_unchecked(30usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(4usize)) };
                        let b = unsafe { *(witness.get_unchecked(31usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(4usize)) };
                        let b = unsafe { *(witness.get_unchecked(32usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(4usize)) };
                        let b = unsafe { *(witness.get_unchecked(33usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(25usize)) };
                        let b = unsafe { *(witness.get_unchecked(91usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(27usize)) };
                        let b = unsafe { *(witness.get_unchecked(87usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let a = unsafe { *(witness.get_unchecked(113usize)) };
                        individual_term = individual_term.sub(cs, &a);
                    }
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(3usize)) };
                        let b = unsafe { *(witness.get_unchecked(22usize)) };
                        a = a.mul(cs, &b);
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(32768u32));
                        a = a.mul_by_base(cs, &coeff);
                        a
                    };
                    {
                        let mut a = unsafe { *(witness.get_unchecked(3usize)) };
                        let b = unsafe { *(witness.get_unchecked(23usize)) };
                        a = a.mul(cs, &b);
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(32768u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(3usize)) };
                        let b = unsafe { *(witness.get_unchecked(24usize)) };
                        a = a.mul(cs, &b);
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(32768u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(3usize)) };
                        let b = unsafe { *(witness.get_unchecked(26usize)) };
                        a = a.mul(cs, &b);
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(32768u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(3usize)) };
                        let b = unsafe { *(witness.get_unchecked(28usize)) };
                        a = a.mul(cs, &b);
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(32768u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(3usize)) };
                        let b = unsafe { *(witness.get_unchecked(29usize)) };
                        a = a.mul(cs, &b);
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(32768u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(3usize)) };
                        let b = unsafe { *(witness.get_unchecked(30usize)) };
                        a = a.mul(cs, &b);
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(32768u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(3usize)) };
                        let b = unsafe { *(witness.get_unchecked(31usize)) };
                        a = a.mul(cs, &b);
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(32768u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(3usize)) };
                        let b = unsafe { *(witness.get_unchecked(32usize)) };
                        a = a.mul(cs, &b);
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(32768u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(3usize)) };
                        let b = unsafe { *(witness.get_unchecked(33usize)) };
                        a = a.mul(cs, &b);
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(32768u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(4usize)) };
                        let b = unsafe { *(witness.get_unchecked(22usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2147450879u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(4usize)) };
                        let b = unsafe { *(witness.get_unchecked(23usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2147450879u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(4usize)) };
                        let b = unsafe { *(witness.get_unchecked(24usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2147450879u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(4usize)) };
                        let b = unsafe { *(witness.get_unchecked(26usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2147450879u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(4usize)) };
                        let b = unsafe { *(witness.get_unchecked(28usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2147450879u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(4usize)) };
                        let b = unsafe { *(witness.get_unchecked(29usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2147450879u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(4usize)) };
                        let b = unsafe { *(witness.get_unchecked(30usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2147450879u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(4usize)) };
                        let b = unsafe { *(witness.get_unchecked(31usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2147450879u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(4usize)) };
                        let b = unsafe { *(witness.get_unchecked(32usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2147450879u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(4usize)) };
                        let b = unsafe { *(witness.get_unchecked(33usize)) };
                        a = a.mul(cs, &b);
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(2147450879u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(6usize)) };
                        let b = unsafe { *(witness.get_unchecked(27usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(22usize)) };
                        let b = unsafe { *(witness.get_unchecked(48usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(23usize)) };
                        let b = unsafe { *(witness.get_unchecked(48usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(24usize)) };
                        let b = unsafe { *(witness.get_unchecked(48usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(25usize)) };
                        let b = unsafe { *(witness.get_unchecked(92usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(26usize)) };
                        let b = unsafe { *(witness.get_unchecked(48usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(28usize)) };
                        let b = unsafe { *(witness.get_unchecked(48usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(29usize)) };
                        let b = unsafe { *(witness.get_unchecked(48usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(30usize)) };
                        let b = unsafe { *(witness.get_unchecked(48usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(31usize)) };
                        let b = unsafe { *(witness.get_unchecked(48usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(32usize)) };
                        let b = unsafe { *(witness.get_unchecked(48usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(33usize)) };
                        let b = unsafe { *(witness.get_unchecked(48usize)) };
                        a = a.mul(cs, &b);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(22usize)) };
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(131072u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(23usize)) };
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(131072u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(24usize)) };
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(131072u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(26usize)) };
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(131072u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(28usize)) };
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(131072u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(29usize)) };
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(131072u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(30usize)) };
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(131072u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(31usize)) };
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(131072u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(32usize)) };
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(131072u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let mut a = unsafe { *(witness.get_unchecked(33usize)) };
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(131072u32));
                        a = a.mul_by_base(cs, &coeff);
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let a = unsafe { *(witness.get_unchecked(114usize)) };
                        individual_term = individual_term.sub(cs, &a);
                    }
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let a = unsafe { *(witness.get_unchecked(49usize)) };
                        a
                    };
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let a = unsafe { *(witness.get_unchecked(13usize)) };
                        a
                    };
                    {
                        let mut a = unsafe { *(witness.get_unchecked(54usize)) };
                        let coeff = MersenneField::allocate_constant(cs, Mersenne31Field(2u32));
                        individual_term = a.mul_by_base_and_add(cs, &coeff, &individual_term);
                    }
                    {
                        let a = unsafe { *(memory.get_unchecked(10usize)) };
                        individual_term = individual_term.sub(cs, &a);
                    }
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let a = unsafe { *(witness.get_unchecked(15usize)) };
                        a
                    };
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(29usize)) };
                        a = a.negated(cs);
                        a
                    };
                    {
                        let a = unsafe { *(memory.get_unchecked(15usize)) };
                        individual_term = individual_term.sub(cs, &a);
                    }
                    let constant_term = MersenneField::allocate_constant(cs, Mersenne31Field(1u32));
                    individual_term = individual_term.add_base(cs, &constant_term);
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(33usize)) };
                        a = a.negated(cs);
                        a
                    };
                    {
                        let a = unsafe { *(memory.get_unchecked(22usize)) };
                        individual_term = individual_term.sub(cs, &a);
                    }
                    let constant_term = MersenneField::allocate_constant(cs, Mersenne31Field(1u32));
                    individual_term = individual_term.add_base(cs, &constant_term);
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let a = {
                let value = unsafe { *(witness.get_unchecked(3usize)) };
                value
            };
            let b = {
                let value = unsafe { *(witness.get_unchecked(4usize)) };
                value
            };
            let c = unsafe { *(stage_2.get_unchecked(0usize)) };
            {
                let contribution = {
                    let individual_term = {
                        let mut individual_term = a;
                        individual_term = individual_term.mul(cs, &b);
                        individual_term = individual_term.sub(cs, &c);
                        individual_term
                    };
                    individual_term
                };
                accumulated_contribution =
                    accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
            }
            {
                let contribution = {
                    let individual_term = {
                        let acc_value = unsafe { *(stage_2.get_unchecked(9usize)) };
                        let mut denom = lookup_argument_gamma;
                        denom = denom.add(cs, &a);
                        denom = denom.add(cs, &b);
                        denom = denom.mul(cs, &lookup_argument_gamma);
                        denom = denom.add(cs, &c);
                        denom = denom.mul(cs, &acc_value);
                        let mut numerator = lookup_argument_two_gamma;
                        numerator = numerator.add(cs, &a);
                        numerator = numerator.add(cs, &b);
                        let mut individual_term = denom;
                        individual_term = individual_term.sub(cs, &numerator);
                        individual_term
                    };
                    individual_term
                };
                accumulated_contribution =
                    accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
            }
        }
        {
            let a = {
                let value = unsafe { *(witness.get_unchecked(5usize)) };
                value
            };
            let b = {
                let value = unsafe { *(witness.get_unchecked(6usize)) };
                value
            };
            let c = unsafe { *(stage_2.get_unchecked(1usize)) };
            {
                let contribution = {
                    let individual_term = {
                        let mut individual_term = a;
                        individual_term = individual_term.mul(cs, &b);
                        individual_term = individual_term.sub(cs, &c);
                        individual_term
                    };
                    individual_term
                };
                accumulated_contribution =
                    accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
            }
            {
                let contribution = {
                    let individual_term = {
                        let acc_value = unsafe { *(stage_2.get_unchecked(10usize)) };
                        let mut denom = lookup_argument_gamma;
                        denom = denom.add(cs, &a);
                        denom = denom.add(cs, &b);
                        denom = denom.mul(cs, &lookup_argument_gamma);
                        denom = denom.add(cs, &c);
                        denom = denom.mul(cs, &acc_value);
                        let mut numerator = lookup_argument_two_gamma;
                        numerator = numerator.add(cs, &a);
                        numerator = numerator.add(cs, &b);
                        let mut individual_term = denom;
                        individual_term = individual_term.sub(cs, &numerator);
                        individual_term
                    };
                    individual_term
                };
                accumulated_contribution =
                    accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
            }
        }
        {
            let a = {
                let value = unsafe { *(witness.get_unchecked(7usize)) };
                value
            };
            let b = {
                let value = unsafe { *(witness.get_unchecked(8usize)) };
                value
            };
            let c = unsafe { *(stage_2.get_unchecked(2usize)) };
            {
                let contribution = {
                    let individual_term = {
                        let mut individual_term = a;
                        individual_term = individual_term.mul(cs, &b);
                        individual_term = individual_term.sub(cs, &c);
                        individual_term
                    };
                    individual_term
                };
                accumulated_contribution =
                    accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
            }
            {
                let contribution = {
                    let individual_term = {
                        let acc_value = unsafe { *(stage_2.get_unchecked(11usize)) };
                        let mut denom = lookup_argument_gamma;
                        denom = denom.add(cs, &a);
                        denom = denom.add(cs, &b);
                        denom = denom.mul(cs, &lookup_argument_gamma);
                        denom = denom.add(cs, &c);
                        denom = denom.mul(cs, &acc_value);
                        let mut numerator = lookup_argument_two_gamma;
                        numerator = numerator.add(cs, &a);
                        numerator = numerator.add(cs, &b);
                        let mut individual_term = denom;
                        individual_term = individual_term.sub(cs, &numerator);
                        individual_term
                    };
                    individual_term
                };
                accumulated_contribution =
                    accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
            }
        }
        {
            let a = {
                let value = unsafe { *(witness.get_unchecked(9usize)) };
                value
            };
            let b = {
                let value = unsafe { *(witness.get_unchecked(10usize)) };
                value
            };
            let c = unsafe { *(stage_2.get_unchecked(3usize)) };
            {
                let contribution = {
                    let individual_term = {
                        let mut individual_term = a;
                        individual_term = individual_term.mul(cs, &b);
                        individual_term = individual_term.sub(cs, &c);
                        individual_term
                    };
                    individual_term
                };
                accumulated_contribution =
                    accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
            }
            {
                let contribution = {
                    let individual_term = {
                        let acc_value = unsafe { *(stage_2.get_unchecked(12usize)) };
                        let mut denom = lookup_argument_gamma;
                        denom = denom.add(cs, &a);
                        denom = denom.add(cs, &b);
                        denom = denom.mul(cs, &lookup_argument_gamma);
                        denom = denom.add(cs, &c);
                        denom = denom.mul(cs, &acc_value);
                        let mut numerator = lookup_argument_two_gamma;
                        numerator = numerator.add(cs, &a);
                        numerator = numerator.add(cs, &b);
                        let mut individual_term = denom;
                        individual_term = individual_term.sub(cs, &numerator);
                        individual_term
                    };
                    individual_term
                };
                accumulated_contribution =
                    accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
            }
        }
        {
            let a = {
                let value = unsafe { *(witness.get_unchecked(11usize)) };
                value
            };
            let b = {
                let value = unsafe { *(witness.get_unchecked(12usize)) };
                value
            };
            let c = unsafe { *(stage_2.get_unchecked(4usize)) };
            {
                let contribution = {
                    let individual_term = {
                        let mut individual_term = a;
                        individual_term = individual_term.mul(cs, &b);
                        individual_term = individual_term.sub(cs, &c);
                        individual_term
                    };
                    individual_term
                };
                accumulated_contribution =
                    accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
            }
            {
                let contribution = {
                    let individual_term = {
                        let acc_value = unsafe { *(stage_2.get_unchecked(13usize)) };
                        let mut denom = lookup_argument_gamma;
                        denom = denom.add(cs, &a);
                        denom = denom.add(cs, &b);
                        denom = denom.mul(cs, &lookup_argument_gamma);
                        denom = denom.add(cs, &c);
                        denom = denom.mul(cs, &acc_value);
                        let mut numerator = lookup_argument_two_gamma;
                        numerator = numerator.add(cs, &a);
                        numerator = numerator.add(cs, &b);
                        let mut individual_term = denom;
                        individual_term = individual_term.sub(cs, &numerator);
                        individual_term
                    };
                    individual_term
                };
                accumulated_contribution =
                    accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
            }
        }
        {
            let contribution = {
                let individual_term = {
                    let a = unsafe { *(memory.get_unchecked(0usize)) };
                    let b = unsafe { *(memory.get_unchecked(1usize)) };
                    let c = unsafe { *(stage_2.get_unchecked(5usize)) };
                    let mut individual_term = a;
                    individual_term = individual_term.mul(cs, &b);
                    individual_term = individual_term.sub(cs, &c);
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let a = unsafe { *(memory.get_unchecked(0usize)) };
                    let b = unsafe { *(memory.get_unchecked(1usize)) };
                    let c = unsafe { *(stage_2.get_unchecked(5usize)) };
                    let acc_value = unsafe { *(stage_2.get_unchecked(14usize)) };
                    let mut denom = lookup_argument_gamma;
                    denom = denom.add(cs, &a);
                    denom = denom.add(cs, &b);
                    denom = denom.mul(cs, &lookup_argument_gamma);
                    denom = denom.add(cs, &c);
                    denom = denom.mul(cs, &acc_value);
                    let mut numerator = lookup_argument_two_gamma;
                    numerator = numerator.add(cs, &a);
                    numerator = numerator.add(cs, &b);
                    let mut individual_term = denom;
                    individual_term = individual_term.sub(cs, &numerator);
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let a = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(45usize)) };
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(524288u32));
                        a = a.mul_by_base(cs, &coeff);
                        a
                    };
                    {
                        let a = unsafe { *(memory.get_unchecked(6usize)) };
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let a = unsafe { *(setup.get_unchecked(0usize)) };
                        individual_term = individual_term.sub(cs, &a);
                    }
                    individual_term
                };
                individual_term
            };
            let mut b = {
                let individual_term = {
                    let mut individual_term = {
                        let a = unsafe { *(memory.get_unchecked(7usize)) };
                        a
                    };
                    {
                        let a = unsafe { *(setup.get_unchecked(1usize)) };
                        individual_term = individual_term.sub(cs, &a);
                    }
                    {
                        let a = unsafe { *(witness.get_unchecked(45usize)) };
                        individual_term = individual_term.sub(cs, &a);
                    }
                    let constant_term =
                        MersenneField::allocate_constant(cs, Mersenne31Field(524288u32));
                    individual_term = individual_term.add_base(cs, &constant_term);
                    individual_term
                };
                individual_term
            };
            b = b.sub_base(cs, &memory_timestamp_high_from_sequence_idx);
            let c = unsafe { *(stage_2.get_unchecked(6usize)) };
            {
                let contribution = {
                    let individual_term = {
                        let mut individual_term = a;
                        individual_term = individual_term.mul(cs, &b);
                        individual_term = individual_term.sub(cs, &c);
                        individual_term
                    };
                    individual_term
                };
                accumulated_contribution =
                    accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
            }
            {
                let contribution = {
                    let individual_term = {
                        let acc_value = unsafe { *(stage_2.get_unchecked(15usize)) };
                        let mut denom = lookup_argument_gamma;
                        denom = denom.add(cs, &a);
                        denom = denom.add(cs, &b);
                        denom = denom.mul(cs, &lookup_argument_gamma);
                        denom = denom.add(cs, &c);
                        denom = denom.mul(cs, &acc_value);
                        let mut numerator = lookup_argument_two_gamma;
                        numerator = numerator.add(cs, &a);
                        numerator = numerator.add(cs, &b);
                        let mut individual_term = denom;
                        individual_term = individual_term.sub(cs, &numerator);
                        individual_term
                    };
                    individual_term
                };
                accumulated_contribution =
                    accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
            }
        }
        {
            let a = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(46usize)) };
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(524288u32));
                        a = a.mul_by_base(cs, &coeff);
                        a
                    };
                    {
                        let a = unsafe { *(memory.get_unchecked(11usize)) };
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let a = unsafe { *(setup.get_unchecked(0usize)) };
                        individual_term = individual_term.sub(cs, &a);
                    }
                    let constant_term =
                        MersenneField::allocate_constant(cs, Mersenne31Field(2147483646u32));
                    individual_term = individual_term.add_base(cs, &constant_term);
                    individual_term
                };
                individual_term
            };
            let mut b = {
                let individual_term = {
                    let mut individual_term = {
                        let a = unsafe { *(memory.get_unchecked(12usize)) };
                        a
                    };
                    {
                        let a = unsafe { *(setup.get_unchecked(1usize)) };
                        individual_term = individual_term.sub(cs, &a);
                    }
                    {
                        let a = unsafe { *(witness.get_unchecked(46usize)) };
                        individual_term = individual_term.sub(cs, &a);
                    }
                    let constant_term =
                        MersenneField::allocate_constant(cs, Mersenne31Field(524288u32));
                    individual_term = individual_term.add_base(cs, &constant_term);
                    individual_term
                };
                individual_term
            };
            b = b.sub_base(cs, &memory_timestamp_high_from_sequence_idx);
            let c = unsafe { *(stage_2.get_unchecked(7usize)) };
            {
                let contribution = {
                    let individual_term = {
                        let mut individual_term = a;
                        individual_term = individual_term.mul(cs, &b);
                        individual_term = individual_term.sub(cs, &c);
                        individual_term
                    };
                    individual_term
                };
                accumulated_contribution =
                    accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
            }
            {
                let contribution = {
                    let individual_term = {
                        let acc_value = unsafe { *(stage_2.get_unchecked(16usize)) };
                        let mut denom = lookup_argument_gamma;
                        denom = denom.add(cs, &a);
                        denom = denom.add(cs, &b);
                        denom = denom.mul(cs, &lookup_argument_gamma);
                        denom = denom.add(cs, &c);
                        denom = denom.mul(cs, &acc_value);
                        let mut numerator = lookup_argument_two_gamma;
                        numerator = numerator.add(cs, &a);
                        numerator = numerator.add(cs, &b);
                        let mut individual_term = denom;
                        individual_term = individual_term.sub(cs, &numerator);
                        individual_term
                    };
                    individual_term
                };
                accumulated_contribution =
                    accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
            }
        }
        {
            let a = {
                let individual_term = {
                    let mut individual_term = {
                        let mut a = unsafe { *(witness.get_unchecked(47usize)) };
                        let coeff =
                            MersenneField::allocate_constant(cs, Mersenne31Field(524288u32));
                        a = a.mul_by_base(cs, &coeff);
                        a
                    };
                    {
                        let a = unsafe { *(memory.get_unchecked(18usize)) };
                        individual_term = individual_term.add(cs, &a);
                    }
                    {
                        let a = unsafe { *(setup.get_unchecked(0usize)) };
                        individual_term = individual_term.sub(cs, &a);
                    }
                    let constant_term =
                        MersenneField::allocate_constant(cs, Mersenne31Field(2147483645u32));
                    individual_term = individual_term.add_base(cs, &constant_term);
                    individual_term
                };
                individual_term
            };
            let mut b = {
                let individual_term = {
                    let mut individual_term = {
                        let a = unsafe { *(memory.get_unchecked(19usize)) };
                        a
                    };
                    {
                        let a = unsafe { *(setup.get_unchecked(1usize)) };
                        individual_term = individual_term.sub(cs, &a);
                    }
                    {
                        let a = unsafe { *(witness.get_unchecked(47usize)) };
                        individual_term = individual_term.sub(cs, &a);
                    }
                    let constant_term =
                        MersenneField::allocate_constant(cs, Mersenne31Field(524288u32));
                    individual_term = individual_term.add_base(cs, &constant_term);
                    individual_term
                };
                individual_term
            };
            b = b.sub_base(cs, &memory_timestamp_high_from_sequence_idx);
            let c = unsafe { *(stage_2.get_unchecked(8usize)) };
            {
                let contribution = {
                    let individual_term = {
                        let mut individual_term = a;
                        individual_term = individual_term.mul(cs, &b);
                        individual_term = individual_term.sub(cs, &c);
                        individual_term
                    };
                    individual_term
                };
                accumulated_contribution =
                    accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
            }
            {
                let contribution = {
                    let individual_term = {
                        let acc_value = unsafe { *(stage_2.get_unchecked(17usize)) };
                        let mut denom = lookup_argument_gamma;
                        denom = denom.add(cs, &a);
                        denom = denom.add(cs, &b);
                        denom = denom.mul(cs, &lookup_argument_gamma);
                        denom = denom.add(cs, &c);
                        denom = denom.mul(cs, &acc_value);
                        let mut numerator = lookup_argument_two_gamma;
                        numerator = numerator.add(cs, &a);
                        numerator = numerator.add(cs, &b);
                        let mut individual_term = denom;
                        individual_term = individual_term.sub(cs, &numerator);
                        individual_term
                    };
                    individual_term
                };
                accumulated_contribution =
                    accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
            }
        }
        {
            let contribution = {
                let individual_term = {
                    let src0 = {
                        let value = unsafe { *(witness.get_unchecked(48usize)) };
                        value
                    };
                    let src1 = {
                        let value = unsafe { *(witness.get_unchecked(49usize)) };
                        value
                    };
                    let src2 = {
                        let value = unsafe { *(witness.get_unchecked(50usize)) };
                        value
                    };
                    let mut denom = lookup_argument_linearization_challenges[2];
                    let table_id = MersenneField::allocate_constant(cs, Mersenne31Field(23u32));
                    denom = denom.mul_by_base(cs, &table_id);
                    let mut t = lookup_argument_linearization_challenges[1];
                    t = t.mul_by_base(cs, &src2);
                    denom = denom.add(cs, &t);
                    let mut t = lookup_argument_linearization_challenges[0];
                    t = t.mul_by_base(cs, &src1);
                    denom = denom.add(cs, &t);
                    denom = denom.add(cs, &src0);
                    denom = denom.add(cs, &lookup_argument_gamma);
                    let one = MersenneField::one(cs);
                    let mut individual_term = denom;
                    individual_term =
                        individual_term.mul(cs, &unsafe { *(stage_2.get_unchecked(18usize)) });
                    individual_term = individual_term.sub_base(cs, &one);
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let src0 = {
                        let individual_term = {
                            let mut individual_term = {
                                let a = unsafe { *(witness.get_unchecked(3usize)) };
                                a
                            };
                            {
                                let mut a = unsafe { *(witness.get_unchecked(50usize)) };
                                let coeff =
                                    MersenneField::allocate_constant(cs, Mersenne31Field(65536u32));
                                individual_term =
                                    a.mul_by_base_and_add(cs, &coeff, &individual_term);
                            }
                            individual_term
                        };
                        individual_term
                    };
                    let src1 = {
                        let value = unsafe { *(witness.get_unchecked(51usize)) };
                        value
                    };
                    let src2 = {
                        let value = unsafe { *(witness.get_unchecked(52usize)) };
                        value
                    };
                    let mut denom = lookup_argument_linearization_challenges[2];
                    let table_id = MersenneField::allocate_constant(cs, Mersenne31Field(24u32));
                    denom = denom.mul_by_base(cs, &table_id);
                    let mut t = lookup_argument_linearization_challenges[1];
                    t = t.mul_by_base(cs, &src2);
                    denom = denom.add(cs, &t);
                    let mut t = lookup_argument_linearization_challenges[0];
                    t = t.mul_by_base(cs, &src1);
                    denom = denom.add(cs, &t);
                    denom = denom.add(cs, &src0);
                    denom = denom.add(cs, &lookup_argument_gamma);
                    let one = MersenneField::one(cs);
                    let mut individual_term = denom;
                    individual_term =
                        individual_term.mul(cs, &unsafe { *(stage_2.get_unchecked(19usize)) });
                    individual_term = individual_term.sub_base(cs, &one);
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let src0 = {
                        let value = unsafe { *(witness.get_unchecked(53usize)) };
                        value
                    };
                    let src1 = {
                        let value = unsafe { *(witness.get_unchecked(54usize)) };
                        value
                    };
                    let src2 = {
                        let value = unsafe { *(witness.get_unchecked(55usize)) };
                        value
                    };
                    let mut denom = lookup_argument_linearization_challenges[2];
                    let table_id = MersenneField::allocate_constant(cs, Mersenne31Field(11u32));
                    denom = denom.mul_by_base(cs, &table_id);
                    let mut t = lookup_argument_linearization_challenges[1];
                    t = t.mul_by_base(cs, &src2);
                    denom = denom.add(cs, &t);
                    let mut t = lookup_argument_linearization_challenges[0];
                    t = t.mul_by_base(cs, &src1);
                    denom = denom.add(cs, &t);
                    denom = denom.add(cs, &src0);
                    denom = denom.add(cs, &lookup_argument_gamma);
                    let one = MersenneField::one(cs);
                    let mut individual_term = denom;
                    individual_term =
                        individual_term.mul(cs, &unsafe { *(stage_2.get_unchecked(20usize)) });
                    individual_term = individual_term.sub_base(cs, &one);
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let src0 = {
                        let value = unsafe { *(witness.get_unchecked(56usize)) };
                        value
                    };
                    let src1 = {
                        let value = unsafe { *(witness.get_unchecked(57usize)) };
                        value
                    };
                    let src2 = {
                        let value = unsafe { *(witness.get_unchecked(58usize)) };
                        value
                    };
                    let mut denom = lookup_argument_linearization_challenges[2];
                    let table_id = MersenneField::allocate_constant(cs, Mersenne31Field(12u32));
                    denom = denom.mul_by_base(cs, &table_id);
                    let mut t = lookup_argument_linearization_challenges[1];
                    t = t.mul_by_base(cs, &src2);
                    denom = denom.add(cs, &t);
                    let mut t = lookup_argument_linearization_challenges[0];
                    t = t.mul_by_base(cs, &src1);
                    denom = denom.add(cs, &t);
                    denom = denom.add(cs, &src0);
                    denom = denom.add(cs, &lookup_argument_gamma);
                    let one = MersenneField::one(cs);
                    let mut individual_term = denom;
                    individual_term =
                        individual_term.mul(cs, &unsafe { *(stage_2.get_unchecked(21usize)) });
                    individual_term = individual_term.sub_base(cs, &one);
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let src0 = {
                        let individual_term = {
                            let mut individual_term = {
                                let a = unsafe { *(witness.get_unchecked(56usize)) };
                                a
                            };
                            {
                                let mut a = unsafe { *(witness.get_unchecked(57usize)) };
                                let coeff =
                                    MersenneField::allocate_constant(cs, Mersenne31Field(128u32));
                                individual_term =
                                    a.mul_by_base_and_add(cs, &coeff, &individual_term);
                            }
                            {
                                let mut a = unsafe { *(witness.get_unchecked(58usize)) };
                                let coeff =
                                    MersenneField::allocate_constant(cs, Mersenne31Field(1024u32));
                                individual_term =
                                    a.mul_by_base_and_add(cs, &coeff, &individual_term);
                            }
                            {
                                let mut a = unsafe { *(witness.get_unchecked(14usize)) };
                                let coeff =
                                    MersenneField::allocate_constant(cs, Mersenne31Field(65536u32));
                                individual_term =
                                    a.mul_by_base_and_add(cs, &coeff, &individual_term);
                            }
                            individual_term
                        };
                        individual_term
                    };
                    let src1 = {
                        let individual_term = {
                            let mut individual_term = {
                                let a = unsafe { *(witness.get_unchecked(15usize)) };
                                a
                            };
                            {
                                let mut a = unsafe { *(witness.get_unchecked(16usize)) };
                                let coeff =
                                    MersenneField::allocate_constant(cs, Mersenne31Field(2u32));
                                individual_term =
                                    a.mul_by_base_and_add(cs, &coeff, &individual_term);
                            }
                            {
                                let mut a = unsafe { *(witness.get_unchecked(17usize)) };
                                let coeff =
                                    MersenneField::allocate_constant(cs, Mersenne31Field(4u32));
                                individual_term =
                                    a.mul_by_base_and_add(cs, &coeff, &individual_term);
                            }
                            {
                                let mut a = unsafe { *(witness.get_unchecked(18usize)) };
                                let coeff =
                                    MersenneField::allocate_constant(cs, Mersenne31Field(8u32));
                                individual_term =
                                    a.mul_by_base_and_add(cs, &coeff, &individual_term);
                            }
                            {
                                let mut a = unsafe { *(witness.get_unchecked(19usize)) };
                                let coeff =
                                    MersenneField::allocate_constant(cs, Mersenne31Field(16u32));
                                individual_term =
                                    a.mul_by_base_and_add(cs, &coeff, &individual_term);
                            }
                            {
                                let mut a = unsafe { *(witness.get_unchecked(20usize)) };
                                let coeff =
                                    MersenneField::allocate_constant(cs, Mersenne31Field(32u32));
                                individual_term =
                                    a.mul_by_base_and_add(cs, &coeff, &individual_term);
                            }
                            {
                                let mut a = unsafe { *(witness.get_unchecked(21usize)) };
                                let coeff =
                                    MersenneField::allocate_constant(cs, Mersenne31Field(64u32));
                                individual_term =
                                    a.mul_by_base_and_add(cs, &coeff, &individual_term);
                            }
                            {
                                let mut a = unsafe { *(witness.get_unchecked(22usize)) };
                                let coeff =
                                    MersenneField::allocate_constant(cs, Mersenne31Field(128u32));
                                individual_term =
                                    a.mul_by_base_and_add(cs, &coeff, &individual_term);
                            }
                            {
                                let mut a = unsafe { *(witness.get_unchecked(23usize)) };
                                let coeff =
                                    MersenneField::allocate_constant(cs, Mersenne31Field(256u32));
                                individual_term =
                                    a.mul_by_base_and_add(cs, &coeff, &individual_term);
                            }
                            {
                                let mut a = unsafe { *(witness.get_unchecked(24usize)) };
                                let coeff =
                                    MersenneField::allocate_constant(cs, Mersenne31Field(512u32));
                                individual_term =
                                    a.mul_by_base_and_add(cs, &coeff, &individual_term);
                            }
                            {
                                let mut a = unsafe { *(witness.get_unchecked(25usize)) };
                                let coeff =
                                    MersenneField::allocate_constant(cs, Mersenne31Field(1024u32));
                                individual_term =
                                    a.mul_by_base_and_add(cs, &coeff, &individual_term);
                            }
                            {
                                let mut a = unsafe { *(witness.get_unchecked(26usize)) };
                                let coeff =
                                    MersenneField::allocate_constant(cs, Mersenne31Field(2048u32));
                                individual_term =
                                    a.mul_by_base_and_add(cs, &coeff, &individual_term);
                            }
                            {
                                let mut a = unsafe { *(witness.get_unchecked(27usize)) };
                                let coeff =
                                    MersenneField::allocate_constant(cs, Mersenne31Field(4096u32));
                                individual_term =
                                    a.mul_by_base_and_add(cs, &coeff, &individual_term);
                            }
                            {
                                let mut a = unsafe { *(witness.get_unchecked(28usize)) };
                                let coeff =
                                    MersenneField::allocate_constant(cs, Mersenne31Field(8192u32));
                                individual_term =
                                    a.mul_by_base_and_add(cs, &coeff, &individual_term);
                            }
                            {
                                let mut a = unsafe { *(witness.get_unchecked(29usize)) };
                                let coeff =
                                    MersenneField::allocate_constant(cs, Mersenne31Field(16384u32));
                                individual_term =
                                    a.mul_by_base_and_add(cs, &coeff, &individual_term);
                            }
                            {
                                let mut a = unsafe { *(witness.get_unchecked(30usize)) };
                                let coeff =
                                    MersenneField::allocate_constant(cs, Mersenne31Field(32768u32));
                                individual_term =
                                    a.mul_by_base_and_add(cs, &coeff, &individual_term);
                            }
                            {
                                let mut a = unsafe { *(witness.get_unchecked(31usize)) };
                                let coeff =
                                    MersenneField::allocate_constant(cs, Mersenne31Field(65536u32));
                                individual_term =
                                    a.mul_by_base_and_add(cs, &coeff, &individual_term);
                            }
                            {
                                let mut a = unsafe { *(witness.get_unchecked(32usize)) };
                                let coeff = MersenneField::allocate_constant(
                                    cs,
                                    Mersenne31Field(131072u32),
                                );
                                individual_term =
                                    a.mul_by_base_and_add(cs, &coeff, &individual_term);
                            }
                            {
                                let mut a = unsafe { *(witness.get_unchecked(33usize)) };
                                let coeff = MersenneField::allocate_constant(
                                    cs,
                                    Mersenne31Field(262144u32),
                                );
                                individual_term =
                                    a.mul_by_base_and_add(cs, &coeff, &individual_term);
                            }
                            {
                                let mut a = unsafe { *(witness.get_unchecked(34usize)) };
                                let coeff = MersenneField::allocate_constant(
                                    cs,
                                    Mersenne31Field(524288u32),
                                );
                                individual_term =
                                    a.mul_by_base_and_add(cs, &coeff, &individual_term);
                            }
                            {
                                let mut a = unsafe { *(witness.get_unchecked(35usize)) };
                                let coeff = MersenneField::allocate_constant(
                                    cs,
                                    Mersenne31Field(1048576u32),
                                );
                                individual_term =
                                    a.mul_by_base_and_add(cs, &coeff, &individual_term);
                            }
                            {
                                let mut a = unsafe { *(witness.get_unchecked(36usize)) };
                                let coeff = MersenneField::allocate_constant(
                                    cs,
                                    Mersenne31Field(2097152u32),
                                );
                                individual_term =
                                    a.mul_by_base_and_add(cs, &coeff, &individual_term);
                            }
                            individual_term
                        };
                        individual_term
                    };
                    let src2 = {
                        let individual_term =
                            MersenneField::allocate_constant(cs, Mersenne31Field(0u32));
                        individual_term
                    };
                    let mut denom = lookup_argument_linearization_challenges[2];
                    let table_id = MersenneField::allocate_constant(cs, Mersenne31Field(1u32));
                    denom = denom.mul_by_base(cs, &table_id);
                    let mut t = lookup_argument_linearization_challenges[1];
                    t = t.mul_by_base(cs, &src2);
                    denom = denom.add(cs, &t);
                    let mut t = lookup_argument_linearization_challenges[0];
                    t = t.mul_by_base(cs, &src1);
                    denom = denom.add(cs, &t);
                    denom = denom.add(cs, &src0);
                    denom = denom.add(cs, &lookup_argument_gamma);
                    let one = MersenneField::one(cs);
                    let mut individual_term = denom;
                    individual_term =
                        individual_term.mul(cs, &unsafe { *(stage_2.get_unchecked(22usize)) });
                    individual_term = individual_term.sub_base(cs, &one);
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let src0 = {
                        let value = unsafe { *(memory.get_unchecked(9usize)) };
                        value
                    };
                    let src1 = {
                        let value = unsafe { *(witness.get_unchecked(59usize)) };
                        value
                    };
                    let src2 = {
                        let value = unsafe { *(witness.get_unchecked(60usize)) };
                        value
                    };
                    let mut denom = lookup_argument_linearization_challenges[2];
                    let table_id = MersenneField::allocate_constant(cs, Mersenne31Field(16u32));
                    denom = denom.mul_by_base(cs, &table_id);
                    let mut t = lookup_argument_linearization_challenges[1];
                    t = t.mul_by_base(cs, &src2);
                    denom = denom.add(cs, &t);
                    let mut t = lookup_argument_linearization_challenges[0];
                    t = t.mul_by_base(cs, &src1);
                    denom = denom.add(cs, &t);
                    denom = denom.add(cs, &src0);
                    denom = denom.add(cs, &lookup_argument_gamma);
                    let one = MersenneField::one(cs);
                    let mut individual_term = denom;
                    individual_term =
                        individual_term.mul(cs, &unsafe { *(stage_2.get_unchecked(23usize)) });
                    individual_term = individual_term.sub_base(cs, &one);
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let src0 = {
                        let value = unsafe { *(witness.get_unchecked(61usize)) };
                        value
                    };
                    let src1 = {
                        let value = unsafe { *(witness.get_unchecked(62usize)) };
                        value
                    };
                    let src2 = {
                        let value = unsafe { *(witness.get_unchecked(63usize)) };
                        value
                    };
                    let mut denom = lookup_argument_linearization_challenges[2];
                    let table_id = MersenneField::allocate_constant(cs, Mersenne31Field(16u32));
                    denom = denom.mul_by_base(cs, &table_id);
                    let mut t = lookup_argument_linearization_challenges[1];
                    t = t.mul_by_base(cs, &src2);
                    denom = denom.add(cs, &t);
                    let mut t = lookup_argument_linearization_challenges[0];
                    t = t.mul_by_base(cs, &src1);
                    denom = denom.add(cs, &t);
                    denom = denom.add(cs, &src0);
                    denom = denom.add(cs, &lookup_argument_gamma);
                    let one = MersenneField::one(cs);
                    let mut individual_term = denom;
                    individual_term =
                        individual_term.mul(cs, &unsafe { *(stage_2.get_unchecked(24usize)) });
                    individual_term = individual_term.sub_base(cs, &one);
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let src0 = {
                        let value = unsafe { *(witness.get_unchecked(64usize)) };
                        value
                    };
                    let src1 = {
                        let value = unsafe { *(witness.get_unchecked(65usize)) };
                        value
                    };
                    let src2 = {
                        let value = unsafe { *(witness.get_unchecked(66usize)) };
                        value
                    };
                    let table_id = unsafe { *(witness.get_unchecked(67usize)) };
                    let mut denom = lookup_argument_linearization_challenges[2];
                    denom = denom.mul(cs, &table_id);
                    let mut t = lookup_argument_linearization_challenges[1];
                    t = t.mul_by_base(cs, &src2);
                    denom = denom.add(cs, &t);
                    let mut t = lookup_argument_linearization_challenges[0];
                    t = t.mul_by_base(cs, &src1);
                    denom = denom.add(cs, &t);
                    denom = denom.add(cs, &src0);
                    denom = denom.add(cs, &lookup_argument_gamma);
                    let one = MersenneField::one(cs);
                    let mut individual_term = denom;
                    individual_term =
                        individual_term.mul(cs, &unsafe { *(stage_2.get_unchecked(25usize)) });
                    individual_term = individual_term.sub_base(cs, &one);
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let src0 = {
                        let value = unsafe { *(witness.get_unchecked(68usize)) };
                        value
                    };
                    let src1 = {
                        let value = unsafe { *(witness.get_unchecked(69usize)) };
                        value
                    };
                    let src2 = {
                        let value = unsafe { *(witness.get_unchecked(70usize)) };
                        value
                    };
                    let table_id = unsafe { *(witness.get_unchecked(71usize)) };
                    let mut denom = lookup_argument_linearization_challenges[2];
                    denom = denom.mul(cs, &table_id);
                    let mut t = lookup_argument_linearization_challenges[1];
                    t = t.mul_by_base(cs, &src2);
                    denom = denom.add(cs, &t);
                    let mut t = lookup_argument_linearization_challenges[0];
                    t = t.mul_by_base(cs, &src1);
                    denom = denom.add(cs, &t);
                    denom = denom.add(cs, &src0);
                    denom = denom.add(cs, &lookup_argument_gamma);
                    let one = MersenneField::one(cs);
                    let mut individual_term = denom;
                    individual_term =
                        individual_term.mul(cs, &unsafe { *(stage_2.get_unchecked(26usize)) });
                    individual_term = individual_term.sub_base(cs, &one);
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let src0 = {
                        let value = unsafe { *(witness.get_unchecked(72usize)) };
                        value
                    };
                    let src1 = {
                        let value = unsafe { *(witness.get_unchecked(73usize)) };
                        value
                    };
                    let src2 = {
                        let value = unsafe { *(witness.get_unchecked(74usize)) };
                        value
                    };
                    let table_id = unsafe { *(witness.get_unchecked(75usize)) };
                    let mut denom = lookup_argument_linearization_challenges[2];
                    denom = denom.mul(cs, &table_id);
                    let mut t = lookup_argument_linearization_challenges[1];
                    t = t.mul_by_base(cs, &src2);
                    denom = denom.add(cs, &t);
                    let mut t = lookup_argument_linearization_challenges[0];
                    t = t.mul_by_base(cs, &src1);
                    denom = denom.add(cs, &t);
                    denom = denom.add(cs, &src0);
                    denom = denom.add(cs, &lookup_argument_gamma);
                    let one = MersenneField::one(cs);
                    let mut individual_term = denom;
                    individual_term =
                        individual_term.mul(cs, &unsafe { *(stage_2.get_unchecked(27usize)) });
                    individual_term = individual_term.sub_base(cs, &one);
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let src0 = {
                        let value = unsafe { *(witness.get_unchecked(76usize)) };
                        value
                    };
                    let src1 = {
                        let value = unsafe { *(witness.get_unchecked(77usize)) };
                        value
                    };
                    let src2 = {
                        let value = unsafe { *(witness.get_unchecked(78usize)) };
                        value
                    };
                    let table_id = unsafe { *(witness.get_unchecked(79usize)) };
                    let mut denom = lookup_argument_linearization_challenges[2];
                    denom = denom.mul(cs, &table_id);
                    let mut t = lookup_argument_linearization_challenges[1];
                    t = t.mul_by_base(cs, &src2);
                    denom = denom.add(cs, &t);
                    let mut t = lookup_argument_linearization_challenges[0];
                    t = t.mul_by_base(cs, &src1);
                    denom = denom.add(cs, &t);
                    denom = denom.add(cs, &src0);
                    denom = denom.add(cs, &lookup_argument_gamma);
                    let one = MersenneField::one(cs);
                    let mut individual_term = denom;
                    individual_term =
                        individual_term.mul(cs, &unsafe { *(stage_2.get_unchecked(28usize)) });
                    individual_term = individual_term.sub_base(cs, &one);
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let m = unsafe { *(witness.get_unchecked(0usize)) };
                    let t = unsafe { *(setup.get_unchecked(2usize)) };
                    let mut denom = lookup_argument_gamma;
                    denom = denom.add(cs, &t);
                    let mut individual_term = denom;
                    individual_term =
                        individual_term.mul(cs, &unsafe { *(stage_2.get_unchecked(29usize)) });
                    individual_term = individual_term.sub(cs, &m);
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let m = unsafe { *(witness.get_unchecked(1usize)) };
                    let t = unsafe { *(setup.get_unchecked(3usize)) };
                    let mut denom = lookup_argument_gamma;
                    denom = denom.add(cs, &t);
                    let mut individual_term = denom;
                    individual_term =
                        individual_term.mul(cs, &unsafe { *(stage_2.get_unchecked(30usize)) });
                    individual_term = individual_term.sub(cs, &m);
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let m = unsafe { *(witness.get_unchecked(2usize)) };
                    let mut denom = lookup_argument_linearization_challenges[2];
                    let table_id = unsafe { *(setup.get_unchecked(7usize)) };
                    denom = denom.mul(cs, &table_id);
                    let mut t = lookup_argument_linearization_challenges[1];
                    t = t.mul(cs, &unsafe { *(setup.get_unchecked(6usize)) });
                    denom = denom.add(cs, &t);
                    let mut t = lookup_argument_linearization_challenges[0];
                    t = t.mul(cs, &unsafe { *(setup.get_unchecked(5usize)) });
                    denom = denom.add(cs, &t);
                    let t = unsafe { *(setup.get_unchecked(4usize)) };
                    denom = denom.add(cs, &t);
                    denom = denom.add(cs, &lookup_argument_gamma);
                    let mut individual_term = denom;
                    individual_term =
                        individual_term.mul(cs, &unsafe { *(stage_2.get_unchecked(31usize)) });
                    individual_term = individual_term.sub(cs, &m);
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let m = unsafe { *(memory.get_unchecked(27usize)) };
                    let mut denom = delegation_argument_linearization_challenges[2];
                    let mut timestamp_high = unsafe { *(setup.get_unchecked(1usize)) };
                    timestamp_high =
                        timestamp_high.add_base(cs, &memory_timestamp_high_from_sequence_idx);
                    denom = denom.mul(cs, &timestamp_high);
                    let mut timestamp_low = unsafe { *(setup.get_unchecked(0usize)) };
                    let in_cycle_timestamp =
                        MersenneField::allocate_constant(cs, Mersenne31Field(3u32));
                    timestamp_low = timestamp_low.add_base(cs, &in_cycle_timestamp);
                    let mut t = delegation_argument_linearization_challenges[1];
                    t = t.mul(cs, &timestamp_low);
                    denom = denom.add(cs, &t);
                    let mem_abi_offset = unsafe { *(memory.get_unchecked(29usize)) };
                    let mut t = delegation_argument_linearization_challenges[0];
                    t = t.mul(cs, &mem_abi_offset);
                    denom = denom.add(cs, &t);
                    let t = unsafe { *(memory.get_unchecked(28usize)) };
                    denom = denom.add(cs, &t);
                    denom = denom.add(cs, &delegation_argument_gamma);
                    let mut individual_term = denom;
                    individual_term =
                        individual_term.mul(cs, &unsafe { *(stage_2.get_unchecked(32usize)) });
                    individual_term = individual_term.sub(cs, &m);
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let final_borrow_value = unsafe { *(witness.get_unchecked(44usize)) };
            let one = MersenneField::one(cs);
            let mut final_borrow_minus_one = final_borrow_value;
            final_borrow_minus_one = final_borrow_minus_one.sub_base(cs, &one);
            {
                let contribution = {
                    let individual_term = {
                        let value_to_constraint = unsafe { *(memory.get_unchecked(0usize)) };
                        let mut individual_term = final_borrow_minus_one;
                        individual_term = individual_term.mul(cs, &value_to_constraint);
                        individual_term
                    };
                    individual_term
                };
                accumulated_contribution =
                    accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
            }
            {
                let contribution = {
                    let individual_term = {
                        let value_to_constraint = unsafe { *(memory.get_unchecked(1usize)) };
                        let mut individual_term = final_borrow_minus_one;
                        individual_term = individual_term.mul(cs, &value_to_constraint);
                        individual_term
                    };
                    individual_term
                };
                accumulated_contribution =
                    accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
            }
            {
                let contribution = {
                    let individual_term = {
                        let value_to_constraint = unsafe { *(memory.get_unchecked(2usize)) };
                        let mut individual_term = final_borrow_minus_one;
                        individual_term = individual_term.mul(cs, &value_to_constraint);
                        individual_term
                    };
                    individual_term
                };
                accumulated_contribution =
                    accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
            }
            {
                let contribution = {
                    let individual_term = {
                        let value_to_constraint = unsafe { *(memory.get_unchecked(3usize)) };
                        let mut individual_term = final_borrow_minus_one;
                        individual_term = individual_term.mul(cs, &value_to_constraint);
                        individual_term
                    };
                    individual_term
                };
                accumulated_contribution =
                    accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
            }
            {
                let contribution = {
                    let individual_term = {
                        let value_to_constraint = unsafe { *(memory.get_unchecked(4usize)) };
                        let mut individual_term = final_borrow_minus_one;
                        individual_term = individual_term.mul(cs, &value_to_constraint);
                        individual_term
                    };
                    individual_term
                };
                accumulated_contribution =
                    accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
            }
            {
                let contribution = {
                    let individual_term = {
                        let value_to_constraint = unsafe { *(memory.get_unchecked(5usize)) };
                        let mut individual_term = final_borrow_minus_one;
                        individual_term = individual_term.mul(cs, &value_to_constraint);
                        individual_term
                    };
                    individual_term
                };
                accumulated_contribution =
                    accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
            }
        }
        {
            let contribution = {
                let individual_term = {
                    let address_low = unsafe { *(memory.get_unchecked(0usize)) };
                    let mut t = memory_argument_linearization_challenges[0usize];
                    t = t.mul(cs, &address_low);
                    let mut numerator = t;
                    let address_high = unsafe { *(memory.get_unchecked(1usize)) };
                    let mut t = memory_argument_linearization_challenges[1usize];
                    t = t.mul(cs, &address_high);
                    numerator = numerator.add(cs, &t);
                    numerator = numerator.add(cs, &memory_argument_gamma);
                    let mut denom = numerator;
                    let value_low = unsafe { *(memory.get_unchecked(2usize)) };
                    let mut t = memory_argument_linearization_challenges[4usize];
                    t = t.mul(cs, &value_low);
                    denom = denom.add(cs, &t);
                    let value_high = unsafe { *(memory.get_unchecked(3usize)) };
                    let mut t = memory_argument_linearization_challenges[5usize];
                    t = t.mul(cs, &value_high);
                    denom = denom.add(cs, &t);
                    let timestamp_low = unsafe { *(memory.get_unchecked(4usize)) };
                    let mut t = memory_argument_linearization_challenges[2usize];
                    t = t.mul(cs, &timestamp_low);
                    denom = denom.add(cs, &t);
                    let timestamp_high = unsafe { *(memory.get_unchecked(5usize)) };
                    let mut t = memory_argument_linearization_challenges[3usize];
                    t = t.mul(cs, &timestamp_high);
                    denom = denom.add(cs, &t);
                    let accumulator = unsafe { *(stage_2.get_unchecked(33usize)) };
                    let mut individual_term = accumulator;
                    individual_term = individual_term.mul(cs, &denom);
                    individual_term = individual_term.sub(cs, &numerator);
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let address_contribution = {
                        let address_low = unsafe { *(memory.get_unchecked(10usize)) };
                        let mut address_contribution =
                            memory_argument_linearization_challenges[0usize];
                        address_contribution = address_contribution.mul(cs, &address_low);
                        let one = MersenneField::one(cs);
                        address_contribution = address_contribution.add_base(cs, &one);
                        address_contribution
                    };
                    let value_low = unsafe { *(memory.get_unchecked(8usize)) };
                    let mut value_contibution = memory_argument_linearization_challenges[4usize];
                    value_contibution = value_contibution.mul(cs, &value_low);
                    let value_high = unsafe { *(memory.get_unchecked(9usize)) };
                    let mut t = memory_argument_linearization_challenges[5usize];
                    t = t.mul(cs, &value_high);
                    value_contibution = value_contibution.add(cs, &t);
                    let mut numerator = memory_argument_gamma;
                    numerator = numerator.add(cs, &address_contribution);
                    numerator = numerator.add(cs, &value_contibution);
                    let mut denom = numerator;
                    let read_timestamp_low = unsafe { *(memory.get_unchecked(6usize)) };
                    let mut read_timestamp_contibution =
                        memory_argument_linearization_challenges[2usize];
                    read_timestamp_contibution =
                        read_timestamp_contibution.mul(cs, &read_timestamp_low);
                    let read_timestamp_high = unsafe { *(memory.get_unchecked(7usize)) };
                    let mut t = memory_argument_linearization_challenges[3usize];
                    t = t.mul(cs, &read_timestamp_high);
                    read_timestamp_contibution = read_timestamp_contibution.add(cs, &t);
                    let access_idx = MersenneField::allocate_constant(cs, Mersenne31Field(0u32));
                    let mut write_timestamp_low = unsafe { *(setup.get_unchecked(0usize)) };
                    write_timestamp_low = write_timestamp_low.add_base(cs, &access_idx);
                    let mut write_timestamp_contibution =
                        memory_argument_linearization_challenges[2usize];
                    write_timestamp_contibution =
                        write_timestamp_contibution.mul(cs, &write_timestamp_low);
                    let mut write_timestamp_high = unsafe { *(setup.get_unchecked(1usize)) };
                    write_timestamp_high =
                        write_timestamp_high.add_base(cs, &memory_timestamp_high_from_sequence_idx);
                    let mut t = memory_argument_linearization_challenges[3usize];
                    t = t.mul(cs, &write_timestamp_high);
                    write_timestamp_contibution = write_timestamp_contibution.add(cs, &t);
                    numerator = numerator.add(cs, &write_timestamp_contibution);
                    denom = denom.add(cs, &read_timestamp_contibution);
                    let accumulator = unsafe { *(stage_2.get_unchecked(34usize)) };
                    let previous = unsafe { *(stage_2.get_unchecked(33usize)) };
                    let mut individual_term = accumulator;
                    individual_term = individual_term.mul(cs, &denom);
                    let mut t = previous;
                    t = t.mul(cs, &numerator);
                    individual_term = individual_term.sub(cs, &t);
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let address_contribution = {
                        let address_low = unsafe { *(memory.get_unchecked(16usize)) };
                        let mut address_contribution =
                            memory_argument_linearization_challenges[0usize];
                        address_contribution = address_contribution.mul(cs, &address_low);
                        let address_high = unsafe { *(memory.get_unchecked(17usize)) };
                        let mut t = memory_argument_linearization_challenges[1usize];
                        t = t.mul(cs, &address_high);
                        address_contribution = address_contribution.add(cs, &t);
                        let is_register = unsafe { *(memory.get_unchecked(15usize)) };
                        address_contribution = address_contribution.add(cs, &is_register);
                        address_contribution
                    };
                    let value_low = unsafe { *(memory.get_unchecked(13usize)) };
                    let mut value_contibution = memory_argument_linearization_challenges[4usize];
                    value_contibution = value_contibution.mul(cs, &value_low);
                    let value_high = unsafe { *(memory.get_unchecked(14usize)) };
                    let mut t = memory_argument_linearization_challenges[5usize];
                    t = t.mul(cs, &value_high);
                    value_contibution = value_contibution.add(cs, &t);
                    let mut numerator = memory_argument_gamma;
                    numerator = numerator.add(cs, &address_contribution);
                    numerator = numerator.add(cs, &value_contibution);
                    let mut denom = numerator;
                    let read_timestamp_low = unsafe { *(memory.get_unchecked(11usize)) };
                    let mut read_timestamp_contibution =
                        memory_argument_linearization_challenges[2usize];
                    read_timestamp_contibution =
                        read_timestamp_contibution.mul(cs, &read_timestamp_low);
                    let read_timestamp_high = unsafe { *(memory.get_unchecked(12usize)) };
                    let mut t = memory_argument_linearization_challenges[3usize];
                    t = t.mul(cs, &read_timestamp_high);
                    read_timestamp_contibution = read_timestamp_contibution.add(cs, &t);
                    let access_idx = MersenneField::allocate_constant(cs, Mersenne31Field(1u32));
                    let mut write_timestamp_low = unsafe { *(setup.get_unchecked(0usize)) };
                    write_timestamp_low = write_timestamp_low.add_base(cs, &access_idx);
                    let mut write_timestamp_contibution =
                        memory_argument_linearization_challenges[2usize];
                    write_timestamp_contibution =
                        write_timestamp_contibution.mul(cs, &write_timestamp_low);
                    let mut write_timestamp_high = unsafe { *(setup.get_unchecked(1usize)) };
                    write_timestamp_high =
                        write_timestamp_high.add_base(cs, &memory_timestamp_high_from_sequence_idx);
                    let mut t = memory_argument_linearization_challenges[3usize];
                    t = t.mul(cs, &write_timestamp_high);
                    write_timestamp_contibution = write_timestamp_contibution.add(cs, &t);
                    numerator = numerator.add(cs, &write_timestamp_contibution);
                    denom = denom.add(cs, &read_timestamp_contibution);
                    let accumulator = unsafe { *(stage_2.get_unchecked(35usize)) };
                    let previous = unsafe { *(stage_2.get_unchecked(34usize)) };
                    let mut individual_term = accumulator;
                    individual_term = individual_term.mul(cs, &denom);
                    let mut t = previous;
                    t = t.mul(cs, &numerator);
                    individual_term = individual_term.sub(cs, &t);
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let address_contribution = {
                        let address_low = unsafe { *(memory.get_unchecked(23usize)) };
                        let mut address_contribution =
                            memory_argument_linearization_challenges[0usize];
                        address_contribution = address_contribution.mul(cs, &address_low);
                        let address_high = unsafe { *(memory.get_unchecked(24usize)) };
                        let mut t = memory_argument_linearization_challenges[1usize];
                        t = t.mul(cs, &address_high);
                        address_contribution = address_contribution.add(cs, &t);
                        let is_register = unsafe { *(memory.get_unchecked(22usize)) };
                        address_contribution = address_contribution.add(cs, &is_register);
                        address_contribution
                    };
                    let mut numerator = memory_argument_gamma;
                    numerator = numerator.add(cs, &address_contribution);
                    let mut denom = numerator;
                    let read_value_low = unsafe { *(memory.get_unchecked(20usize)) };
                    let mut read_value_contibution =
                        memory_argument_linearization_challenges[4usize];
                    read_value_contibution = read_value_contibution.mul(cs, &read_value_low);
                    let read_value_high = unsafe { *(memory.get_unchecked(21usize)) };
                    let mut t = memory_argument_linearization_challenges[5usize];
                    t = t.mul(cs, &read_value_high);
                    read_value_contibution = read_value_contibution.add(cs, &t);
                    let write_value_low = unsafe { *(memory.get_unchecked(25usize)) };
                    let mut write_value_contibution =
                        memory_argument_linearization_challenges[4usize];
                    write_value_contibution = write_value_contibution.mul(cs, &write_value_low);
                    let write_value_high = unsafe { *(memory.get_unchecked(26usize)) };
                    let mut t = memory_argument_linearization_challenges[5usize];
                    t = t.mul(cs, &write_value_high);
                    write_value_contibution = write_value_contibution.add(cs, &t);
                    numerator = numerator.add(cs, &write_value_contibution);
                    denom = denom.add(cs, &read_value_contibution);
                    let read_timestamp_low = unsafe { *(memory.get_unchecked(18usize)) };
                    let mut read_timestamp_contibution =
                        memory_argument_linearization_challenges[2usize];
                    read_timestamp_contibution =
                        read_timestamp_contibution.mul(cs, &read_timestamp_low);
                    let read_timestamp_high = unsafe { *(memory.get_unchecked(19usize)) };
                    let mut t = memory_argument_linearization_challenges[3usize];
                    t = t.mul(cs, &read_timestamp_high);
                    read_timestamp_contibution = read_timestamp_contibution.add(cs, &t);
                    let access_idx = MersenneField::allocate_constant(cs, Mersenne31Field(2u32));
                    let mut write_timestamp_low = unsafe { *(setup.get_unchecked(0usize)) };
                    write_timestamp_low = write_timestamp_low.add_base(cs, &access_idx);
                    let mut write_timestamp_contibution =
                        memory_argument_linearization_challenges[2usize];
                    write_timestamp_contibution =
                        write_timestamp_contibution.mul(cs, &write_timestamp_low);
                    let mut write_timestamp_high = unsafe { *(setup.get_unchecked(1usize)) };
                    write_timestamp_high =
                        write_timestamp_high.add_base(cs, &memory_timestamp_high_from_sequence_idx);
                    let mut t = memory_argument_linearization_challenges[3usize];
                    t = t.mul(cs, &write_timestamp_high);
                    write_timestamp_contibution = write_timestamp_contibution.add(cs, &t);
                    numerator = numerator.add(cs, &write_timestamp_contibution);
                    denom = denom.add(cs, &read_timestamp_contibution);
                    let accumulator = unsafe { *(stage_2.get_unchecked(36usize)) };
                    let previous = unsafe { *(stage_2.get_unchecked(35usize)) };
                    let mut individual_term = accumulator;
                    individual_term = individual_term.mul(cs, &denom);
                    let mut t = previous;
                    t = t.mul(cs, &numerator);
                    individual_term = individual_term.sub(cs, &t);
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = unsafe { *(stage_2_next_row.get_unchecked(37usize)) };
                    let mut t = unsafe { *(stage_2.get_unchecked(37usize)) };
                    t = t.mul(cs, &unsafe { *(stage_2.get_unchecked(36usize)) });
                    individual_term = individual_term.sub(cs, &t);
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        let divisor = divisors[0usize];
        accumulated_contribution = accumulated_contribution.mul(cs, &divisor);
        accumulated_contribution
    };
    every_row_except_last_contribution
}
#[allow(
    unused_braces,
    unused_mut,
    unused_variables,
    unsafe_op_in_unsafe_fn,
    unused_assignments
)]
unsafe fn evaluate_every_row_except_two<F: SmallField, CS: ConstraintSystem<F>>(
    cs: &mut CS,
    random_point: MersenneQuartic<F>,
    witness: &[MersenneQuartic<F>],
    memory: &[MersenneQuartic<F>],
    setup: &[MersenneQuartic<F>],
    stage_2: &[MersenneQuartic<F>],
    witness_next_row: &[MersenneQuartic<F>],
    memory_next_row: &[MersenneQuartic<F>],
    stage_2_next_row: &[MersenneQuartic<F>],
    quotient_alpha: MersenneQuartic<F>,
    quotient_beta: MersenneQuartic<F>,
    divisors: &[MersenneQuartic<F>; 6usize],
    lookup_argument_linearization_challenges: [MersenneQuartic<F>;
        NUM_LOOKUP_ARGUMENT_LINEARIZATION_CHALLENGES],
    lookup_argument_gamma: MersenneQuartic<F>,
    lookup_argument_two_gamma: MersenneQuartic<F>,
    memory_argument_linearization_challenges: [MersenneQuartic<F>;
        NUM_MEM_ARGUMENT_LINEARIZATION_CHALLENGES],
    memory_argument_gamma: MersenneQuartic<F>,
    delegation_argument_linearization_challenges : [MersenneQuartic < F > ; NUM_DELEGATION_ARGUMENT_LINEARIZATION_CHALLENGES],
    delegation_argument_gamma: MersenneQuartic<F>,
    public_inputs: &[MersenneField<F>; 4usize],
    aux_proof_values: &WrappedProofAuxValues<F>,
    aux_boundary_values: WrappedAuxArgumentsBoundaryValues<F>,
    memory_timestamp_high_from_sequence_idx: MersenneField<F>,
    delegation_type: MersenneField<F>,
    delegation_argument_interpolant_linear_coeff: MersenneQuartic<F>,
) -> MersenneQuartic<F> {
    let every_row_except_two_last_contribution = {
        let mut accumulated_contribution = {
            let individual_term = {
                let mut individual_term = unsafe { *(witness.get_unchecked(113usize)) };
                let t = unsafe { *(witness_next_row.get_unchecked(3usize)) };
                individual_term = individual_term.sub(cs, &t);
                individual_term
            };
            individual_term
        };
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = unsafe { *(witness.get_unchecked(114usize)) };
                    let t = unsafe { *(witness_next_row.get_unchecked(48usize)) };
                    individual_term = individual_term.sub(cs, &t);
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let intermedaite_borrow_value = unsafe { *(witness.get_unchecked(43usize)) };
            let final_borrow_value = unsafe { *(witness.get_unchecked(44usize)) };
            let this_low = unsafe { *(memory.get_unchecked(0usize)) };
            let this_high = unsafe { *(memory.get_unchecked(1usize)) };
            let one = MersenneField::one(cs);
            let mut final_borrow_minus_one = final_borrow_value;
            final_borrow_minus_one = final_borrow_minus_one.sub_base(cs, &one);
            {
                let contribution = {
                    let individual_term = {
                        let next_low = unsafe { *(memory_next_row.get_unchecked(0usize)) };
                        let aux_low = unsafe { *(witness.get_unchecked(11usize)) };
                        let shift = MersenneField::allocate_constant(cs, Mersenne31Field(1 << 16));
                        let mut individual_term = intermedaite_borrow_value;
                        individual_term =
                            individual_term.mul_by_base_and_add(cs, &shift, &this_low);
                        individual_term = individual_term.sub(cs, &next_low);
                        individual_term = individual_term.sub(cs, &aux_low);
                        individual_term
                    };
                    individual_term
                };
                accumulated_contribution =
                    accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
            }
            {
                let contribution = {
                    let individual_term = {
                        let next_high = unsafe { *(memory_next_row.get_unchecked(1usize)) };
                        let aux_high = unsafe { *(witness.get_unchecked(12usize)) };
                        let shift = MersenneField::allocate_constant(cs, Mersenne31Field(1 << 16));
                        let mut individual_term = final_borrow_value;
                        individual_term = individual_term.mul_by_base(cs, &shift);
                        individual_term = individual_term.add(cs, &this_high);
                        individual_term = individual_term.sub(cs, &intermedaite_borrow_value);
                        individual_term = individual_term.sub(cs, &next_high);
                        individual_term = individual_term.sub(cs, &aux_high);
                        individual_term
                    };
                    individual_term
                };
                accumulated_contribution =
                    accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
            }
        }
        let divisor = divisors[1usize];
        accumulated_contribution = accumulated_contribution.mul(cs, &divisor);
        accumulated_contribution
    };
    every_row_except_two_last_contribution
}
#[allow(
    unused_braces,
    unused_mut,
    unused_variables,
    unsafe_op_in_unsafe_fn,
    unused_assignments
)]
unsafe fn evaluate_last_row_and_zero<F: SmallField, CS: ConstraintSystem<F>>(
    cs: &mut CS,
    random_point: MersenneQuartic<F>,
    witness: &[MersenneQuartic<F>],
    memory: &[MersenneQuartic<F>],
    setup: &[MersenneQuartic<F>],
    stage_2: &[MersenneQuartic<F>],
    witness_next_row: &[MersenneQuartic<F>],
    memory_next_row: &[MersenneQuartic<F>],
    stage_2_next_row: &[MersenneQuartic<F>],
    quotient_alpha: MersenneQuartic<F>,
    quotient_beta: MersenneQuartic<F>,
    divisors: &[MersenneQuartic<F>; 6usize],
    lookup_argument_linearization_challenges: [MersenneQuartic<F>;
        NUM_LOOKUP_ARGUMENT_LINEARIZATION_CHALLENGES],
    lookup_argument_gamma: MersenneQuartic<F>,
    lookup_argument_two_gamma: MersenneQuartic<F>,
    memory_argument_linearization_challenges: [MersenneQuartic<F>;
        NUM_MEM_ARGUMENT_LINEARIZATION_CHALLENGES],
    memory_argument_gamma: MersenneQuartic<F>,
    delegation_argument_linearization_challenges : [MersenneQuartic < F > ; NUM_DELEGATION_ARGUMENT_LINEARIZATION_CHALLENGES],
    delegation_argument_gamma: MersenneQuartic<F>,
    public_inputs: &[MersenneField<F>; 4usize],
    aux_proof_values: &WrappedProofAuxValues<F>,
    aux_boundary_values: WrappedAuxArgumentsBoundaryValues<F>,
    memory_timestamp_high_from_sequence_idx: MersenneField<F>,
    delegation_type: MersenneField<F>,
    delegation_argument_interpolant_linear_coeff: MersenneQuartic<F>,
) -> MersenneQuartic<F> {
    let last_row_and_zero_contribution = {
        let mut accumulated_contribution = {
            let individual_term = {
                let mut individual_term = unsafe { *(stage_2.get_unchecked(29usize)) };
                let t = unsafe { *(stage_2.get_unchecked(9usize)) };
                individual_term = individual_term.sub(cs, &t);
                let t = unsafe { *(stage_2.get_unchecked(10usize)) };
                individual_term = individual_term.sub(cs, &t);
                let t = unsafe { *(stage_2.get_unchecked(11usize)) };
                individual_term = individual_term.sub(cs, &t);
                let t = unsafe { *(stage_2.get_unchecked(12usize)) };
                individual_term = individual_term.sub(cs, &t);
                let t = unsafe { *(stage_2.get_unchecked(13usize)) };
                individual_term = individual_term.sub(cs, &t);
                let t = unsafe { *(stage_2.get_unchecked(14usize)) };
                individual_term = individual_term.sub(cs, &t);
                individual_term
            };
            individual_term
        };
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = unsafe { *(stage_2.get_unchecked(30usize)) };
                    let t = unsafe { *(stage_2.get_unchecked(15usize)) };
                    individual_term = individual_term.sub(cs, &t);
                    let t = unsafe { *(stage_2.get_unchecked(16usize)) };
                    individual_term = individual_term.sub(cs, &t);
                    let t = unsafe { *(stage_2.get_unchecked(17usize)) };
                    individual_term = individual_term.sub(cs, &t);
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = unsafe { *(stage_2.get_unchecked(31usize)) };
                    let t = unsafe { *(stage_2.get_unchecked(18usize)) };
                    individual_term = individual_term.sub(cs, &t);
                    let t = unsafe { *(stage_2.get_unchecked(19usize)) };
                    individual_term = individual_term.sub(cs, &t);
                    let t = unsafe { *(stage_2.get_unchecked(20usize)) };
                    individual_term = individual_term.sub(cs, &t);
                    let t = unsafe { *(stage_2.get_unchecked(21usize)) };
                    individual_term = individual_term.sub(cs, &t);
                    let t = unsafe { *(stage_2.get_unchecked(22usize)) };
                    individual_term = individual_term.sub(cs, &t);
                    let t = unsafe { *(stage_2.get_unchecked(23usize)) };
                    individual_term = individual_term.sub(cs, &t);
                    let t = unsafe { *(stage_2.get_unchecked(24usize)) };
                    individual_term = individual_term.sub(cs, &t);
                    let t = unsafe { *(stage_2.get_unchecked(25usize)) };
                    individual_term = individual_term.sub(cs, &t);
                    let t = unsafe { *(stage_2.get_unchecked(26usize)) };
                    individual_term = individual_term.sub(cs, &t);
                    let t = unsafe { *(stage_2.get_unchecked(27usize)) };
                    individual_term = individual_term.sub(cs, &t);
                    let t = unsafe { *(stage_2.get_unchecked(28usize)) };
                    individual_term = individual_term.sub(cs, &t);
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = unsafe { *(stage_2.get_unchecked(32usize)) };
                    let mut t = random_point;
                    t = t.mul(cs, &delegation_argument_interpolant_linear_coeff);
                    individual_term = individual_term.sub(cs, &t);
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        let divisor = divisors[5usize];
        accumulated_contribution = accumulated_contribution.mul(cs, &divisor);
        accumulated_contribution
    };
    last_row_and_zero_contribution
}
#[allow(
    unused_braces,
    unused_mut,
    unused_variables,
    unsafe_op_in_unsafe_fn,
    unused_assignments
)]
pub unsafe fn evaluate_quotient<F: SmallField, CS: ConstraintSystem<F>>(
    cs: &mut CS,
    random_point: MersenneQuartic<F>,
    witness: &[MersenneQuartic<F>],
    memory: &[MersenneQuartic<F>],
    setup: &[MersenneQuartic<F>],
    stage_2: &[MersenneQuartic<F>],
    witness_next_row: &[MersenneQuartic<F>],
    memory_next_row: &[MersenneQuartic<F>],
    stage_2_next_row: &[MersenneQuartic<F>],
    quotient_alpha: MersenneQuartic<F>,
    quotient_beta: MersenneQuartic<F>,
    divisors: &[MersenneQuartic<F>; 6usize],
    lookup_argument_linearization_challenges: [MersenneQuartic<F>;
        NUM_LOOKUP_ARGUMENT_LINEARIZATION_CHALLENGES],
    lookup_argument_gamma: MersenneQuartic<F>,
    lookup_argument_two_gamma: MersenneQuartic<F>,
    memory_argument_linearization_challenges: [MersenneQuartic<F>;
        NUM_MEM_ARGUMENT_LINEARIZATION_CHALLENGES],
    memory_argument_gamma: MersenneQuartic<F>,
    delegation_argument_linearization_challenges : [MersenneQuartic < F > ; NUM_DELEGATION_ARGUMENT_LINEARIZATION_CHALLENGES],
    delegation_argument_gamma: MersenneQuartic<F>,
    public_inputs: &[MersenneField<F>; 4usize],
    aux_proof_values: &WrappedProofAuxValues<F>,
    aux_boundary_values: WrappedAuxArgumentsBoundaryValues<F>,
    memory_timestamp_high_from_sequence_idx: MersenneField<F>,
    delegation_type: MersenneField<F>,
    delegation_argument_interpolant_linear_coeff: MersenneQuartic<F>,
) -> MersenneQuartic<F> {
    let every_row_except_last_contribution = unsafe {
        evaluate_every_row_except_last(
            cs,
            random_point,
            witness,
            memory,
            setup,
            stage_2,
            witness_next_row,
            memory_next_row,
            stage_2_next_row,
            quotient_alpha,
            quotient_beta,
            divisors,
            lookup_argument_linearization_challenges,
            lookup_argument_gamma,
            lookup_argument_two_gamma,
            memory_argument_linearization_challenges,
            memory_argument_gamma,
            delegation_argument_linearization_challenges,
            delegation_argument_gamma,
            public_inputs,
            aux_proof_values,
            aux_boundary_values,
            memory_timestamp_high_from_sequence_idx,
            delegation_type,
            delegation_argument_interpolant_linear_coeff,
        )
    };
    let every_row_except_two_last_contribution = unsafe {
        evaluate_every_row_except_two(
            cs,
            random_point,
            witness,
            memory,
            setup,
            stage_2,
            witness_next_row,
            memory_next_row,
            stage_2_next_row,
            quotient_alpha,
            quotient_beta,
            divisors,
            lookup_argument_linearization_challenges,
            lookup_argument_gamma,
            lookup_argument_two_gamma,
            memory_argument_linearization_challenges,
            memory_argument_gamma,
            delegation_argument_linearization_challenges,
            delegation_argument_gamma,
            public_inputs,
            aux_proof_values,
            aux_boundary_values,
            memory_timestamp_high_from_sequence_idx,
            delegation_type,
            delegation_argument_interpolant_linear_coeff,
        )
    };
    let last_row_and_zero_contribution = unsafe {
        evaluate_last_row_and_zero(
            cs,
            random_point,
            witness,
            memory,
            setup,
            stage_2,
            witness_next_row,
            memory_next_row,
            stage_2_next_row,
            quotient_alpha,
            quotient_beta,
            divisors,
            lookup_argument_linearization_challenges,
            lookup_argument_gamma,
            lookup_argument_two_gamma,
            memory_argument_linearization_challenges,
            memory_argument_gamma,
            delegation_argument_linearization_challenges,
            delegation_argument_gamma,
            public_inputs,
            aux_proof_values,
            aux_boundary_values,
            memory_timestamp_high_from_sequence_idx,
            delegation_type,
            delegation_argument_interpolant_linear_coeff,
        )
    };
    let first_row_contribution = {
        let mut accumulated_contribution = {
            let individual_term = {
                let mut individual_term = unsafe { *(memory.get_unchecked(0usize)) };
                let t = aux_boundary_values.lazy_init_first_row[0];
                individual_term = individual_term.sub_base(cs, &t);
                individual_term
            };
            individual_term
        };
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = unsafe { *(memory.get_unchecked(1usize)) };
                    let t = aux_boundary_values.lazy_init_first_row[1];
                    individual_term = individual_term.sub_base(cs, &t);
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = unsafe { *(memory.get_unchecked(2usize)) };
                    let t = aux_boundary_values.teardown_value_first_row[0];
                    individual_term = individual_term.sub_base(cs, &t);
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = unsafe { *(memory.get_unchecked(3usize)) };
                    let t = aux_boundary_values.teardown_value_first_row[1];
                    individual_term = individual_term.sub_base(cs, &t);
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = unsafe { *(memory.get_unchecked(4usize)) };
                    let t = aux_boundary_values.teardown_timestamp_first_row[0];
                    individual_term = individual_term.sub_base(cs, &t);
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = unsafe { *(memory.get_unchecked(5usize)) };
                    let t = aux_boundary_values.teardown_timestamp_first_row[1];
                    individual_term = individual_term.sub_base(cs, &t);
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = unsafe { *(witness.get_unchecked(3usize)) };
                    let t = public_inputs[0usize];
                    individual_term = individual_term.sub_base(cs, &t);
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = unsafe { *(witness.get_unchecked(48usize)) };
                    let t = public_inputs[1usize];
                    individual_term = individual_term.sub_base(cs, &t);
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let one = MersenneField::one(cs);
                    let mut individual_term = unsafe { *(stage_2.get_unchecked(37usize)) };
                    individual_term = individual_term.sub_base(cs, &one);
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        let divisor = divisors[2usize];
        accumulated_contribution = accumulated_contribution.mul(cs, &divisor);
        accumulated_contribution
    };
    let one_before_last_row_contribution = {
        let mut accumulated_contribution = {
            let individual_term = {
                let mut individual_term = unsafe { *(memory.get_unchecked(0usize)) };
                let t = aux_boundary_values.lazy_init_one_before_last_row[0];
                individual_term = individual_term.sub_base(cs, &t);
                individual_term
            };
            individual_term
        };
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = unsafe { *(memory.get_unchecked(1usize)) };
                    let t = aux_boundary_values.lazy_init_one_before_last_row[1];
                    individual_term = individual_term.sub_base(cs, &t);
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = unsafe { *(memory.get_unchecked(2usize)) };
                    let t = aux_boundary_values.teardown_value_one_before_last_row[0];
                    individual_term = individual_term.sub_base(cs, &t);
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = unsafe { *(memory.get_unchecked(3usize)) };
                    let t = aux_boundary_values.teardown_value_one_before_last_row[1];
                    individual_term = individual_term.sub_base(cs, &t);
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = unsafe { *(memory.get_unchecked(4usize)) };
                    let t = aux_boundary_values.teardown_timestamp_one_before_last_row[0];
                    individual_term = individual_term.sub_base(cs, &t);
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = unsafe { *(memory.get_unchecked(5usize)) };
                    let t = aux_boundary_values.teardown_timestamp_one_before_last_row[1];
                    individual_term = individual_term.sub_base(cs, &t);
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = unsafe { *(witness.get_unchecked(113usize)) };
                    let t = public_inputs[2usize];
                    individual_term = individual_term.sub_base(cs, &t);
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        {
            let contribution = {
                let individual_term = {
                    let mut individual_term = unsafe { *(witness.get_unchecked(114usize)) };
                    let t = public_inputs[3usize];
                    individual_term = individual_term.sub_base(cs, &t);
                    individual_term
                };
                individual_term
            };
            accumulated_contribution =
                accumulated_contribution.mul_and_add(cs, &quotient_alpha, &contribution);
        }
        let divisor = divisors[3usize];
        accumulated_contribution = accumulated_contribution.mul(cs, &divisor);
        accumulated_contribution
    };
    let last_row_contribution = {
        let mut accumulated_contribution = {
            let individual_term = {
                let mut individual_term = unsafe { *(stage_2.get_unchecked(37usize)) };
                let t = aux_proof_values.memory_grand_product_accumulator_final_value;
                individual_term = individual_term.sub(cs, &t);
                individual_term
            };
            individual_term
        };
        let divisor = divisors[4usize];
        accumulated_contribution = accumulated_contribution.mul(cs, &divisor);
        accumulated_contribution
    };
    let mut quotient = every_row_except_last_contribution;
    quotient = quotient.mul_and_add(cs, &quotient_beta, &every_row_except_two_last_contribution);
    quotient = quotient.mul_and_add(cs, &quotient_beta, &first_row_contribution);
    quotient = quotient.mul_and_add(cs, &quotient_beta, &one_before_last_row_contribution);
    quotient = quotient.mul_and_add(cs, &quotient_beta, &last_row_contribution);
    quotient = quotient.mul_and_add(cs, &quotient_beta, &last_row_and_zero_contribution);
    quotient
}
