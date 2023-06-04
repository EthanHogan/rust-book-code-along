fn main() {
    // 4.1, Variable Scope
    {
        // stored on Stack
        let s = "hello";
        println!("{s}");
    }
    // println!("{s}"); // cant print here. s not in scope

    // 4.1, The String Type
    // stored on Heap
    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("{}", s);

    // 4.1, Memory and Allocation
    {
        // stored on Heap
        let t = String::from("hello");
        println!("{t}");
    }
    // println!("{t}"); // cant print here t not in scope

    // 4.1, Variables and Data Interacting with Move
    // stored on Stack
    let x = 5;
    let y = x;
    println!("x is {} and y is {}", x, y);

    // stored on Heap
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("s1 is {s1}"); // cant do this because the value has moved from s1 to s2. s1 has already been GC
    println!("s2 us {s2}");

    // 4.1, Variables and Data Interacting with Clone
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    // 4.1, Stack-Only Data: Copy
    let x = 5;
    let mut y = x;
    // integers are stored on the stack
    y += 1;
    println!("x = {}, y = {}", x, y);

    // 4.1, Ownership and Functions
    let mut s = String::from("hello");
    s = takes_ownership(s);
    println!("some_string is: {}", s);

    let mut x = 5;
    x = makes_copy(x);
    x += 1;
    println!("x is now {x}");

    fn takes_ownership(some_string: String) -> String {
        println!("{}", some_string);
        String::from("goodbye")
    }

    fn makes_copy(mut some_integer: i32) -> i32 {
        some_integer += 1;
        println!("some_integer is now {}", some_integer);
        some_integer
    }

    // 4.1, Retrun Values and Scope

    let s1 = gives_ownership();
    println!("s1 is {s1}");

    let s2 = String::from("hello");
    println!("s2 is {s2}");
    let s3 = takes_and_gives_back(s2);
    // println!("s2 is now {s2}"); // cant print s2, ownership ahs been giving to takes_and_gives_back
    println!("s3 is {s3}");

    fn gives_ownership() -> String {
        String::from("yours")
    }

    fn takes_and_gives_back(a_string: String) -> String {
        // println!("does this have access to s2? s2 is: {s2}"); // no, can not access s2 here.
        a_string
    }

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);

    fn calculate_length(s: String) -> (String, usize) {
        let length = s.len();

        (s, length)
    }
}
