/*
좀 더 다양한 방식의 열거자도 알아보자

아래 코드를 살펴보면 이 열거자는 각기 다른 타입을 이용해 4개의 값을 정의하고 있음.
1. Quit 값은 연관 데이터를 전혀 가지지 않음.
2. Move 값은 익명 구조체(anonymous struct) 를 포함함.
3. Write 값은 하나의 String 값을 포함함.
4. ChangeColor 값은 3개의 i32 값을 포함함.
근데 이렇게 4개 선언하는 거 보면 구조체랑 유사함.
한가지 다른점은 struct 키워드 안쓰고 열거자의 개별 값들은 모두 Message타입에 속해있다는 점? 그정도?
*/
enum Message{
    Quit,
    Move(x: i32, y: i32),
    Write(String),
    ChangeColor(i32,i32,i32),
}

//아래 구조체는 위의 열거자와 온전히 동일한 내용을 가질 수 있도록 작성한 것임.
struct QuitMessage; //유닛 구조체
struct MoveMessage{ x: i32, y: i32 }; //그냥 구조체
struct WriteMessage(String); //튜플 구조체
struct ChangeColorMessage(i32, i32, i32); //튜플 구조체

/*
구조체와 동일하게 열거자에서도 impl블록을 사용할 수 있다.
열거자에 메소드를 부여함으로서 더욱 실용성이 높아져버림
*/
impl Message{
    fn call(&self){
        //여기에 메소드 본문 작성
        println!("띠링띠링");
    }
}

let m = Message::Write(String::from("hello"));
m.call();
//머.. 이런식으로..
