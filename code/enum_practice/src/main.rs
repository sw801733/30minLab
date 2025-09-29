enum IpAddrKind {
    V4(String),
    V6(String),
} 
// 각 베리언트에 데이터를 추가하여 구조체를 사용할 필요가 없어짐
// 열거형은 각기 다른 타입의 데이터를 가질 수 있음

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// Option<T> 열거형은 T 타입의 값을 가지거나 아무 값도 가지지 않을 수 있음을 나타냄
// null 대신 Option<T>를 사용하여 null 참조로 인한 오류 방지
// rustc 컴파일러는 null 참조를 허용하지 않음
// null 대신 Option<T>를 사용하여 null 참조로 인한 오류 방지

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}

fn main() {
    let x : i32 = 5;
    let y : Option<i32> = Some(5);
    // let sum = x + y; // 컴파일 오류 발생
    // Option<T> 를 T 로 변환해야 한다.
    // 이러한 방식으로 널로 인해 발생하는 실제로는 널인데 널이 아닌 값에 접근하는 오류를 방지할 수 있다.

    let five: Option<i32> = Some(5);
    let six = plus_one(five);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => move_player(dice_roll),
    }
    let coin = Coin::Quarter;
    let mut count = 0;
    match coin {
        Coin::Quarter => count += 25,
        _ => count += 1,
    }
    // match 문에서 모든 경우를 다루지 않으면 컴파일 오류 발생
    // 모든 경우를 다루지 않으려면 if let 문을 사용
    if let Coin::Quarter = coin {
        count += 25;
    } else {
        count += 1;
    }
    println!("count: {}", count);
}