use num_bigint::{BigInt, BigUint, RandBigInt};
use rand::thread_rng;
use num_integer::Integer;

pub fn generate_coefficients(secret: BigInt, threshold: usize, q: &BigUint) -> Vec<BigInt> {
    let mut rng = thread_rng();
    let mut coeffs = vec![secret];
    for _ in 1..threshold {
        coeffs.push(rng.gen_biguint_below(q).into());
    }
    coeffs
}

pub fn evaluate_polynomial(coeffs: &[BigInt], x: usize, q: &BigUint) -> BigInt {
    let x_bigint = BigInt::from(x);
    coeffs.iter().rev().fold(BigInt::from(0), |acc, coeff| {
        (acc * &x_bigint + coeff).mod_floor(&q.clone().into())
    })
}

pub fn evaluate_commitments(x: usize, commits: &[String], q: &BigUint) -> BigInt {
    let x_bigint = BigInt::from(x);
    commits.iter().rev().fold(BigInt::from(0), |acc, commit| {
        (acc * &x_bigint + BigInt::parse_bytes(commit.as_bytes(), 10).unwrap()).mod_floor(&q.clone().into())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::BigInt;
    use rand::thread_rng;
    use num_prime::RandPrime;

    #[test]
    fn test_generate_coefficients() {
        let mut rng = thread_rng();
        let q = rng.gen_prime(256, None);
        let secret = BigInt::from(42);

        let coeffs = generate_coefficients(secret, 3, &q);
        assert_eq!(coeffs.len(), 3);  // Ensure correct number of coefficients
    }

    #[test]
    fn test_evaluate_polynomial() {
        let mut rng = thread_rng();
        let q = rng.gen_prime(256, None);
        let secret = BigInt::from(10);
        let coeffs = generate_coefficients(secret, 3, &q);

        let result = evaluate_polynomial(&coeffs, 1, &q);
        assert!(result >= BigInt::from(0));  // Ensure non-negative result
    }
}