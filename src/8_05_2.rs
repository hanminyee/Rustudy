/*
러스트는 모든 경우를 다 처리하고 싶지 않을 떄 사용할 수 있는 패턴도 있음.
예를들어 우리가 1,2,3,4,5,6,7일떄만 값을 처리할건데 나머지는 필요없다 하면 나머지 값들을 "_" 라는 자리지정자로 처리하면 됨.
아래 코드를 보자.
*/

fn main(){
    let some_u8_value = 0u8;
    match some_u8_value{
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        5 => println!("five"),
        6 => println!("six"),
        7 => println!("seven"),
        _ => (),
    }
}
/*
"_" <-- 이 패턴은 모든 값에 일치함을 의미함.
그래서 match 표현식의 가장 마지막에 추가하면 앞에 나열했던 패턴에 일치하지 않는 나머지 모든 패턴에 일치하게됨.
() 는 단순한 유닛값이고, 이 경우에는 아무 일도 일어나지 않음.
그래서 이 코드는 _ 패턴 이전의 패턴에 일치하지 않는 모든 값에 대해서 아무런 작업도 실행하지 않음.

근데, 이 표현식 자체가 단 한개의 경우만 처리해야 할 때는 너무 장황함 ㅇㅈ?
그래서 if let 구문을 제공함
if let 구문은 if와 let 구문을 조합해서 여러 경우 중 한 가지만 처리하고, 나머지는 고려하고 싶지 않을 때 더욱 간편하게 처리가 가능함.
다음의 코드는 Option<u8> 타입의 값이 3일 때 만을 처리하는 코드임
*/
fn ex(){
    let some_u8_value = Some(0u8);
    match some_u8_value{
        Some(3) => println!("Three"),
        _ => (),
    }
}
/*
이 코드는 주어진 값이 Some(3)에 해당하는 경우만 처리하고, 나머지 값이나 None값은 처리하지 않음.
근데 이것도 개빡치는게 "_ => ()" 이새끼 너무 장황해보임.
이때 나오는 것이 iflet 구문임
아래 코드를 보자
*/
fn iflet(){
    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value{
        println!("Three");
    }
}
/*
이 코드는 ex()함수와 동일한 동작을 하는 코드임.
if let 문법은 하나의 패턴과 표현식을 등호 기호를 이용해 조합함. 
얘를 쓰면 입력해야 할 코드의 양도 적고 들여쓰기도 적어짐.
근데 match표현식이 제공하는 모든 경우의 수를 검사하는 기능을 활용할 수가 없음.
match 와 iflet 중 어떤것을 사용할지는 개인의 몫임.

마지막으로 설명하자면 if let 문법은 주어진 값에 대해서 하나의 패턴만 검사하고 나머지 값은 무시함.
match 표현식과 다른점이 바로 "나머지 값들을 검사조차 안함"임. match는 검사함.

iflet 구문은 elst를 포함할 수 있음
아래 코드를 보자
*/
fn ifletelse(){
    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value{
        println!("Three");
    } else{
        println!("It is not Three");
    }
}
/*
위 코드는 3이면 Three를 아니면 3이 아니라고 출력함. 이런식으로 else 사용 가능.
그래서 결론적으로는 프로그램 로직을 짜는데 match가 너무 스케일 크다 싶으면 if let 쓰시고 아 이건 다 검사해야한다 싶으면 match쓰셈
*/
