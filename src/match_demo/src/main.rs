// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(State),
// }


// #[derive(Debug)]
// enum State {
//     Alaska,
//     Alabama,
// }


// fn value_in_cents(coin : Coin) -> u8 {
//     match coin {
//         Coin::Penny => {
//             println!("Penny!");
//             1
//         },
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("Quarter from {:#?}", state);
//             25
//         },
//     }
// }

// fn main() {
//     value_in_cents(Coin::Quarter(State::Alaska));
// }


// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         None => None,
//         Some(i) => Some(i + 1),
//     }
// }

// fn main() {
//     let five = Some(5);
//     let six = plus_one(five);
//     let none = plus_one(None);
// } 

fn main() {
    let one:Option<i32> = Some(1);
    match one {
        Some(1) => println!("1"),
        Some(2) => println!("2"),
        Some(3) => println!("3"),
        _ => println!("useless number"),
    }
}