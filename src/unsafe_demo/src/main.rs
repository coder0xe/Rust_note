unsafe fn danger() {
    println!("This function is dangerous");
}

extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    unsafe {
        println!("r1: {}", *r1);
        println!("r2: {}", *r2);
        danger();
    }
    let a = -1;
    unsafe {
        println!("{}", abs(a));
    }
}
