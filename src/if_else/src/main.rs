fn main() {
    let condition = true;

    let mut number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);

    let mut cnt = 0;
    let result = loop {
        cnt += 1;

        if cnt == 10 {
            break cnt * 2;
        }
    };
    println!("The result is: {}", result);

    while number != 0 {
        println!("{}", number);
        number -= 1;
    }

    let arr = [10, 20, 30, 40, 50];
    for element in arr.iter() {
        println!("The value is: {}", element);
    }
    
    for i in (1..4).rev() {
        println!("{}", i);
    }
}
