// Simple CLI Based Number Guessing Game


use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*;

fn main(){
    let secret_num = rand::thread_rng().gen_range(1..=100);
    println!("Number Guessing Game....");

    loop{
        let mut guess = String::new();
        println!("{}","Enter Your Guess :".purple());
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");
        let guess:u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("{}","Alert : Enter number only".red());
            continue;
           },
        };

        println!("You Guessed : {guess}");

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("{}","Too small!".bright_yellow()),
            Ordering::Greater => println!("{}","Too Big!".bright_yellow()),  
            Ordering::Equal => {
                println!("{}","You Win".green());
                break;
            }    
        }
    }

}