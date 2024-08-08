use std::collections::{HashMap};

fn main() {
    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);
    // let v = vec![String::from("Blue"), String::from("Yellow")];
    // let v1 = vec![10,50];
    // let scores = v.iter().zip(v1.iter());
    // let h: HashMap<_, _> = scores.collect();
    // for i in h {
    //     println!("{:?}", i);
    // }
    // let mut h = HashMap::new();
    // let s1 = String::from("hello");
    // let s2 = String::from("world");
    // h.insert(&s1, &s2);
    // println!("s1 is {}, s2 is {}", s1, s2);
    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);
    // scores.insert(String::from("Blue"), 20);
    // for (k,v) in &scores {
    //     println!("k is {}, v is {}", k, v);
    // }
    // match scores.get("Red") {
    //     Some(_) => println!("Find!"),
    //     None => println!("Can't find!"),
    // }
    // let e = scores.entry(String::from("Blue"));
    // println!("{:?}", e);

    let str = "hello dqr from SCSE BUAA";
    let mut map = HashMap::new();
    for word in str.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    for (k,v) in map {
        println!("k is {}, v is {}", k, v);
    }
}
