use std::io; // input output library (io comes from standard library), "stad::io" can be used to accpt user input

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new(); // variable is immutable by default, to make variable mutable, put "mut", new() function create a new,empty string 

    io::stdin()
        .read_line(&mut guess) // & -> reference, read_line return "Result" value, which its variants are Ok and Err
        .expect("Failed to read line"); 
    
    println!("You guessed: {guess}");
}
