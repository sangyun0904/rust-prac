## Struct

- struct는 tuple과 비슷하게 여러 값들을 들고 있을 수 있다. 다만 struct의 값들은 다른 타입들을 가질 수 있다. 또한 struct의 각 값들은 이름을 가진다. 그로인해 값을 가지고 올 때 인덱스를 알고있을 필요가 없다. 

- struct을 정의하고 나서 그 struct를 사용하기 위해선 instance를 생성해야한다. 인스턴스는 인스턴스 이름 후에 중괄호를 열고 struct안의 각 값들을 넣는 방식으로 생성한다. 

- 인스턴스의 값을 가지고 오기 위해선 dot notation을 사용한다. Dot notation을 통해 값을 변경할 수도 있다. 단 값을 바꾸기 위해서는 전체 인스턴스가 mutable해야 한다. Rust는 인스턴스의 특정 필드만 mutable 한 것을 허락하지 않는다. 함수에서 마지막에 인스턴스를 생성함으로써 인스턴스를 리턴 할 수도 있다. 

- 기존 인스턴스를 조금 변경하여 새로운 인스턴스를 생성할 수 있다. Struct update syntax라고 부른다. 

- Rust는 tuple과 비슷하게 생긴 struct즉 tuple structs라는 개념이 있다. Tuple struct는 struct의 이름은 잇지만 struct내의 fileld의 이름은 존재하지 않는 경우이다. Struct tuple은  tuple 전체에 이름을 부여하고 싶을 때 유용하다. 

- 아무것도 정의하지 않는 struct또한 선언할 수 있다. Unit-like struct라고 부르며 나중에 어떤 특성을 부여해야 하지만 현재 아무런 정보를 가지고 있지 않을 때 유용하게 사용될 수 있다. 


## Method 

메서드는 함수와 비슷하지만 struct 안에 정의 되어 있다는 점이 다르다. 그리고 모든 메서드의 첫번째 파라미터는 struct의 instance를 의미하는 self가 들어간다. 

Struct 내의 field명과 같은 이름의 메서드를 생성할 수 있다. 예를 들어 rectangle 의 width가 0보다 큰 값을 가지고 있는 지 width() 메서드를 작성할 수 있다. 

주로 field와 같은 이름의 메서드를 작성할 때 이를 getter함수로 사용하는 경우가 많다. getter함수는 struct의 fileld는 private하게 유지 하면서 갓을 불러오는 메서드만 외부에 노출하여 field를 read only 하게 만들어 주고 싶을 때 사용하는 함수이다. 다른 언어들은 이 getter함수를 자동으로 제공하는 언어도 있지만 rust는 그렇지 않다. 

Impl 내의 self 파라미터를 받지 않는 함수를 associated function이라고 부른다. 이 함수는 메서드가 아니다. (&self 파라미터를 받지 않기 때문에) String::from 의 from 또한 associated function이다. 주로 이 함수는 생성자 함수를 만들 때 사용한다. 

struct은 또한 여러개의 impl을 가질 수 있다. impl을 여러개로 나눌 일은 많지 않지만 나중에 generic type과 traits를 다룰 때 용이할 수 있다. 