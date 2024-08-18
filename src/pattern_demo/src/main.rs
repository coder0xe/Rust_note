// fn main() {
//     let x = 3;

//     match x {
//         1 | 2 => println!("one or two"),
//         y => println!("{}", y),
//     }

//     let y = 'j';

//     match y {
//         'a'..'j' => println!("early ASCII"),
//         'j'..='z' => println!("lately ASCII"),
//         _ => println!("else"),
//     }
// }
// struct Point {
//     x: u32,
//     y: u32,
// }

// fn main () {
//     let p = Point{x: 1, y: 2};
//     let Point{x: a, y: b} = p;
//     assert_eq!(a, 1);
//     assert_eq!(b, 2);

//     let Point{x, y} = p;
//     assert_eq!(x, 1);
//     assert_eq!(y, 2);

//     match p {
//         Point{x:0, y} => println!("x is 0"),
//         Point{x, y: 0} => println!("y is 0"),
//         Point { x, y } => println!("Normal"),
//     }
// }

// fn main() {
//     let a = (1,2,3,4,5);
//     match a {
//         (first, .., last) => {
//             println!("first is {}", first);
//         }
//     }
// }

fn main() {
    let num = Some(3);

    match num {
        Some(x) if x < 5 => {
            println!("less than 5");
        },
        _ => {
            println!("eg 5");
        }
    }
}