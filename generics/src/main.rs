//////////////////////
//////////////////////
////////      Generics use case        ////////
//////////////////////
/////////////////////


// fn largest_i32(list: &[i32]) -> &i32 {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn largest_char(list: &[char]) -> &char {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let result = largest_i32(&number_list);
//     println!("The largest number is {result}");

//     let char_list = vec!['y', 'm', 'a', 'q'];

//     let result = largest_char(&char_list);
//     println!("The largest char is {result}");
// }


// use std::cmp::PartialOrd;
// // in the previous code we see we need to create same func for diff types of data
// // but generics solves this porblem using <T>
// fn largest<T: PartialOrd> (list: &[T]) -> &T{

//     let mut res = &list[0];

//     // but this simply gives error, because there are some tyoes which are not comparable without knwoing some specific rules
//     for item in list{
//         if item > res{
//             res = item
//         }
//     }

//     // so we have to use PartialOrd rule here
//     // a trait which we need to imp to comparison


//   res
// }

// fn main(){
//     let number_list = [1,2,3,4];

//     let result = largest(&number_list);

//     println!("result is {result}");

//     let char_list = ['a','b','c','d'];

//     let result = largest(&char_list);

//     println!("result is {result}");
// }










// //////////////////////
// //////////////////////
// ////////        Generics with structs      ////////
// //////////////////////
// /////////////////////


// // taking similar types in struct
// struct Point<T> {
//     x:T,
//     y:T,
// }

// // taking diff types in struct
// struct Point2<T, U>{
//     x: T,  // just simple variables
//     y: U,  // just simple variables
// }

// fn main(){
//     let point_a = Point{
//         x: 1,
//         y: 2,
//     };

//     let point_b = Point{
//         x: 1.0,
//         y: 2.0,
//     };

//     let point_c = Point2{
//         x: 1,
//         y: 2.0,
//     };
// }






//////////////////////
//////////////////////
////////     Generics with enums         ////////
//////////////////////
/////////////////////

// enum Option<T> {
//     Some(T),
//     None,
// }

// enum Result<T>{
//     Ok(T),
//     Err(E),
// }


//////////////////////
//////////////////////
////////     Generics with impl         ////////
//////////////////////
/////////////////////

#[derive(Debug)]
struct Point<T, U>{
    x: T,  // just simple variables
    y: U,  // just simple variables
}

// why two times written generics
// first to declare the generic type
// second to use the type in struct
impl<T,U> Point<T,U> {

    fn new(x: T,y: U) -> Self{
        Self {x,y}
    }

}

fn main(){

    let point_a = Point::new(1,2);

    println!("point_a is {:?}", point_a);
}