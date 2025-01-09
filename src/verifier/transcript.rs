use std::cell::Ref;

use super::*;
use zkos_verifier::blake2s_u32::{
    BLAKE2S_DIGEST_SIZE_U32_WORDS,
    BLAKE2S_BLOCK_SIZE_U32_WORDS,
};

const USE_REDUCED_BLAKE2_ROUNDS: bool = true;


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SeedWrapped<F: SmallField>(pub [UInt32<F>; BLAKE2S_DIGEST_SIZE_U32_WORDS]);


#[derive(Clone, Copy, Debug, Default)]
pub struct Blake2sWrappedTranscript;

impl Blake2sWrappedTranscript {

    pub fn commit_initial_using_hasher<F: SmallField, CS: ConstraintSystem<F>>(
        cs: &mut CS,
        hasher: &mut Blake2sStateGate<F>,
        input: &[UInt32<F>],
    ) -> SeedWrapped<F> {
        let mut offset = 0;
        unsafe {
            hasher.reset(cs);
            Self::commit_inner(cs, hasher, input, &mut offset);
            Self::flush(cs, hasher, offset);
        }

        SeedWrapped(hasher.read_state_for_output())
    }

    pub fn commit_with_seed_using_hasher<F: SmallField, CS: ConstraintSystem<F>>(
        cs: &mut CS,
        hasher: &mut Blake2sStateGate<F>,
        seed: &mut SeedWrapped<F>,
        input: &[UInt32<F>],
    ) {
        let mut offset = 0;
        unsafe {
            hasher.reset(cs);
            Self::commit_inner(cs, hasher, &seed.0, &mut offset);
            Self::commit_inner(cs, hasher, input, &mut offset);
            Self::flush(cs, hasher, offset);
        }

        *seed = SeedWrapped(hasher.read_state_for_output());
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
                let mut dst_ptr: *mut UInt32<F> = hasher
                    .input_buffer
                    .as_mut_ptr()
                    .add(buffer_offset);
                
                let words_to_use =
                    core::cmp::min(remaining, BLAKE2S_BLOCK_SIZE_U32_WORDS - buffer_offset);
                
                core::ptr::copy_nonoverlapping(input_ptr, dst_ptr, words_to_use);

                remaining -= words_to_use;
                buffer_offset += words_to_use;
                input_ptr = input_ptr.add(words_to_use);
                dst_ptr = dst_ptr.add(words_to_use);
                // zero out the rest
                while dst_ptr < hasher.input_buffer.as_mut_ptr_range().end {
                    dst_ptr.write(UInt32::zero(cs));
                    dst_ptr = dst_ptr.add(1);
                }
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
        offset: usize
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
            core::ptr::copy_nonoverlapping(seed.0.as_ptr(), dst_ptr, BLAKE2S_DIGEST_SIZE_U32_WORDS);

            dst_ptr = dst_ptr.add(BLAKE2S_DIGEST_SIZE_U32_WORDS);
            // and if we need more - we will hash it with the increasing sequence counter
            for _ in 1..(num_rounds as u32) {
                Self::draw_randomness_inner(cs, hasher, seed);
                core::ptr::copy_nonoverlapping(seed.0.as_ptr(), dst_ptr, BLAKE2S_DIGEST_SIZE_U32_WORDS);
                dst_ptr = dst_ptr.add(BLAKE2S_DIGEST_SIZE_U32_WORDS);
            }
        }
    }

    fn draw_randomness_inner<F: SmallField, CS: ConstraintSystem<F>>(
        cs: &mut CS,
        hasher: &mut Blake2sStateGate<F>, 
        seed: &mut SeedWrapped<F>
    ) {
        unsafe {
            hasher.reset(cs);
            core::ptr::copy_nonoverlapping(seed.0.as_ptr(), hasher.input_buffer.as_mut_ptr(), BLAKE2S_DIGEST_SIZE_U32_WORDS);
            let mut dst_ptr = hasher.input_buffer.as_mut_ptr().add(BLAKE2S_DIGEST_SIZE_U32_WORDS);
            while dst_ptr < hasher.input_buffer.as_mut_ptr_range().end {
                dst_ptr.write(UInt32::zero(cs));
                dst_ptr = dst_ptr.add(1);
            }
        }

        if Blake2sStateGate::<F>::SUPPORT_SPEC_SINGLE_ROUND {
            unsafe {
                hasher.spec_run_sinlge_round_into_destination::<CS, USE_REDUCED_BLAKE2_ROUNDS>(
                    cs, 
                    BLAKE2S_DIGEST_SIZE_U32_WORDS,
                    &mut seed.0 as *mut _,
                );
            }
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

    pub fn verify_pow_using_hasher<F: SmallField, CS: ConstraintSystem<F>>(
        cs: &mut CS,
        hasher: &mut Blake2sStateGate<F>,
        seed: &mut SeedWrapped<F>,
        nonce: [UInt32<F>; 2],
        pow_bits: UInt32<F>,
    ) {
        // assert!(pow_bits <= 32);
        unsafe {
            hasher.reset(cs);
            // first we can just take values from the seed
            core::ptr::copy_nonoverlapping(seed.0.as_ptr(), hasher.input_buffer.as_mut_ptr(), BLAKE2S_DIGEST_SIZE_U32_WORDS);

            // LE words of nonce
            hasher.input_buffer[8] = nonce[0];
            hasher.input_buffer[9] = nonce[1];
            let mut dst_ptr = hasher.input_buffer.as_mut_ptr().add(BLAKE2S_DIGEST_SIZE_U32_WORDS + 2);
            while dst_ptr < hasher.input_buffer.as_mut_ptr_range().end {
                dst_ptr.write(UInt32::zero(cs));
                dst_ptr = dst_ptr.add(1);
            }

            hasher.run_round_function::<CS, USE_REDUCED_BLAKE2_ROUNDS>(
                cs,
                BLAKE2S_DIGEST_SIZE_U32_WORDS + 2,
                true,
            );
        }

        // check that first element is small enough
        // assert!(
        //     hasher.state[0] <= (0xffffffff >> pow_bits),
        //     "we expect {} bits of PoW using nonce {}, but top word is 0x{:08x} and full state is {:?}",
        //     pow_bits,
        //     nonce,
        //     hasher.state[0],
        //     &hasher.state,
        // );
        

        // copy it out
        *seed = SeedWrapped(hasher.read_state_for_output());
    }

}