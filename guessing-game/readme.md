# Guess the Number Game

This code implements a simple "Guess the Number" game in Rust. The program generates a random number between 1 and 100, and the user needs to guess the correct number.

## How to Play

1. The program will prompt the user to input their guess.
2. Enter a number and press Enter to submit your guess.
3. The program will provide feedback based on your guess:
   - If your guess is lower than the target number, it will display "You guessed too low, Try again".
   - If your guess is higher than the target number, it will display "You guessed too high, Try again".
   - If your guess is correct, it will display "You guessed it right in [number of attempts] times, number was [target number]".

## Code Explanation

- The `rand` crate is used to generate a random number between 1 and 100. It is imported using `use rand::Rng;`.
- The `std::io` module is imported to handle user input using `std::io::stdin()`.
- The program generates a random number using `rand::thread_rng().gen_range(1, 101)`.
- The program sets up a `while` loop to repeatedly prompt the user for guesses until they guess the correct number.
- Inside the loop, the user's input is read and stored in a `String` variable using `io::stdin().read_line(&mut user_input)`.
- The user's input is parsed into an `i32` using `user_input.trim().parse()`.
- The program uses a `match` statement to compare the user's input with the target number and provides appropriate feedback based on the comparison.
- If the user's guess matches the target number, the loop is terminated by setting `start` to `false`.

Feel free to modify this code and customize it according to your needs. Have fun playing the "Guess the Number" game!
