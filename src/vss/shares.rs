use crate::vss::polynomial;
use crate::vss::utils::{bigint_to_string, string_to_bigint};
use num_bigint::{BigInt, BigUint};
use num_integer::Integer;
use num_traits::One;

pub fn generate_shares(
    secret: &Vec<u8>,
    n: usize,
    threshold: usize,
    q: &BigUint,
) -> (Vec<(usize, String)>, Vec<String>) {
    let secret_int = BigInt::from_bytes_be(num_bigint::Sign::Plus, &secret);
    let coeffs = polynomial::generate_coefficients(secret_int, threshold, q);

    let shares = (1..=n)
        .map(|x| {
            let share = polynomial::evaluate_polynomial(&coeffs, x, q);
            (x, bigint_to_string(&share))
        })
        .collect();

    let commits = coeffs.iter().map(|c| bigint_to_string(c)).collect();
    (shares, commits)
}

pub fn verify_share(share: &str, x: usize, commits: &[String], q: &BigUint) -> bool {
    let share = string_to_bigint(share);
    polynomial::evaluate_commitments(x, commits, q) == share
}

pub fn reconstruct_secret(shares: &[(usize, String)], q: &BigUint) -> Vec<u8> {
    let mut secret = BigInt::from(0);

    for (i, (xi, share_i_str)) in shares.iter().enumerate() {
        let xi_bigint = BigInt::from(*xi);

        let mut num = BigInt::one(); // Initialize numerator as 1
        let mut den = BigInt::one(); // Initialize denominator as 1

        let share_i = string_to_bigint(share_i_str); // Convert share to BigInt

        for (j, (xj, _)) in shares.iter().enumerate() {
            if i != j {
                let xj_bigint = BigInt::from(*xj); // Convert xj to BigInt

                num = (num * -xj_bigint.clone()).mod_floor(&q.clone().into());
                den = (den * (xi_bigint.clone() - xj_bigint)).mod_floor(&q.clone().into());
            }
        }

        let q_as_bigint: BigInt = q.clone().into();

        let lagrange_coeff = (num * den.modpow(&(q_as_bigint.clone() - 2_u32), &q_as_bigint))
            .mod_floor(&q_as_bigint);

        secret = (secret + share_i * lagrange_coeff).mod_floor(&q_as_bigint);
    }

    secret.to_bytes_be().1
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_prime::RandPrime;
    use rand::thread_rng;

    #[test]
    fn test_generate_shares() {
        let mut rng = thread_rng();
        let q = rng.gen_prime(256, None);
        let secret = vec![1, 2, 3, 4]; // Arbitrary secret bytes

        let (shares, commits) = generate_shares(&secret, 4, 2, &q);
        assert_eq!(shares.len(), 4); // Ensure correct number of shares
        assert_eq!(commits.len(), 2); // Ensure threshold commitments are generated
    }

    #[test]
    fn test_verify_share() {
        let mut rng = thread_rng();
        let q = rng.gen_prime(256, None);
        let secret = vec![5, 6, 7, 8];

        let (shares, commits) = generate_shares(&secret, 4, 2, &q);
        let (x, share) = &shares[0];

        assert!(verify_share(share, *x, &commits, &q));
    }
}