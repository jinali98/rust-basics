use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("<--- Guess the Number Game --->");

    let secret_number = rand::thread_rng().gen_range(1..=10);

    println!("Secret number is : {secret_number}");

    println!("Input your guess");

    // mutable variable 
    let mut guess = String::new();


    // this is to get the user input and store that in the guess var
    // & is used to refer the same var instead of creating copies of it
    io::stdin()
        .read_line(&mut guess)
        .expect("Cannot read the input");

    // shadowing allow to re use the same variable name to conver the value of guess from 
    // string type to u32 number type.
    // shadowning is diff from mutating.
    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    println!("You guessed: {guess}");


    match guess.cmp(&secret_number) {
        Ordering::Less =>  println!("too small"),
        Ordering::Greater => println!("too big"),
        Ordering::Equal => println!("correct number"),
        
    }

}
