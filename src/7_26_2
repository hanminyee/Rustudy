use std::io; 
use rand::Rng;
use std::cmp::Ordering;
/*
얘는 뭐냐?
새로운 use 구문을 사용한거임.
표준 라이브러리한테서 std::cmp::Ordering 타입을 로드함. Result타입이랑 마찬가지로 Order역시 열거자임.
이 열거자는 Ok, Err로 표현되어지던 Result타입의 열거자와 다르게 Less, Greater, Equal 값 중 하나를 표현함.

*/

fn main() {
    println!("숫자를 밪혀봅시다!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("사용자가 맞혀야 할 숫자: {}", secret_number);

    println!("정답이라고 생각하는 숫자를 입력하세요.");

    let mut guess = String::new(); 

    io::stdin().read_line(&mut guess)
        .expect("입력한 값을 읽지 못했습니다.");

    let guess: u32 = guess.trim().parse()
        .expect("입력한 값이 올바른 숫자가 아닙니다.");
    /*
    얘 없이 돌리면 ㅈ버그뜸.
    일단 러스트 특징인데 러스트는 정적인 타입 시스템을 갖추고있음. 즉, 사용자가 지정한 타입들이 안맞으면 응애 해버림.
    근데? 또 타입추론도 지원함.
    그래서 우리가 이전에 let guess = String::new()코드를 작성하면 러스트는 guess 변수가 String 타입이어야 한다는 점을 스스로 이해하고 개발자에게 타입을 지정할 것을 강요하지 않음
    근데 우리가 만든 secret_number 변수는 숫자타입임.
    그리고 1~100사이의 값을 가질 수 있는 숫자타입은 32bit 정수인 i32, 64bit 정수인 i64 등 몇가지가 있음.
    러스트는 일단 숫자에 대해서는 기본적으로 i32를 씀. 그래서 별도로 지정하지 않는 이상 러스트는 i32로 이해함.
    얘 없으면 에러가 뜨는 이유는 아래의 match문에서 숫자와 문자열을 비교해버리는 불상사를 일으킨거임;;
    그래서 프로그램이 문자열로 읽은 값을 실제 숫자로 변환해야 정답으로 지정한 값과 비교할 수 있음.

    여기서 보면 아까 선언했던 guess를 다시 선언해버리는 미친짓을 하는데, 러스트는 이렇게하면 값을 덮어씌워버림.
    새로 선언한 guess변수에는 guess.trim().parse()라는 표현식의 결과를 바인딩함.
    trim() 메소드는 문자열 양쪽 끝의 공백을 제거한다.
    u32타입은 오로지 숫자에 해당하는 문자만 처리할 수 있지만, read_line메소드가 사용자의 입력을 읽으려면 반드시 엔터키를 입력해야한다.
    그리고 이 엔터키는 줄바꿈 문자로서 문자열에 덧붙인다.
    그래서 trim을 호출해서 \n을 제거하고 숫자만 남게 해주는거임.

    parse메소드는 문자열을 구문분석해서 숫자로 변환함.
    이 메소드는 다양한 타입의 숫자를 구문분석 할 수 있음
    따라서 let guess: u32처럼 정확히 원하는 숫자타입을 러스트에 알려줘야함. 그래야 알아먹음
    guess 뒤의 : 는 변수 타입 지정하는거임 ㅇㅇ u32니까 unsigned int 32bit 생각하면 될듯.

    이제 아래에서는 u32타입의 guess와 숫자인 secret_number를 비교하니까 러스트는 stcret_number가 u32구나! 라고 생각하는거임.
     */
    
    println!("입력한 값: {}", guess); 

    match guess.cmp(&secret_number){
        Ordering::Less => println!("입력한 숫자가 작습니다!"),
        Ordering::Greater => println!("입력한 숫자가 큽니다!"),
        Ordering::Equal => println!("정답!"),
    }
    /*
    얘가 좀 신기함. 얘는 match구문이라는 것임
    match 구문은 여러개의 가지로 구정됨. 각각의 가지는 패턴과 match의 시작에 주어진 값이 이 패턴과 일치할 때 실행될 코드로 구성됨.
    match표현식에 지정된 값을 읽어 각가지 패턴과 차례대로 비교함. match표현식과 패턴은 러스트의 강력한 기능 중 하나임.
    왜와이? 졷버그 잘잡아서. 왜 잘잡냐? 다양한 상황을 효율적으로 처리가 가능하니까. 세세하게는 6장, 18장 ㄱㄱ

    일단 이 match식이 어떻게 처리가 되냐면
    사용자가 입력한 값은 50이라고 가정, 난수는 38이라고 가정하자.
    guess 뒤의 cmp 메소드는 50이랑 38을 비교하고 50이 38보다 큰 숫자니까 Ordering::Greater 를 리턴함.(열거자 중 하나)
    그러면 match 표현식이 Ordering::Greater값을 가져서 각가지 패턴을 확인하기 시작함.
    어 첫째놈은 less네..
    어? 둘째놈 이새끼 이거 Greater이구만!
    하면 => 뒤에 있는 코드를 실행함.
    더 이상 비교할 필요가 없으니까 바로 match문은 종료됨
     */
}
