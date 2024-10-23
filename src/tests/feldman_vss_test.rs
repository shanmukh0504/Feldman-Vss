use feldman_vss::FeldmanVSS;
use rand::thread_rng;
use num_prime::RandPrime;

const SECRET: &str = "Gowtham is the best";

#[test]
fn test_split_and_verify_shares() {
    let mut rng = thread_rng();
    let q = rng.gen_prime(256, None);

    let secret_bytes = SECRET.as_bytes().to_vec();
    let vss = FeldmanVSS::split(secret_bytes, 4, 2, &q);

    assert_eq!(vss.shares.len(), 4);
    assert_eq!(vss.commits.len(), 2);

    // Verify all shares
    for (x, share) in &vss.shares {
        assert!(FeldmanVSS::verify(share, *x, &vss.commits, &q));
    }
}

#[test]
fn test_reconstruct_secret() {
    let mut rng = thread_rng();
    let q = rng.gen_prime(256, None);

    let secret_bytes = SECRET.as_bytes().to_vec();
    let vss = FeldmanVSS::split(secret_bytes.clone(), 4, 2, &q);

    // Use the first two shares to reconstruct the secret
    let shares_to_use = &vss.shares[0..2];
    let reconstructed = FeldmanVSS::reconstruct(shares_to_use, &q);

    assert_eq!(reconstructed, secret_bytes);  // Ensure correct reconstruction
}