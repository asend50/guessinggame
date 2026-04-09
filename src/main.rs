/*
By: <Asen Doiron>
Date: 2026-04-08
Program Details: <The purpose of this program is to allow the user to guess a number from 1 to 100 and see if they are correct.>
*/

use rand::Rng;
use std::io;

fn main() {
    let mut rng = rand::rng();
    let random_int: i32 = rng.random_range(1..=100);
    println!("Welcome to the guessing game that picks a random number between 1 and 100, so you can guess it until you get it right!");
loop {
    let mut input = String::new();
    println!("Guess a number from 1 to 100:");
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
        }else if diff >= 0 {
            println!("BOILING, your guess was:{}", number);
        if number == random_int {
        println!("You guessed correctly! The number was: {}", random_int);
        break;
    } else {
        }}
    }}

