fn main() {
    // let mut s = String::new();

    // let s1 = "initial contents".to_string();
    // let s2 = String::from("initial contents");
    // let mut s = String::from("foo");
    // let s1 = String::from("bar");
    //s.push_str(&s1);
    //s.push('l');
    //s += &s1;
    // let s = format!("{}-{}",s,s1);
    // println!("{}", s);
    let s1 = String::from("hello world");
    for i in s1.chars() {
        println!("{}", i);
    }
    for i in s1.bytes() {
        println!("{}", i);
    }
}
