    1) By default all the var are immutable in rust

    fn main() {
    let x = 5;
    x = 6;   // ❌ Error
}

    2) We must have to use (mut) to make it mutable

    fn main() {
    let mut x = 5;
    println!("x = {}", x);

    x = 6;
    println!("x = {}", x);
}

    3) There are two variants for any result
        a. Ok
        b. Err
    4) Adding rand lib
        a. Rust doesn’t have any func to create rand numbers
        b. However, it supports creates.io to import and use
        c. Steps:
            i. In cargo.toml under dependecies
    5) Match (exhaustive -> pushes to limits)
        a. It is just like switch case in other languages
        b. It enforces to implement all the possible scenarios
        c. It is made up of arms
            i. Cases are called as arms here

    6) Rust auto detects types
    let x = 5;       // integer
let y = 3.14;    // float
let name = "Rust";
let flag = true;

    but you can also assign types
    let x: i32 = 5;


    7) Rust allows shadowing 
    fn main() {
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("x = {}", x);
}

    Ans x = 12

    let x = 5;
    let x = "hello";
    the above eaxmple works because of shadowing

    8) Constants in rust
    Constants cannot change and must have type.

    const PI: f64 = 3.14;

