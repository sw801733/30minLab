use std::io;

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // constant

fn main() {
    let x = 5; // immutable by default
    let mut y = 10; // mutable variable
    println!("The value of x is: {}", x);
    // x = 6; // This line would cause a compilation error
    // println!("The value of x is: {}", x);

    //shadowing
    let x = x + 1; // new immutable variable shadows previous x
    {
        let x = x * 2; // new x shadows outer x
        println!("The value of x in the inner scope is: {}", x);
    }

    let mut spaces = "   "; // mutable variable
    //spaces = spaces.len(); // variable type cannot be changed

    //i8, i16, i32, i64, i128, isize
    //u8, u16, u32, u64, u128, usize

    //f32, f64
    let z: f32 = 3.0; // f32
    let z2 = 3.0; // f64 by default

    //bool
    let t = true;
    let f: bool = false; // with explicit type annotation

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    let tup: (i32, f64, u8) = (500, 6.4, 1); // tuple with type annotation
    let (x, y, z) = tup; // destructuring

    let x: i32 = tup.0; // tuple access

    let a = [1, 2, 3, 4, 5]; // array
    let a: [i32; 5] = [1, 2, 3, 4, 5]; // array with type annotation
    let a = [3; 5]; // all elements are 3
    let first = a[0]; // array access

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    //rust use snake case for variable and function names
    another_function(x + 1);

    // rust doesn't return value from assignment statements
    // unlike other languages like C, C++, Java, JavaScript, Python
    // rust's code blocks are expressions
    // let x = (let y = 6); // this is invalid

    if x < 5 {
        println!("x is less than 5");
    } else if x == 5 {
        println!("x is 5");
    } else {
        println!("x is greater than 5");
    }

    let number = if x < 5 { 10 } else { 20 }; // if is an expression
    // if and else arms must return the same type
    //let number = if x < 5 { 10 } else { "20" }; // this line would cause a compilation error

    let number = 5;

    // if number { // this line would cause a compilation error
    if number != 0 { // correct way
        println!("number is true");
    }

    loop {
        println!("again!");
        break; // exit the loop
    }

    let result = loop {
        break 5; // return value from loop
    };

    while number != 0 {
        println!("number is not zero");
        break; // to avoid infinite loop
    }

    let a = [10, 20, 30, 40, 50];

    for element in a { // iterate over array
        println!("the value is: {element}");
    }

    for number in (1..4).rev() { // range and reverse
        println!("{number}!");
    }

}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

// In Rust, function's return value is equivalent to the value of the final expression in the function body
fn five() -> i32 {
    5 // no semicolon, so this is an expression that returns a value
}

fn plus_one(x: i32) -> i32 {
    // x + 1; // with semicolon, so this is a statement that does not return a value
    // so this function will return ()
    x + 1 // without semicolon, so this is an expression that returns a value
}