use num_bigint::BigInt;
use num_traits::{Zero, One};
use std::str::FromStr;

fn is_valid_odd(n: &BigInt) -> bool {
    n % 2u8 == BigInt::one() && n > &BigInt::zero()
}

pub fn reverse_collatz_to_g1(start_str: &str) -> (String, u64) {
    let mut n = BigInt::from_str(start_str).expect("Invalid input");
    let g1_roots: Vec<BigInt> = (4..=400)
        .step_by(2)
        .map(|k| (BigInt::one() << k) - 1u8)
        .map(|v| v / 3u8)
        .collect();

    let mut total_k: u64 = 0;

    loop {
        if g1_roots.contains(&n) {
            for k in (4..=400).step_by(2) {
                let g1_val = ((BigInt::one() << k) - 1u8) / 3u8;
                if g1_val == n {
                    total_k += k;
                    return (n.to_string(), total_k);
                }
            }
        }

        let t = &n * 3u8 + 1u8;

        let mut k = 0;
        let mut d = t.clone();
        while &d % 2u8 == BigInt::zero() {
            d = d / 2u8;
            k += 1;
        }

        if k == 0 {
            return ("No valid reverse step".to_string(), total_k);
        }

        let m = &t >> k;
        if !is_valid_odd(&m) {
            return ("Invalid reverse step (even m)".to_string(), total_k);
        }

        n = m;
        total_k += k;
    }
}

fn main() {
    let input = "77777777777777777777";
    let (g1, steps) = reverse_collatz_to_g1(input);
    println!("✅ Reached G1 root: {}", g1);
    println!("🧮 Total cumulative steps (Σk): {}", steps);
}
