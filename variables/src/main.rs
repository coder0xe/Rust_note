fn main() {
    let _guess:isize = "42".parse().expect("Not a number!");
    let x = 2.0;
    let y = true;
    let z = 'z';
    // how to type a emoji
    // list all emoji
    // https://emojipedia.org/
    let emoji = '😻';
    let tup = (500, 6.4, 1);
    let (a, b, c) = tup;
    println!("The value of a is: {}", a);
    println!("The value of b is: {}", b);
    println!("The value of c is: {}", c);
    println!("The value of tup.0 is: {}", tup.0);
    println!("The value of tup.1 is: {}", tup.1);
    println!("The value of tup.2 is: {}", tup.2);

    let arr = [1, 2, 3, 4, 5];
    let index = [5, 6, 7, 8, 9];
    println!("The value of arr[5] is: {}", arr[index[1]]);
}

