fn main() {
    // floating point
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    // boolean
    let t = true;
    let f : bool = false;
    // character: it can represent a lot more than just ASCII
    let c = 'z';
    let z: char = 'z';
    let heart_eyed_cat = '😻';
    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (a, b, c) = tup;
    println!("The value of b is: {b}");
    // use period with index to access the value in tuple
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    println!("five hundred is {five_hundred}, and six point four is {six_point_four}");
    // array: element of an array must have the same type, array is fixed length
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let [d, e, f, g, h] = arr;
    println!("array: {d}, {e}, {f}, {g}, {h}");
    let first = arr[0]; // access elements of an array using indexing
    let second = arr[1];
    println!("first and second index of above array is: {first} and {second}");
}
