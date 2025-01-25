use super::*;

pub fn binary_parallel_select<F: SmallField, T: Selectable<F>, CS: ConstraintSystem<F>, const N: usize>(
    cs: &mut CS,
    elements: &[[T; N]],
    bits: &[Boolean<F>],
) -> [T; N] {
    assert_eq!(elements.len(), 1 << bits.len());
    assert!(bits.len() > 0);

    let mut input_space = Vec::with_capacity(elements.len() / 2);
    let mut dst_space = Vec::with_capacity(elements.len() / 2);

    for (idx, bit) in bits.iter().enumerate() {
        let src = if idx == 0 { elements } else { &input_space };

        debug_assert_eq!(elements.len() % 2, 0);
        dst_space.clear();

        for src in src.array_chunks::<2>() {
            let [a, b] = src;
            // NOTE order here
            let selected = T::parallel_select(cs, *bit, b, a);
            dst_space.push(selected);
        }

        std::mem::swap(&mut dst_space, &mut input_space);
    }

    assert_eq!(input_space.len(), 1);

    input_space.pop().unwrap()
}