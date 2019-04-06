use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("~~~~Guess game~~~~");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);

    loop {

        let mut guess = String::new();

        println!("enter your input");

        io::stdin().read_line(&mut guess)
            .expect("Failed to get input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("It's less"),
            Ordering::Greater => println!("It's more"),
            Ordering::Equal => {
                println!("You guessed it right! You won!!");
                break;
            }
        }
    }
}