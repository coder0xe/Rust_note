# <div align = "center">Rust学习笔记</div>

> * Ubuntu22.04 
> * Rust 1.8.0

## 1. Hello World

* ``rust``文件以``.rs``为后缀
* 编译``rust``代码需要使用``rustc filename.rs``进行编译
* 但``rustc``只适合简单的``rust``程序

```rust
fn main() {
    println!("hello world");
}
```

## 2. Cargo

### 2.1 Cargo创建项目

* ``Cargo``是``Rust``的构建系统和包管理工具：构建代码、下载依赖的库、构建这些库

  * 这里我的版本为``1.80.0``

* 使用``Cargo``创建项目

  ```shell
  cargo new project_name
  ```

* 使用``Cargo``创建的项目下有一个``src``文件夹(下有``main.rs``)和``Cargo.toml``

### 2.2 Cargo.toml

* TOML(Tom's Obivious, Minimal Language)，是Cargo的配置格式

  ```toml
  [package]
  name = "rust"
  version = "0.1.0"
  edition = "2021"
  
  [dependencies]
  ```

* package下方的内容使用来配置包(package)的

* dependencies中列出项目的依赖项

* **在Rust中，代码的包称为crate**

### 2.3 Cargo构建项目

> 在项目根目录下执行命令

* ``cargo build``：编译，可执行文件生成在``target/debug``目录下
* ```cargo run```：编译+执行
* 第一次运行会在顶层生成```cargo.lock```文件

* ``cargo check``：检查代码能否通过编译而不产生可执行文件，比``cargo build``速度更快

* 默认情况下``cargo build``用于开发时构建项目，当需要发布程序时，使用``cargo build --release``(加一个参数)
  * 代码运行时间更短但编译时间变长
  * 可执行文件生成在``target/release``目录下

## 3.猜数游戏

### 3.1 增加依赖项

​	我们需要增加一个随机数包``rand``，此时需要在项目的``Cargo.toml``中的``[dependencies]``增加对应的包以及版本(这里Dependi插件会进行版本检查，可以使用``^``表示兼容此版本的其他版本都可以)

```toml
# Cargo.toml
[dependencies]
rand = "^0.8.5"
```

​	cargo会下载对应的包以及该包依赖的其他包，并在编译时进行整合。cargo提供了包环境的管理，可以发现在``Cargo.lock``中有这些包的版本信息等，当其他人运行我们的代码时，**通过``Cargo.lock``一定会得到相同的复现结果。**

```toml
# Cargo.lock 
# 以rand为例
[[package]]
name = "rand"
version = "0.8.5"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "34af8d1a0e25924bc5b7c43c079c942339d8f0a8b57c39049bef581b46327404"
dependencies = [
 "libc",
 "rand_chacha",
 "rand_core",
]
```

​	如果需要更新package,可以使用``cargo update``进行index表的更新，之后使用index表进行包的安装而忽略``Cargo.lock``，最后将新的版本信息更新到``Cargo.lock``中，但不会更新到``Cargo.toml``中。

### 3.2 code

```rust
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
```

## 4. Rust基础



## TIPS: useful plugins for RUST

* rust analyzer：Run/Debug(当然可以通过命令行)
* Code LLDB：代码调试
* Even Better TOML：编辑.toml文件
* Dependi：管理crates版本

* git-commit-plugin：规范提交信息