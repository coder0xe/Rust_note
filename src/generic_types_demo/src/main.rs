struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn mixup<V, W> (self, other : Point<V, W>) -> Point<T, W> {
        Point {
            x : self.x,
            y : other.y,
        }
    }
}

impl Point<i32, i32> {
    fn x1(&self) -> &i32 {
        &self.x
    }
}

// enum Option<T> {
//     Some(T),
//     None,
// }

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

fn main () {
    let integer = Point{x: 1, y: 2};
    let float = Point{x: 1.1, y: 2.2};
    let point = integer.mixup(float);
    println!("x is {}, y is {}", point.x, point.y);
}
