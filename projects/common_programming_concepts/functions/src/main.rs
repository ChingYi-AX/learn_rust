fn main() {
    println!("Hello, world!");

    print_labeled_measurement(8, 'c');

    let number = five();
    let plus_num = plus_one(number);
    println!("we have {number} and plus one is {plus_num}.");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    // you must declare the type of each parameter
    // statement: perform some action and do not return value 
    let _y = 6;
    // expression: can be part of statement, evaluate to a resultant value (e.g., bracket after y =)
    let y = {
        let x =3;
        x + 1
    };
    println!("The measurement is: {value}{unit_label}. The return value is: {y}");
}

fn five() -> i32 {
    // must declare the type of return value after an arrow (->)
    5 // 5 with no semicolon because it’s an expression whose value we want to return
}

fn plus_one(x: i32) -> i32 {
    x + 1 // add semicolon(;) make expression into statement and cause error, for statement doesn't return value
}