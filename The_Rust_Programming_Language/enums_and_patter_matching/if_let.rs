fn main() 
{
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum if configured to be {}", max),
        // None 값으로 아무 것도 하지 않을 때 match 를 사용하면 _ => () 라는 보일러 플레이트 코드가 발생한다.
        _ => (),
    }
    // 이 코드를 다음과 같이 if let 키워드를 사용하여 바꿀 수 있다. 
    // 그러나 코드는 줄일 수 있지만 match 가 제공하는 exhaustive cheking (모든 케이스에 대해 검사하는) 기능은 사용하지 않게 된다. 
    if let Some(max) = config_max {
        println!("The maximum if configured to be {}", max)
    }

    // 즉, if let 키워드를 match에서 한가지 패턴만 확인할 때 사용하는 syntax sugar로 
    // 추가로 else 문을 활용하여 나머지 패턴에 대한 코드도 추가할 수 있다. 
    let mut count = 0; 
    let coin = Coin::Quarter(UsState::Alaska);
    // match coin {
    //     Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    //     _ => count += 1,
    // }

    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}