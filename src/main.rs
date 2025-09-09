use ferris_says::say;
use std::io::{stdout, stdin, Write, BufWriter};
use std::cmp::Ordering;
use rand::Rng;

fn welcoming() {
    let stdout = stdout();
    let out = String::from("Hello! My name is Ferry!
                        Let's play a guessing game!
                        I'm thinking about a number between 1 and 100.
                        Try to guess the number in my head!");
    let width = out.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(&out, width, &mut writer).unwrap();
}

fn get_user_input(guess: &mut String) -> i32 {
    print!("\nGuess a number: ");
    stdout().flush().unwrap();
    stdin()
        .read_line(guess)
        .expect("Failed to get user input.");

    let guess: i32 = guess.trim().parse().expect("Please type a number!");
    return guess;
}

fn check_answer(guess: i32, answer: i32) -> bool {
    match guess.cmp(&answer) {
        Ordering::Less => {
            println!("Ferris: Too small!");
            return false;
        }
        Ordering::Equal => {
            println!("Ferris: You got it! Congratulations!");
            return true;
        }
        Ordering::Greater => {
            println!("Ferris: Too big!");
            return false;
        }
    }
}

fn main() {
    welcoming();

    let number: i32 = rand::thread_rng().gen_range(1..=100);
    
    let mut win: bool = false;
    while !win {
        let mut guess = String::new();
        let guess: i32 = get_user_input(&mut guess);
        win = check_answer(guess, number);
    }

    println!("\n===== END GAME =====");
}
