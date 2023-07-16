use std::io;
use rand::Rng;

fn main() {
    println!("1. exercise of condition:");
    condition();

    println!("\n2. exercise of simple loop:");
    simple_loop();

    println!("\n3.exercise of multiple loops:");
    multiple_loops();

    println!("\n4.exercise of while loop:");
    while_loop();
    
    println!("\n5.exercise of for loop:");
    for_loop();

    println!("\n6.exercise of safe for loop:");
    safe_for_loop();
}

fn condition() {
    // Condition
    println!("Enter a number bigger than 10:");
    let mut entered_number = String::new(); 
    io::stdin()
    .read_line(&mut entered_number) // & -> reference, read_line return "Result" value, which its variants are Ok and Err
    .expect("Failed to read line"); 

    let number: u32 = entered_number.trim().parse().unwrap();
    let generated_number = rand::thread_rng().gen_range(1..=10);
    let allow_radom = if number > 50 {true} else {false}; // each "arm" of if/else must be same type
    let generated_number = if allow_radom { generated_number } else { 2 };

    if number % generated_number == 0 {
        println!("your number can be divided by {generated_number} ✅")
    } else if number % generated_number != 0 {
        println!("your number can't be divided by {generated_number} 🚫")
    }

}

fn simple_loop() {
    // repetition with loops
    let mut counter = 0;

    loop {
        counter += 1;
        if counter == 10 {
            break println!("the number is already added to 10!")
        }
    };

}

fn multiple_loops(){
    let mut count = 0;
    'counting_up: loop { // outer loop, count up to count == 2
        println!("count = {count}");
        let mut remaining = 10;

        loop { // inner loop: counts down from 10 to 9
            println!("remaining = {remaining}");
            if remaining == 9 {
                break; // break that doesn’t specify a label will exit the inner loop only
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("end count = {count}");
}

fn while_loop(){
    // method 1
    let mut number = 3;
    while number !=0 {
        println!("{number}!");

        number -= 1;
    }
    println!("Liftoff!!!");

    // method 2
    let number = [3, 2, 1];
    let mut index = 0;
    while index <3 {
        println!("{}!", number[index]);
        index += 1;
    }
    println!("Liftoff!!!");
}

fn for_loop(){
    let numbers = [10, 20, 30, 40];
    for number in numbers{
        println!("the value of the collection(array) is: {number}");
    }
}

fn safe_for_loop(){
    for number in (1..4).rev() {
        println!("{number}!")
    }
    println!("Liftoff!!!")
}