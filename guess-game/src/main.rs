use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("~~~~Guess game~~~~");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    // comment this line out for more fun! ;)
    //println!("The secret number is: {}", secret_number);

    loop {
        let mut guess = String::new();

        println!("enter your input");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to get input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guess: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("No.. it's way less ☹️"),
            Ordering::Greater => {
                println!("Nope.. you are going way above the secret number ☹️")
            }
            Ordering::Equal => {
                println!("You guessed it right! You won!! 🎉");
                break;
            }
        }
    }

    println!("~~~~Bye bye~~~~");
}
