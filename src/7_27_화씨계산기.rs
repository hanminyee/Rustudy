use std::io;

fn main(){

    println!("섭씨 온도를 입력하세요");

    let mut temperture = String::new(); //새로운 string 인스턴스 생성해서 temperture 에 바인딩

    io::stdin().read_line(&mut temperture).expect("입력한 값을 읽는데 실패했습니다."); //io 라이브러리의 stdin()에 있는 함수인 read_line 으로 한 줄을 입력받아서 mut temperture에 바인딩

    let temperture: f64 = temperture.trim().parse().expect("입력한 값이 올바르지 않습니다."); //.trim을 통해 입력받을 때 들어오는 \n 없애고 parse를 통해 형변환

    let firetemperture = (temperture * 1.8) + 32.0; //f64로 형 지정된 temperture을 화씨로 변환

    println!("화씨온도는 {}도 입니다.", firetemperture); //출력
}
