/*
이제부턴 변수와 데이터가 상호작용 하는 방식에 대해 알아보자.
*/
fn main(){
    // 아래 코드라 5를 변수 x에 대입한 후, 변수 x값의 복사본을 변수 y에 대입하는 코드라는 점은 금방 알 수 있다.
    let x = 5;
    let y = x;

    //근데 String을 사용하는 경우는 좀 다르다.
    let s1 = String::from("hello");
    let s2 = s1;
    /*
    이 코드는 앞선 코드와 거의 같아보여서 동작할 것 같지만, 위와 같은 방식으로 동작하지 않는다.

    우선 String타입은 내부적으로 "메모리에 대한 포인터", "길이", "용량" 이 3가지 부분으로 구성된다. 이 데이터는 스택에 저장됨.
    길이는 String타입의 콘텐츠가 현재 어느 정도의 메모리를 사용중인지 바이트 단위로 표현한 값임.
    용량은 String타입이 운영체제로부터 할당받은 총 메모리를 바이트 단위로 표현한 것임.
    마지막으로 메모리에 대한 포인터는 실제 String의 문자들이 저장된 힙 영역을 포인팅하는 포인터 변수를 담고있음.

    자 그럼 생각해보자. 과연 s1안의 문자들을 s2에 대입하려고 할 때, 우리는 문자들을 복사할 필요가 있을까?
    정답은 아니요다.
    왜냐면 우리는 스택 영역에 존재하는 s1의 기본적인 3가지 정보만을 가지고도 s2가 s1과 동일한 값을 가지게 만들 수 있다.
    그래서 대부분의 언어들은 s1의 기본적이 3가지들을 다 복사해서 s2에게 넘긴다. 그러면 s2는 s1과 동일한 메모리에 대한 포인터를 가지고 같은 길이와 용량을 가지게 된다.

    근데 러스트는 좀 다르다. 러스트는 일단 변수가 범위를 벗어나면 자동으로 drop함수를 호출해서 해당 변수의 메모리를 정리한다.
    그래서 다른 언어들이 하는 방식대로 정리한다면, 변수 s1과 s2의 범위가 다르면 두 변수가 모두 같은 메모리를 해제하려고 하기 때문에 "해제 에러"가 난다.
    이것은 러스트가 싫어하는 메모리 안전성 버그 중 하나임.

    러스트는 할당된 메모리를 복사하는 대신, 변수 s1이 더 이상 유효하지 않다고 판단하기 때문에 s1이 범위를 벗어날 때 메모리를 해제할 필요가 없음.
    그래서 변수 s2에 변수 s1을 복사한 후 변수 s1을 다시 사용하려고 하면 에러가 남.(이거 그대로 실행하면 에러남)

    러스트에선 이러한 방식을 "이동"이라고 부름. 메모리의 소유권이 s1에서 s2로 넘어간 것임
    그래서 s1을 다시 참조하려고 하면 얘는 컴파일러가 더이상 유효하지 않은 변수라고 생각하니까 메모리 해체의 필요가 없음.
     */
}
