// Putting the specifier :? inside the curly brackets tells println! we want to use an output format called Debug. The Debug trait enables us to print our struct in a way that is useful for developers so we can see its value while we’re debugging our code.
// Rust does include functionality to print out debugging information, but we have to explicitly opt in to make that functionality available for our struct. To do that, we add the outer attribute #[derive(Debug)] just before the struct definition
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main()
{
    let width1 = 30;
    let height1 = 50;

    println!(
        "the area of the rectangle is {} square pixels.",
        area1(width1, height1)
    );

    let rect1 = (30, 50);
    println!(
        "the area of the rectangle is {} square pixels.",
        area2(rect1)
    );

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "the area of the rectangle is {} square pixels.",
        area3(&rect2)
    );

    println!("rect2 is {:?}", rect2);
    // We can put dbg! around the expression 30 * scale and, because dbg! returns ownership of the expression’s value,
    dbg!(&rect2);

    println!(
        "the area of the rectangle is {} square pixels.",
        rect2.area()
    );

    let rect3 = Rectangle {
        width: 40, 
        height: 60,
    };

    println!(
        "rect3 can hold reac2 : {}",
        rect3.can_hold(&rect2)
    );

    let square1 = Rectangle::square(20);
    println!(
        "square1 area size is {}\nsquare1 can hold rect2 : {}",
        square1.area(),
        square1.can_hold(&rect2)
    );

}


fn area1(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimensiolns: (u32, u32)) -> u32 {
    dimensiolns.0 * dimensiolns.1
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}