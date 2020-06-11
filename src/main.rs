use rand::Rng;
use std::cmp::Ordering;
use std::io;

const MAX_TRIES: u32 = 5;
const EXIT_VALUES: [&str;4] = ["quit", "exit", "stop", "done"];
const MAX_RESETS: u32 = 3;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut tries: u32 = 0;
    let mut resets: u32 = 0;

    loop {
        println!("Please input your guess");
        let mut guess = String::new();
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        if EXIT_VALUES.contains(&guess.trim()) {
            println!("Exiting...");
            break;
        }

        if (resets == MAX_RESETS) && (guess.trim() == "reset") {
            println!("You have reached the max amount of resets! You cannot use anymore.");
        }

        if (guess.trim() == "reset") && (resets != MAX_RESETS) {
            tries = 0;
            if resets < MAX_RESETS { resets = resets + 1; }
            println!("Tries reset to {}! (You have {} resets left)\n", MAX_TRIES, MAX_RESETS - resets);
        }
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        
        println!("You guessed: {}", guess);
        tries = tries + 1;
        

        if tries == MAX_TRIES && guess != secret_number {
            println!("You maxed out your tries! Exiting...");
            break;
        }
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!\n");
                println!("Your tries: {}", tries);
                break;
            }
        }
        
        println!("\nYou have {} tries left (Total: {}).\n", MAX_TRIES - tries, MAX_TRIES);
    }
}
