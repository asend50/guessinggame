/*
By: <Asen Doiron>
Date: 2026-04-08
Program Details: <The purpose of this program is to allow the user to guess a number from 1 to 100 until they guess the correct number, then the program will ask if the user wants to play again>
*/

use rand::Rng;
use std::io;

fn main() {
    let mut rng = rand::rng();
    let mut random_int: i32 = rng.random_range(1..=100);
    println!("Type a number between 1 and 100:");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let number: i32 = input.trim().parse().expect("Please type a number!");
        let diff = (number as i32 - random_int as i32).abs();

        if diff >= 50 {
            println!("FREEING, your guess was:{}", number);
        } else if diff >= 25 {
            println!("COLD, your guess was:{}", number);
        } else if diff >= 15 {
            println!("COOL, your guess was:{}", number);
        } else if diff >= 10 {
            println!("WARM, your guess was:{}", number);
        } else if diff >= 5 {
            println!("HOT, your guess was:{}", number);
        } else if diff > 0 {
            println!("BOILING, your guess was:{}", number);
        } else if diff == 0 {
            println!("You guessed correctly! The number was: {}", number);
            println!("Would you like to play again? (y/n)");

            let mut newgame = String::new();
            io::stdin().read_line(&mut newgame).expect("Failed to read line");

            let newgame = newgame.trim();
            if newgame.trim() == "y" || newgame.trim() == "Y" {
                random_int = rng.random_range(1..=100);
                println!("Type a number between 1 and 100:");
            }

            if newgame.trim() == "n" || newgame.trim() == "N" {
                println!("Thanks For Playing!");
                break;
            }
        }
    }
}
