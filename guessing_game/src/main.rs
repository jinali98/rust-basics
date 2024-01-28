use std::io;

fn main() {
    println!("<--- Guess the Number Game --->");

    println!("Input your guess");

    // mutable variable 
    let mut guess = String::new();


    // this is to get the user input and store that in the guess var
    // & is used to refer the same var instead of creating copies of it
    io::stdin()
        .read_line(&mut guess)
        .expect("Cannot read the input");

    println!("You guessed: {guess}");

}
