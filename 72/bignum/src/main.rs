use num_bigint::BigInt;
use num_traits::{Zero};
use std::env;
use std::process::exit;

fn add_ix(a: &BigInt, b: &BigInt) -> BigInt { a + b }
fn sub_ix(a: &BigInt, b: &BigInt) -> BigInt { a - b }
fn mul_ix(a: &BigInt, b: &BigInt) -> BigInt { a * b }

fn div_ix(a: &BigInt, b: &BigInt) -> BigInt {
    if b.is_zero() { eprintln!("Error: Division by zero"); exit(1); }
    a / b
}

fn rem_ix(a: &BigInt, b: &BigInt) -> BigInt {
    if b.is_zero() { eprintln!("Error: Division by zero"); exit(1); }
    a % b
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: {} <hex1> <hex2> <op>", args[0]);
        exit(1);
    }

    let a = BigInt::parse_bytes(args[1].trim_start_matches("0x").as_bytes(), 16)
        .expect("Invalid hex number");
    let b = BigInt::parse_bytes(args[2].trim_start_matches("0x").as_bytes(), 16)
        .expect("Invalid hex number");

    let result = match args[3].as_str() {
        "ADD" => add_ix(&a, &b),
        "SUB" => sub_ix(&a, &b),
        "MUL" => mul_ix(&a, &b),
        "QUO" => div_ix(&a, &b),
        "REM" => rem_ix(&a, &b),
        _ => { eprintln!("Error: Unknown operator {}", args[3]); exit(1); }
    };

    println!("{:x}", result);
}

