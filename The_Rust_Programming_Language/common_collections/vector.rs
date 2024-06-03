fn main() {

    let v: Vec<i32> = Vec::new();
    let w = vec![1, 2, 3];


    // add mut to update vector 
    let mut u = Vec::new();
    u.push(5);
    u.push(6);
    u.push(7);
    u.push(8);


    let third: &i32 = &u[2];
    println!("the third element is {third}");

    let third: Option<&i32> = u.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    let fifth: Option<&i32> = u.get(4);
    match fifth {
        Some(fifth) => println!("The fifth element is {fifth}"),
        None => println!("There is no fifth element."),
    }
}