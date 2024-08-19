extern "C" {
    fn abs(x: i32) -> i32;
}
fn main() {
    unsafe {
        println!("abs of -1 is {}", abs(-1));
    }
}