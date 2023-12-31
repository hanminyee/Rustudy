fn main() {
    let x = 5;
    println!("x의 값: {}", x);
    x = 6;
    println!("x의 값: {}", x);
}
/*
러스트는 기본적으로 변수가 불변이다.
앞서 설명했듯이 변수를 수정가능하게 하고싶으면 변수를 선언할 때 mut를 넣어주면 된다.
즉, 2번째 줄을 let mut x = 5; 라고 수정하면 코드가 올바르게 돌아간다.

근데, 왜 이렇게 만들었을까?
생각해보면 정말 간단하다. 우리가 이 변수가 절대 변하지 않을 것으로 생각하고 이용했는데
뒤에 코드가 이 값을 변경한다면, 그것은 치명적인 버그로 이어질 수 있다.
이런 종류의 버그는 찾기도 빡셈
그래서 러스트는 이러한 문제를 없애기 위해 디폴트로 변경이 불가능하게 만든 것임.
그리고 이 불변하는 것을 변하게 만드는 걸 컴파일 과정에서 잡아주기때문에 버그를 없애는데에 효율적임.
*/


/*
변수의 값을 변경할 수 없다는 개념은 다른 프로그래밍 언어의 개념에서 "상수"의 개념과 유사하다.
러스트에서의 불변 변수는 상수처럼 한 번 할당한 값을 변경할 수 없다.
근데 상수와 불변 변수 사이에는 몇가지 차이점이 있다.

우선, 상수에는 mut 키워드를 사용할 수 없다.
상수는 기본 선언만으로 불변 속성이 아니라 항상 불변이다.

또한 상수는 let 키워드 대신 const 키워드를 사용해서 선언해야한다.
또한 할당할 값의 데이터 타입을 반드시 지정해야한다.
예를들어 상수를 선언할 때에는
const MAX_POINTS: u32 = 100_000;
이런식으로 반드시 지정해야한다.(언더바는 자릿수 구분할 때 사용해주는거임. 그냥 알아들음 컴파일러가)

또한 상수는 전역 범위를 비롯해 우너하는 어떤 범위 안에서도 선언할 수 있다!

상수는 프로그램이 실행 중인 동안에는 선언된 범위 안에서 항상 유효한 값이라서
게임에서 플레이어가 얻을 수 있는 최고 점수나 빛의 속도 등과 같이 프로그램의 여기저기에서 사용해야할 값을 선언하기에 적합하다.

또한 프로그램 전체에서 사용하는 하드코드(상수나 변수에 들어가는 값을 소스코드에 직접 쓰는 방식) 된 값에 상수로 이름을 지정하는 것 역시
코드를 유지보수 하는 개발자에게 그 값의 의미를 전달하기에 적합함.
또한 하드코드 된 값을 변경해야 할 때 한 곳에서만 변경하면 되니까 존내조음
*/
