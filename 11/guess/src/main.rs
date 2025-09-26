

use std::fs::File;
use std::io::{Read, stdin};

const RED: &str="\u{001b}[31m";
const GRN: &str="\u{001b}[32m";
const YLW: &str="\u{001b}[33m";
const WHT: &str="\u{001b}[0m";

const WORDS: [&str;5]=["rotas", "opera", "tenet", "arepo", "sator"];

fn main() {
    // reading random bites from dev/urandom
    let mut devrnd =File::open("/dev/urandom").unwrap();
    let mut buffer=[0u8; (usize::BITS /8) as usize];
    devrnd.read_exact(&mut buffer).unwrap();

    //Now we pick a random word
    let secret=usize::from_le_bytes(buffer) % WORDS.len();
    let answer= String::from(WORDS[secret]);

    //Debugging

    println!("Guess the 5-letter word");

    loop {
        //take user input

        let mut guess = String::new();
        stdin().read_line(&mut guess).unwrap();
        let guess= guess.trim().to_lowercase();

        //check to see if it's a valid word

        if !WORDS.contains(&guess.as_str()) {
            println!("{RED}Invalid Word{WHT}");
            continue;
        }

        //printing color coded letters

        for i in 0..5 {
            let g_char =guess.chars().nth(i).unwrap();
            let a_char=answer.chars().nth(i).unwrap();

            if g_char==a_char {
                print!("{GRN}{g_char}{WHT}");
            }else if answer.contains(g_char) {
                print!("{YLW}{g_char}{WHT}");
            } else {
                print!("{g_char}");
            }
        }
        println!();

        //Checking to see if it wins

        if guess ==answer {
            println!("{GRN} Correct! The word was {answer}.{WHT}");
            break;
        }
    }
}
