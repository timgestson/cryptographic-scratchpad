use core::cmp::max;
use math::{fields::f64::BaseElement as Felt, FieldElement};
use rand::prelude::*;
use std::vec::Vec;

/* Berlekamp-Massey Algorithm
 * https://en.wikipedia.org/wiki/Berlekamp–Massey_algorithm
 * to find the shortest linear recurrence
 * as explained here https://mzhang2021.github.io/cp-blog/berlekamp-massey/
 * also finds the minimal charectaristic polynomial
 * of a linearly recurrent sequence in a field (See fibanacci test for details)
 */
fn berlekamp_massey(series: &[Felt]) -> Vec<Felt> {
    let mut c = Vec::<Felt>::new();
    let mut old_c = Vec::<Felt>::new();
    let mut best_c_index_failure: Option<usize> = None;
    for i in 0..series.len() {
        // calculate descrepency
        let delta = c
            .iter()
            .enumerate()
            .map(|(j, &cval)| cval * series[i - j - 1])
            .fold(series[i], |acc, elem| acc - elem);

        // if descrepency is zero, continue
        if delta == Felt::ZERO {
            continue;
        }

        match best_c_index_failure {
            None => {
                for _ in 0..(i + 1) {
                    c.push(Felt::new(random::<u64>()));
                }
                best_c_index_failure = Some(i);
            }
            Some(index) => {
                // negate sequence
                let mut d: Vec<Felt> = old_c.iter().map(|&elem| Felt::ZERO - elem).collect();
                // insert 1 on left
                d.insert(0, Felt::ONE);

                // series * delta / d(f + 1)
                let df1 = d
                    .iter()
                    .enumerate()
                    .map(|(j, &dval)| dval * series[index - j])
                    .fold(Felt::ZERO, |a, b| a + b);

                let coef = delta / df1;
                d = d.iter().map(|&elem| elem * coef).collect();
                for _ in 0..(i - index - 1) {
                    d.insert(0, Felt::ZERO);
                }

                // Temp copy of c
                let temp = c.clone();
                c.resize(max(d.len(), c.len()), Felt::ZERO);

                c = c.iter().zip(d.iter()).map(|(&c, &d)| c + d).collect();

                // Update old_c if there is a better left endpoint
                if i - temp.len() > index - old_c.len() {
                    old_c = temp;
                    best_c_index_failure = Some(i);
                }
            }
        }
    }
    c
}

fn minimal_characteristic_polynomial(c: &[Felt]) -> Vec<Felt> {
    // negate
    let mut p: Vec<Felt> = c.iter().map(|&elem| Felt::ZERO - elem).collect();
    // Insert highest level polynomial
    p.insert(0, Felt::ONE);
    p
}

#[test]
fn test_berlekamp_massey() {
    /* Fibinacci
     * As seen here https://mathcircle.berkeley.edu/sites/default/files/BMC6/ps/linear.pdf
     * 1 * F(n-1) + 1 * F(n-2) = F(n)  forall n >= 2
     * F(n+2) = F(n+1) + F(n)  forall n >= 0
     * 0 = F(n+2) − F(n+1) − F(n)
     * minimal charecteristic polynomial
     * 0 = x^2 - x - 1
     */
    let series: Vec<Felt> = [0_u64, 1, 1, 2, 3, 5, 8, 13]
        .iter()
        .map(|&num| Felt::new(num))
        .collect();
    let lr = berlekamp_massey(&series);
    assert_eq!(vec![Felt::new(1), Felt::new(1)], lr);
    assert_eq!(
        minimal_characteristic_polynomial(&lr),
        vec![Felt::ONE, Felt::ZERO - Felt::ONE, Felt::ZERO - Felt::ONE]
    );

    // 1,2,3,8,16
    // 2 F(n-1) = F(n)
    let series: Vec<Felt> = [1_u64, 2, 4, 8, 16]
        .iter()
        .map(|&num| Felt::new(num))
        .collect();
    assert_eq!(vec![Felt::new(2)], berlekamp_massey(&series));

    // 0,1,1,3,5,11,21
    // 1 F(n-1) + 2 F(n-2) = F(n)
    let series: Vec<Felt> = [0_u64, 1, 1, 3, 5, 11, 21]
        .iter()
        .map(|&num| Felt::new(num))
        .collect();
    assert_eq!(vec![Felt::new(1), Felt::new(2)], berlekamp_massey(&series));

    // 1,8,10,26,46
    // 1 F(n-1) + 2 F(n-2) = F(n)
    let series: Vec<Felt> = [1_u64, 8, 10, 26, 46]
        .iter()
        .map(|&num| Felt::new(num))
        .collect();
    assert_eq!(vec![Felt::new(1), Felt::new(2)], berlekamp_massey(&series));

    // 1,3,5,11,25,59,141,339
    // 3 F(n-1) - 1 F(n-2) - 1 F(n-3)
    let series: Vec<Felt> = [1_u64, 3, 5, 11, 25, 59, 141, 339]
        .iter()
        .map(|&num| Felt::new(num))
        .collect();
    assert_eq!(
        vec![Felt::new(3), Felt::ZERO - Felt::ONE, Felt::ZERO - Felt::ONE],
        berlekamp_massey(&series)
    );
}
