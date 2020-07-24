fn main() {
    // References and Borrowing
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let s = String::from("hello");

    change(&s);

    // Mutable References
    let mut s = String::from("hello");

    change_mut_reference(&mut s);
    println!("mut reference s = {}", s);

    let mut s = String::from("hello");

    /**
    let r1 = &mut s;
    let r2 = &mut s; // compile error,mutable references have one big restriction: you can have only one mutable reference to a particular piece of data in a particular scope.

    println!("{}, {}", r1, r2);
    */

    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);

    // Dangling References
    // let reference_to_nothing = dangle(); compile error
    let reference_to_nothing = no_dangle();
    println!("reference_to_nothing = {}", reference_to_nothing);
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
// it refers to, nothing happens.

fn change(some_string: &String) {
    // some_string.push_str(", world"); compile error Just as variables are immutable by default, so are references. We’re not allowed to modify something we have a reference to.
}

fn change_mut_reference(some_string: &mut String) {
    some_string.push_str(", world");
}

// compile error
//fn dangle() -> &String { // dangle returns a reference to a String
//    let s = String::from("hello"); // s is a new String
//
//    &s // we return a reference to the String, s
//} // Here, s goes out of scope, and is dropped. Its memory goes away.
// Danger!

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}