// enum IpAddrKind {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

// fn main() {
//     let home = IpAddrKind::V4(127,0,0,1);
//     let loopback = IpAddrKind::V6(String::from("::1"));
// }

// 标准库中的IpAddr类型
// struct Ipv4Addr {
//     // code
// }

// struct Ipv6Addr {
//     //code
// }

// enum IpAddr {
//     V4(Ipv4Addr),
//     V6(Ipv6Addr),
// }

// enum Message {
//     Quit,
//     Move(i32, i32),
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// impl Message {
//     fn call(&self) {

//     }
// }

// fn main () {
//     let q = Message::Quit;
//     let m = Message::Move(12,24);
//     let w = Message::Write(String::from("Hello"));
//     let c = Message::ChangeColor(0,255,255);

//     m.call();
// }

fn main () {
    // let some_number = Some(5);
    // let some_string = Some("A String");
    // let absent_number: Option<i32> = None;

    let x : i8 = 5;
    let y : Option<i8> = Some(5);
    let z : Option<i8> = None;

    let mut sum = x + y.unwrap();
    if !z.is_none() {
        sum += z.unwrap();
    } else {
        println!("z is None");
    }

    println!("sum is {}", sum);
}