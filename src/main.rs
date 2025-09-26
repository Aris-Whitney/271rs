
fn main() {
    dbg!(hamming::weight_u8(0x33_u8));
}

use num_bigint::BigUint;
use num_traits::FromPrimitive;

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 || n == 3 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}


fn generate_primes(count: usize) -> Vec<u64> {
    let mut primes = vec![2];
    let mut val = 3;

    while primes.len() < count {
        if is_prime(val) {
            primes.push(val);
        }
        val += 2;
    }

    primes
}

fn sqrt_fraction_64(prime: u64) -> u64 {
    let prime_big = BigUint::from_u64(prime).unwrap() << 128;
    let int_part = (prime as f64).sqrt().floor() as u64;
    let int_big = BigUint::from_u64(int_part).unwrap() << 64;

    let mut low: u64 = 0;
    let mut high: u64 = u64::MAX;
    let mut ans: u64 = 0;

    while low <= high {
        // Use checked_add to prevent overflow, break if overflow would occur
        let mid = match low.checked_add((high - low) / 2) {
            Some(m) => m,
            None => break,
        };

        let mid_big = BigUint::from_u64(mid).unwrap();
        let candidate = &int_big + &mid_big;
        let square = &candidate * &candidate;

        if square <= prime_big {
            ans = mid;
            // Safe add 1 or break if would overflow
            low = match mid.checked_add(1) {
                Some(val) => val,
                None => break,
            };
        } else {
            if mid == 0 {
                break;
            }
            high = mid - 1;
        }
    }

    ans
}


fn cbrt_fraction_64(prime: u64) -> u64 {
    let cube_root = (prime as f64).cbrt();
    let int_part = cube_root.floor();
    let frac_part = cube_root - int_part;
    (frac_part * (1u128 << 64) as f64) as u64
}

fn main() {
     let primes_8 = generate_primes(8);

    println!("First 8 primes:");
    for &p in &primes_8 {
        println!("{}", p);
    }

    println!("\nConstants (fractional parts of sqrt(prime) * 2^64):");
    for &p in &primes_8 {
        let constant = sqrt_fraction_64(p);
        println!("sqrt({:02}) frac * 2^64 = {:016x}", p, constant);
    }

    println!("\nconst SQRT_CONSTANTS: [u64; 8] = [");
    for &p in &primes_8 {
        let constant = sqrt_fraction_64(p);
        println!("    0x{:016x},", constant);
    }
    println!("];");

    // === SHA-512 style constants (cbrt) ===
    let primes_80 = generate_primes(80);

    println!("\n\nSHA-512 Round Constants (fractional parts of cbrt(prime) * 2^64):");
    println!("const CBRT_CONSTANTS: [u64; 80] = [");
    for &p in &primes_80 {
        let constant = cbrt_fraction_64(p);
        println!("    0x{:016x},", constant);
    }
    println!("];");
}
