

// 1) read entire file as string
 // return Type: Result<String, Err>
 // use this when file is small

use std::fs::File;
use std::io::Write;
use std::io::Result;

// fn main() {
//     // path we need to mention here like ../ because file is present outside
//     let content = fs::read_to_string("../help.txt")
//         .expect("Failed to read file");

//     println!("{}", content);
// }

//2) Opening a file
// give some result like fd, file location, read-write permissions
fn main() -> Result<()>{
    let file = File::open("output.txt");

    match file{
        Ok(file) => println!{"File is {:?}", file},
        Err(e) => println!("Error: {:?}", e),
    }
    Ok(())
}

// there is one long way, if need control over content we use that

fn main() -> Result<()> {

    // question mark ?
    // is used for If Ok(file) → continue
    // If Err(e) → exit function immediately
    let mut file = File::open("example.txt")?;


    // just creating a new empty string in whcih content will be append
    let mut contents = String::new();

    // rust reads bytes from files
    // converts bytes to string
    file.read_to_string(&mut contents)?;

    // print the content
    println!("File contents:\n{}", contents);

    // return types for result (Neccessary)
    Ok(())
}

// // 3) Writing data to file
// fn main(){
// let mut file = fs::File::create("output.txt").unwrap();

// // using b we need to convert to bytes, because we cant write strings to files in computer
// // can easily append data
// file.write_all(b"Hello World\n This is Shivansh Here").unwrap();

// // one more way to write the file
// // Cons: overwrites file everytime, no append and all
// fs::write("output2.txt", "Hello writing again in new file\n By new method");
// }


// 4) Deleting or removing a file
use std::fs;

fn main() -> std::io::Result<()> {
    fs::remove_file("output.txt")?;
    println!("File deleted successfully.");
    Ok(())
}