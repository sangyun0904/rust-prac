fn main() 
{
    let mut s = String::from("hello");

    let r1 = &mut s;

    // error 1
    // let r2 = &mut s;
    //          ^^^^^^ second mutable borrow occurs here

    // error 2
    // println!("{}, {}", r1, s);
    // -----------------------^-
    // |                      |
    // |                      immutable borrow occurs here
    // mutable borrow later used here

    print!("{}", r1);
    println!(", {}", s);
}