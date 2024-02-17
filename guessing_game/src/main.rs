use std::io;  //using input/output library form the standard library
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Guess a number b/w 1-100:");

    loop {
        let mut guess = String::new();

        io::stdin()
        .read_line(&mut guess) 
        .expect("failed to read line"); 
        
        // let guess: u32 = guess.trim().parse().expect("You entered something that is logically crap, please enter a number instead.");

        // Parse the user's input into an unsigned 32-bit integer (u32)
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {println!("You entered something that is logically crap, please enter a number instead.");
            continue;}
        };
        
        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("\ntoo big.. "),
            Ordering::Less => println!("\nToo small.. "), 
            Ordering::Equal => {
                println!("you winnnnn");
                break;
            }
        }
        println!("guess it again: ")
    }
}