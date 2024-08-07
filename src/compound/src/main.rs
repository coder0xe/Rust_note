fn main() {
    let x : i32 = add(1,2);
    println!("x = {}", x);
    let y = {
        let a = 1;
        a + 2; // statement
    };
    println!("y = {}", y); // error
    let z = {
        let a = 1;
        a + 2 // expression
    };
    println!("z = {}", z);
}

fn add(a:i32,b:i32) -> i32 {
    a + b
}
