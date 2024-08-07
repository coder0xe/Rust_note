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

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.length
    }

    fn can_hold(&self, other : &Rectangle) -> bool {
        self.width > other.width && self.length > other.length
    }

    fn square(size : u32) -> Rectangle {
        Rectangle {
            width : size,
            length : size,
        }
    }
}

fn main() {
    let rec = Rectangle {
        width :40,
        length : 50,
    };

    let rec1 = Rectangle {
        width : 20,
        length : 30,
    };

    let s = Rectangle::square(20);

    println!("rec can hold rec1 : {}", rec.can_hold(&rec1));

    println!("area is {}", rec.area());

    println!("square : {:#?}", s);

}