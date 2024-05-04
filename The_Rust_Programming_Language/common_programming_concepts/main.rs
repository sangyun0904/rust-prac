fn main()
{
    // tuple : collection of values
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("The value of x, y, z is: {x}, {y}, {z}");
    println!("five hundred, six point four, one: {five_hundred}, {six_point_four}, {one}");

    // array : every element of array must have same type 
    // array has fixed length 
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let b = [3; 5];

    let first = a[0];
    let third = b[0];

    println!("first : {first}, third: {third}");

    // useful when you know the number of elements will not need to change 
    let months = ["January", "Feburary", "March", "April", "May", "June", "July",
                  "August", "September", "October", "November", "December"];
    let sep = months[8];
    println!("September : {sep}");
}