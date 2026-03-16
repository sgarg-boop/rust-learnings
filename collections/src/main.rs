//////////////////////
//////////////////////
////////     vectors         ////////
//////////////////////
/////////////////////

// fn main(){

//     // 1st way to declare vector
//     // by default Vec<i32> type
//     // ele in vector must have same type
//     let mut numbers = Vec::new();
//     numbers.push(1);
//     numbers.push(2);
//     numbers.push(3);
//     println!("numbers are {:?}", numbers);


//     // 2nd way to declare vector
//     // vec is macro in rust
//     // shortcut to create vector in rust

//     let mut numbers = vec![1,2,3];
//     println!("numbers are {:?}", numbers);

//     // reading ele in vector
//     // 2 ways: via indexing or using get method
//     // via indexing we can use as ref
//     // via get method it will return type Option-> need to use match case

//     // indexing example
//     let third = numbers[2];
//     println!("third is {}", third);

//     // but if we try to get out of bound ele, then it will crash
//     // to handle this we can use get() which returns Option
//     let fourth = numbers.get(39);

//     match fourth{
//         Some(fourth) => println!("fourth ele is {}", fourth),
//         None => println!("No fourth ele"),
//     }

//     // iterating values in vector
//     for i in &numbers{
//         println!("i is {}", i);
//     }

//     // if we want to change value in vector
//     // we need to access de-ref value (becuse ref value is address and we need value)
//     for i in &mut numbers{
//         *i += 1;
//     }

//     println!("numbers are {:?}", numbers);

//     // storing diff types of data in vectors
//     // create an enum and declare type of vector as enum


// }


//////////////////////
//////////////////////
////////     Strings         ////////
//////////////////////
/////////////////////

// fn main(){

//     let s1 = String::from("hello");
//     let s2 = String::from ("world");

//     // because in add fucntion one should be string and other string slice
//     // let s3: String = s1 + &s2;  // s1 is moved to s3 and s2 is borrowed
//     // println!("s3 is {}", s3);

//     // we can use format macro insted of this
//     let s3 = String::from("duniya");

//     // there is one more pro here : it takes ref, no ownership is taken
//     let s4 = format!("{}-{}-{}", s1, s2, s3);
//     println!("s4 is {}", s4);
// }



//////////////////////
//////////////////////
////////     hashmaps         ////////
//////////////////////
/////////////////////

use std::collections::HashMap;

fn main(){

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 20);

    println!("scores are {:?}", scores);
    println!("Blue team has {} points", scores.get("Red").unwrap_or(&0));

    // for loop in hashmaps
    for (key, value) in &scores{
        println!("{} -> {}", key, value);
    }

    // we can use or_insert function

    // count occ of words in string
    let str = "Hello world Hello Rust";

    let mut map = HashMap::new();

    for word in str.split_whitespace(){
        let count = map.entry(word).or_insert(0);
        *count +=1;
    }

    println!("map is {:?}", map);


}
