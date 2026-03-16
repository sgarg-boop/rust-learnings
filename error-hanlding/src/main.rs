use std::fs::File;
use std::io::ErrorKind;

fn main() {

    // try to hanlde recoverable error using Result

    // let res = match divide(4,0){
    //     Ok(res) => res,
    //     Err(err) =>{
    //         println!("Error: {}", err);
    //         -1
    //     }
    // };
    // println!("res is {:?}", res);


    // file opening and creating
    let file_open_res = File::open("hello.txt");

    let file_res = match file_open_res{
        Ok(file) => file,
        Err(error) => match error.kind(){
            ErrorKind::NotFound => match File::create("hello.txt"){
                Ok(fc) => fc,
                Err(e) => panic!("problem creating file: {:?}", e),
            }
            other_error => {
                panic!("problem opening file: {:?}", other_error);
            }
        }
    };

}
// fn divide (a: i32, b:i32) -> Result<i32, String>{
//     if b ==0 {
//         return Err(String::from("Cannot divide by zero"));
//     }
//     Ok(a/b)
// }
