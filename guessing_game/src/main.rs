use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    // let : create variable, immutable by default - the value won't change
    // mut : makes the variable mutable
    let mut guess = String::new(); 

    io::stdin()
        // passing inpu value to guess variable
        // read_line puts whatever the user enters into the string we pass to it, but it also returns a Result value. Result is an enumeration
        // Result's ’s variants are Ok and Err. The Ok variant indicates the successful. 
        // The Err means failed, and Err contains information about how or why the operation failed.
        .read_line(&mut guess)
        // If this instance of Result is an Err value, expect will cause the program to crash and display the message that you passed as an argument to expect.
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
