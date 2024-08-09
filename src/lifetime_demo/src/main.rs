// fn main() {
//     let x = "hello world";
//     let y = "brother YUAN is a thief";
//     let z = longest(x, y);
//     println!("{}", z);
// }

// fn longest<'a> (x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl <'a> ImportantExcerpt<'a>{
    fn level(&self) -> i32 {  // lifetime of operation's return value
        3
    }
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please : {}", announcement);
        self.part
    }
}

fn main() {
    let s: &'static str = "hello";
}