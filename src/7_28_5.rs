/* 얘 실행시키면 프로그램 ^c 눌러서 끝내기 전 까지는 계속돌아감.
fn main() {
    loop{
        println!("다시 실행!");
    }
}
*/
fn loops() {
    /*
    이게 쪼끔 특이한데. 러스트에서는 반복문에서 특정 값을 리턴할 수 있다.
    반복문에서 반복을 중단시키는 break 키워드 뒤에 리턴하고자 하는 값을 집어넣으면 리턴이 됨.
    밑에 경우에는 counter가 0이니까 반복을 10번하고 if문에 걸려서 break를 하는데 counter*2 를 리턴하니까 result 값에는 20이 대입되게 됨.
     */
    let mut counter = 0;

    let result = loop{
        counter += 1;

        if counter == 10{
            break counter * 2;
        }
    };
}

fn whiles(){
    /*
    러스트의 while문은 다른 while문과 별반 다를바가 없긴함. while문 뒤의 조건이 충족되지 않을 동안 실행됨.
    아래의 프로그램은 간단한 핵폭탄 발사 시스템임.
     */
    let mut number = 3;

    while number != 0{
        println!("{}!", number);
        
        number = number - 1;
    }
    println!("발사!");


    /*
    요거는 while문을 이용해서 배열의 각 요소를 반복처리하는거임.
    근데 뭔가 이상하지않음? 메모리 안정성을 위한 언어라면서 while문에 index < 5 이지랄 싸가지가 없음
    그래서 우리는 생각해야함.
    아! 이걸 잘못쓰면 메모리 오류가 나거나 배열 값들을 다 읽기 전에 종료가 되어버리는 부분이구나!
    그래서 우리는 for 문을 쓰는거임
     */
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5{
        println!("요소의 값: {}", a[index]);
        
        index = index + 1;
    }
}

fn main(){
    /*
    이 코드는 위에 있는 while문이랑 똑같은 역할을 한다. 
    for루프는 파이썬이랑 유사한데
    for (변수 이름) in (반복 횟수) 로 가져가면 된다. 
    아래에서는 변수이름에 element가 들어간거고 a라는 배열의 아이템들을 하나하나 읽어오는 iter() 기능을 이용한거임
     */
    let a = [10, 20, 30, 40, 50];

    for element in a.iter(){
        println!("요소의 값: {}", element);
    }


    /*
    이거는 위에있던 핵폭탄 발사 시스템을 for문으로 변경한 것임
    (1..4).rev() 이런 씹덕같은 표현을 하긴 하는데 이거는 1~3까지의 숫자를 .rev()한 값임
    .rev()는 범위를 뒤집어서 생성하는 메소드임.
    0~5까지 세고싶으면 (0..6).rev()라고 표현하면 됨
     */
    for number in (1..4).rev(){
        println!("{}!", number);
    }
    println!("발싸!!!");
}
