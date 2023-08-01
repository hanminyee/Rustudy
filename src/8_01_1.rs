/*
구조체는 튜플과 유사하다.
다른 종류의 데이터들이 모여있다는 특징이 같다. 
근데, 튜플과는 다르게 각 데이터에 다른 이름들을 부여할 수 있어서 값의 의미를 더 분명히 할 수 있슴.
그리고, 각 데이터에 이름이 있어서 튜플보다 더 유연함. 왜와이?
참조할 데이터를 가리키거나 인스턴스의 값을 읽을 때 데이터의 순서에 의존할 필요가 없기때문(그냥 이름으로 부르면 만사 오케이라는;;).

선언은 아래와 같이 C와 매우 유사함. 그냥 무지성 struct 갈기고 뒤에 구조체 이름이랑 안에 내부 요소 넣으면 끝임. 내부는 "필드(field)"라고 부름
주의해야할 것은 콤마로 구분된다는거 정도?
*/
struct User{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

/*
실제 선언할 때는 이런 과정을 거쳐야함.
이것 또한 콤마로 구분된다는거 주의
*/
fn main(){
    let mut user1 = User{
        email: String::from("hanmin0786@gmail.com"),
        username: String::from("hanminyee"),
        active: true,
        sign_in_count: 1,
    };

    //변경하고싶으면 (인스턴스).(변수명)과 같이 "."사용하면 됨.
    user1.email = String::from("hanmin0786@handong.ac.kr");
    println!("{} : {}   {}  {}", user1.username, user1.email, user1.active, user1.sign_in_count);

    let user2 = modified_build_user("jinju.gmail.com".to_string(), "jinju lee".to_string());    //.to_string()메소드는 &str타입을 String으로 변경해줌(다른 타입도 가능한지는 잘 모름;)
    println!("{} : {}   {}  {}", user2.username, user2.email, user2.active, user2.sign_in_count);
}

/*
근데 러스트에서 구조체의 인스턴스는 불변이 아니다. 가변이다.
또한 러스트는 구조체의 몇몇 필드만을 가변 데이터로 표시하는 것을 지원하지 않음.
그래서 구조체의 새로운 인스턴스도 함수를 이용해 생성하는 것이 가능.
아래의 함수를 보자.
*/
fn build_user(email: String, username: String) -> User{
    User{
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    } //이건 리턴값이라 구문이 아님!! 그래서 세미콜론 안들어감!!!
}

/*
근데, 이건 너무 비효율적이지 않나?
변수이름도 계속 반복이고.. 필드랑 변수이름 계속 일일히 쳐줘야하고..
그래서 이걸 해결하는 방법이 있슴!!
그거슨 바로 "필드 초기화 단축 문법"~

위의 함수를 보면 함수의 파라미터 이름과 구조체의 필드 이름이 같다.
그래서 필드 초기화 단축 문법을 이용해 이메일이나 username등 변수 이름을 일일히 안쳐줘도 괜찮음.
아래 함수를 보자
*/
fn modified_build_user(email: String, username: String) -> User{
    User{
        email,
        username,
        active: true,
        sign_in_count: 1,
    } //세미콜론 안들어감!!!
}
//이렇게 하면 아까와 똑같은 결과를 얻을 수 있다!
