//Trying to foloow rust doc for this.

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    println!("Hello, Welcome to gussing Game.");

    let secret_number = rand::thread_rng().gen_range(1,101);
    //println!("The Secret number is: {}",secret_number);

    loop{
        println!("\nPlease input your guess: ");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read input.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
            //.expect("Plesae type a number");
        };

        //println!("You Guessed - {}",guess);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}
