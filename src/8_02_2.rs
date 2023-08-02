/*
이번엔 컨텍스트 안에 메소드를 하나 더 추가해보자.
이번에는 Rectangle 구조체의 인스턴스에 또 다른 인스턴스를 전달해서 첫 번째 Rectangle인스턴스의 면적이 두 번째 인스턴스의 면적을 완전히 포함한다면 true, 아니면 false를 리턴하는 메소드를 구현해보자.
메소드 이름은 can_hold이며, 또다른 Rectangle 구조체의 불변 인스턴스를 대여하여 파라미터로 사용한다.
*/
#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle{
    fn area(&self) -> u32{
        self.width * self.height
    }
    
    fn can_hold(&self, other: &Rectangle) -> bool{ //self는 이 함수가 호출되었을 때 "."앞에있는 친구
        self.width> other.width && self.height > other.height
    }

    //얘는 갑자기 들어갔는데, 연관함수라고 해서 self파라미터 없이도 존재할 수 있는 친구임. 아래의 코드는 입력받은 크기를 가진 정사각형 Rectangle 인스턴스를 리턴하는 친구 리턴해주는 친구
    //얘는 호출할 때 "::" 문법 사용함
    fn square(size: u32) -> Rectangle{
        Rectangle {width: size, height: size}
    }
}

//이런식으로 같은 Rectangle의 컨텍스트지만 블럭을 나누어도 상관없음 똑같이 작동함. 아래의 코드는 너비 높이 2배해주는 코드임
impl Rectangle{
    fn doubling(&mut self){
        self.width = self.width*2;
        self.height = self.height*2;
    }
}

fn main(){

    let sq = Rectangle::square(3); //이런식으로 :: 문법 사용함.

    let mut rect1 = Rectangle{
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle{
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle{
        width: 60,
        height: 45,
    };

    println!("rect1은 rect2를 포함하는가? {}", rect1.can_hold(&rect2));
    println!("rect1은 rect3를 포함하는가? {}", rect1.can_hold(&rect3));

    rect1.doubling();
    println!("{} {}", rect1.width, rect1.height);
}
