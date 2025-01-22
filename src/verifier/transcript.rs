// use std::cell::Ref;

use super::*;
use boojum::cs::implementations::pow;
use zkos_verifier::blake2s_u32::{BLAKE2S_BLOCK_SIZE_U32_WORDS, BLAKE2S_DIGEST_SIZE_U32_WORDS};

const USE_REDUCED_BLAKE2_ROUNDS: bool = true;

#[derive(Clone, Copy, Debug)] //, PartialEq, Eq)]
pub struct SeedWrapped<F: SmallField>(pub [Word<F>; BLAKE2S_DIGEST_SIZE_U32_WORDS]);

#[derive(Clone, Copy, Debug, Default)]
pub struct Blake2sWrappedTranscript;

impl Blake2sWrappedTranscript {
    pub fn commit_initial_using_hasher<F: SmallField, CS: ConstraintSystem<F>>(
        cs: &mut CS,
        hasher: &mut Blake2sStateGate<F>,
        input: &[UInt32<F>],
    ) -> SeedWrapped<F> {
        let mut offset = 0;
        hasher.reset();
        Self::commit_inner(cs, hasher, input, &mut offset);
        Self::flush(cs, hasher, offset);

        SeedWrapped(hasher.read_state_for_output())
    }

    pub fn commit_with_seed_using_hasher<F: SmallField, CS: ConstraintSystem<F>>(
        cs: &mut CS,
        hasher: &mut Blake2sStateGate<F>,
        seed: &mut SeedWrapped<F>,
        input: &[UInt32<F>],
    ) {
        let mut offset = 0;
        hasher.reset();
        Self::commit_seed_inner(hasher, &seed, &mut offset);
        Self::commit_inner(cs, hasher, input, &mut offset);
        Self::flush(cs, hasher, offset);

        *seed = SeedWrapped(hasher.read_state_for_output());
    }

    fn commit_seed_inner<F: SmallField>(
        hasher: &mut Blake2sStateGate<F>,
        seed: &SeedWrapped<F>,
        offset: &mut usize,
    ) {
        debug_assert!(offset == &0);
        hasher.input_buffer[..BLAKE2S_DIGEST_SIZE_U32_WORDS].copy_from_slice(&seed.0);
        *offset = BLAKE2S_DIGEST_SIZE_U32_WORDS;
    }

    fn commit_inner<F: SmallField, CS: ConstraintSystem<F>>(
        cs: &mut CS,
        hasher: &mut Blake2sStateGate<F>,
        input: &[UInt32<F>],
        offset: &mut usize,
    ) {
        debug_assert!(input.len() > 0);
        // hasher is in the proper state, and we just need to drive it effectively computing blake2s hash over input sequence
        let input_len_words = input.len();
        let effective_input_len = *offset + input_len_words;
        let mut num_rounds = effective_input_len / BLAKE2S_BLOCK_SIZE_U32_WORDS;
        if effective_input_len % BLAKE2S_BLOCK_SIZE_U32_WORDS > 0 {
            num_rounds += 1;
        }
        let mut remaining = input_len_words;
        let mut input_ptr = input.as_ptr();
        let mut buffer_offset = *offset;
        unsafe {
            for _ in 0..num_rounds {
                let words_to_use =
                    core::cmp::min(remaining, BLAKE2S_BLOCK_SIZE_U32_WORDS - buffer_offset);

                let input_slice = core::slice::from_raw_parts(input_ptr, words_to_use);
                hasher.input_buffer[buffer_offset..(buffer_offset + words_to_use)]
                    .iter_mut()
                    .zip(input_slice)
                    .for_each(|(dst, src)| {
                        *dst = Word {
                            inner: src.to_le_bytes(cs),
                        };
                    });

                remaining -= words_to_use;
                buffer_offset += words_to_use;
                input_ptr = input_ptr.add(words_to_use);
                // zero out the rest
                hasher.input_buffer[buffer_offset..]
                    .iter_mut()
                    .for_each(|el| {
                        *el = Word {
                            inner: [UInt8::zero(cs); 4],
                        };
                    });
                if remaining > 0 {
                    debug_assert_eq!(buffer_offset, BLAKE2S_BLOCK_SIZE_U32_WORDS);
                    hasher.run_round_function::<CS, USE_REDUCED_BLAKE2_ROUNDS>(
                        cs,
                        BLAKE2S_BLOCK_SIZE_U32_WORDS,
                        false,
                    );
                    buffer_offset = 0;
                }
            }
        }
        *offset = buffer_offset;
    }

    fn flush<F: SmallField, CS: ConstraintSystem<F>>(
        cs: &mut CS,
        hasher: &mut Blake2sStateGate<F>,
        offset: usize,
    ) {
        hasher.run_round_function::<CS, USE_REDUCED_BLAKE2_ROUNDS>(cs, offset, true);
    }

