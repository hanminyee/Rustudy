use std::io; 
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("숫자를 밪혀봅시다!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("사용자가 맞혀야 할 숫자: {}", secret_number);
    
    loop{
        println!("정답이라고 생각하는 숫자를 입력하세요.");

        let mut guess = String::new(); 

        io::stdin().read_line(&mut guess)
            .expect("입력한 값을 읽지 못했습니다.");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("입력한 값: {}", guess); 

        match guess.cmp(&secret_number){
            Ordering::Less => println!("입력한 숫자가 작습니다!"),
            Ordering::Greater => println!("입력한 숫자가 큽니다!"),
            Ordering::Equal => {
                println!("정답!");
                break;
            }
        }
    }
    /*
    반복문 존나 특이하게 생김 그냥 무지성 loop 박아버림;;
    얘 키워드 자체가 무한반복을 위해 만들어짐. 근데 탈출 어캐하노 ㅅㅂ
    그래서 우리는 종료할 방법을 찾아냄
    그것이 바로 break문 (ㅗㅜ 쉣)
    자세히보면 match문에 Equal 가지에 중괄호로 묶여서 break가 들어간게 보일거임.
    이래놓으면 프로그램 끝남

    근데... 만약에 사용자가 입력을 등신같이 하면 어떡하지?
    라는 생각은 모두가 하게 될 것임
    그래서 예외처리를 해줘야함.
    물론 위에 형변환 시키는 과정에서 예외처리문을 썼지만, 이 방법을 쓰면 그냥 프로그램이 죽어버림.
    그래서 우리는 Result 타입을 리턴하는 친구인걸 아니까 Ok 또는 Err가 나올 것을 알고있음.
    즉, match문을 추가해준 다음 형변환을 하면 되는거임!!
    그래서 우리는 각각의 타입에 상응하는 상황을 적용시키면 되는 것임.
    Ok(num) => num,
    Err(_) => continue,
    를 추가해주면 됨.
    문법이 개특이하긴 한데 별 수 없음 이게
     */
}
