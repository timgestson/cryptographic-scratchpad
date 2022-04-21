use rand::prelude::*;
use std::vec::Vec;
use math::{fields::f64::BaseElement as Felt};

// Shamir's Secret Sharing Algorithm adapted from https://en.wikipedia.org/wiki/Shamir%27s_Secret_Sharing
fn main() {
    let secret = Felt::new(43253243242);
    let vector = split_secret(secret);
    let result = lagrange_interpolation(Felt::new(0), vector[0..3].to_vec());
    println!("{}", result);
}

fn split_secret(secret: Felt) -> Vec<(Felt, Felt)> {
    let (a, b) = generate_two_randoms();
    let mut pairs = Vec::<(Felt, Felt)>::new();
    for x in 1..6 {
        pairs.push(calculate_polynomial_pairs(secret, a, b, Felt::new(x)));
    }
    pairs
}

// Require 3 shares of the 6 to unlock secret
fn calculate_polynomial_pairs(a0: Felt, a1: Felt, a2: Felt, x: Felt) -> (Felt, Felt){
    (x, a0 + a1 * x + a2 * x * x)
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
    let secret = Felt::new(10);
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