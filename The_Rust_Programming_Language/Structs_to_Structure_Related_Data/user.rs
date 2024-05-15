struct User {
    active: bool,
    username: String,
    email: String, 
    sign_in_count: u64,
}

fn main() 
{
    let mut user1 = User {
        active: true,
        username: String::from("someusername124"),
        email: String::from("sumone@example.com"),
        sign_in_count: 1,
    };

    println!("{}", user1.username);
    user1.username = String::from("Sangyoon");
    println!("{}", user1.username);

    let user2 = buildUser(String::from("Yoon"), String::from("yoon@gmail.com"));

    println!("{}", user2.email);

    // struct update syntax를 활용하면 코드를 확 줄일 수 있다. 
    // let user3 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };
    let user3  = User {
        email: String::from("another@example.com"),
        ..user1
    };

    println!("user3 username : {}, email : {}", user3.username, user3.email);

    // user1.username의 ownership이 user3로 옮겨갔기 때문에 user1.usernme을 사용할 수 없다.
    // println!("user1 : {}", user1.username);
    //                        ^^^^^^^^^^^^^^ value borrowed here after move
}

fn buildUser(username: String, email: String) -> User {
    User {
        active: true,
        // 파라미터와 strut field명이 같을 때 굳이 이름을 두번 반복해서 적지 않아도 된다. 이를 init shorthand syntax이라고 부른다. 
        // username: username
        username,
        email,
        sign_in_count: 1
    }
} 