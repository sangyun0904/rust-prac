fn main() {
    println!("Hello, world!");

    another_functions(5);
    print_labeled_measurement(5, 'h');

    /*
    * Because Rust is an expression-based language, this is an important distinction to understand. 
    * Other languages don’t have the same distinctions, so let’s look at what statements and expressions are and how their differences affect the bodies of functions.

    * Statements are instructions that perform some action and do not return a value.
    * Expressions evaluate to a resultant value. Let’s look at some examples.
    */

    // let x = (let y = 6); // error (let y = 6) is a statement not expression
    // This is expression
    let u = {
        let v = 3;
        v + 1
    };

    println!("The value of u is: {u}");

    let fiv = five();
    println!("The value of fiv is: {fiv}");
    
    let a = plus_one(10);
    println!("The value of a is: {a}");
    
}

// rust doesn't care where you define your functions.
// In function signatures, you must declare the type of each parameter
fn another_functions(x: i32) {
    println!("Another function.");
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