    pub fn draw_randomness_using_hasher<F: SmallField, CS: ConstraintSystem<F>>(
        cs: &mut CS,
        hasher: &mut Blake2sStateGate<F>,
        seed: &mut SeedWrapped<F>,
        dst: &mut [UInt32<F>],
    ) {
        debug_assert_eq!(
            dst.len() % BLAKE2S_DIGEST_SIZE_U32_WORDS,
            0,
            "please pad the dst buffer to the multiple of {}",
            BLAKE2S_DIGEST_SIZE_U32_WORDS
        );
        let num_rounds = dst.len() / BLAKE2S_DIGEST_SIZE_U32_WORDS;
        unsafe {
            let mut dst_ptr: *mut UInt32<F> = dst.as_mut_ptr();
            // first we can just take values from the seed
            let dst_slice = core::slice::from_raw_parts_mut(dst_ptr, BLAKE2S_DIGEST_SIZE_U32_WORDS);
            dst_slice
                .iter_mut()
                .zip(seed.0.iter())
                .for_each(|(dst, src)| {
                    *dst = UInt32::from_le_bytes(cs, src.inner);
                });

            dst_ptr = dst_ptr.add(BLAKE2S_DIGEST_SIZE_U32_WORDS);
            // and if we need more - we will hash it with the increasing sequence counter
            for _ in 1..(num_rounds as u32) {
                Self::draw_randomness_inner(cs, hasher, seed);
                let dst_slice =
                    core::slice::from_raw_parts_mut(dst_ptr, BLAKE2S_DIGEST_SIZE_U32_WORDS);
                dst_slice
                    .iter_mut()
                    .zip(seed.0.iter())
                    .for_each(|(dst, src)| {
                        *dst = UInt32::from_le_bytes(cs, src.inner);
                    });
                dst_ptr = dst_ptr.add(BLAKE2S_DIGEST_SIZE_U32_WORDS);
            }
        }
    }

    fn draw_randomness_inner<F: SmallField, CS: ConstraintSystem<F>>(
        cs: &mut CS,
        hasher: &mut Blake2sStateGate<F>,
        seed: &mut SeedWrapped<F>,
    ) {
        hasher.reset();
        hasher.input_buffer[..BLAKE2S_DIGEST_SIZE_U32_WORDS].copy_from_slice(&seed.0);
        hasher.input_buffer[BLAKE2S_DIGEST_SIZE_U32_WORDS..]
            .iter_mut()
            .for_each(|el| {
                *el = Word {
                    inner: [UInt8::zero(cs); 4],
                }
            });

        if Blake2sStateGate::<F>::SUPPORT_SPEC_SINGLE_ROUND {
            unimplemented!()
            // unsafe {
            //     hasher.spec_run_sinlge_round_into_destination::<CS, USE_REDUCED_BLAKE2_ROUNDS>(
            //         cs,
            //         BLAKE2S_DIGEST_SIZE_U32_WORDS,
            //         &mut seed.0 as *mut _,
            //     );
            // }
        } else {
            // we take the seed + sequence id, and produce hash
            hasher.run_round_function::<CS, USE_REDUCED_BLAKE2_ROUNDS>(
                cs,
                BLAKE2S_DIGEST_SIZE_U32_WORDS,
                true,
            );

            seed.0 = hasher.read_state_for_output();
        }
    }

    pub fn verify_pow_using_hasher<
        F: SmallField,
        CS: ConstraintSystem<F>,
        const POW_BITS: usize,
    >(
        cs: &mut CS,
        hasher: &mut Blake2sStateGate<F>,
        seed: &mut SeedWrapped<F>,
        nonce: [UInt32<F>; 2],
        // pow_bits: u32,
    ) {
        hasher.reset();
        // first we can just take values from the seed
        hasher.input_buffer[..BLAKE2S_DIGEST_SIZE_U32_WORDS].copy_from_slice(&seed.0);
        // LE words of nonce
        hasher.input_buffer[8] = Word {
            inner: nonce[0].to_le_bytes(cs),
        };
        hasher.input_buffer[9] = Word {
            inner: nonce[1].to_le_bytes(cs),
        };
        hasher.input_buffer[BLAKE2S_DIGEST_SIZE_U32_WORDS + 2..]
            .iter_mut()
            .for_each(|el| {
                *el = Word {
                    inner: [UInt8::zero(cs); 4],
                }
            });

        hasher.run_round_function::<CS, USE_REDUCED_BLAKE2_ROUNDS>(
            cs,
            BLAKE2S_DIGEST_SIZE_U32_WORDS + 2,
            true,
        );

        // check that first element is small enough assert!(hasher.state[0] <= (0xffffffff >> pow_bits));
        assert!(POW_BITS > 16);
        let zero = UInt8::zero(cs);
        let first_el_high_0 = hasher.extended_state[0].inner[2];
        let first_el_high_1 = hasher.extended_state[0].inner[3];
        dbg!(zero);
        dbg!(first_el_high_0);
        dbg!(first_el_high_1);
        // FIXME
        /*
        let _ = zero.sub_no_overflow(cs, first_el_high_0);
        let _ = zero.sub_no_overflow(cs, first_el_high_1);
        let first_el_low =
            UInt16::from_le_bytes(cs, hasher.extended_state[0].inner[..2].try_into().unwrap());
        let pow_bits_mask = (0xffffffff as u32 >> POW_BITS) as u16;
        let pow_bits_mask = UInt16::allocated_constant(cs, pow_bits_mask);
        let _ = pow_bits_mask.sub_no_overflow(cs, first_el_low);*/

        // copy it out
        *seed = SeedWrapped(hasher.read_state_for_output());
    }
}
