use std::io; // input output library (io comes from standard library), "stad::io" can be used to accept user input
use rand::Rng;
use std::cmp::Ordering; // bring a type called std::cmp::Ordering from the standard library, the Ordering type is an enum and has the variants Less, Greater, and Equal

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");
    loop{
        println!("Please input your guess.");

        let mut guess = String::new(); // variable is immutable by default, to make variable mutable, put "mut", new() function create a new,empty string 

        io::stdin()
            .read_line(&mut guess) // & -> reference, read_line return "Result" value, which its variants are Ok and Err
            .expect("Failed to read line"); 
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // _, is a catchall value, we want to match all Err value
        };
        // convert the String input into a real number type (an unsigned 32-bit number) so we can compare it numerically
        // trim method: on a String instance will eliminate any whitespace at the beginning and end
        // parse method: convert string to another type
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) { // cmp is a method to compare two value
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("That's right! You win!");
                break;
            }
        } // match is made up of arms, an arm consists of a pattern to match against, code runs if the value given to match fits that arm’s pattern in turn
    }    
}
