fn main() {
    // variables
    let mut x = 5; // make variables mutable to reassign the value
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    
    // constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // constants aren’t just immutable by default, convention is uppercase
    println!("The value of constant THREE HOURS IN SECONDS is: {THREE_HOURS_IN_SECONDS}");

    // shadowing
    let y = 5;
    let y = y + 1; // won't work without "let"
    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }
    println!("The value of y is: {y}")
}
