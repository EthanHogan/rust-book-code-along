fn main() {
    // 4.2, References and Borrowing
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    fn calculate_length(s: &String) -> usize {
        s.len()
    }
    // fn change(some_string: &String) {
    //     // some_string.push_str(", world"); // cannot borrow `*some_string` as mutable, as it is behind a `&` reference
    // }

    // 4.2, Mutable References
    let mut s = String::from("hello");
    let mut n = 0;
    change(&mut s);
    println!("s is {}", s);

    add_one(&mut n);
    println!("n is {}", n);

    fn change(some_string: &mut String) {
        some_string.push_str(", world");
    }
    fn add_one(some_int: &mut i32) {
        *some_int += 1;
    }

    // let mut s = String::from("hello");

    // let r1 = &mut s;
    // let r2 = &mut s; // cannot borrow `s` as mutable moret than once at a time

    // println!("{}, {}", r1, r2);

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
                 // let r3 = &mut s; // BIG PROB
    println!("{} and {}", r1, r2);

    let r3 = &mut s;
    println!("{}", r3);

    // 4.2, Dangling References
    // let reference_to_nothing = dangle();
    let reference_to_something = no_dangle();
    println!("reference_to_something is: {}", reference_to_something);

    // fn dangle() -> &String {
    //     let s = String::from("hello");
    //     &s
    // }

    fn no_dangle() -> String {
        String::from("hello")
    }
}
