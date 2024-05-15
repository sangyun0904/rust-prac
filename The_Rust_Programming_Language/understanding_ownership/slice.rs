fn main()
{
    let mut s = String::from("hello world");
    let first_s = first_word(&s);
    println!("{}", first_s);
    
    s.clear();
    println!("{}", s);

    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{}, {}", hello, world);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}