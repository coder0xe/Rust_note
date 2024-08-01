# <div align = "center">Rust学习笔记</div>

> * Ubuntu22.04 
> * Rust 1.8.0
> * 可以使用rustup doc查看官方文档(在安装rust时都已经下载到本地)
> * 使用firefox打开时会遇到权限错误，可以使用转移文件夹并与原文件夹进行软链接的方式解决，参考[askubuntu](https://askubuntu.com/questions/1419528/how-do-i-access-rust-documentation)

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

## 4. Rust基础

### 4.1 变量与可变性

* 声明变量使用``let``关键字

* 默认情况下，变量是不可变，**在声明变量时，在变量前加上``mut``就可以使变量可变**

  ```rust
  let mut x = 5;
  ```

* 声明常量使用``const``关键字，他的类型必须被标注

  * 常量可以在任何作用域中声明，包括全局作用域

  * 常量只可以绑定到常量表达式，无法绑定到函数的调用结果或之恩那个在运行时才能计算出的值

  * **命名规范：Rust中常量使用全大写字母，每个单词之间使用下划线分开**

    ```rust
    const MAX_POINTS: u32 = 100_000;
    ```

> Rust的编译器十分智能，对于使用let声明的变量，其变量类型会由编译器进行推断而不需要明确指出

* **Shadowing(隐藏)**
  * 在Rust中可以使用相同的变量名声明新的变量，新的变量会shadow之前的同名变量
  * 后续代码中变量为代表的是新变量
  * **shadow和把变量声明为mut是不同的**
    * 不使用let关键字，重新给非mut变量赋值产生编译错误
    * 使用let声明出的新变量也是不可变的
    * **使用let声明的同名新变量类型可以与之前不同**

### 4.2 数据类型

* **标量和复合类型**

* Rust是静态编译语言，在编译时必须知道所有变量的类型

  * **基于使用的值，Rust编译器通常能够推断出他的具体类型**

  * 但如果可能的类型比较多，例如把String转为整数的parse方法，就必须添加类型标注，否则可能出错，例如下例中42被推断为``u32/i32``都是合理的，需要手动指定

    ```rust
    let guess:i32= "42".parse().expect("Not a number!");
    ```

#### 4.2.1 标量类型

* 一个标量类型代表一个单个的值

* Rust有四个主要的标量类型

  * 整数类型

  * 浮点类型
  * 布尔类型
  * 字符类型

##### 4.2.1.1 整数类型

* 无符号``u``/有符号``i``
* 位宽：8/16/32/64/128/arch
* 关于arch：``isize/usize``位宽由计算机架构决定
* 整数字面值：Dec/Hex/Octal/Binary/Byte,可以使用下划线分割增强可读性
* 除了byte类型外，**数值字面值都允许使用类型后缀，例如10u32**
* 整数的默认类型为``i32``
* 整数溢出：
  * 在调试模式下编译(``build``)：``Rust``会检查整数溢出，如果发生溢出，程序在运行时就会``panic``
  * 在发布模式下编译(``build --release``)：``Rust``不会检查可能导致``panic``的整数溢出，如果发生溢出，``Rust``会进行环绕操作，例如256->0,257->1

##### 4.2.1.2 浮点类型

* 单精度/双精度：``f32/f64``
* 默认类型为``f64``（在现代CPU上f32与f64速度差不多，而且f64速度更快）
* 支持加减乘除余等``+-*/%``

##### 4.2.1.3 布尔类型

* true/false

##### 4.2.1.4 字符类型

* char类型被用来表述语言中最基础的单个字符

* **字符类型的字面值使用单引号**

* **与其他语言不同，Rust中字符类型占用4字节大小**

* Rust中字符类型是一个``Unicode``标量值，可以表示比ASCII多得多的字符内容：拼音、中日韩文、emoji表情等

  ```rust
  let emoji = '😻';
  ```

#### 4.2.2 复合类型

* 符合类型可以将多个值放在一个类型中
* Rust提供了两种基础的符合类型：元组/数组

##### 4.2.2.1 Tuple

* 将多个类型的多个值放在一个元组中

* Tuple长度是固定的，一旦声明就无法修改

* **创建tuple：在小括号中，将值用逗号分开，tuple中每个位置对应一个类型，其中各元素类型不必相同，可以显示指明每个元素的类型，也可以让编译器自动推断**

  ```rust
  let tup = (500, 6.4, 1);
  let tup1:(i32, f64, u8) = (500, 6.4, 1);
  ```

* **获取tuple元素值**：可以使用模式匹配来**解构**一个tuple来获取元素的值

  ```rust
  let tup = (500, 6.4, 1);
  let (a, b, c) = tup;
  println!("The value of a is: {}", a);
  println!("The value of b is: {}", b);
  println!("The value of c is: {}", c);
  ```

* **访问tuple的元素**：在tuple变量使用点标记法，后接元素的索引号

  ```rust
  println!("The value of tup.0 is: {}", tup.0);
  println!("The value of tup.1 is: {}", tup.1);
  println!("The value of tup.2 is: {}", tup.2);
  ```

##### 4.2.2.2 Array

* 数组中元素类型必须相同

* **声明一个数组**：用中括号声明，逗号分隔

  ```rust
  let arr = [1, 2, 3, 4, 5];
  ```

* **用处**：如果想让数据存放在栈内存上，而不是堆内存上，或者想要保有固定个数的元素，使用数组更好

* 标准库提供了Vector，比数组更加灵活(长度可变)

* **数组的类型**：``[类型; 长度]``

  * 另一种声明数组的方法：**如果数组中每个元素值都相同，可以在中括号中指定值，后接数组长度**

    ```rust
    let a = [3;5];
    // let a = [3,3,3,3,3];
    ```

* **访问数组的元素**：数组是在stack上分配的单个的块内存，可以使用**索引**进行访问

  * **数组越界**：编译通过、运行时报错(``runtime panic``)(**不允许继续访问相应地址的内存，与C/C++不同**)
    * 注：**绝大部分数组越界在编译时就会发现并报错**

### 4.3 函数

* 声明函数使用```fn```关键字
* 针对函数和变量名，Rust采用``snake case``命名规范：所有的字母都是小写的，单词之间使用下划线分开
* 

## TIPS: useful plugins for RUST

* rust analyzer：Run/Debug(当然可以通过命令行)
* Code LLDB：代码调试
* Even Better TOML：编辑.toml文件
* Dependi：管理crates版本

* git-commit-plugin：规范提交信息