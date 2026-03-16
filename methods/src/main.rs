
#[derive(Debug)]
struct Rectangle{
    width : u32,
    height : u32,
}

impl Rectangle{

    // methdod 1
    fn area(&self) -> u32{
        self.width * self.height
    }
}

fn main(){

    let rect = Rectangle{
        width : 10,
        height : 20,
    };


    // use rect.area
    println!("Area for  {:?} is {}", rect,rect.area() );


}
