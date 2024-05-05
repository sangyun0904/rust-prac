fn main()
{
    let condition = true;

    // Because if is an expression, we can use it on the right side of a let statement
    // to assign the outcome to a variable
    let number = if condition { 5 } else { 6 };

    // error : expected integer, found '&str
    // let number = if condition { 5 } else { "six" };
    
    println!("The vlaue of number is: {number}");



    // You might also need to pass the result of that operation out of the loop to the rest of your code. 
    // To do this, you can add the value you want returned after the break expression you use to stop the loop; 
    // that value will be returned out of the loop so you can use it, as shown here:
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");


    // If you have loops within loops, break and continue apply to the innermost loop at that point.
    // You can optionally specify a loop label on a loop that you can then use with break or continue
    // to specify that those keywords apply to the labeled loop instead of the innermost loop
    // Loop labels must begin with a single quote ('). 
    let mut count = 0; 
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    // count = 0
    // remaining = 10 
    // remaining = 9 
    // count = 1 
    // remaining = 10 
    // remaining = 9 
    // count = 2 
    // remaining  = 10

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    // rev() : reverse
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!!");
}