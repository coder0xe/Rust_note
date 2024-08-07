use std::io; // prelude 标准io库
use rand::Rng; // trait 接口
use std::cmp::Ordering; // enum

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("the secret number is {}", secret_number);

    loop {
        println!("guess a number!"); 
        // ！表示println是一个宏

        let mut guess = String::new(); // let 声明变量 
        // rust中变量默认为不可变的 immutable 如果需要变量，则可以增加mut关键字
        
        io::stdin().read_line(&mut guess).expect("can not read a number"); // 方法参数按照引用传递
        // io::Result Ok,Err
        
        println!("Your number is {}", guess);
        // {}表示占位符 其中的内容为guess

        // let guess:u32 = guess.trim().parse().expect("Form Err! Please type a new number!");
        // 进行类型转换 将字符串类型String转换为u32
        // 可以使用原变量名 对旧变量进行隐藏

        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // 使用match来解决异常 
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too SMALL!"),
            Ordering::Greater => println!("Too BIG!"),
            Ordering::Equal => {
                println!("YOU WIN!");
                break;
            }
        }
    }
    
}
