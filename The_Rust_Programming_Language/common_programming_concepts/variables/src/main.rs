const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    //Compiling variables v0.1.0 (file:///projects/variables)
    //error[E0384]: cannot assign twice to immutable variable `x`
    //let x = 5;

    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scoe is: {x}")
    }
    println!("The value of x is: {x}")
}

