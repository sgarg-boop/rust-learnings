fn main() {
    // if else
    // let number = 10;

    // if number > 5 {
    //     println!("Number is greater than 5");
    // } else {
    //     println!("Number is 5 or less");
    // }

    // // using if as an expression
    // // expressions return something

    // let y = is_even(4);

    // let x = if y {10} else {20};
    // println!("x is {}", x);





    ///////////
    //Loops //
    ///////////

    /**
     * 1) loop (infinite)
     */

    // loop{
    //     println!("Hello World");
    // }

    // let mut num = 1;
    // loop{
    //     println!("num is {}", num);

    //     if num == 10{
    //         break;
    //     }

    //     num+=1;
    // }
    // println!(" this is end");

    // considering loop as a result
    // let mut counter = 0;

    // let result = loop{

    //     counter += 1;

    //     if counter == 10{
    //         break counter * 2;
    //     }

    // };
    // println!("result is {}", result);


    /**
     * 2) break and continue
     */
    // for i in 1..10{
    //     if i == 5{
    //         continue;
    //     }
    //     if i == 7{
    //         break;
    //     }
    //     println!("i is {}", i);
    // }

    ///////
    //Loop Labels (used for nested loops) //
    // Desc: In rust loops labels are used when you have nested loops and you want to control which loop shoiuld break or continue
    // what is loop label: a name given to a loop, the label starts with ' and ends with '
    ///////

    // breaking an outer loop
        'outer: for i in 1..5{
            for j in 1..5{
                if j==3{
                    break 'outer;
                }
                println!("i = {}, j = {}", i, j);
            }
        }
    println!(" new loop started");
    // using continue with loops
    'outer: for i in 1..4{
        for j in 1..4{
            if j==2{
                continue 'outer;
            }
            println!("i ={}, j = {}", i, j);
        }
    }

    // 3) while loop with condition
    let mut num = 1;
    while num <= 10{
        println!("num is {}", num);
        num += 1;
    }

    // panic condition for while loop
    let arr = [1,2,35,46];
    let mut index = 0;

    // while index < 5{
    //     println!("i: {}, v: {}", index,arr[index]);
    //     index += 1;
    // }

    // the above thing gets panicked, and it is slow also because in each arm it has to check if index is less than 5
    // so we use for loop

    for x in arr{
        println!("x is {}", x);
    }

}


// fn is_even( x: i32 ) -> bool {
//     x % 2 == 0
// }