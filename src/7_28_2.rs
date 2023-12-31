/*
이 함수는 "하나의"구문으로 이루어져 있다.
함수는 기본적으로 구문과 표현식으로 나뉘는데, 러스트는 표현식 기반 언어라서 구문과 표현식을 구분하는 것은 매우 중요함. 다른언어는 구분을 두지 않음.
구문 : 어떤 동작을 실행하지만 값을 리턴하지 않는 명령
표현식 : 최종 결괏값으로 평가(evaluate)되는 명령
*/
fn main() {
    let y = 6; //이 식은 구문이다. 왜? 어떤 값을 리턴하지는 않으니까! 함수선언도 구문임
    let x = (let z = 6); //이거 컴파일 에러남 왜와이? let z = 6 이라는 구문은 값을 리턴하지 않기 때문. x에 아무런 값도 대입되지 않음. 그래서 ㅈ버그남. 괄호빼면 정상적으로 돌아간다카더라
}


// 여기부턴 표현식
fn expression(){
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };
}

/*
생각해보자. 5+6 이라는 표현식은 11이라는 값으로 "평가"된다. 근데 그렇게 생각해보면 표현식은 구문의 일부가 될 수 있음
왜? let y = 6; 이라는 식 또한 6이라는 값으로 평가될 수 있으니까. 함수호출도, 매크로호출도 다 표현식임.

let y 부분을 잘 봐보자
우선 let x = 3; 으로 x를 덮어쓰고 거기에 +1 을 해서 y에 대입한다.
근데 잘 보면 x+1이 세미콜론으로 안끝난다.
이게 표현식의 특징이다. 표현식은 마지막에 세미콜론을 포함하지 않는다. 여기에 세미콜론을 추가하면 표현식이 구문으로 바뀌어서 값을 리턴하지 않는다. 즉, y에 대입이 안됨.

자세히 설명하자면 일단 y뒤의 중괄호 부분은 "4"라는 값으로 평가된다. 왜? x = 3에다가 x+1해주면 4니까.
근데 만약에 x+1에 세미콜론을 붙이면 그건 더이상 표현식이 아니라 구문이 되어서 리턴값이 없어지게 된다. 이거 대가리 존나아프겠는데..
*/
