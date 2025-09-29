fn build_user(email: String, username: String) -> User {
    User {
        active: bool::from(true),
        username,
        email,
        sign_in_count: 1,
    }
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32); // 튜플 구조체
struct Point(i32, i32, i32); // 튜플 구조체
// Color와 Point는 서로 다른 타입

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // impl 블록 안에 메서드 정의
    // impl 블록 내에 구현된 모든 함수를 연관 함수라고 부른다.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {

    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    let mut user2 = build_user(
        String::from("another@example.com"),
        String::from("anotherusername123"),
    );

    let user = User {
        email: String::from("another@example.com"),
        ..user1
    }; // user1의 나머지 필드들을 user에 복사
    // user1은 더 이상 유효하지 않음
    // 다만 active와 sign_in_count는 Copy 트레이트를 구현했기 때문에 복사됨


    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    println!("The area of the rectangle is {} square pixels.", rect1.area());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
}