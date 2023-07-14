use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number");
    println!("Please input your guess");

    let number: i32 = rand::thread_rng().gen_range(1, 101);
    let mut start: bool = true;
    let mut times: i64 = 0;

    while start {
        let mut user_input: String = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");
        let user_input: i32 = user_input.trim().parse().expect("Please type a number");

        match user_input {
            x if x < number => {
                times += 1;
                println!("You guessed too low, Try again")
            }
            x if x > number => {
                times += 1;
                println!("You guessed too high, Try again")
            }
            _ => {
                start = false;
                println!(
                    "You guessed it right in {} times, number was {} ",
                    times, number
                )
            }
        }
    }
}
