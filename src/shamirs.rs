use rand::prelude::*;
use std::vec::Vec;
use math::{fields::f64::BaseElement as Felt, FieldElement};

/* Shamir's Secret Sharing Algorithm adapted from https://en.wikipedia.org/wiki/Shamir%27s_Secret_Sharing
// Using Winterfell's FieldElements so I dont have to reimplement Finite Fields
```
let secret = Felt::new(43253243242);
// Split the secret into 6 points of a quadratic polynomial
let vector = split_secret(secret);
// Perform lagrange interpolation on 3 points to determine the y intercept (x = 0)
let result = lagrange_interpolation(Felt::new(0), vector[0..3].to_vec());
println!("{}", result);
```
*/

fn split_secret(secret: Felt) -> Vec<(Felt, Felt)> {
    let (a, b) = generate_two_randoms();
    let mut pairs = Vec::<(Felt, Felt)>::new();
    for x in 1..6 {
        let x = Felt::new(x);
        pairs.push((x, evaluate_polynomial_at(&[secret, a, b], x)));
    }
    pairs
}

fn evaluate_polynomial_at(polynomial: &[Felt], x: Felt) -> Felt {
    polynomial.iter().enumerate().map(|(degree, &(mut coef))| {
        for _ in 0..degree {
            coef *= x;
        }
        coef
    }).fold(Felt::ZERO, |a, b| a + b)
}

fn generate_two_randoms() -> (Felt, Felt) {
    let a = random::<u64>();
    let b = random::<u64>();
    (Felt::new(a), Felt::new(b))
}

fn lagrange_interpolation(x: Felt, points: Vec<(Felt, Felt)>) -> Felt {
    let mut result = Felt::new(0);
    let points_len = points.len();
    for i in 0..points_len {
        let mut acc = points[i].1;
        for j in 0..points_len {
            if i != j {
                let den = points[i].0 - points[j].0;
                let num = x - points[j].0;
                acc *= num / den;
            }
        }
        result += acc;
    }
    result
}

#[test]
fn test_shamirs_secret(){
    let secret = Felt::new(54354325);
    let vector = split_secret(secret);
    // Assert 3 points gives the correct y intercept
    assert_eq!(secret, lagrange_interpolation(Felt::new(0), vector[0..3].to_vec()));
    // Assert a different 3 points gives the correct y intercept
    assert_eq!(secret, lagrange_interpolation(Felt::new(0), vector[2..5].to_vec()));
    // Assert 4 points gives the correct y intecept 
    assert_eq!(secret, lagrange_interpolation(Felt::new(0), vector[0..4].to_vec()));
    // Assert 2 points does not give up the secret
    assert_ne!(secret, lagrange_interpolation(Felt::new(0), vector[0..2].to_vec()));
}