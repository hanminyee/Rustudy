//문자열 슬라이스
fn main(){
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{}{}", hello, world); //띄어쓰기 없이 나옴. 공백 포함 안하나봄;; 뭔지 모르겠다 나도;
}
/*
문자열 슬라이스는 String의 일부에 대한 참조이다.
문자열 슬라이스는 전체 String에 대한 참조를 얻어노는 것이 아닌, 지정된 부분에 대한 참조만을 얻어오게 된다.
대괄호 안에 [(시작인덱스)..(끝인덱스)]를 지정해주어 범위를 이용해 슬라이스를 생성함.
이때 끝 인덱스가 포함안된다는걸 주의하자(컴공식 사고방식 ㅇㅋ?)
내부적으로는 시작위치로부터의 슬라이스의 길이 즉, (끝인덱스)-(시작인덱스)만큼의 데이터를 저장하는 구조체임.
따라서 변수 world는 7번째 문자부터 5개의 문자를 참조하는 슬라이스가 됨.
*/

fn slices(){
    let s = String::from("hello");
    let slice = &s[0..2];
    let slice = &s[..2]; //위에꺼랑 똑같음

    let len = s.len();
    let slice = &s[3..len];
    let slice = &s[3..]; //위에꺼랑 똑같음

    let slice = &s[0..len]; //첨부터 끝까지 참조할 때 쓰는거
    let slice = &s[..]; //위에꺼랑 똑같음
}
