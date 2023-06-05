fn main() {
    // 4.3, The Slice Type
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear();

    println!("word is {} and s is {}", word, s);

    // 4.3, String Slices
    let s = String::from("hello world");

    let hello = &s[0..5]; // same as: &s[..5];
    let world = &s[6..11];

    println!("{} {}", hello, world);
    let some = String::from("four");

    let len = some.len();
    let four = &some[0..len]; // == &s[..len] == &s[0..] == &s[..]

    let first = &some[0..1];
    let second = &some[1..2];
    let third = &some[2..3];
    let fourth = &some[3..4];
    // let fifth = &some[4..5]; // out of bounds error;

    println!("four: {}", four);
    println!("first: {}", first);
    println!("second: {}", second);
    println!("third: {}", third);
    println!("fourth: {}", fourth);
    // println!("fifth: {}", fifth);

    println!("first word is {}", first_word_again(&s));
    println!("second word is {}", second_word(&s));

    let s = String::from("hello world");

    let word = first_word_again(&s);

    // s.clear(); // error! cannot borrow `s` as mutable because it is also borrowed immutable

    println!("the first word is: {}", word);

    // 4.3, String Literals as Slices

    // 4.3, String Slices as Parameters

    // 4.3, Other Slices
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_again(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    s
}

fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    let mut indices_of_spaces = Vec::new();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            indices_of_spaces.push(i);
            if indices_of_spaces.len() == 2 {
                let first_space = indices_of_spaces[0] + 1; // skip the space itself
                return &s[first_space..i];
            }
        }
    }
    if indices_of_spaces.len() == 1 {
        let first_space = indices_of_spaces[0] + 1; // skip the space itself
        return &s[first_space..];
    }
    s
}
