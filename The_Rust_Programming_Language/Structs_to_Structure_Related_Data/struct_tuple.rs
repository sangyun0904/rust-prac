struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() 
{
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let red = Color(255, black.1, black.2);
    // 같은 데이터 타입이라도 다른 tuple struct의 field 값을 파라미터로 넣을 수 없다. 
    // let origin2 = Point(Color.0, 0, 0);
    //                           ^ unknown field
}