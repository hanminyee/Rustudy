/*
기존에 존재하는 인스턴스에서 몇가지 필드의 값만 수정한 상태의 새 구조체 인스턴스가 필요할 때가 있다고 함(그게 언젠데 ㅅㅂ).
이럴 때는 구조체 갱신 문법을 쓰면 됨.
*/
struct User{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
fn main(){
    let mut user1 = User{
        email: String::from("hanmin0786@gmail.com"),
        username: String::from("hanminyee"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User{
        email: String::from("hanmin0786@handong.ac.kr"),
        username: String::from("ParkHanmin"),
        acrive: user1.active,
        sign_in_count: user1.sign_in_count,
    }; //이런식으로 원래 있던거 쓸 수 있음.

    //근데 더 간단하게 하는 방법이 존재함
    let user3 = User{
        email: String::from("jinju@gmail.com"),
        username: String::from("jinju"),
        ..user1    //.. 문법은 따로 명시하지 않은 친구는 그 뒤의 인스턴스의 값을 따라가라는 이야기임.
    };
}

/*
튜플 구조체라는 것도 존재함.
얘는 튜플과 유사하게 생긴 구조체인데, 튜플 구조체는 구조체에는 이름을 부여하지만, 각각의 필드에 이름을 부여하지 않고 타입만 지정함.
튜플 구조체는 일반적인 구조체처럼 필드에 이름을 부여하는 것이 귀찮거나 필요없거나 튜플 자체에만 이름을 부여해서 다른 튜플들과 다른 타입으로 구분하고자 할 때 유용함.
튜플구조체를 정의하려면 struct키워드와 구조체 이름, 각각의 타입을 차례대로 나열하면 됨.
*/
fn tupleStruct(){
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0,0,0);
    let origin = Point(0,0,0);
    /*
    여기서 우리가 주의깊게 봐야할 점은 "Origin"과 "Black"은 다른 타입이라는 것이다.
    왜냐면 이 둘은 서로 다른 튜플 구조체의 인스턴스이기 때문임.
    Color 와 Point는 같은 타입들을 포함하지만, 서로 다른 타입임.
    그래서 파라미터로 전달하거나 할 때 Color타입을 파라미터로 사용하는 함수에 Point타입을 전달할 수 없음.   <-- 이점이 튜플과 다른점. 나머지는 똑같이 작동
     */
}

/*
러스트에는 필드가 하나도 없는 구조체도 선언할 수 있음.
이런 구조체를 "유사 구조체"라고 함.
얘는 어떤 타입의 트레이트를 구현해야 하지만, 타입에 저장할 데이터는 딱히 없을 때 쓸 수 있음. 근데 이건 10장에서 더 자세히 다를 예정.

그리고 왜 &str 문자열 슬라이스 타입이 아니라 String타입을 사용하는지 의문이 생길 수 있는데, 이건 "수명"이라는 개념과 관련되어있음.
이건 그래서 10장에 가서 더 자세히 살펴볼 예정.
그래서 위에 코드에서
let user1 = User{
    email: "someone@gmail.com",
    username: "someuser",
    active: true,
    sign_in_count: 1,
};
이렇게 하고 컴파일 돌리면 에러가 뜸. 문자열 슬라이스 타입은 수명을 지정해줘야 하기 때문임. 이건 10장가서 더 보도록 하자
*/
