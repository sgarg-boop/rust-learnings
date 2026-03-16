

1) packages: folde name
2) creates:
the smallest things that rust code can compile
it is the actual rust code complied ino a bianry or library

2.1) binary create
produces an external program
fn main() {
    println!("Hello Rust");
}
it must have a mian fucntion because this code is executable 
src/main.rs: becomes a runnable program

2.2) library create
produces a library that can be used by other programs
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
src/lib.rs: becomes a library


relation b.w pkg and crate
1) pkg can have multiple creates
2) 0 or 1 lib create
3) multiple bianry crates

my_project/
 ├── Cargo.toml
 └── src
      ├── main.rs      → binary crate
      ├── lib.rs       → library crate
      └── bin
           ├── tool1.rs → binary crate
           └── tool2.rs → binary crate



when we create any module, it will be found at {}, src/file_name.rs , foldername/mod.rs
writing crate:: (in use as import) will start finding from root crate

modules:
1) there are two types of paths

1.1)abs: starting from crate and then the path
use crate::a::b

1.2)relative: start from current and then the path
super::a::b

mod front_of_house{
    mod hosting{
        fn add_to_waitlist(){}
    }
}

pub fn eat_at_res(){

    // abs
    crate::front_of_house::hosting::add_to_waitlist();

    // relative
    front_of_house::hosting::add_to_waitlist();
}


2) if using abs path, we need to amke thgings pub
3) if using relative path we can use private things at same level
4) super is used to cll one upside level func