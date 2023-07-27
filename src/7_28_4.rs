/*
if문 평범함 다른점은 괄호 없다 이정도?
근데 알아둬야 할 것은 if문은 반드시 불리안타입을 리턴해야 한다는 것임.
예를들어 첫번째 if문에 number만 띡 적혀있으면 이건 불리안타입으로 리턴하는게 아니라서 버그가 터짐.(number만 적어두면 i32리턴 표현식이니까 그런듯)

뭐 if number != 0 이런거도 가능하고 else if 문 당연히 가능함.
if문 안에 뭐 %, *, +, -, == 같은 기본적인거 다 가능함.

근데 else if 문 많으면 개 지저분한거 인정? 그래서 러스트는 개쩌는 방법인 match문을 만들어냄 근데 이건 6장가서 가르쳐줌 ㅠㅠ
*/
fn main() {
    let number = 3;

    if number < 5{
        println!("조건이 일치합니다!");
    }
    else if number == 3{
        println!("딱 맞추셨네요!");
    }
    else {
        println!("조건이 일치하지 않습니다.");
    }



/*
이런 미친짓도 가능함
if문 자체가 표현식이라 let구문 오른쪽에도 사용이 가능함. 그래서 if표현식의 결과가 number에 대입됨.
그리고 주의해야 할 점은 if문의 각 가지(branches)가 리턴하는 결과는 모두 같은 타입이어야 함.
만약에 밑에서 else{ "six" }; 하면 에러뜸.
*/

    let condition = true;
    let number = if condition{
        5
    } else {
        6
    };
    
    println!("number의 값 : {}", number);
}
