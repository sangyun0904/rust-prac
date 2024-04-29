use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // range expression we’re using here takes the form start..=end and is inclusive on the lower and upper bounds.
    // ex) 1..=100 -> a number between 1 and 100.
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop {
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

        // Rust has a strong, static type system. However, it also has type inference.
        // we want to convert the String the program reads as input into a real number type so we can compare it numerically to the secret number. 
        // But wait, doesn’t the program already have a variable named guess? It does, but helpfully Rust allows us to shadow the previous value of guess with a new one.
        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        println!("You guessed: {guess}");

        // A match expression is made up of arms.
        // An arm consists of a pattern to match against, and the code that should be run if the value given to match fits that arm’s pattern.
        // Rust takes the value given to match and looks through each arm’s pattern in turn.
        match guess.cmp(&secret_number) {
            // Ordering type is another enum and has the variants Less, Greater, and Equal.
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Too win!");
                break;
            }
        }
    }
}
