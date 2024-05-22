fn main()
{
    println!("3 Dimes = {} Cents", 3 * value_in_cents(Coin::Dime));
    println!("5 Quarter = {} Cents", 5 * value_in_cents(Coin::Quarter));

    let mut dice_roll = 3;

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn add_fancy_hat() {
    println!("add_fancy_hat!");
}
fn remove_fancy_hat() {
    println!("add_fancy_hat!");
}