fn main() {
    let one:Option<i32> = Some(1);
    if let Some(1) = one {
        println!("let if 1");
    } else {
        println!("let if useless number");
    }

    match one {
        Some(1) => println!("match 1"),
        _ => println!("match useless number"),
    }
}
