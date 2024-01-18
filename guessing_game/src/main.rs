
use std::io;
use std::cmp::Ordering;

use rand::Rng;

fn main() {

    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=100);
        
    println!("Secret number: {secret_number}");  

    loop {
        println!("input your guess");
        
        let mut guess = String::new();
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        
        //let guess: u32 = guess.trim().parse().expect("Enter a number");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Error... Try again:");
                continue;
            }
        };
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("smaller"),
            Ordering::Greater => println!("bigger"),
            Ordering::Equal => {
                println!("win");
                break;
            }
        }    
        
        println!("You guessed: {guess}");
    }       
}