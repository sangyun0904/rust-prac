fn main() {
    // i32 : integer
    let x: i32 = 5; // initialized and used 
    // let y: i32; // uninitialized and unused! -> warning 
    let _y: i32; // uninitialized and unused! -> no warning!!!
    let mut z: i32 = 1;
    // x += 2; // compile error! -> annot assign twice to immutable variable `x`
    z += 2; 

    assert_eq!(x, 5); 
    assert_eq!(z, 3);
    println!("Success!");

    // println!("{}, world!", h); // compile error! -> cannot find value `h` in this scope
    define_h();
}

fn define_h() {
    let h: &str = "hello";

    println!("{}, world!", h);
}