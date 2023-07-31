/*
이제 문자열 슬라이스를 배웠으니까 아까 했던 첫 글자만 리턴하는 forst_word함수를 다시 써보자
*/
fn first_word(s: &String) -> &str{
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[0..i];
        }
    }
    &s[..]
}
/*
이렇게 적으면 첫 글자만 리턴하는 역할을 수행하게 도니다. 근데 왜 굳이 이렇게 끄냐?
우선 아까 예로 들었던 second_word함수를 생각해보면, 이 함수는 이전엔 복잡한 리턴타입을 가지고있었다. 그런데
fn second_word(s: &String) -> &str 로 first와 동일하게 사용할 수 있다.(동일하게 슬라이스를 리턴하도록)
또한 String 타입에 대한 참조의 유효성을 컴파일러가 보장해주니까 더욱 안전하고 직관적인 API를 갖추게 됨.
*/

fn main(){
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear();
    println!("the first word is {}", word);
}
/*
아까는 에러가 뭔지 안보여주면서 안돌아가던 코드인데 이건 에러가 뭔지 보여주면서 돌아감
왜냐면 이전에 배웠던 불변참조를 이미 생성한 경우에 가변참조를 생성할 수 없는데, 이 코드는 String타입의 값을 제거해야해서 가변참조가 필요하고, 이미 가변참조가 있는데 가변참조 써서 ㅈ망한거임.
이렇게 쓰면? 이런 에러를 컴파일 시점에 잡아줌.
*/

fn slices(){
    let s = "Hello, World!"; //변수 s의 타입은 &str이다. 즉, 바이너리의 어느 한 지점을 가리키는 슬라이스임. 그래서 문자열 리터럴은 항상 불변이다. &str은 불변참조라서.
}
