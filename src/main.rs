extern crate rand; //new-2

use std::io;
use std::cmp::Ordering; //new-3
use rand::Rng;//new-2

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1,101);//new-2
    //println!("the secret number is {}", secret_number); //new-2

    loop{ //new-4
        println!("Please input your guess.");
        
        //create string
        let mut guess = String::new();

        //get number
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        //new-3.5: string to real-num, shadow prev "guess"
        let guess: u32 = match guess.trim().parse() { //new-4.6 eval Result from .parse
            Ok(num) => num, //new-4.6 Handle error by non-number values
            Err(_) => continue,
        };
        //.expect("Please type a number!");

    
    println!("You guessed: {}", guess);
    //new-3
    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
            break; //new-4.5
        }
    }
    }
}
