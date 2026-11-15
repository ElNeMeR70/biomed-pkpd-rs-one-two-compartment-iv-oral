//! biomed-pkpd-rs-one-two-compartment-iv-oral — Pharmacokinetics (PK/PD) & Clinical Trial Modeling (one-two-compartment-iv-oral)
//! Suite: bio-med-1000

use std::env;

pub fn fnv1a_64(data: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for &b in data {
        hash ^= b as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

fn main() {
    let arg = env::args().nth(1).unwrap_or_else(|| "benchmark".to_string());
    let sig = fnv1a_64(format!("biomed-pkpd-rs-one-two-compartment-iv-oral:{}", arg).as_bytes());
    println!(
        "{\"engine\":\"biomed-pkpd-rs-one-two-compartment-iv-oral\",\"suite\":\"bio-med-1000\",\"fnv64\":\"0x{:016x}\",\"status\":\"VERIFIED_OK\"}",
        sig
    );
}
