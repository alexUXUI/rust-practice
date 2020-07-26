use std::io;
use rand::Rng;
use std::cmp::Ordering;

const INTRO_TEXT: &str = "------------------\n Guess the number!\n------------------";
const PROMPT_TEXT: &str = "\nPlease input your guess: \n";
const STDIN_ERROR: &str = "Failed to read line!";
const INPUT_ERROR: &str = "Please enter a number";

fn main() {
    println!("{}", INTRO_TEXT);
    run_game();
}

fn run_game() {
    let secret_number = rand::thread_rng().gen_range(1,4);

    loop {
        println!("{}", PROMPT_TEXT);

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect(STDIN_ERROR);

        println!("You guessed: {}", guess);

        // use variable shadowing to overwrite "guess" and convert it to a number
        // so that we can compare it to the user's guess
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", INPUT_ERROR);
                continue; // repeats the loop from the start
            }
        };
        
        match eval_win_or_lose(guess, &secret_number) {
            GameResult::Win => break,
            GameResult::Lose => continue
        }
    }
}

enum GameResult {
    Win,
    Lose
}

fn eval_win_or_lose(guess: u32, secret_number: &u32) -> GameResult {
    return match guess.cmp(&secret_number) {
        Ordering::Less => {
            println!("Too low");
            GameResult::Lose
        },
        Ordering::Greater => {
            println!("Too high");
            GameResult::Lose
        },
        Ordering::Equal => {
            println!("You win! 🙌");
            GameResult::Win
        }
    };
}
