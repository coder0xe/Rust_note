// fn main() {
//     let v1 = vec![1, 2, 3];
//     let va_iter = v1.iter();
//     for val in va_iter {
//         println!("{}", val);
//     }
// }

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let mut counter = Counter::new();
    assert_eq!(counter.next(), Some(1));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test01() {
        let v1 = vec![1, 2, 3];
        let mut va_iter = v1.iter();
        assert_eq!(va_iter.next(), Some(&1));
        assert_eq!(va_iter.next(), Some(&2));
        assert_eq!(va_iter.next(), Some(&3));
    }
}


