#![allow(non_camel_case_types)]

pub struct ix {
    pub sign: bool,      // false = positive, true = negative
    pub vals: Vec<u64>,  // little-endian storage: least-significant first
}

impl ix {
    /// Convert a hex string (with or without "0x") to ix
    pub fn from_hex(hex: &str) -> Self {
        let hex = hex.strip_prefix("0x").unwrap_or(hex);
        let mut vals = Vec::new();

        let mut hex = hex.to_string();
        while hex.len() % 16 != 0 {
            hex = "0".to_string() + &hex;
        }

        // Read 16-character chunks from the end (LSW first)
        for chunk in hex.as_bytes().rchunks(16) {
            let s = std::str::from_utf8(chunk).unwrap();
            let val = u64::from_str_radix(s, 16).unwrap();
            vals.push(val);
        }

        // Remove trailing zeros
        while vals.last() == Some(&0) {
            vals.pop();
        }

        ix { sign: false, vals }
    }

    /// Convert ix back to a hex string
    pub fn to_hex(&self) -> String {
        if self.vals.is_empty() {
            return "0".to_string();
        }

        let mut s = String::new();
        if self.sign {
            s.push('-');
        }

        for val in self.vals.iter().rev() {
            s.push_str(&format!("{:016x}", val));
        }

        // Remove leading zeros
        while s.len() > 1 && &s[0..1] == "0" {
            s = s[1..].to_string();
        }

        s
    }
}

/// Add/subtract magnitudes (absolute values)
pub fn add_mag(a_vals: &Vec<u64>, b_vals: &Vec<u64>) -> Vec<u64> {
    let mut result = Vec::new();
    let mut carry: u64 = 0;
    let n = a_vals.len().max(b_vals.len());

    for i in 0..n {
        let x = *a_vals.get(i).unwrap_or(&0);
        let y = *b_vals.get(i).unwrap_or(&0);
        let (sum1, overflow1) = x.overflowing_add(y);
        let (sum2, overflow2) = sum1.overflowing_add(carry);
        carry = (overflow1 as u64) + (overflow2 as u64);
        result.push(sum2);
    }

    if carry != 0 {
        result.push(carry);
    }

    result
}

pub fn sub_mag(a_vals: &Vec<u64>, b_vals: &Vec<u64>) -> Vec<u64> {
    let mut result = Vec::new();
    let mut borrow: u64 = 0;

    for i in 0..a_vals.len() {
        let x = *a_vals.get(i).unwrap_or(&0);
        let y = *b_vals.get(i).unwrap_or(&0);
        let (sub1, overflow1) = x.overflowing_sub(y);
        let (sub2, overflow2) = sub1.overflowing_sub(borrow);
        borrow = (overflow1 as u64) + (overflow2 as u64);
        result.push(sub2);
    }

    // Remove trailing zeros
    while result.last() == Some(&0) {
        result.pop();
    }

    result
}

/// Compare magnitudes
pub fn gte_mag(a_vals: &Vec<u64>, b_vals: &Vec<u64>) -> bool {
    if a_vals.len() != b_vals.len() {
        return a_vals.len() > b_vals.len();
    }
    for (x, y) in a_vals.iter().rev().zip(b_vals.iter().rev()) {
        if x != y {
            return x > y;
        }
    }
    true
}

/// Addition of ix numbers (only positive for now)
pub fn add_ix(a: &ix, b: &ix) -> ix {
    ix {
        sign: false,
        vals: add_mag(&a.vals, &b.vals),
    }
}

/// Subtraction of ix numbers (only positive for now)
pub fn sub_ix(a: &ix, b: &ix) -> ix {
    let (big, small) = if gte_mag(&a.vals, &b.vals) {
        (a, b)
    } else {
        (b, a)
    };
    ix {
        sign: if gte_mag(&a.vals, &b.vals) { false } else { true },
        vals: sub_mag(&big.vals, &small.vals),
    }
}

