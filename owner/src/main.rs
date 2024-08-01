// fn main() {
//     // s is not valid here, it’s not yet declared
//     let s = "hello"; // s is valid from this point forward
//                            // do stuff with s
//     println!("{}", s);
// } // this scope is now over, and s is no longer valid

// fn main() {
//     let mut s = String::from("hello"); // s comes into scope

//     s.push_str(", world!"); // push_str() appends a literal to a String

//     println!("{}", s); // This will print `hello, world!`

//     // let s2 = s; // s2 is a copy of the pointer, length, and capacity of s
//     // println!("{}", s); // This will throw an error because s has been moved to s2\

//     let s2 = s.clone(); // s2 is a copy of the pointer, length, and capacity of s
//     println!("s: {}, s2: {}", s, s2); // This will print `hello, world!` twice
// }

// fn main() {
//     let x = 5;
//     let y = x;
//     println!("x: {}, y: {}", x, y);
// }

// fn main() {
//     let s = String::from("hello"); // s comes into scope
//     takes_ownership(s); // s's value moves into the function and so is no longer valid here
//     // println!("{}", s); // This will throw an error because s has been moved to the function

//     let x = 5; // x comes into scope
//     makes_copy(x); // x would move into the function, but i32 is Copy, so it’s okay to still use x afterward
//     println!("{}", x); // This will print `5`

// }

// fn takes_ownership(some_string: String) { // some_string comes into scope
//     println!("{}", some_string);
// } // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

// fn makes_copy(some_integer: i32) { // some_integer comes into scope
//     println!("{}", some_integer);
// } // Here, some_integer goes out of scope. Nothing special happens.

// fn main() {
//     let s1 = gives_ownership(); // gives_ownership moves its return value into s1
//     let s2 = String::from("hello"); // s2 comes into scope

//     let s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back, which also moves its return value into s3
//     println!("s1: {}, s3: {}", s1, s3);
// }

// fn gives_ownership() -> String {
//     let some_string = String::from("hello");
//     some_string
// }

// fn takes_and_gives_back(a_string: String) -> String {
//     a_string
// }

fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}