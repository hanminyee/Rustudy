#[derive(Debug)]
/*
match 흐름 연산자라는게 있는데, 이걸 좀 써볼거임
얘는 일련의 패턴과 값을 비교해 일치하는 패턴에 지정된 코드를 실행함.
패턴은 리터럴, 변수이름, 와일드카드를 비롯해 다양한 값으로 구성이 가능.
얘는 읽기도 쉽고 컴파일러가 모든 경우의 수가 처리되고 있는지 확인할 수 있다는 점이 좋음.

match자체에 대해 설명해보자면 "동전분리기"임.
기계에 동전을 넣으면 알아서 분리되듯이 지정된 값을 각 패턴에 맞는지 비교한 후 처음으로 일치하는 패턴을 찾으면 그 패턴과 관련된 코드블록을 실행함.
아래 코드를 보자.
*/
enum Coin{
    Penny,
    Nickle,
    Dime,
    Quarter,
}
fn value_in_cents(coin: Coin) -> u32{
    match coin{
        Coin::Penny => {
            println!("Lucky!");
            1
        }
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main(){
    println!("{}",value_in_cents(Coin::Penny));
}
/*
위 코드는 각각의 동전들을 분리해서 리턴하는 match문이다. 근데 여기서 코드블록으로 만들고싶으면 Penny 블록처럼 하면 됨
*/
