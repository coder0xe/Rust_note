use std::ops::Deref;

struct MyBox<T> (T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    // let x = MyBox::new(1);
    // assert_eq!(*x, 1);
    let s = MyBox::new(String::from("World"));
    hello(&s)
}

fn hello(s: &str) {
    println!("Hello {}", s);
}
