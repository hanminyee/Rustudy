/*
함수는 자신을 호출한 코드에 값을 리턴할 수 있는데, 리턴값에 이름을 부여하지는 않지만 "->"를 써서 타입을 부여해줘야함.
러스트에서 함수의 리턴값은 함수 본문의 마지막 표현식의 값이라고 생각하면 됨.
아래 함수에서는 i32타입의 5가 되겠지?

근데 밑에 five 함수를 보면 함수나 매크로나 let같은 그 어떤 구문도 사용하지 않고 리턴값이 5인 함수를 만들어냄.

결국 여기서 중요한 두가지는
첫째, let x = five(); 라는 메인함수의 한 줄은 함수의 리턴값을 이용해서 변수를 초기화하는 방법을 보여줌.
    이 방법은 let x = 5; 라는 구문과 같은 결과를 만듦
둘째, five함수는 파라미터가 없고, 리턴값의 타입을 정의하고는 있지만, 함수의 본문에서는 세미콜론도 없이 5만 덩그러니 놓여있음.
    왜냐면 5가 리턴하고자 하는 값을 나타내는 표현식이기 때문임.
*/
fn five() -> i32{
    5
}
fn main(){
    let x = five();

    println!("x의 값: {}", x);

    let x = plus_one(5);
    println!("x의 값 : {}", x);
}
fn plus_one(x: i32) -> i32{
    x+1
}
/*
위의 plus_one함수는 파라미터가 있음. 그리고 리턴하는 값은 파라미터로 받은 값에 +1을 한 값을 리턴함.
여기에는 새로운 변수 선언도 없으며 하나의 표현식과 리턴하는 타입의 명시만 있을 뿐임.
개지리는데?

만약에 세미콜론을 붙이면 타입 불일치 에러가 뜸.
왜냐면 위 함수는 i32 타입을 리턴하도록 선언되었지만, 구문은 값으로 평가되지 않고 빈 튜플인 ()로 표현되어서 i32형태로 리턴할 수 없음. 그래서 버그남
*/