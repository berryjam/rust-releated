fn main() {
    let word = String::from("hello world");
    println!("first_word of s = {} , offset = {}", word, first_word(&word));

    // String Slices
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{} {}", hello, world);

    let s = String::from("hello");

    let slice = &s[0..2];
    let slice = &s[..2];

    let s = String::from("hello");

    let len = s.len();

    let slice = &s[3..len];
    let slice = &s[3..];

    let s = String::from("hello");

    let len = s.len();

    let slice = &s[0..len];
    let slice = &s[..];

    let mut s = String::from("hello world");

    let word = first_word(&s);

//    s.clear(); // error!

    println!("the first word is: {}", word);

    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word works on slices of string literal
    let word = first_word(&my_string_literal[..]);

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}