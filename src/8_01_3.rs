/*
구조체를 이용한 새 프로그램을 짜보자.
사각형의 픽셀단위의 너비와 높이를 이용해 면적을 구하는 프로그램 만들긔~
*/
struct Rectangle{
    width: u32,
    height:u32,
}
fn main(){
    let rect1 = Rectangle{ 
        width: 30, 
        height: 50
    };

    println!{
        "사각형의 면적: {} 제곱픽셀", area(&rect1)
    };
}
fn area(rectangle: &Rectangle) -> u32{ //구조체 불변 인스턴스에 대한 대여를 통해 값에 접근. 구조체의 소유권을 가져오는 것 보단 잠깐 빌리는게 나음 ㅇㅈ?
    rectangle.width * rectangle.height
}
