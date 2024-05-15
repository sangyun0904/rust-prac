fn main() 
{
    let s1 = String::from("hello world");
    let s2 = &s1[..];

    println!("{}, {}", s1, s2);
}