1) Rust has a memory management system called ownership system (very unique)
    Most languages use:
        • Garbage Collector → Java, Python
        • Manual memory management → C, C++
        a. Example
        fn main() {
            let s1 = String::from("hello");
            let s2 = s1;
            println!("{}", s1); // ❌ error
        }
        b. Ownership moved from s1 to s2
2) Borrow checker
        a. Types of borrow
            i. Immutable borrow
            ii. Mutable borrow
        b. Like in C and C++ allow problems like
            i. Dangling pointers
            ii. Data races
            iii. Double free
            iv. Memory corruption
        c. Rust prevents these things at compile time
            i. Borrowing means: Using a value without taking ownerships
            ii. Borrow check rule
                1) Either have many immutable ref (can't change or reassign value for a particular variable)
                2) Have one mutable ref
                3) But not both
            iii. Example
            let mut x = 10;
            let r1 = &x;
            let r2 = &mut x; // ❌ error
            iv. Here in example, r1 borrows x immutably (&x)
            v. After that r2 has a mutable borrow of x
            vi. Since, r1 still exists rust blocks the mutable borrow
            vii. Because if we try to read value of r1 and gets changed by r2 then it will cause issue (memory leak)
            viii. Someone reading the value while someone is modifying it (main prevention)
            ix. Many reads or one write is supported by rust (prevent logical inconsistency)
3) Enums with data
4) Rust does not have null pointers ( as of present in java, C++)
        a. Instead rust have (Option)
        b. Option has two things
            i. Some
            ii. None
        c. Most explicit way to write option is using match expression
            i. Other ways are if else
            ii. If let
            iii. Unwrap
5) Result type for error handling
        a. Safe error handling
        b. No hidden exceptions, everything need to write explicitly





