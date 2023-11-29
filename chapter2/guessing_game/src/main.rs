use rand::Rng;
use std::cmp::Ordering;
use std::io;

enum UserInput {
    Number(i32),
    Invalid(String),
    Quit(),
}

fn get_input() -> UserInput {
    println!("Enter a number, or q[uit]");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    match input.trim().parse() {
        Ok(value) => return UserInput::Number(value),
        Err(_) => {
            let thechar = input.trim().chars().nth(0);
            if let Some(letter) = thechar {
                if letter == 'q' {
                    return UserInput::Quit();
                } else {
                    return UserInput::Invalid(input);
                }
            } else {
                return UserInput::Invalid(input);
            }
        }
    }
}

fn main() {
    println!("Welcome to the Guessing Game!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    //println!("Secret: {secret_number}");

    loop {
        println!("What's your guess?");
        let input: UserInput = get_input();
        match input {
            UserInput::Number(guess) => {
                println!("You guessed {guess}");
                match guess.cmp(&secret_number) {
                    Ordering::Less => println!("Too small!"),
                    Ordering::Greater => println!("Too big!"),
                    Ordering::Equal => {
                        println!("You got it!");
                        break;
                    }
                }
            }
            UserInput::Quit() => {
                println!("bye");
                break;
            }
            UserInput::Invalid(str) => {
                println!("Invalid string entered: '{}'", str.trim());
            }
        }
    }
}
