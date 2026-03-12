    1) Rust is a statically typed language
    2) Like In js we can assign any type of value like if an int is there we can also pass string to it afterwards
    3) But rust is static:
        a. We can change data type for the variables
    4) If parsing data type, we must assign annotations, because it should know to which type we need to convert it
    5) Scalar types
        a. Rep single value -> means no array or list
        b. Rust has 4 primary
            i. Int
            ii. Floating points
            iii. Bools
            iv. Chars
    6) Compound types
        a. Arrays , list, tuples
    7) We can't assign positive or negative num to unsigned types
        a. Most significant bit if 1 -> number is negative ( 1 num)
    8) For signed we can store the values upto
        a. If i8
            i. -2 raise power 7   to 2 raise power 7 -1
            ii. Equals -128 to 127
    9) For unsigned
        a. 0 to 2 raise power n  - 1
    10)
    11) If we write b at front it will give us binary value
        a. Like b'A' will give 65 (byte u8 only)
        b. Like 0b1010 will convert it to decimal base 2
            i. Will give 10
    12) Integer overflow
        a. Cargo run : debug mode
        b. If any int gets overflow at any runtime code gets panicked: during cargo run
        c. During cargo run --release
            i. If range is 255 (u8)
            ii. It wraps to new things
            iii. Like 256 becomes 0
            iv. 257 becomes 1 and so on
        d. The followed approach for cargo run is to write
            i. .wrapping_add
    13) Floating point types
        a. F32 and f64
        b. Default: 64
        c. All floating are signed
    14) Boolean type
        a. Symbol: bool
    15) Character types
        a. Let c = 'p'
        b. Char has single quotes

Till here we have completed scalar data types
Now going forward with compound

    1) Tuples
        a. We can add multiple types of data in one
        b. Fixed length
        c. Let tup: (type1, type2, type3) = ();
        d. The tuple without any values has a special name unit
            i. If a function doesn’t return any data, it will return unit
            ii. Example: if any simple function printing hello world , it is returning a tuple even we don’t write there
    2) Arrays
        a. Arrays must have same type unlike in Js (any)
        b. Have fixed length
        c. Representation
            i. Let b: [i32; 5] = [10; 5]
            ii. Where 10 is number and 5 is size
            iii. If we place ; with , then it will treat that as array of  size 2
