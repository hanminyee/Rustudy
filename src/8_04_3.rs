/*
match의 또다른 장점은 패턴에 일치하는 ㄱ밧의 일부를 바인딩할 수 있다는 것이다.
아래 수정된 코드를 살펴보자
아래 코드는 각 주에서 발행된 25센트짜리 동전에 대한 수집요소이다.
*/

#[derive(Debug)]

enum UsState{
    Alabama,
    Alaska, //생략
}

enum Coin{
    Penny,
    Nickle,
    Dime,
    Quarter(UsState), //열거자의 변수로 UsState를 추가해줌
}
fn value_in_cents(coin: Coin) -> u32{
    match coin{
        Coin::Penny => 1,
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn main(){
    println!("{}",value_in_cents(Coin::Quarter((UsState::Alaska))));
}
/*
위의 메인처럼 출력하면 coin변수는 Coin::Quarter(UsState::Alaska)값을 가지게 된다.
즉, 열것값에 저장된 주 이름을 바인딩해서 출력한 것
*/
