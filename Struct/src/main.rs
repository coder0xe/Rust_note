// struct User{
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
// }

// fn build_user(email: String, username: String) -> User {
//     User {
//         email,
//         username,
//         active: true,
//         sign_in_count: 1,
//     }
// }

// fn main() {
//     let user1 = User {
//         email: String::from("abc@126.com"),
//         username: String::from("abc"),
//         active: true,
//         sign_in_count: 1,
//     };

//     let user2 = User {
//         email: String::from("2237362@buaa.edu.cn"),
//         username: String::from("2237362"),
//         ..user1
//     }
// }
#[derive(Debug)]
struct Rectangle {
    width : u32,
    length : u32,
}

fn main() {
    let w = 30;
    let l = 50;
    let rec = Rectangle {
        width : w,
        length : l,
    };

    println!("{:#?}", rec);

    println!("{}", area(&rec));

}

fn area(rect : &Rectangle) -> u32 {
    rect.width * rect.length
}