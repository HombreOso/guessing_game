use rand::Rng;
use std::io;
use std::cmp::Ordering;



fn are_further_trials_possible(num_of_incorrect_trials: &mut u8, limit_incor_trials: &u8) -> bool {
    num_of_incorrect_trials = num_of_incorrect_trials + 1;
    let remaining_trials: u8 = limit_incor_trials - num_of_incorrect_trials;
    if num_of_incorrect_trials >= limit_incor_trials {
        println!("THE GAME IS OVER!");
        return false;
    } else {
        println!("You have {remaining_trials} more trials");
        return true;
    }
}


fn main() {
    println!("Guess the number!");

    

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
                println!("Too small!");
                incorrect_guess_counter = incorrect_guess_counter + 1;
                println!("You have {} more trials", LIMIT_INCORRECT_GUESS - incorrect_guess_counter);
                if incorrect_guess_counter == LIMIT_INCORRECT_GUESS {
                    break;
                };
            },
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }    
    }

    
}