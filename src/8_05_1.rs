/*
이전에 우리는 Option<T>타입이 Some(T) 값을 가질 때 그 안에 저장된 T의 값을 꺼내 쓸 수 있다고 배웠다.
Option<T>는 match표현식에서도 사용할 수 있다. 아래의 예제를 보자.
*/
fn plus(x: Option<i32>) -> Option<i32>{
    match x{
        None => None,
        Some(i) => Some(i + 1),
    }
}
fn main(){
    let five = Some(5);
    let six = plus(five);
    let none = plus(None);

    println!("five = {:?} / six = {:?} / none = {:?}", five, six, none);
}
/*
위의 코드를 살펴보자.
우선 plus함수를 처음 실행할 때, plus(five)를 호출하면 함수 본문 안의 변수 x는 Some(5)라는 값을 갖는다. 그런 후 이 값을 match표현식의 각 패턴과 비교함.
그럼, match표현식에서 Some에 대응하는 코드들을 실행시켜서 결론적으로 Some(x)에는 5+1인 6이 저장됨. 이와같이 six에도 Some(6)가 저장됨.

우리가 알아야할 것이 있는데 match표현식은 반드시 모든 경우를 처리해야함
아래 코드를 보자
*/
fn plus_one(x: Option<i32>) -> Option<i32>{
    match x{
        Some(i) => Some(i + 1),
    }
}
/*
얘는 컴파일이 안됨.
왜와이? Option열거자 중에 None 값을 처리하지 않았기 때문에 컴파일 에러가 남.
그래서 match구문은 반드시 모든 경우의 수를 처리해주어야 실행할 수 있음.
그리고 컴파일러가 어떤거 빼먹었는지도 알려줌.
다른 열거자들에서도 동일하게 열거자 안의 모든 경우를 처리해야만 match구문은 실행될 수 있음.
*/
