use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    
    println!("Guessing Game!");

    let random_num: u8 = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is {random_num}");

    loop {
        println!("Enter a number");

        let mut guess = String::new();
    
         io::stdin().read_line(&mut guess)
         .expect("Invalid input");
    
        let guess = match guess.trim().parse::<u8>() {
            Ok(n) => n,
            Err(_e) => continue
        };
    
        match guess.cmp(&random_num) {
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Greater => {
                println!("Too high");
            }
            Ordering::Less => {
                println!("Too low");
            }
        };
    }

}
