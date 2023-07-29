//소유권과 함수

/*
값을 함수에 전달한다는 행위는, 값을 변수에 대입하는 것과 유사하다.
변수를 함수에 전달하면 대입과 마찬가지로 값의 이동이나 복사가 이루어진다.
*/

fn main(){
    let s = String::from("hello"); //변수 s가 범위 내에 생성됨

    takes_ownership(s); //변수 s의 값이 함수 내로 이동함.
                        //이 시점부터 변수 s는 더 이상 유효하지 않음.
    let x = 5;          //변수 x가 범위 내에 생성됨

    makes_copy(x);      //변수 x의 값이 함수 내로 이동함.
                        //하지만 i32타입은 복사를 수행하므로 변수 x는 이 시점 이후로도 여전히 유효함.
}//이 지점에서 변수 x가 범위를 벗어난 후, 다음으로 변수 s도 범위를 벗어남.
//하지만 변수 s의 값(소유권)은 함수 내로 이동했기 때문에 아무런 일도 일어나지 않음.

fn takes_ownership(some_string: String){ //some_string변수가 범위 내에 생성됨
    println!("{}", some_string);
}//이 지점에서 some_string변수가 범위를 벗어나며 drop함수가 호출됨
//따라서 some_string변수가 사용중이던 메모리가 해제됨

fn makes_copy(sone_integer: i32){//some_integer 변수가 범위 내에 생성됨.
    println!("{}", sone_integer);
} //이 지점에서 some_integer변수가 범위를 벗어남. 하지만 아무 일도 일어나지 않음. 왜?
//i32와 같은 타입은 소유권을 가져가는 것이 아닌 변수를 복사한 것이라 drop함수가 실행 안되도 괜찮음.



fn return(){
    let s1 = gives_ownership(); //gives_ownership 함수의 리턴값이 변수 s1으로 옮겨짐
    
    let s2 = String::from("hello"); //변수 s2가 범위 내에 생성됨

    let s3 = takes_and_gives_back(s2); //변수 s2가 takes_and_gives_back함수로 옮겨간 후 리턴값은 변수 s3로 옮겨짐
}// 이 시점에서 변서 s3이 범위를 벗어나며 drop됨
//변수 s2역시 범위를 벗어나지만, takes_and_gives_back 함수로 소유권이 옮겨졌기 때문에 아무 일도 일어나지 않음.
//s1 역시 범위를 벗어났기 때문에 drop됨

fn gives_ownership() -> String{ //gives_ownership함수의 리턴값은 호출한 함수로 옮겨진다.

    let some_string = String::from("hello"); //변수 some_string이 범위 내에 생성된다.

    some_string //some_string변수가 리턴되면 호출한 함수로 옮겨진다.
}

fn takes_and_gives_back(a_string: String) -> String { //변수 a_string이 범위 내에 생성된다.
    a_string //변수 a_string을 리턴하면 그 값이 호출한 함수로 옮겨진다.
}


/*
소유권은 내먼 같은 패턴을 따른다. 값을 다른 변수에 할당하면 소유권이 옮겨진다.
힙 메모리에 저장된 변수의 데이터는 소유권이 다른 변수로 옮겨지지 않았다면 범위를 벗어날 때 drop함수에 의해서 제거된다.(옮겨지면 제거 할 필요가 없기때문)

근데 모든 함수가 소유권을 확보하고 다시 리턴하고 하면 너무 복잡하다.
함수에 값을 전달할 때 소유권은 이전하지 않고싶으면 어떻게 할까?
튜플을 이용해서 할 수는 있는데 그런 방법이 사용되는 경우는 극히 드물고, "참조"와 "대여"라는 개념이 등장한다.(다음꺼에서..)
*/
