use std::env;
use io::*;  // matches your crate name

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        println!("Usage: <num1> <num2> <OP>");
        return;
    }

    let a = ix::from_hex(&args[1]);
    let b = ix::from_hex(&args[2]);
    let op = &args[3];

    let result = match op.as_str() {
        "ADD" => add_ix(&a, &b),
        "SUB" => sub_ix(&a, &b),
        // Placeholder for future operations
        //"MUL" => mul_ix(&a, &b),
        //"DIV" => div_ix(&a, &b),
        _ => panic!("Operation not implemented. Use ADD or SUB."),
    };

    print!("{}", result.to_hex());
}

