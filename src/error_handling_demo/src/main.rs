use std::collections::btree_map::Values;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::Read;

// fn read_username_from_file() -> Result<String, io::Error> { // Result <T,E>
//     let f = File::open("hello.txt");
//     let mut f = match f {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };
//     let mut s = String::new();
//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => Err(e),
//     }
// }

// fn read_username_from_file() -> Result<String, io::Error> { // Result <T,E>
//     let mut f = File::open("hello.txt")?;
//     let mut s = String::new();
//     f.read_to_string(&mut s)?;
//     Ok(s)
// }

// fn read_username_from_file() -> Result<String, io::Error> { // Result <T,E>
//     let mut s = String::new();
//     File::open("hello.txt")?.read_to_string(&mut s)?; // 链式调用
//     Ok(s)
// }

pub struct Guess {
    value : i32,
}

impl Guess {
    pub fn new(value : i32) -> Guess { // constructer
        if !(1..=100).contains(&value) {
            panic!("invalid number : not in the field [1,100]");
        }

        Guess {value}
    } 

    pub fn value(&self) -> i32 { // operation
        self.value
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // let _result = read_username_from_file();
    // println!("{:?}", _result);
    let f = File::open("hello.txt")?;
    Ok(())
    // panic!("crash and burn");
    // let v = vec![1, 2, 3];
    // v[99];
    // let f = match File::open("hello.txt") {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Failed to create file : {:?}", e),
    //         }
    //         other_error => panic!("Failed to open the file : {:?}", other_error),
    //     }
    // };
    // let f = File::open("hello.txt").unwrap();
    // let f = File::open("hello.txt").expect("What the hell?");
}
