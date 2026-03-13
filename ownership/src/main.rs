// fn main() {

//     // the things which are unknown at compile time are stored in heap

//     let name = String::from("Shivansh");

//     take_ownership(name);   // here owner is shifted to take_ownership function, drops the name here

//     // println!("name is {}", name); // gives error because as name is moved to take_ownership function, it is no longer valid here

//     let s = gives_ownership();    // s takes the ownership of the string returned by gives_ownership function
//     println!("s is {}", s);  // prints string here , after this main function ends, s will be dropped
// }

// fn take_ownership(s: String) {
//     println!("name is {}", s);
// }

// fn gives_ownership() -> String{
//     let n = String::from("String is returned");
//     n
// }

// what should we do if we want ownership back and also use that variable
fn main(){

    let s1 = String::from("Hello");

    // for cal len we have to pass in another function
    let (s1,len) = length(s1);

    println!("String is {s1} and length is {len}");

}

fn length(s: String) -> (String, usize){

    let len = s.len();

    (s,len)

}