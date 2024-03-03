use rand::Rng;
use std::io;
use std::cmp::Ordering;

const LIMIT_INCORRECT_GUESS: u8 = 3;

fn are_further_trials_possible(num_of_incorrect_trials: u8) -> bool {
    
    let remaining_trials: u8 = LIMIT_INCORRECT_GUESS - num_of_incorrect_trials;
    if num_of_incorrect_trials >= LIMIT_INCORRECT_GUESS {
        println!("THE GAME IS OVER!");
        return false;
    } else {
        println!("You have {remaining_trials} more trials");
        return true;
    }
}


fn main() {
    println!("Guess the number!");
    let incorrect_guess_counter: u8 = 0;

    

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
    
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("You typed non-number value >> SKIP!");
                continue;
            }
        };
    
        println!("You guessed: {guess}");
    
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                let incorrect_guess_counter = incorrect_guess_counter + 1; 
                println!("Too small!");
                let cond_to_continue: bool = are_further_trials_possible(incorrect_guess_counter);
                if !cond_to_continue {
                    break
                }
                
            },
            Ordering::Greater => {
                let incorrect_guess_counter = incorrect_guess_counter + 1; 
                println!("Too big!");
                let cond_to_continue: bool = are_further_trials_possible(incorrect_guess_counter);
                if !cond_to_continue {
                    break
                }
            },
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }    
    }

    
}