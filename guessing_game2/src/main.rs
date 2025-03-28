use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("guess the number.. ");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("the secret number is {}", secret_number);

    println!("please input your guess: ");

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

        println!("you guessed: {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Equal => {println!("you won"); break;},
            Ordering::Greater => println!("too big"),
        }
        
        
    }
}
