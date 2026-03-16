// struct User{
//     username : String,
//     email : String,
//     sign_in_count : u64,
//     active : bool,
// }

// fn main() {

//     let mut user1 = User{
//         email : String::from("sgarg@sg.com"),
//         username : String::from("sgarg"),
//         active : true,
//         sign_in_count : 1,
//     };

//     let name = user1.username;
//     user1.username = String::from("new_name");

//     println!("name is {}", name);
//     println!("user1 is {}", user1.username);

// }

// struct Rectangle{
//     width : u32,
//     height : u32,
// }

// fn main(){

//     let rect = Rectangle{
//         width : 10,
//         height : 20,
//     };

//     println!("area is {}", area(rect));

// }

// fn area(rec: Rectangle) -> u32 {

//         rec.width * rec.height
// }

// debug mode in rust

#[derive(Debug)]
struct Rectangle{
    width : u32,
    height : u32,
}

fn main(){

    let rect = Rectangle{
        width : 10,
        height : 20,
    };

    let area = area(&rect);

    println!("Area for  {:?} is {}", rect,area );


}

fn area(rec: &Rectangle) -> u32 {

        rec.width * rec.height
}
