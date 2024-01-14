use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Guess the number! ");
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("only numbers are allowed \n\n try again");
                continue;
            }
        };
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low!!!!!!! \n\n lets try again, shall we \n\n"),
            Ordering::Equal => {
                println!("YOU WIN!!!!!!!");
                println!("Secret number is: {secret_number}");
                break;
            }
            Ordering::Greater => println!("Too HIgh!!!!!! \n\n lets try again, shall we \n\n"),
        }
    }
}
