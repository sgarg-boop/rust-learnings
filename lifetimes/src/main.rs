fn main() {
    let s1 = "Hello";
    let res;

    {
        let s2 = "World";
        res = longest(s1, s2);
    }
    // here the res scope should end here, but it is not because of lifetime

    println!("Res is {}", res);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    }
    else{
        y
    }
}
