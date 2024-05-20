## Enum 정의

structs이 관련된 여러 field값들을 그룹화 해 준다면 enum은 가능한 선택지의 값들을 묶어준다. 
enum을 사용하기 좋은 예시로 IP 주소를 들 수 있다. IP 주소는 V4, V6 두가지 버전이 있다.  또한 그 두가지 선택지만 존재한다. 하지만 어떤한 IP도 V4 버전이면서 동시에 V6버전일 수 없다. 이럴 때 enum을 유용하게 사용할 수 있는데 그 이뉴는 enum은 여러 선택지의 값 중 단 하나만 선택할 수 있기 때문이다. 

``` rust

enum IpAddrKind {
    V4,
    V6,
}

let four = IpAddKind::V4;
let six = IpAddKind::V6

```

우리는 enum 각각의 값을 인스턴스로 생성할 수 있고, 더블 콜론으로 구분하여 표현한다.

``` rust 

enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr:V6(String::from("::1"));


```

또한 위와 같이 enum에 직접 데이터를 붙여줄 수도 있다. 그리고 함수가 되어 String argument를 받아 IpAddr 타입을 리턴 할 수도 있다. 

``` rust 

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);

let loopback = IpAddr:V6(String::from("::1"));


```

추가로 위와 같이 여러 값을 받아 enum 타입을 리턴할 수도 있다. 

``` rust 

struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

```

struct을 통해 생성한 데이터 타입또한 argument로 받아 enum 타일을 리턴할 수 있다. 

### Option Enum의 사용 

Option또한 Enum 타입의 한 종류인데 Rust는 null값을 가지고 있지 않다 그러나 Option을 null 대신 활용할 수 있다. Rust의 None 또한 Oprion enum의 한 종류이다. 

``` rust 

enum Option<T> {
    None,
    Some(T),
}

let some_number = Some(5);
let some_char = Some('e');

let absent_number: Option<i32> = None;

```

Option은 enum은 많이 사용되는 만큼 앞에 Option::을 붙일 필요 없이 None과 Some을 바로 사용할 수 있다. Option<T>는 null을 사용하는 것보다 유용하다. 그 이유는 T와 Option<T>를 다른 타입으로 취급할 수 있기 때문이다. 

Rust에서 T값과 Option<T> 값의 연산은 허용하지 않고 컴파일 에러가 난다. 이렇게 null 값이 존재할 가능성을 애초에 배재하는 것은 개발자가 코드에 확신을 갖게 하는데에 도움이 된다. 