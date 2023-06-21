fn main() {
    println!("Hello, cargo!");
}
// cargo new hello_cargo  
// cd hello_cargo
// method 1:
// in the new package (hello_cargo), run: "cargo build", it creates an executable file in target/debug/hello_cargo
// run ./target/debug/hello_cargo
// method 2: or "cargo run" to compile

// cargo check to make sure it's executable (and it's quicker without producing a binary )

// cargo build --release (two different profiles: one for development, when you want to rebuild quickly and often, and another for building the final program you’ll give to a user that won’t be rebuilt repeatedly and that will run as fast as possible. )