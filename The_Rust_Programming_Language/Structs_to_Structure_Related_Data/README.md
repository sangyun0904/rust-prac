# Struct

- struct는 tuple과 비슷하게 여러 값들을 들고 있을 수 있다. 다만 struct의 값들은 다른 타입들을 가질 수 있다. 또한 struct의 각 값들은 이름을 가진다. 그로인해 값을 가지고 올 때 인덱스를 알고있을 필요가 없다. 

- struct을 정의하고 나서 그 struct를 사용하기 위해선 instance를 생성해야한다. 인스턴스는 인스턴스 이름 후에 중괄호를 열고 struct안의 각 값들을 넣는 방식으로 생성한다. 

- 인스턴스의 값을 가지고 오기 위해선 dot notation을 사용한다. Dot notation을 통해 값을 변경할 수도 있다. 단 값을 바꾸기 위해서는 전체 인스턴스가 mutable해야 한다. Rust는 인스턴스의 특정 필드만 mutable 한 것을 허락하지 않는다. 함수에서 마지막에 인스턴스를 생성함으로써 인스턴스를 리턴 할 수도 있다. 

- 기존 인스턴스를 조금 변경하여 새로운 인스턴스를 생성할 수 있다. Struct update syntax라고 부른다. 

- Rust는 tuple과 비슷하게 생긴 struct즉 tuple structs라는 개념이 있다. Tuple struct는 struct의 이름은 잇지만 struct내의 fileld의 이름은 존재하지 않는 경우이다. Struct tuple은  tuple 전체에 이름을 부여하고 싶을 때 유용하다. 

- 아무것도 정의하지 않는 struct또한 선언할 수 있다. Unit-like struct라고 부르며 나중에 어떤 특성을 부여해야 하지만 현재 아무런 정보를 가지고 있지 않을 때 유용하게 사용될 수 있다. 
