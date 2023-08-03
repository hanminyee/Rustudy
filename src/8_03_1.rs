/*
이전 장들에서 "열거자" 라는 것에 대한 개념을 들은 적이 있을건데, 그걸 배울거임.
어떤 상황에서 열거자가 사용도리까?
예를들면 우리가 IP주소를 다루고 있다고 하자.
현재의 IP주소는 버전 4와 버전 6 등, 2가지가 표준으로 사용됨.
그래서 우리 프로그램이 다룰 IP주소의 형식은 이 둘 중 하나가 도리 것임.
따라서 우리는 가능한 모든 값에 이름을 부여해 열거할 수 있는데, 이럴때 열거자를 씀.

모든 IP주소는 버전 4 or 6을 사용하지만 동시에 두 형식을 지원하는 것은 불가능.
이런게 열거자에 적합함. 열거자는 반드시 내부의 값 중 하나만 사용할 수 있기 때문임.
아래의 코드를 보자.
*/
enum IpAddrKind{
    V4,
    V6
}

fn route(ip_type: IpAddrKind){} //리턴 테스트용 함수


//그럼, 열거자 인스턴스를 생성하는건 어떻게 하지?
fn main() {
    //이런식으로 :: 키워드를 통해 이루어짐.
    let version4 = IpAddrKind::V4;
    let version6 = IpAddrKind::V6;

    //위의 route함수에서 열거자의 값을 호출하려면 이렇게 쓰면 됨.
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
}


/*
근데, 그럼 열거자는 이름만 가지고 내부에 데이터는 어캐 집어넣나요?
그런 상황을 해결하기 위해서 구조체를 배운거임.
구조체를 통해서 내부적인 데이터를 결정해줄 수 있음.

아래를 보자
*/
struct IpAddr{
    kind: IpAddrKind,
    address: String,
}
let home = IpAddr{
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
}
let loopback = IpAddr{
    kind: IpAddrKind::V6,
    address: String::from("::1"),
}
//이런식으로 구조체 선언을 통해서 내부에 실제 주소값을 넣어줄 수 있다. 인스턴스는 구조체의 방식에 열거자 방식 즉, (구조체 변수명): (열거자명)::(열거자 변수명) 으로 사용하면 됨.

/*
근데 이것도 너무 귀찮다?
굳이 구조체를 선언해야하나?
싶으면 그냥 무지성으로 열거자에 String박을 수 있음
아래를 보자
*/
enum IpAddr{
    V4(String),
    V6(String),
}
let home = IpAddr::V4(String::from("127.0.0.1"));
let loopback = IpAddr::V6(String::from("::1"));
//이런식으로 가능함. 구조체 대신 열거자를 사용하면 열거자에 나열된 각각의 값은 서로 다른 타입과 다른 수의 연관 데이터를 보유할 수 있음.
//예를들면 버전 4 형식은 0에서 255 사이의 값 4개로 구성되는데 이걸 구현하기 위해선 구조체로는 어려움. 근데 열거자는 가능.
//아래 코드를 보자.

enum IpAddr{
    V4(u8, u8, u8, u8),
    V6(String),
}
let home = IpAddr::V4(127,0,0,1);
let loopback = IpAddr::V6(String::from("::1"));

/*
러스트에서는 IP주소를 저장하는 작업을 표준 라이브러리에서 제공한다.
표준라이브러리에서는 Ipv4Addr , Ipv6Addr 로 나누어서 각각을 다른 구조체로 선언한 후, 열거자에서
enum IpAddr{
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
과 같이 열거자 내부에 다른 구조체를 넣는다.
이런 방식도 가능하다는 것을 알고있어야함. 왜냐면 딱봐도 이거 존나 많이 쓸거같다 ㅇㅈ?
*/
