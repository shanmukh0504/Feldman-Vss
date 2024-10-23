pub mod shares;
pub mod polynomial;
pub mod utils;

use num_bigint::{BigUint, RandBigInt};
use num_prime::RandPrime;
use rand::thread_rng;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeldmanVSS {
    pub shares: Vec<(usize, String)>,
    pub commits: Vec<String>,
    pub cipher: Vec<u8>,
}

impl FeldmanVSS {
    pub fn new() -> (BigUint, BigUint) {
        let mut rng = thread_rng();
        let q = rng.gen_prime(256, None);
        let g = rng.gen_biguint_below(&q);
        (q, g)
    }

    pub fn split(secret: Vec<u8>, n: usize, threshold: usize, q: &BigUint) -> Self {
        let (shares, commits) = shares::generate_shares(&secret, n, threshold, q);
        FeldmanVSS { shares, commits, cipher: secret }
    }    

    pub fn verify(share: &str, x: usize, commits: &[String], q: &BigUint) -> bool {
        shares::verify_share(share, x, commits, q)
    }

    pub fn reconstruct(shares: &[(usize, String)], q: &BigUint) -> Vec<u8> {
        shares::reconstruct_secret(shares, q)
    }
}
