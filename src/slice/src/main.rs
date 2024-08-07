// fn main() {
//     let mut s = String::from("hello world");
//     let word = first_word(&s);
//     s.clear();
//     println!("{}", word);
// }

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }
//     s.len()
// }

// fn main() {
//     let s = String::from("hello world");
//     let hello = &s[..5];
//     let world = &s[6..];
//     let whole = &s[..];
//     println!("{} {} {}", hello, world, whole);
// }

// fn main() {
//     let s = String::from("hello world");
//     let hello = first_word(&s);
//     println!("{}", hello);
// }

// fn first_word(s: &String) -> &str { // return a string slice
//     let bytes = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[..i];
//         }
//     }
//     &s[..]
// }

// fn main() {
//     let s = String::from("hello world");
//     let hello = first_word(&s);
//     let hello1 = first_word(&s[..]);
//     println!("{} {}", hello, hello1);
//     let s1 = "hello world";
//     let hello2 = first_word(s1);
//     println!("{}", hello2);

// }

// fn first_word(s: &str) -> &str { 
//     let bytes = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[..i];
//         }
//     }
//     &s[..]
// }

fn main() {
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    for i in slice {
        println!("{}", i);
    }
}