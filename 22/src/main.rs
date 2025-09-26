use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

const RED: u8 = 31;
const GREEN: u8 = 32;
const YELLOW: u8 = 33;

const TOP: &str = "┌───┬───┬───┬───┬───┐";
const MID: &str = "├───┼───┼───┼───┼───┤";
const BOT: &str = "└───┴───┴───┴───┴───┘";

fn load_words(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("Failed to open word list");
    let reader = BufReader::new(file);
    reader
        .lines()
        .filter_map(Result::ok)
        .filter(|line| line.len() == 5)
        .map(|line| line.to_lowercase())
        .collect()
}

fn get_random_index(max: usize) -> usize {
    let mut file = File::open("/dev/random").expect("Cannot open /dev/random");
    let mut buf = [0u8; 2];
    file.read_exact(&mut buf).expect("Failed to read /dev/random");
    let num = u16::from_ne_bytes(buf);
    (num as usize) % max
}

fn print_letter(c: char, color_code: u8) {
    print!("│ \x1b[{}m{}\x1b[0m ", color_code, c);
}

fn evaluate_guess(guess: &str, answer: &str) {
    for (i, gc) in guess.chars().enumerate() {
        let ac = answer.chars().nth(i).unwrap();
        let color = if gc == ac {
            GREEN
        } else if answer.contains(gc) {
            YELLOW
        } else {
            RED
        };
        print_letter(gc, color);
    }
    println!("│");
}

fn draw_board(guesses: &[String], answer: &str) {
    println!("\x1b[2J"); // Clear screen
    println!("{}", TOP);
    for i in 0..5 {
        evaluate_guess(&guesses[i], answer);
        println!("{}", MID);
    }
    evaluate_guess(&guesses[5], answer);
    println!("{}", BOT);
}

fn main() {
    let word_list = load_words("words.txt");
    if word_list.is_empty() {
        println!("Word list is empty.");
        return;
    }

    let answer = word_list[get_random_index(word_list.len())].clone();
    let mut guesses = vec!["     ".to_string(); 6];
    let mut attempts = 0;

    println!("\x1b[2J"); // Clear screen
    println!("Use lowercase only btw.");

    while attempts < 6 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let guess = input.trim().to_lowercase();

        if guess.len() != 5 {
            println!("Guess must be 5 letters.");
            continue;
        }

        if !word_list.contains(&guess) {
            println!("Not a valid word!");
            continue;
        }

        guesses[attempts] = guess.clone();
        draw_board(&guesses, &answer);

        if guess == answer {
            println!("Winner!");
            return;
        }

        attempts += 1;
    }

    println!("Game over :( The word was: {}", answer);
}

