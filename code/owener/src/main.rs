fn main() {
    let s1 = gives_ownership();         // gives_ownership이 자신의 반환 값을 s1로
                                        // 이동시킵니다

    let s2 = String::from("hello");     // s2가 스코프 안으로 들어옵니다

    let s3 = takes_and_gives_back(s2);  // s2는 takes_and_gives_back로 이동되는데,
                                        // 이 함수 또한 자신의 반환 값을 s3로
                                        // 이동시킵니다

    let r1 = String::from("hello");
    let len = calculate_length(&r1); // r1의 참조자를 전달합니다
    println!("The length of '{}' is {}.", r1, len);

    //let s = String::from("hello"); // 불변 참조자 s가 된다.
    let mut s = String::from("hello"); // 가변 참조자 s가 된다.
    change(&s);                   // s의 참조자를 change에 전달합니다

    let r1 = &mut s;
    let r2 = &mut s; // 두 번째 가변 참조자 r2를 만들려고 하면 오류가 발생합니다
    // 이를 통해 Rust 는 데이터 경쟁을 방지합니다

    let r1 = &s1;
    let r2 = &s1; // 여러 개의 불변 참조자는 허용됩니다
    //let r3 = &mut s1; // 불변 참조자가 존재하는 동안에는 가변 참조자를 만들 수 없습니다
    
    let r1 = &s1;
    let r2 = &s1;
    println!("{} and {}", r1, r2); // r1과 r2는 더 이상 사용되지 않습니다
    let r3 = &mut s; // 이제는 새로운 가변 참조자를 만들 수 있습니다
    println!("{}", r3);
    // 서로 스코프가 겹치지 않기 때문에, r3는 유효합니다.

    let mut s = String::from("hello world");
    let word = first_word(&s); // word는 5를 가리킵니다
    s.clear();
    // word는 5를 가리키지만, s는 비어 있습니다
    // 따라서 word로 문자열을 읽으려고 하면 잘못된 값을 얻게 됩니다
    // 이 문제를 해결하기 위해 슬라이스를 사용합니다

} // 여기서 s3가 스코프 밖으로 벗어나면서 버려집니다. s2는 이동되어서 아무 일도
  // 일어나지 않습니다. s1은 스코프 밖으로 벗어나고 버려집니다.

  // &String -> &str 슬라이스로 변경
  // 문자열 리터럴은 특정 지점을 가리키는 포인터와 길이를 포함하는 슬라이스입니다
  // 따라서 문자열 리터럴은 &str 타입입니다
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            //return i;
            return &s[0..i];
        }
    }

    &s[..]
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s // s는 스코프 밖으로 벗어나고, 참조자가 가리키는 값도 함께 사라집니다.
} // 따라서 이 코드는 컴파일되지 않습니다.  

fn change(some_string: &String) {
    some_string.push_str(", world"); 
} 
fn calculate_length(s: &String) -> usize { // s는 String에 대한 참조자입니다
    s.len()
} // 여기서 s가 스코프 밖으로 벗어나지만, 참조자이기 때문에 아무 일도 일어나지 않습니다.

fn gives_ownership() -> String {             // gives_ownership은 자신의 반환 값을
                                             // 자신의 호출자 함수로 이동시킬
                                             // 것입니다

    let some_string = String::from("yours"); // some_string이 스코프 안으로 들어옵니다

    some_string                              // some_string이 반환되고
                                             // 호출자 함수 쪽으로
                                             // 이동합니다
}

// 이 함수는 String을 취하고 같은 것을 반환합니다
fn takes_and_gives_back(a_string: String) -> String { // a_string이 스코프 안으로
                                                      // 들어옵니다

    a_string  // a_string이 반환되고 호출자 함수 쪽으로 이동합니다
}