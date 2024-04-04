fn main() {
    // This is an example of a line comment.

    // println!("Hello World!");

    /*
        This is another type of comment, a block comment. In general,
        line comments are the recommended comment style. But block comments 
        are extreamly useful for terporarily diabling chunk of code.

        println!("I'm a Rustacean!");
     */

    let x = 5 + /* 90 + */ 5;
    println!("Is 'x' 10 or 100? x = {}", x);
}