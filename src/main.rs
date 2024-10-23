use feldman_vss::vss;
use vss::FeldmanVSS;

const SECRET: &str = "Gowtham is the best";
const THRESHOLD: usize = 2;
const TOTAL_SHARES: usize = 4;

fn main() {
    let (q, _g) = FeldmanVSS::new();

    let secret_bytes = SECRET.as_bytes().to_vec();
    let vss = FeldmanVSS::split(secret_bytes, TOTAL_SHARES, THRESHOLD, &q);

    println!("Shares:");
    for (x, share) in &vss.shares {
        println!("Party {}: Share = {}", x, share);
    }

    println!("\nVerifying Shares:");
    for (x, share) in &vss.shares {
        let valid = FeldmanVSS::verify(share, *x, &vss.commits, &q);
        println!("Share {} is valid: {}", x, valid);
    }

    let shares_to_use = &vss.shares[0..THRESHOLD];
    let reconstructed = FeldmanVSS::reconstruct(shares_to_use, &q);

    println!("\nReconstructed Secret: {}", String::from_utf8(reconstructed).unwrap());
}